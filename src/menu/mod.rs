//! Finite mounted menu trees over intrinsic controls and host-owned top layers.
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::future::Future;
use std::rc::{Rc, Weak};

use glory::render::{
    Action, Change, CheckMode, Direction, Event, EventKind, FocusMode, Layer, LayerPlacement,
    MenuCommand, MenuItemKind, MenuItemSemantics, Navigation, NodeId, Orientation, Patch,
    PointerPhase, PopupKind, Role, SemanticId, Semantics,
};
use glory::style::{Edges, Length};
use glory::ui::IntoChildren;
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};

mod content;
mod search;
use content::Sections;
pub use content::{MenuLabel, menu_label};
pub use search::{MAX_MENU_SEARCH_BYTES, MenuSearch, menu_search};

use crate::appearance::{Common, common_methods, configure, observe};
use crate::collection::{
    Collection, CollectionLimits, CollectionMember, RovingFocus, Typeahead, TypeaheadConfig,
};
use crate::composite::{self, Callback, Observers};
use crate::state::{Control, DelayFactory, Dirty, OwnerDelay, Value};

pub const MAX_MENU_ITEMS: usize = 1024;
pub const MAX_MENU_LEVEL_ITEMS: usize = 256;
pub const MAX_MENU_DEPTH: usize = 8;
type Delay = DelayFactory;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Action,
    Checkbox,
}
pub struct MenuItem {
    common: Common,
    key: String,
    label: Vary<String>,
    kind: ItemKind,
    checked: Value<bool>,
    changed: Option<Box<dyn FnMut(bool)>>,
    activated: Option<Box<dyn FnMut()>>,
    close: bool,
    check_configured: bool,
    sections: Sections,
    visible: Vary<bool>,
}
pub fn menu_item(key: impl Into<String>) -> MenuItem {
    MenuItem {
        common: Common::default(),
        key: key.into(),
        label: "".into(),
        kind: ItemKind::Action,
        checked: Value::default(),
        changed: None,
        activated: None,
        close: true,
        check_configured: false,
        sections: Sections::default(),
        visible: true.into(),
    }
}
pub fn menu_checkbox(key: impl Into<String>) -> MenuItem {
    MenuItem {
        kind: ItemKind::Checkbox,
        close: false,
        ..menu_item(key)
    }
}
impl MenuItem {
    common_methods!();
    pub fn visible(mut self, visible: impl Into<Vary<bool>>) -> Self {
        self.visible = visible.into();
        self
    }
    /// Passive content before the label, for action items.
    pub fn left_section(mut self, children: impl IntoChildren) -> Self {
        self.sections.left = Some(children.into_children());
        self
    }
    /// Passive content after the label, for action items.
    pub fn right_section(mut self, children: impl IntoChildren) -> Self {
        self.sections.right = Some(children.into_children());
        self
    }
    pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.label = value.into();
        self
    }
    pub fn on_activate(mut self, callback: impl FnMut() + 'static) -> Self {
        self.activated = Some(Box::new(callback));
        self
    }
    pub fn close_on_select(mut self, value: bool) -> Self {
        self.close = value;
        self
    }
    pub fn checked(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.checked.source = Some(value.into());
        self.check_configured = true;
        self
    }
    pub fn default_checked(mut self, value: bool) -> Self {
        self.checked.default = value;
        self.check_configured = true;
        self
    }
    pub fn on_checked_change(mut self, callback: impl FnMut(bool) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self.check_configured = true;
        self
    }
}
pub struct MenuRadioItem {
    common: Common,
    key: String,
    label: Vary<String>,
    visible: Vary<bool>,
}
pub fn menu_radio_item(key: impl Into<String>, label: impl Into<Vary<String>>) -> MenuRadioItem {
    MenuRadioItem {
        common: Common::default(),
        key: key.into(),
        label: label.into(),
        visible: true.into(),
    }
}
impl MenuRadioItem {
    common_methods!();
    pub fn visible(mut self, visible: impl Into<Vary<bool>>) -> Self {
        self.visible = visible.into();
        self
    }
}
pub struct MenuRadioGroup {
    key: String,
    label: Vary<String>,
    value: Value<String>,
    changed: Option<Box<dyn FnMut(String)>>,
    items: Vec<MenuRadioItem>,
    close: bool,
}
pub fn menu_radio_group(key: impl Into<String>) -> MenuRadioGroup {
    MenuRadioGroup {
        key: key.into(),
        label: "".into(),
        value: Value::default(),
        changed: None,
        items: Vec::new(),
        close: false,
    }
}
impl MenuRadioGroup {
    pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.label = value.into();
        self
    }
    pub fn value(mut self, value: impl Into<Vary<String>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.value.default = value.into();
        self
    }
    pub fn on_value_change(mut self, callback: impl FnMut(String) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = MenuRadioItem>) -> Self {
        let remaining = (MAX_MENU_LEVEL_ITEMS + 1).saturating_sub(self.items.len());
        self.items.extend(items.into_iter().take(remaining));
        self
    }
    pub fn close_on_select(mut self, value: bool) -> Self {
        self.close = value;
        self
    }
}
pub struct MenuSubmenu {
    common: Common,
    key: String,
    label: Vary<String>,
    editor: Option<Box<MenuSearch>>,
    width: Option<f64>,
    position: Option<glory::render::AnchorPlacement>,
    open: Value<bool>,
    changed: Option<Box<dyn FnMut(bool)>>,
    entries: Vec<MenuEntry>,
    overflow: bool,
    sections: Sections,
    visible: Vary<bool>,
}
pub fn menu_submenu(key: impl Into<String>) -> MenuSubmenu {
    MenuSubmenu {
        editor: None,
        width: None,
        position: None,
        common: Common::default(),
        key: key.into(),
        label: "".into(),
        open: Value::default(),
        changed: None,
        entries: Vec::new(),
        overflow: false,
        sections: Sections::default(),
        visible: true.into(),
    }
}
impl MenuSubmenu {
    common_methods!();
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
    pub fn position(mut self, position: glory::render::AnchorPlacement) -> Self {
        self.position = Some(position);
        self
    }
    pub fn search(mut self, editor: MenuSearch) -> Self {
        self.editor = Some(Box::new(editor));
        self
    }
    pub fn visible(mut self, visible: impl Into<Vary<bool>>) -> Self {
        self.visible = visible.into();
        self
    }
    pub fn left_section(mut self, children: impl IntoChildren) -> Self {
        self.sections.left = Some(children.into_children());
        self
    }
    pub fn right_section(mut self, children: impl IntoChildren) -> Self {
        self.sections.right = Some(children.into_children());
        self
    }
    pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.label = value.into();
        self
    }
    pub fn open(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.open.source = Some(value.into());
        self
    }
    pub fn default_open(mut self, value: bool) -> Self {
        self.open.default = value;
        self
    }
    pub fn on_open_change(mut self, callback: impl FnMut(bool) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn item(mut self, item: impl Into<MenuEntry>) -> Self {
        push_entry(&mut self.entries, &mut self.overflow, item.into());
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        if self.overflow {
            return self;
        }
        for item in items.into_iter().take(MAX_MENU_LEVEL_ITEMS + 1) {
            push_entry(&mut self.entries, &mut self.overflow, MenuEntry::Item(item));
            if self.overflow {
                break;
            }
        }
        self
    }
    pub fn radio_group(self, group: MenuRadioGroup) -> Self {
        self.item(group)
    }
    pub fn submenu(self, submenu: MenuSubmenu) -> Self {
        self.item(submenu)
    }
    pub fn separator(mut self, key: impl Into<String>) -> Self {
        push_entry(
            &mut self.entries,
            &mut self.overflow,
            MenuEntry::Separator(key.into()),
        );
        self
    }
}
fn entry_size(entry: &MenuEntry) -> usize {
    match entry {
        MenuEntry::RadioGroup(group) => group.items.len(),
        MenuEntry::Submenu(menu) => 1 + menu.entries.iter().map(entry_size).sum::<usize>(),
        _ => 1,
    }
}
fn entry_width(entry: &MenuEntry) -> usize {
    match entry {
        MenuEntry::RadioGroup(group) => group.items.len(),
        _ => 1,
    }
}
fn entry_depth(entry: &MenuEntry) -> usize {
    match entry {
        MenuEntry::Submenu(menu) => 1 + menu.entries.iter().map(entry_depth).max().unwrap_or(0),
        _ => 0,
    }
}
fn push_entry(entries: &mut Vec<MenuEntry>, overflow: &mut bool, entry: MenuEntry) {
    if *overflow {
        return;
    }
    let total = entries.iter().map(entry_size).sum::<usize>() + entry_size(&entry);
    let width = entries.iter().map(entry_width).sum::<usize>() + entry_width(&entry);
    if total > MAX_MENU_ITEMS
        || width > MAX_MENU_LEVEL_ITEMS
        || entry_depth(&entry) >= MAX_MENU_DEPTH
        || matches!(&entry, MenuEntry::Submenu(menu) if menu.overflow)
    {
        *overflow = true;
        return;
    }
    entries.push(entry);
}
pub enum MenuEntry {
    Item(MenuItem),
    Label(MenuLabel),
    RadioGroup(MenuRadioGroup),
    Submenu(MenuSubmenu),
    Separator(String),
}
impl From<MenuLabel> for MenuEntry {
    fn from(label: MenuLabel) -> Self {
        Self::Label(label)
    }
}
impl From<MenuItem> for MenuEntry {
    fn from(item: MenuItem) -> Self {
        Self::Item(item)
    }
}
impl From<MenuRadioGroup> for MenuEntry {
    fn from(item: MenuRadioGroup) -> Self {
        Self::RadioGroup(item)
    }
}
impl From<MenuSubmenu> for MenuEntry {
    fn from(item: MenuSubmenu) -> Self {
        Self::Submenu(item)
    }
}

pub struct Menu {
    common: Common,
    key: String,
    label: Vary<String>,
    editor: Option<Box<MenuSearch>>,
    open: Value<bool>,
    changed: Option<Box<dyn FnMut(bool)>>,
    entries: Vec<MenuEntry>,
    overflow: bool,
    anchor: Option<SemanticId>,
    return_focus: Option<SemanticId>,
    direction: Direction,
    looping: bool,
    typeahead: TypeaheadConfig,
    delay: Option<Delay>,
    trigger: Option<Cage<Option<NodeId>>>,
    dropdown: bool,
    position: Option<glory::render::AnchorPlacement>,
    panel_width: Option<f64>,
    context: Option<ContextConfig>,
    bar: Option<bar::BarItem>,
}
pub fn menu(key: impl Into<String>) -> Menu {
    Menu {
        editor: None,
        common: Common::default(),
        key: key.into(),
        label: "Menu".into(),
        open: Value::default(),
        changed: None,
        entries: Vec::new(),
        overflow: false,
        anchor: None,
        return_focus: None,
        direction: Direction::Ltr,
        looping: true,
        typeahead: TypeaheadConfig::default(),
        delay: None,
        trigger: None,
        dropdown: false,
        position: None,
        panel_width: None,
        context: None,
        bar: None,
    }
}
pub struct DropdownMenu {
    inner: Menu,
}
struct ContextConfig {
    target: Box<dyn Widget>,
    name: Vary<String>,
    policy: glory::render::ContextEditPolicy,
    reference: Option<Cage<Option<NodeId>>>,
}
pub struct ContextMenu {
    inner: Menu,
    target: Option<Box<dyn Widget>>,
    target_name: Vary<String>,
    policy: glory::render::ContextEditPolicy,
    reference: Option<Cage<Option<NodeId>>>,
}
pub fn context_menu(key: impl Into<String>) -> ContextMenu {
    ContextMenu {
        inner: menu(key),
        target: None,
        target_name: "Context menu area".into(),
        policy: Default::default(),
        reference: None,
    }
}
pub fn dropdown_menu(key: impl Into<String>) -> DropdownMenu {
    DropdownMenu {
        inner: Menu {
            dropdown: true,
            ..menu(key)
        },
    }
}
impl Menu {
    common_methods!();
    pub fn width(mut self, width: f64) -> Self {
        self.panel_width = Some(width);
        self
    }
    pub fn position(mut self, position: glory::render::AnchorPlacement) -> Self {
        self.position = Some(position);
        self
    }
    pub fn search(mut self, editor: MenuSearch) -> Self {
        self.editor = Some(Box::new(editor));
        self
    }
    pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.label = value.into();
        self
    }
    pub fn open(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.open.source = Some(value.into());
        self
    }
    pub fn default_open(mut self, value: bool) -> Self {
        self.open.default = value;
        self
    }
    pub fn on_open_change(mut self, callback: impl FnMut(bool) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn item(mut self, item: impl Into<MenuEntry>) -> Self {
        push_entry(&mut self.entries, &mut self.overflow, item.into());
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        if self.overflow {
            return self;
        }
        for item in items.into_iter().take(MAX_MENU_LEVEL_ITEMS + 1) {
            push_entry(&mut self.entries, &mut self.overflow, MenuEntry::Item(item));
            if self.overflow {
                break;
            }
        }
        self
    }
    pub fn radio_group(self, group: MenuRadioGroup) -> Self {
        self.item(group)
    }
    pub fn submenu(self, submenu: MenuSubmenu) -> Self {
        self.item(submenu)
    }
    pub fn separator(mut self, key: impl Into<String>) -> Self {
        push_entry(
            &mut self.entries,
            &mut self.overflow,
            MenuEntry::Separator(key.into()),
        );
        self
    }
    pub fn anchor(mut self, id: SemanticId) -> Self {
        self.anchor = Some(id);
        self
    }
    pub fn return_focus(mut self, id: SemanticId) -> Self {
        self.return_focus = Some(id);
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    pub fn loop_focus(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }
    pub fn typeahead(mut self, config: TypeaheadConfig) -> Self {
        self.typeahead = config;
        self
    }
    /// Optional application/host delay. One pending task per level is cancelled
    /// on leave, sibling entry, close and Owner retirement. No global timer.
    pub fn hover_delay_with<F: Fn() -> Fut + 'static, Fut: Future<Output = ()> + 'static>(
        mut self,
        factory: F,
    ) -> Self {
        self.delay = Some(Rc::new(move || Box::pin(factory())));
        self
    }
    pub fn trigger_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.trigger = Some(reference);
        self
    }
}
macro_rules! dropdown_methods {
    ($( $name:ident ( $arg:ident : $ty:ty ) );* $(;)?) => { $(pub fn $name(mut self, $arg: $ty) -> Self { self.inner = self.inner.$name($arg); self })* };
}
impl DropdownMenu {
    dropdown_methods! {
        label(value: impl Into<Vary<String>>); open(value: impl Into<Vary<bool>>); default_open(value: bool);
        on_open_change(callback: impl FnMut(bool) + 'static); item(item: impl Into<MenuEntry>); items(items: impl IntoIterator<Item = MenuItem>);
        radio_group(group: MenuRadioGroup); submenu(submenu: MenuSubmenu); separator(key: impl Into<String>); search(editor: MenuSearch);
        width(width: f64); position(position: glory::render::AnchorPlacement);
        return_focus(id: SemanticId); direction(direction: Direction); loop_focus(looping: bool); typeahead(config: TypeaheadConfig);
        trigger_ref(reference: Cage<Option<NodeId>>); node_ref(reference: Cage<Option<NodeId>>); class(value: impl Into<Vary<String>>);
        name(value: impl Into<Vary<String>>); style(value: impl Into<Vary<glory::style::Style>>); enabled(value: impl Into<Vary<bool>>);
        theme(value: crate::Theme); size(value: crate::Size)
    }
    pub fn unstyled(mut self) -> Self {
        self.inner = self.inner.unstyled();
        self
    }
    pub fn hover_delay_with<F: Fn() -> Fut + 'static, Fut: Future<Output = ()> + 'static>(
        mut self,
        factory: F,
    ) -> Self {
        self.inner = self.inner.hover_delay_with(factory);
        self
    }
    pub fn on(mut self, kind: EventKind, callback: impl FnMut(Event) + 'static) -> Self {
        self.inner = self.inner.on(kind, callback);
        self
    }
}
impl ContextMenu {
    pub fn target(mut self, widget: impl Widget + 'static) -> Self {
        self.target = Some(Box::new(widget));
        self
    }
    pub fn target_name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.target_name = name.into();
        self
    }
    pub fn target_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.reference = Some(reference);
        self
    }
    pub fn editor_policy(mut self, policy: glory::render::ContextEditPolicy) -> Self {
        self.policy = policy;
        self
    }
    dropdown_methods! {
        label(value: impl Into<Vary<String>>); open(value: impl Into<Vary<bool>>);
        on_open_change(callback: impl FnMut(bool) + 'static); item(item: impl Into<MenuEntry>); items(items: impl IntoIterator<Item = MenuItem>);
        radio_group(group: MenuRadioGroup); submenu(submenu: MenuSubmenu); separator(key: impl Into<String>); search(editor: MenuSearch);
        width(width: f64);
        direction(direction: Direction); loop_focus(looping: bool); typeahead(config: TypeaheadConfig);
        node_ref(reference: Cage<Option<NodeId>>); class(value: impl Into<Vary<String>>);
        name(value: impl Into<Vary<String>>); style(value: impl Into<Vary<glory::style::Style>>); enabled(value: impl Into<Vary<bool>>);
        theme(value: crate::Theme); size(value: crate::Size)
    }
    pub fn unstyled(mut self) -> Self {
        self.inner = self.inner.unstyled();
        self
    }
    pub fn hover_delay_with<F: Fn() -> Fut + 'static, Fut: Future<Output = ()> + 'static>(
        mut self,
        factory: F,
    ) -> Self {
        self.inner = self.inner.hover_delay_with(factory);
        self
    }
    pub fn on(mut self, kind: EventKind, callback: impl FnMut(Event) + 'static) -> Self {
        self.inner = self.inner.on(kind, callback);
        self
    }
}
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn validate_width(width: Option<f64>) -> Result {
    if width.is_some_and(|width| !width.is_finite() || !(80.0..=1000.0).contains(&width)) {
        return Err(invalid(
            "menu dropdown widths require finite 80..=1000 CSS pixels",
        ));
    }
    Ok(())
}
fn id(namespace: &str, path: &[String], part: &str) -> Result<SemanticId> {
    let mut value = format!("sapid-menu-{}_{namespace}-{part}", namespace.len());
    for key in path {
        value.push_str(&format!("-{}_{key}", key.len()));
    }
    Ok(SemanticId::new(value)?)
}
fn key(value: &str) -> Result {
    SemanticId::new(value)?;
    if value.len() > 64 {
        return Err(invalid("menu keys exceed 64 bytes"));
    }
    Ok(())
}
fn validate(
    namespace: &str,
    entries: &[MenuEntry],
    path: &mut Vec<String>,
    total: &mut usize,
) -> Result {
    if path.len() >= MAX_MENU_DEPTH || entries.is_empty() {
        return Err(invalid(
            "menu requires nonempty levels within its depth budget",
        ));
    }
    let mut seen = HashSet::new();
    let mut count = 0;
    let mut open_submenu = false;
    for entry in entries {
        let (name, members) = match entry {
            MenuEntry::Item(item) => (&item.key, 1),
            MenuEntry::Label(label) => (&label.key, 1),
            MenuEntry::RadioGroup(group) => (&group.key, group.items.len()),
            MenuEntry::Submenu(menu) => (&menu.key, 1),
            MenuEntry::Separator(key) => (key, 1),
        };
        key(name)?;
        if !seen.insert(name) {
            return Err(invalid("duplicate menu level key"));
        }
        path.push(name.clone());
        id(namespace, path, "item")?;
        id(namespace, path, "panel")?;
        count += members;
        *total += members;
        if count > MAX_MENU_LEVEL_ITEMS || *total > MAX_MENU_ITEMS {
            return Err(invalid("menu mounted item budget exhausted"));
        }
        match entry {
            MenuEntry::Item(item)
                if item.common.focus != FocusMode::Auto
                    || item.kind == ItemKind::Action && item.check_configured
                    || item.kind != ItemKind::Action && !item.sections.is_empty() =>
            {
                return Err(invalid(
                    "menu owns focus; checkbox state needs a checkbox item and passive slots need an action item",
                ));
            }
            MenuEntry::Label(label) if label.common.focus != FocusMode::Auto => {
                return Err(invalid("menu labels are passive captions"));
            }
            MenuEntry::Submenu(menu) => {
                validate_width(menu.width)?;
                if let Some(position) = menu.position {
                    position.validate()?;
                }
                if let Some(editor) = &menu.editor {
                    editor.validate()?;
                }
                let open = match &menu.open.source {
                    Some(source) => source.try_get()?,
                    None => menu.open.default,
                };
                if open && open_submenu {
                    return Err(invalid(
                        "only one submenu at each menu level may initially be open",
                    ));
                }
                open_submenu |= open;
                if menu.overflow {
                    return Err(invalid("submenu configuration budget exhausted"));
                }
                if menu.common.focus != FocusMode::Auto {
                    return Err(invalid("menu owns submenu trigger focus"));
                }
                validate(namespace, &menu.entries, path, total)?;
            }
            MenuEntry::RadioGroup(group) => {
                if group.items.is_empty() {
                    return Err(invalid("menu radio group is empty"));
                }
                let mut radios = HashSet::new();
                for item in &group.items {
                    key(&item.key)?;
                    if !radios.insert(&item.key)
                        || !seen.insert(&item.key)
                        || item.common.focus != FocusMode::Auto
                    {
                        return Err(invalid(
                            "menu radio keys are duplicate or focus is overridden",
                        ));
                    }
                }
            }
            _ => {}
        }
        path.pop();
    }
    Ok(())
}

struct Child {
    trigger: NodeId,
    state: Rc<MenuState>,
}
struct MenuState {
    search_editor: Option<Rc<search::SearchState>>,
    activations: RefCell<Vec<search::Activation>>,
    bar: Option<(String, Weak<bar::BarState>)>,
    open: Control<bool>,
    changed: Callback<bool>,
    visible: Vary<bool>,
    enabled: Vary<bool>,
    collection: Collection,
    roving: RovingFocus,
    search: RefCell<Typeahead>,
    active_child: Cage<Option<NodeId>>,
    children: RefCell<Vec<Child>>,
    parent: Weak<MenuState>,
    root: Weak<MenuState>,
    panel: Cage<Option<NodeId>>,
    trigger: Cage<Option<NodeId>>,
    owner: glory::core::InstanceId,
    pending: Rc<OwnerDelay>,
    hovered: Cell<Option<NodeId>>,
    // One current keyboard/hover focus intent, delivered after visible bindings.
    focus_request: Cage<Option<NodeId>>,
    delay: Option<Delay>,
}
impl MenuState {
    fn cancel_pending(&self) {
        self.pending.cancel();
        self.hovered.set(None);
    }
    fn request(&self, open: bool) -> Result {
        if !self.owner.is_alive() {
            return Ok(());
        }
        if !open {
            // Closing/Tab supersedes an older navigation intent even when the
            // application refuses the controlled close.
            if self.focus_request.try_get()?.is_some() {
                self.focus_request.try_revise(|pending| *pending = None)?;
            }
            self.cancel_pending();
            self.close_children(None)?;
        }
        if !self.owner.is_alive() {
            return Ok(());
        }
        if self.open.source.try_get()? != open {
            self.open.accept(&open)?;
            self.changed.emit(self.owner, open);
        }
        Ok(())
    }
    fn children(&self) -> Vec<(NodeId, Rc<MenuState>)> {
        self.children
            .borrow()
            .iter()
            .map(|child| (child.trigger, child.state.clone()))
            .collect()
    }
    fn close_children(&self, keep: Option<NodeId>) -> Result {
        for (trigger, child) in self.children() {
            if Some(trigger) != keep {
                child.request(false)?;
                if !self.owner.is_alive() {
                    break;
                }
            }
        }
        Ok(())
    }
    fn root_close(&self) -> Result {
        if let Some(root) = self.root.upgrade() {
            root.request(false)?;
        }
        Ok(())
    }
    fn focus_item(&self, _scope: &Scope, item: crate::MountedItem) -> Result {
        self.roving.accept(&item)?;
        if let Some(editor) = &self.search_editor {
            editor
                .highlighted
                .try_revise(|current| *current = Some(item.node))?;
            self.focus_request
                .try_revise(|pending| *pending = editor.editor.get())?;
            return Ok(());
        }
        self.focus_request
            .try_revise(|pending| *pending = Some(item.node))?;
        Ok(())
    }
    fn boundary(&self, scope: &Scope, last: bool) -> Result {
        let items = self.collection.mounted()?;
        let item = if last {
            items.into_iter().rev().find(|item| item.enabled)
        } else {
            items.into_iter().find(|item| item.enabled)
        };
        if let Some(item) = item {
            self.focus_item(scope, item)?;
        }
        Ok(())
    }
    fn open_child(&self, scope: &Scope, from: NodeId, focus: bool) -> Result {
        self.close_children(Some(from))?;
        if !self.owner.is_alive() {
            return Ok(());
        }
        let children = self.children();
        if children
            .iter()
            .any(|(trigger, child)| *trigger != from && child.open.source.get())
        {
            return Ok(());
        }
        if let Some((_, child)) = children.into_iter().find(|(trigger, _)| *trigger == from) {
            child.request(true)?;
            if focus && child.owner.is_alive() {
                child.boundary(scope, false)?;
            }
        }
        Ok(())
    }
    fn navigate(&self, scope: &Scope, from: NodeId, command: Navigation) -> Result {
        if !self.visible.try_get()? {
            return Ok(());
        }
        self.cancel_pending();
        self.search.borrow_mut().clear();
        if let Some(item) = self.roving.destination(from, command)? {
            self.close_children(None)?;
            if self.owner.is_alive() {
                self.focus_item(scope, item)?;
            }
        }
        Ok(())
    }
    fn command(&self, scope: &Scope, from: NodeId, command: MenuCommand) -> Result {
        if !self.owner.is_alive() || !self.enabled.try_get()? {
            return Ok(());
        }
        if matches!(
            command,
            MenuCommand::SearchNext | MenuCommand::SearchPrevious | MenuCommand::SearchCommit
        ) {
            return self.search_command(scope, from, command);
        }
        self.cancel_pending();
        match command {
            MenuCommand::First | MenuCommand::Last => {
                if let Some((key, bar)) = &self.bar
                    && let Some(bar) = bar.upgrade()
                {
                    return bar.open_from_trigger(scope, key, command == MenuCommand::Last);
                }
                self.request(true)?;
                if self.owner.is_alive() {
                    self.boundary(scope, command == MenuCommand::Last)?;
                }
            }
            MenuCommand::Open if self.visible.try_get()? => {
                if !self.children().iter().any(|(trigger, _)| *trigger == from)
                    && let Some((key, bar)) = &self.bar
                    && let Some(bar) = bar.upgrade()
                {
                    bar.move_from_menu(scope, key, true)?;
                } else {
                    self.open_child(scope, from, true)?;
                }
            }
            MenuCommand::Close if self.visible.try_get()? => {
                if let Some((key, bar)) = &self.bar
                    && let Some(bar) = bar.upgrade()
                {
                    return bar.move_from_menu(scope, key, false);
                }
                self.request(false)?;
                if self.parent.upgrade().is_some() && self.owner.is_alive() {
                    composite::focus(scope, self.trigger)?;
                }
            }
            MenuCommand::Exit { backwards } => {
                if let Some(root) = self.root.upgrade() {
                    root.request(false)?;
                    if root.owner.is_alive()
                        && scope.capabilities()?.focus
                        && let Some(node) = root.panel.try_get()?
                    {
                        scope.action(Action::ExitMenu { node, backwards })?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn typeahead(&self, scope: &Scope, from: NodeId, text: &str, at_millis: u64) -> Result {
        if self.search_editor.is_some() {
            return Ok(());
        }
        if !self.visible.try_get()? {
            return Ok(());
        }
        self.cancel_pending();
        let item = self
            .search
            .borrow_mut()
            .search(&self.collection, from, text, at_millis)?;
        if let Some(item) = item {
            self.close_children(None)?;
            if self.owner.is_alive() {
                self.focus_item(scope, item)?;
            }
        }
        Ok(())
    }
    fn pointer(self: &Rc<Self>, scope: &Scope, from: NodeId, phase: PointerPhase) -> Result {
        if phase == PointerPhase::Enter {
            return Ok(());
        }
        if phase == PointerPhase::Move && self.hovered.get() == Some(from) {
            return Ok(());
        }
        self.cancel_pending();
        if phase != PointerPhase::Move || !self.visible.try_get()? {
            return Ok(());
        }
        self.hovered.set(Some(from));
        let Some(item) = self
            .collection
            .mounted()?
            .into_iter()
            .find(|item| item.node == from && item.enabled)
        else {
            return Ok(());
        };
        self.close_children(Some(from))?;
        if !self.owner.is_alive() {
            return Ok(());
        }
        self.focus_item(scope, item)?;
        if !self
            .children
            .borrow()
            .iter()
            .any(|child| child.trigger == from)
        {
            return Ok(());
        }
        let state = Rc::downgrade(self);
        let weak = scope.downgrade();
        self.pending.run(scope, self.delay.as_ref(), move || {
            if let (Some(state), Some(scope)) = (state.upgrade(), weak.upgrade())
                && state.visible.try_get()?
            {
                state.open_child(&scope, from, false)?;
            }
            Ok(())
        })
    }
}

struct Level {
    editor: Option<Box<MenuSearch>>,
    bar: Option<(String, Weak<bar::BarState>)>,
    key: String,
    path: Vec<String>,
    common: Common,
    label: Vary<String>,
    open: Value<bool>,
    changed: Option<Box<dyn FnMut(bool)>>,
    entries: Vec<MenuEntry>,
    anchor: SemanticId,
    return_focus: Option<SemanticId>,
    placement: Vary<LayerPlacement>,
    parent: Weak<MenuState>,
    root: Weak<MenuState>,
    parent_visible: Vary<bool>,
    root_enabled: Vary<bool>,
    direction: Direction,
    looping: bool,
    typeahead: TypeaheadConfig,
    delay: Option<Delay>,
    trigger: Cage<Option<NodeId>>,
}
impl Level {
    fn build(mut self, scope: &Scope) -> Result<Rc<MenuState>> {
        let mut search_config = self.editor.take();
        let search_editor = search_config
            .as_mut()
            .map(|editor| editor.prepare(scope, &self.key, &self.path))
            .transpose()?;
        let panel_id = id(&self.key, &self.path, "panel")?;
        let panel = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let control = self.open.mount(scope);
        let source = control.source.clone();
        let parent = self.parent_visible;
        let root_enabled = self.root_enabled;
        let own_enabled = self.common.enabled.clone();
        let enabled: Vary<bool> = scope
            .bond(move || root_enabled.get() && own_enabled.get())
            .into();
        let available = enabled.clone();
        let visible: Vary<bool> = scope
            .bond(move || source.get() && parent.get() && available.get())
            .into();
        let collection = Collection::new(
            scope,
            CollectionLimits {
                items: MAX_MENU_LEVEL_ITEMS,
                ..Default::default()
            },
        )?;
        let roving = RovingFocus::new(scope, collection.clone(), self.looping)?;
        let search = Typeahead::new(self.typeahead)?;
        let pending = OwnerDelay::new(scope)?;
        let state = Rc::new_cyclic(|this| MenuState {
            search_editor,
            activations: RefCell::new(Vec::new()),
            bar: self.bar,
            open: control,
            changed: Callback::new(self.changed.take()),
            visible: visible.clone(),
            enabled: enabled.clone(),
            collection,
            roving,
            search: RefCell::new(search),
            active_child: scope.cage(None),
            children: RefCell::new(Vec::new()),
            parent: self.parent.clone(),
            root: if self.root.strong_count() == 0 {
                this.clone()
            } else {
                self.root.clone()
            },
            panel,
            trigger: self.trigger,
            owner: scope.id(),
            pending,
            hovered: Cell::new(None),
            focus_request: scope.cage(None),
            delay: self.delay.clone(),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = cleanup.upgrade() {
                state.cancel_pending();
            }
        })?;
        let open = visible.clone();
        let direction = self.direction;
        let placement = self.placement;
        let config = Layer {
            sheet: None,
            modal: false,
            escape: true,
            outside: true,
            focus: glory::render::LayerFocus::Manage,
            placement: LayerPlacement::Anchor(glory::render::AnchorPlacement::new(
                glory::render::AnchorSide::Bottom,
                glory::render::AnchorAlign::Start,
            )),
            anchor: Some(self.anchor),
            initial_focus: state.search_editor.as_ref().map(|editor| editor.id.clone()),
            return_focus: self.return_focus,
        };
        let stable_id = panel_id.clone();
        let typeahead = state.search_editor.is_none();
        let panel_name = self.common.name.clone().unwrap_or(self.label.clone());
        let mut panel_widget = configure!(
            self.common,
            glory::ui::view().role(Role::Menu),
            scope,
            "sapid-menu sapid-layer",
            menu_style(&self.common)
        )
        .name(panel_name)
        .node_ref(panel)
        .focus(FocusMode::Programmatic)
        .semantics(scope.bond(move || {
            let mut config = config.clone();
            config.placement = placement.get();
            composite::metadata(Semantics {
                id: Some(stable_id.clone()),
                layer: Some(config),
                hidden: !open.get(),
                navigation: Some(Orientation::Vertical),
                navigation_direction: direction,
                typeahead,
                ..Default::default()
            })
        }));
        for kind in [
            EventKind::Navigate,
            EventKind::Menu,
            EventKind::Typeahead,
            EventKind::Dismiss,
        ] {
            let live = state.clone();
            let weak = scope.downgrade();
            let owner = scope.id();
            let mut observers = self.common.handlers.remove(&kind).unwrap_or_default();
            panel_widget = panel_widget.on(kind, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = match &event {
                    Event::Navigate { from, command } => live.navigate(&scope, *from, *command),
                    Event::Menu { from, command } => live.command(&scope, *from, *command),
                    Event::Typeahead {
                        from,
                        text,
                        at_millis,
                    } => live.typeahead(&scope, *from, text, *at_millis),
                    Event::Dismiss(_) => live.request(false),
                    _ => Ok(()),
                };
                if let Err(error) = result {
                    scope.report_error(error);
                    return;
                }
                observe(&mut observers, owner, event);
            });
        }
        for (kind, mut observers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            panel_widget =
                panel_widget.on(kind, move |event| observe(&mut observers, owner, event));
        }
        panel_widget.show_in(scope)?;
        let panel_node = panel.get().ok_or(glory::ui::Error::Gone)?;
        let content = scope.child_in(panel_node)?;
        if let Some(editor) = search_config {
            (*editor).show(&content, &self.common, panel_id.clone(), state.clone())?;
        }
        for entry in self.entries {
            match entry {
                MenuEntry::Label(label) => {
                    content::build_label(&content, &self.common, &self.key, &self.path, label)?
                }
                MenuEntry::Item(item) => build_item(
                    &content,
                    &state,
                    &self.common,
                    &self.key,
                    &self.path,
                    &panel_id,
                    item,
                )?,
                MenuEntry::Separator(key) => {
                    let mut path = self.path.clone();
                    path.push(key);
                    glory::ui::view()
                        .role(Role::Separator)
                        .style(if self.common.unstyled {
                            glory::style::Style::new()
                        } else {
                            glory::style::Style {
                                border_color: Some(self.common.theme.border),
                                ..glory::style::Style::new()
                                    .border_style(glory::style::BorderStyle::Solid)
                                    .border_width(Edges {
                                        top: 1.,
                                        bottom: 0.,
                                        left: 0.,
                                        right: 0.,
                                    })
                                    .margin(Edges {
                                        top: 4.,
                                        bottom: 4.,
                                        left: 0.,
                                        right: 0.,
                                    })
                            }
                        })
                        .class(if self.common.unstyled {
                            "sapid-menu-separator sapid-unstyled"
                        } else {
                            "sapid-menu-separator"
                        })
                        .semantics(composite::metadata(Semantics {
                            id: Some(id(&self.key, &path, "item")?),
                            ..Default::default()
                        }))
                        .show_in(&content)?;
                }
                MenuEntry::RadioGroup(group) => build_radios(
                    &content,
                    &state,
                    &self.common,
                    &self.key,
                    &self.path,
                    &panel_id,
                    group,
                )?,
                MenuEntry::Submenu(submenu) => {
                    let mut path = self.path.clone();
                    path.push(submenu.key.clone());
                    let trigger_id = id(&self.key, &path, "item")?;
                    let trigger = submenu
                        .common
                        .node_ref
                        .unwrap_or_else(|| content.cage(None));
                    let child_owner = content.child()?;
                    let child_label = submenu.label.clone();
                    let own_enabled = content::filter_enabled(
                        &content,
                        submenu.common.enabled.clone(),
                        submenu.visible.clone(),
                    );
                    let own_visible = submenu.visible.clone();
                    let parent_visible = visible.clone();
                    let active = state.active_child;
                    let child_visible: Vary<bool> = content
                        .bond(move || {
                            parent_visible.get()
                                && own_visible.get()
                                && trigger.get().is_some_and(|node| active.get() == Some(node))
                        })
                        .into();
                    let child = Level {
                        editor: submenu.editor,
                        bar: None,
                        key: self.key.clone(),
                        path: path.clone(),
                        common: Common {
                            theme: self.common.theme.clone(),
                            size: self.common.size,
                            unstyled: self.common.unstyled || submenu.common.unstyled,
                            style: submenu
                                .width
                                .map(|width| glory::style::Style::new().width(Length::Px(width)))
                                .unwrap_or_default()
                                .into(),
                            ..Default::default()
                        },
                        label: child_label.clone(),
                        open: submenu.open,
                        changed: submenu.changed,
                        entries: submenu.entries,
                        anchor: trigger_id.clone(),
                        return_focus: Some(trigger_id.clone()),
                        placement: (if let Some(position) = submenu.position {
                            LayerPlacement::Anchor(position)
                        } else if direction == Direction::Rtl {
                            LayerPlacement::Anchor(
                                glory::render::AnchorPlacement::new(
                                    glory::render::AnchorSide::Left,
                                    glory::render::AnchorAlign::Start,
                                )
                                .gap(4.0),
                            )
                        } else {
                            LayerPlacement::Anchor(
                                glory::render::AnchorPlacement::new(
                                    glory::render::AnchorSide::Right,
                                    glory::render::AnchorAlign::Start,
                                )
                                .gap(4.0),
                            )
                        })
                        .into(),
                        parent: Rc::downgrade(&state),
                        root: state.root.clone(),
                        parent_visible: child_visible,
                        root_enabled: enabled.clone(),
                        direction,
                        looping: self.looping,
                        typeahead: self.typeahead,
                        delay: self.delay.clone(),
                        trigger,
                    }
                    .build(&child_owner)?;
                    content.retain(child_owner)?;
                    let child_panel = id(&self.key, &path, "panel")?;
                    let expanded = child.visible.clone();
                    let mut sections = submenu.sections;
                    if sections.right.is_none() {
                        sections.right = Some(
                            crate::input_actions::glyph(
                                match submenu.position.map(|position| position.side) {
                                    Some(glory::render::AnchorSide::Left) => {
                                        crate::Icon::ChevronLeft
                                    }
                                    Some(glory::render::AnchorSide::Right) => {
                                        crate::Icon::ChevronRight
                                    }
                                    Some(glory::render::AnchorSide::Top) => crate::Icon::ChevronUp,
                                    Some(glory::render::AnchorSide::Bottom) => {
                                        crate::Icon::ChevronDown
                                    }
                                    None if direction == Direction::Rtl => crate::Icon::ChevronLeft,
                                    None => crate::Icon::ChevronRight,
                                },
                                self.common.theme.foreground,
                            )
                            .class("sapid-menu-chevron")
                            .into_children(),
                        );
                    }
                    let native = sections
                        .decorate(glory::ui::button().node_ref(trigger), child_label.clone());
                    let callback = child.clone();
                    let live = state.clone();
                    let weak = content.downgrade();
                    let mut common = submenu.common;
                    common.enabled = own_enabled.clone();
                    common.theme = self.common.theme.clone();
                    common.size = self.common.size;
                    common.unstyled |= self.common.unstyled;
                    let mut observers = common
                        .handlers
                        .remove(&EventKind::Activate)
                        .unwrap_or_default();
                    let instance = scope.id();
                    let trigger_scope = content::member_scope(&content, submenu.visible)?;
                    search::highlight_style(&trigger_scope, &state, &mut common, trigger);
                    let native = configure!(
                        common,
                        native,
                        &trigger_scope,
                        "sapid-menu-item sapid-menu-submenu-trigger",
                        menu_item_style(&self.common)
                    );
                    let item = mount_member(
                        &trigger_scope,
                        &state,
                        native,
                        &self.common,
                        &mut common,
                        &submenu.key,
                        child_label,
                        trigger,
                        own_enabled,
                        Semantics {
                            id: Some(trigger_id),
                            menu_item: Some(MenuItemSemantics {
                                kind: MenuItemKind::Action,
                                owner: panel_id.clone(),
                            }),
                            popup: Some(PopupKind::Menu),
                            controls: Some(child_panel),
                            ..Default::default()
                        },
                        Some(expanded),
                        None,
                        move |event| {
                            let Some(scope) = weak.upgrade() else { return };
                            if !live.visible.get() {
                                return;
                            }
                            let result = if callback.open.source.get() {
                                callback.request(false)
                            } else {
                                live.open_child(&scope, trigger.get().unwrap_or(panel_node), true)
                            };
                            if let Err(error) = result {
                                scope.report_error(error);
                                return;
                            }
                            observe(&mut observers, instance, event);
                        },
                    )?;
                    content.retain(trigger_scope)?;
                    state.children.borrow_mut().push(Child {
                        trigger: item,
                        state: child,
                    });
                }
            }
        }
        scope.retain(content)?;
        state.roving.repair(None)?;
        let live = state.clone();
        let weak = scope.downgrade();
        scope.structural(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let mut accepted = None;
                for (trigger, child) in live.children() {
                    if child.open.source.try_get()? {
                        if accepted.is_some() {
                            return Err(invalid("only one submenu at each menu level may be open"));
                        }
                        accepted = Some(trigger);
                    }
                }
                if live.active_child.try_get()? != accepted {
                    live.active_child.try_revise(|active| *active = accepted)?;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        let live = state.clone();
        let weak = scope.downgrade();
        scope.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                if !live.enabled.try_get()? {
                    live.open.accept(&false)?;
                }
                let visible = live.visible.try_get()?;
                live.roving.repair(None)?;
                if let Some(editor) = &live.search_editor
                    && let Some(current) = editor.highlighted.try_get()?
                {
                    let items = live.collection.mounted()?;
                    if !visible
                        || !items
                            .iter()
                            .any(|item| item.node == current && item.enabled)
                    {
                        editor.highlighted.try_revise(|value| *value = None)?;
                    }
                }
                let requested = live.focus_request.try_get()?;
                if requested.is_some() {
                    live.focus_request.try_revise(|pending| *pending = None)?;
                }
                if !visible {
                    live.cancel_pending();
                } else if let Some(target) = requested {
                    if live
                        .search_editor
                        .as_ref()
                        .is_some_and(|editor| editor.editor.get() == Some(target))
                    {
                        if scope.is_active(target)? {
                            composite::focus_node(&scope, Some(target))?;
                        }
                        return Ok(());
                    }
                    let items = live.collection.mounted()?;
                    let current = live.roving.current()?;
                    let target = items
                        .iter()
                        .find(|item| item.enabled && item.node == target)
                        .or_else(|| {
                            items
                                .iter()
                                .find(|item| item.enabled && Some(&item.member) == current.as_ref())
                        });
                    if let Some(item) = target
                        && scope.is_active(item.node)?
                    {
                        composite::focus_node(&scope, Some(item.node))?;
                        if scope.capabilities()?.scroll {
                            scope.action(Action::ScrollIntoView(item.node))?;
                        }
                    }
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        if let Some(editor) = &state.search_editor {
            let editor = editor.clone();
            let live = state.clone();
            let weak = scope.downgrade();
            let mut was_visible = visible.get();
            scope.effect(move || {
                let Some(scope) = weak.upgrade() else { return };
                let visible = live.visible.get();
                let closing = was_visible && !visible;
                was_visible = visible;
                if closing && let Err(error) = editor.close(live.owner) {
                    scope.report_error(error);
                }
            });
        }
        Ok(state)
    }
}
fn menu_item_style(common: &Common) -> glory::style::Style {
    let mut style = common
        .theme
        .control(common.size)
        .border_width(Edges::all(0.0))
        .min_height(Length::Px(32.0))
        .padding(Edges {
            top: 5.0,
            bottom: 5.0,
            left: 9.0,
            right: 9.0,
        })
        .display(glory::style::Display::InlineFlex)
        .width(Length::Percent(100.))
        .gap(8.);
    style.align = Some(glory::style::Align::Center);
    style.justify = Some(glory::style::Justify::Start);
    style
}
fn menu_style(common: &Common) -> glory::style::Style {
    let mut style = composite::column(2.0)
        .width(Length::Px(220.0))
        .padding(Edges::all(5.0))
        .radius(common.theme.radius);
    style.background = Some(common.theme.surface);
    style.color = Some(common.theme.foreground);
    style.border_width = Some(Edges::all(1.));
    style.border_style = Some(glory::style::BorderStyle::Solid);
    style.border_color = Some(common.theme.border);
    style
}

/// Construct the intrinsic control in its own mounted Owner, then register it.
/// Both Button and Check expose the same typed member contract.
#[allow(clippy::too_many_arguments)]
fn mount_member<W: MenuControl>(
    scope: &Scope,
    state: &Rc<MenuState>,
    control: W,
    parent: &Common,
    common: &mut Common,
    key: &str,
    label: Vary<String>,
    reference: Cage<Option<NodeId>>,
    own_enabled: Vary<bool>,
    mut semantics: Semantics,
    expanded: Option<Vary<bool>>,
    checked: Option<Vary<bool>>,
    activation: impl FnMut(Event) + 'static,
) -> Result<NodeId> {
    common.theme = parent.theme.clone();
    common.size = parent.size;
    common.unstyled |= parent.unstyled;
    let owner = scope.child()?;
    let token = owner.cage(None::<CollectionMember>);
    let roving = state.roving.clone();
    let searchable = state.search_editor.is_some();
    let focus = owner.bond(move || {
        if !searchable
            && token
                .get()
                .is_some_and(|member| roving.is_tab_stop(&member).unwrap_or(false))
        {
            FocusMode::TabStop
        } else {
            FocusMode::Programmatic
        }
    });
    let root = state.enabled.clone();
    let flag = own_enabled;
    let enabled: Vary<bool> = owner.bond(move || root.get() && flag.get()).into();
    semantics.navigation_item = true;
    let declaration = if searchable {
        Some((
            semantics
                .id
                .clone()
                .ok_or(invalid("menu search actions need stable names"))?,
            semantics
                .menu_item
                .as_ref()
                .ok_or(invalid("menu search actions need item metadata"))?
                .kind,
        ))
    } else {
        None
    };
    let metadata = owner.bond(move || {
        let mut metadata = semantics.clone();
        if let Some(expanded) = &expanded {
            metadata.expanded = Some(expanded.get());
        }
        composite::metadata(metadata)
    });
    let available = enabled.clone();
    let identity = owner.id();
    let mut activation = activation;
    let guarded = move |event| {
        if identity.is_alive() && available.get() {
            activation(event);
        }
    };
    let mut guarded = Some(guarded);
    let dispatch = searchable.then(|| search::handler(guarded.take().expect("one action handler")));
    let control = if let Some(dispatch) = &dispatch {
        let callback = dispatch.clone();
        control.member(
            focus.into(),
            enabled.clone(),
            metadata.into(),
            move |event| callback.borrow_mut()(event),
        )
    } else {
        control.member(
            focus.into(),
            enabled.clone(),
            metadata.into(),
            guarded.take().expect("one action handler"),
        )
    };
    let mounted = control.mount(&owner)?;
    let member = state
        .collection
        .register(mounted.scope(), key, reference, enabled, label)?;
    token.revise(|token| *token = Some(member.clone()));
    let node = reference.get().ok_or(glory::ui::Error::Gone)?;
    if let Some((name, kind)) = declaration {
        state.activations.borrow_mut().push(search::Activation {
            node,
            name,
            kind,
            checked,
            callback: Rc::downgrade(dispatch.as_ref().expect("search action handler")),
        });
        let weak = Rc::downgrade(state);
        owner.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                state
                    .activations
                    .borrow_mut()
                    .retain(|item| item.node != node);
            }
        })?;
    }
    let mut pointer_observers = common
        .handlers
        .remove(&EventKind::Pointer)
        .unwrap_or_default();
    let instance = owner.id();
    owner.on(node, EventKind::Pointer, move |event| {
        observe(&mut pointer_observers, instance, event);
    })?;
    // Passive icons and labels have their own origins. Movement belongs to
    // this member; only crossing its actual boundary cancels a hover task.
    let live = state.clone();
    let weak = owner.downgrade();
    let mut boundary_observers = common
        .handlers
        .remove(&EventKind::PointerWithin)
        .unwrap_or_default();
    owner.on(node, EventKind::PointerWithin, move |event| {
        let Some(scope) = weak.upgrade() else { return };
        if let Event::PointerWithin { phase, .. } = &event
            && matches!(phase, PointerPhase::Move | PointerPhase::Leave)
            && let Err(error) = live.pointer(&scope, node, *phase)
        {
            scope.report_error(error);
            return;
        }
        observe(&mut boundary_observers, instance, event);
    })?;
    let roving = state.roving.clone();
    let weak = owner.downgrade();
    let mut focus_observers = common
        .handlers
        .remove(&EventKind::FocusChanged)
        .unwrap_or_default();
    owner.on(node, EventKind::FocusChanged, move |event| {
        let Some(scope) = weak.upgrade() else { return };
        if event == Event::FocusChanged(true)
            && let Err(error) = roving.observe(&member)
        {
            scope.report_error(error);
            return;
        }
        observe(&mut focus_observers, instance, event);
    })?;
    for (kind, mut observers) in std::mem::take(&mut common.handlers) {
        let live = state.clone();
        let weak = owner.downgrade();
        owner.on(node, kind, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            let result = match &event {
                Event::Navigate { from, command } => live.navigate(&scope, *from, *command),
                Event::Menu { from, command } => live.command(&scope, *from, *command),
                Event::Typeahead {
                    from,
                    text,
                    at_millis,
                } => live.typeahead(&scope, *from, text, *at_millis),
                _ => Ok(()),
            };
            if let Err(error) = result {
                scope.report_error(error);
                return;
            }
            observe(&mut observers, instance, event);
        })?;
    }
    owner.retain(mounted)?;
    scope.retain(owner)?;
    Ok(node)
}
trait MenuControl: Widget + Sized {
    fn member(
        self,
        focus: Vary<FocusMode>,
        enabled: Vary<bool>,
        semantics: Vary<Option<Box<Semantics>>>,
        activation: impl FnMut(Event) + 'static,
    ) -> Self;
}
impl MenuControl for glory::ui::Button {
    fn member(
        self,
        focus: Vary<FocusMode>,
        enabled: Vary<bool>,
        semantics: Vary<Option<Box<Semantics>>>,
        activation: impl FnMut(Event) + 'static,
    ) -> Self {
        self.focus(focus)
            .enabled(enabled)
            .semantics(semantics)
            .on(EventKind::Activate, activation)
    }
}
impl MenuControl for glory::ui::Check {
    fn member(
        self,
        focus: Vary<FocusMode>,
        enabled: Vary<bool>,
        semantics: Vary<Option<Box<Semantics>>>,
        activation: impl FnMut(Event) + 'static,
    ) -> Self {
        self.focus(focus)
            .enabled(enabled)
            .semantics(semantics)
            .on(EventKind::CheckedChanged, activation)
    }
}

fn build_item(
    scope: &Scope,
    state: &Rc<MenuState>,
    parent: &Common,
    namespace: &str,
    path: &[String],
    menu_id: &SemanticId,
    mut item: MenuItem,
) -> Result {
    let owner = content::member_scope(scope, item.visible.clone())?;
    item.common.enabled =
        content::filter_enabled(&owner, item.common.enabled.clone(), item.visible);
    let reference = item.common.node_ref.unwrap_or_else(|| owner.cage(None));
    let enabled = item.common.enabled.clone();
    item.common.theme = parent.theme.clone();
    item.common.size = parent.size;
    item.common.unstyled |= parent.unstyled;
    search::highlight_style(&owner, state, &mut item.common, reference);
    let mut path = path.to_vec();
    path.push(item.key.clone());
    let semantics = Semantics {
        id: Some(id(namespace, &path, "item")?),
        menu_item: Some(MenuItemSemantics {
            kind: if item.kind == ItemKind::Action {
                MenuItemKind::Action
            } else {
                MenuItemKind::Checkbox
            },
            owner: menu_id.clone(),
        }),
        ..Default::default()
    };
    let callback = Callback::new(
        item.activated
            .take()
            .map(|mut callback| Box::new(move |()| callback()) as Box<dyn FnMut(())>),
    );
    let live = state.clone();
    let close = item.close;
    let instance = owner.id();
    let weak = owner.downgrade();
    if item.kind == ItemKind::Action {
        let native = item
            .sections
            .decorate(glory::ui::button().node_ref(reference), item.label.clone());
        let native = configure!(
            item.common,
            native,
            &owner,
            "sapid-menu-item",
            menu_item_style(parent)
        );
        let mut observers = item
            .common
            .handlers
            .remove(&EventKind::Activate)
            .unwrap_or_default();
        mount_member(
            &owner,
            state,
            native,
            parent,
            &mut item.common,
            &item.key,
            item.label,
            reference,
            enabled,
            semantics,
            None,
            None,
            move |event| {
                let Some(scope) = weak.upgrade() else { return };
                if !live.visible.get() {
                    return;
                }
                callback.emit(instance, ());
                if instance.is_alive()
                    && close
                    && let Err(error) = live.root_close()
                {
                    scope.report_error(error);
                    return;
                }
                observe(&mut observers, instance, event);
            },
        )?;
    } else {
        let control = item.checked.mount(&owner);
        let initial = control.source.try_get()?;
        let dirty = Dirty::new(&owner);
        let changed = Callback::new(item.changed.take());
        let accepted = control.clone();
        let pulse = dirty.clone();
        let native = configure!(
            item.common,
            glory::ui::check()
                .label(item.label.clone())
                .checked(initial)
                .node_ref(reference),
            &owner,
            "sapid-menu-item sapid-menu-check",
            menu_item_style(parent)
        );
        let mut observers = item
            .common
            .handlers
            .remove(&EventKind::CheckedChanged)
            .unwrap_or_default();
        let node = mount_member(
            &owner,
            state,
            native,
            parent,
            &mut item.common,
            &item.key,
            item.label,
            reference,
            enabled,
            semantics,
            None,
            Some(control.source.clone()),
            move |event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !live.visible.try_get()? {
                        return Ok(());
                    }
                    if let Event::CheckedChanged(value) = &event {
                        accepted.accept(value)?;
                        pulse.touch()?;
                        changed.emit(instance, *value);
                        if instance.is_alive() {
                            callback.emit(instance, ());
                        }
                        if instance.is_alive() && close {
                            live.root_close()?;
                        }
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                    return;
                }
                observe(&mut observers, instance, event);
            },
        )?;
        let weak = owner.downgrade();
        let mut previous = initial;
        owner.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let value = control.source.try_get()?;
                if dirty.take() || value != previous {
                    scope.patch(node, Patch::Checked(Change::Set(value)))?;
                    previous = value;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
    }
    scope.retain(owner)?;
    Ok(())
}
fn build_radios(
    scope: &Scope,
    state: &Rc<MenuState>,
    parent: &Common,
    namespace: &str,
    path: &[String],
    menu_id: &SemanticId,
    group: MenuRadioGroup,
) -> Result {
    let owner = scope.child()?;
    let mut group_path = path.to_vec();
    group_path.push(group.key);
    let group_id = id(namespace, &group_path, "group")?;
    let control = group.value.mount(&owner);
    let keys = Rc::new(
        group
            .items
            .iter()
            .map(|item| item.key.clone())
            .collect::<Vec<_>>(),
    );
    let known = keys.clone();
    let selected = composite::validated(&owner, &control.source, move |value| {
        if value.is_empty() || known.contains(value) {
            Ok(())
        } else {
            Err(invalid("unknown menu radio value"))
        }
    })?;
    let mut group_widget = glory::ui::view()
        .role(Role::Group)
        .class(if parent.unstyled {
            "sapid-menu-radio-group sapid-unstyled"
        } else {
            "sapid-menu-radio-group"
        })
        .semantics(composite::metadata(Semantics {
            id: Some(group_id.clone()),
            ..Default::default()
        }));
    let label = group.label;
    if !label.get().is_empty() {
        group_widget = group_widget.name(label);
    }
    let reference = owner.cage(None);
    group_widget.node_ref(reference).show_in(&owner)?;
    let content = owner.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
    let dirty = Dirty::new(&owner);
    let callback = Callback::new(group.changed);
    let initial = selected.get();
    let mut references = Vec::new();
    for mut item in group.items {
        let row = content::member_scope(&content, item.visible.clone())?;
        item.common.enabled =
            content::filter_enabled(&row, item.common.enabled.clone(), item.visible);
        item.common.theme = parent.theme.clone();
        item.common.size = parent.size;
        item.common.unstyled |= parent.unstyled;
        let reference = item.common.node_ref.unwrap_or_else(|| row.cage(None));
        search::highlight_style(&row, state, &mut item.common, reference);
        references.push(reference);
        let enabled = item.common.enabled.clone();
        let value = item.key.clone();
        let accepted = control.clone();
        let changed = callback.clone();
        let pulse = dirty.clone();
        let mut item_path = group_path.clone();
        item_path.push(item.key.clone());
        let semantics = Semantics {
            id: Some(id(namespace, &item_path, "item")?),
            menu_item: Some(MenuItemSemantics {
                kind: MenuItemKind::Radio,
                owner: menu_id.clone(),
            }),
            check_group: Some(group_id.clone()),
            ..Default::default()
        };
        let native = configure!(
            item.common,
            glory::ui::check()
                .mode(CheckMode::Radio)
                .label(item.label.clone())
                .checked(initial == item.key)
                .node_ref(reference),
            &row,
            "sapid-menu-item sapid-menu-radio",
            menu_item_style(parent)
        );
        let radio_checked = state.search_editor.as_ref().map(|_| {
            let source = control.source.clone();
            let key = item.key.clone();
            row.bond(move || source.with(|value| value == &key)).into()
        });
        let live = state.clone();
        let close = group.close;
        let weak = row.downgrade();
        let instance = owner.id();
        let mut observers = item
            .common
            .handlers
            .remove(&EventKind::CheckedChanged)
            .unwrap_or_default();
        mount_member(
            &row,
            state,
            native,
            parent,
            &mut item.common,
            &item.key,
            item.label,
            reference,
            enabled,
            semantics,
            None,
            radio_checked,
            move |event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !live.visible.try_get()? {
                        return Ok(());
                    }
                    if event == Event::CheckedChanged(true) {
                        accepted.accept(&value)?;
                        pulse.touch()?;
                        changed.emit(instance, value.clone());
                        if instance.is_alive() && close {
                            live.root_close()?;
                        }
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                    return;
                }
                observe(&mut observers, instance, event);
            },
        )?;
        content.retain(row)?;
    }
    owner.retain(content)?;
    let weak = owner.downgrade();
    let mut previous = initial;
    owner.effect(move || {
        let Some(scope) = weak.upgrade() else { return };
        let result = (|| -> Result {
            let value = selected.try_get()?;
            if dirty.take() || value != previous {
                for (key, reference) in keys.iter().zip(references.iter()) {
                    if let Some(node) = reference.try_get()? {
                        scope.patch(node, Patch::Checked(Change::Set(key == &value)))?;
                    }
                }
                previous = value;
            }
            Ok(())
        })();
        if let Err(error) = result {
            scope.report_error(error);
        }
    });
    scope.retain(owner)?;
    Ok(())
}

impl Widget for Menu {
    fn build(&mut self, scope: &Scope) -> Result {
        validate_width(self.panel_width)?;
        if let Some(position) = self.position {
            position.validate()?;
        }
        if let Some(editor) = &self.editor {
            editor.validate()?;
        }
        if self.overflow {
            return Err(invalid("menu configuration budget exhausted"));
        }
        key(&self.key)?;
        if self.common.focus != FocusMode::Auto {
            return Err(invalid("menu owns its focus scope"));
        }
        validate(&self.key, &self.entries, &mut Vec::new(), &mut 0)?;
        Typeahead::new(self.typeahead)?;
        let trigger_id = id(&self.key, &[], "trigger")?;
        let context = self.context.take();
        let point = scope.cage(glory::render::ContextPoint::default());
        let placement: Vary<LayerPlacement> = if context.is_some() {
            scope
                .bond(move || LayerPlacement::ContextPoint(point.get()))
                .into()
        } else {
            LayerPlacement::Anchor(self.position.unwrap_or(glory::render::AnchorPlacement::new(
                glory::render::AnchorSide::Bottom,
                glory::render::AnchorAlign::Start,
            )))
            .into()
        };
        let anchor = if self.dropdown {
            trigger_id.clone()
        } else if context.is_some() {
            id(&self.key, &[], "context")?
        } else {
            self.anchor
                .clone()
                .ok_or_else(|| invalid("standalone Menu requires an explicit anchor"))?
        };
        let return_focus = if context.is_some() {
            None
        } else {
            Some(self.return_focus.clone().unwrap_or_else(|| anchor.clone()))
        };
        if let Some(width) = self.panel_width {
            let style = self.common.style.clone();
            self.common.style = scope
                .bond(move || style.get().width(Length::Px(width)))
                .into();
        }
        let context_anchor = anchor.clone();
        let trigger = self.trigger.unwrap_or_else(|| scope.cage(None));
        let enabled = self.common.enabled.clone();
        let label = self.label.clone();
        let mut common = std::mem::take(&mut self.common);
        let mut context_observers = if context.is_some() {
            common
                .handlers
                .remove(&EventKind::ContextMenu)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let mut trigger_observers = if self.dropdown {
            common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let menu_observers = Observers::new(if self.dropdown {
            common.handlers.remove(&EventKind::Menu).unwrap_or_default()
        } else {
            Vec::new()
        });
        let shared = menu_observers.clone();
        let instance = scope.id();
        if self.dropdown {
            common.handlers.insert(
                EventKind::Menu,
                vec![Box::new(move |event| shared.emit(instance, event))],
            );
        }
        let theme = common.theme.clone();
        let size = common.size;
        let unstyled = common.unstyled;
        let bar = self.bar.take();
        let state = Level {
            editor: self.editor.take(),
            bar: bar
                .as_ref()
                .map(|config| (config.key.clone(), config.state.clone())),
            key: self.key.clone(),
            path: Vec::new(),
            common,
            label: label.clone(),
            open: std::mem::take(&mut self.open),
            changed: self.changed.take(),
            entries: std::mem::take(&mut self.entries),
            anchor,
            return_focus,
            placement,
            parent: Weak::new(),
            root: Weak::new(),
            parent_visible: true.into(),
            root_enabled: true.into(),
            direction: self.direction,
            looping: self.looping,
            typeahead: self.typeahead,
            delay: self.delay.clone(),
            trigger,
        }
        .build(scope)?;
        if let Some(context) = context {
            let policy = context.policy;
            let shared = enabled.clone();
            let weak = scope.downgrade();
            let controller = state.clone();
            let node = context.reference.unwrap_or_else(|| scope.cage(None));
            let target = glory::ui::view()
                .role(Role::Group)
                .name(context.name)
                .focus(FocusMode::TabStop)
                .node_ref(node)
                .semantics(scope.bond(move || {
                    composite::metadata(Semantics {
                        id: Some(context_anchor.clone()),
                        context_menu: Some(policy),
                        disabled: !shared.get(),
                        ..Default::default()
                    })
                }))
                .on(EventKind::ContextMenu, move |event| {
                    let Some(scope) = weak.upgrade() else { return };
                    let Event::ContextMenu {
                        point: requested, ..
                    } = event
                    else {
                        return;
                    };
                    let result = (|| -> Result {
                        if !controller.enabled.try_get()? {
                            return Ok(());
                        }
                        controller.request(true)?;
                        if controller.owner.is_alive() && controller.open.source.try_get()? {
                            point.try_revise(|value| *value = requested)?;
                        }
                        Ok(())
                    })();
                    if let Err(error) = result {
                        scope.report_error(error);
                        return;
                    }
                    observe(&mut context_observers, instance, event);
                })
                .mount(scope)?;
            let content = target
                .scope()
                .child_in(node.get().ok_or(glory::ui::Error::Gone)?)?;
            context.target.show_in(&content)?;
            target.scope().retain(content)?;
            scope.retain(target)?;
        }
        if self.dropdown {
            let panel_id = id(&self.key, &[], "panel")?;
            let open = state.visible.clone();
            let source = state.clone();
            let weak = scope.downgrade();
            let command = state.clone();
            let key_scope = scope.downgrade();
            let bar_action = bar
                .as_ref()
                .map(|config| (config.key.clone(), config.state.clone()));
            let bar_owner = bar.as_ref().map(|config| config.owner.clone());
            let is_bar = bar.is_some();
            let focus = bar
                .as_ref()
                .map(|config| config.focus.clone())
                .unwrap_or_else(|| FocusMode::Auto.into());
            let mut button = glory::ui::button()
                .class(format!(
                    "sapid-menu-trigger {}{}",
                    theme.class(),
                    if unstyled { " sapid-unstyled" } else { "" }
                ))
                .label(label.clone())
                .enabled(enabled.clone())
                .focus(focus)
                .node_ref(trigger)
                .semantics(scope.bond(move || {
                    composite::metadata(Semantics {
                        id: Some(trigger_id.clone()),
                        popup: Some(PopupKind::Menu),
                        expanded: Some(open.get()),
                        controls: Some(panel_id.clone()),
                        menubar_item: is_bar,
                        navigation_item: is_bar,
                        menu_item: bar_owner.clone().map(|owner| MenuItemSemantics {
                            kind: MenuItemKind::Action,
                            owner,
                        }),
                        ..Default::default()
                    })
                }))
                .on(EventKind::Activate, move |event| {
                    let Some(scope) = weak.upgrade() else { return };
                    if !source.owner.is_alive() || !source.enabled.get() {
                        return;
                    }
                    let result = if let Some((key, bar)) = &bar_action
                        && let Some(bar) = bar.upgrade()
                    {
                        bar.toggle(key).map(|_| ())
                    } else {
                        source.request(!source.open.source.get())
                    };
                    if let Err(error) = result {
                        scope.report_error(error);
                        return;
                    }
                    observe(&mut trigger_observers, instance, event);
                })
                .on(EventKind::Menu, move |event| {
                    let Some(scope) = key_scope.upgrade() else {
                        return;
                    };
                    if let Event::Menu { from, command: key } = &event
                        && let Err(error) = command.command(&scope, *from, *key)
                    {
                        scope.report_error(error);
                        return;
                    }
                    menu_observers.emit(instance, event);
                });
            if !unstyled {
                let available = enabled.clone();
                button = button.style(scope.bond(move || {
                    let mut style = theme.control(size);
                    if !available.get() {
                        style.background = Some(theme.disabled);
                        style.color = Some(theme.disabled_text);
                    }
                    if is_bar {
                        style.background = Some(glory::style::Color(0, 0, 0, 0));
                        style.border_width = Some(Edges::all(0.));
                    }
                    style
                }));
            }
            button.show_in(scope)?;
            if let Some(config) = bar
                && let Some(bar) = config.state.upgrade()
            {
                bar.register(scope, &config.key, trigger, enabled, label, state.clone())?;
            }
        }
        Ok(())
    }
}
impl Widget for DropdownMenu {
    fn build(&mut self, scope: &Scope) -> Result {
        self.inner.build(scope)
    }
}
impl Widget for ContextMenu {
    fn build(&mut self, scope: &Scope) -> Result {
        let target = self
            .target
            .take()
            .ok_or_else(|| invalid("context menu requires a target widget"))?;
        self.inner.context = Some(ContextConfig {
            target,
            name: self.target_name.clone(),
            policy: self.policy,
            reference: self.reference,
        });
        self.inner.build(scope)
    }
}
mod bar;
pub use bar::{MAX_MENUBAR_ROOTS, Menubar, MenubarMenu, menubar, menubar_menu};
