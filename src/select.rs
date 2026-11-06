//! A focus-preserving listbox over intrinsic combobox/option refinements.
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use glory::render::{
    ChoiceCommand, ComboboxKind, Event, EventKind, FocusMode, FormControl, Layer, LayerFocus,
    LayerPlacement, Navigation, OptionSemantics, Orientation, PopupKind, Role, SelectEntry,
    SelectGroup, SelectProps, SemanticId, Semantics,
};
use glory::style::{
    Align, BorderStyle, Color, Display, Edges, FlexDirection, Justify, Length, Limit, Overflow,
    PointerEvents, Position, Style, WhiteSpace,
};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};

use crate::appearance::{Common, common_methods, configure};
use crate::collection::{
    Collection, CollectionLimits, MountedItem, RovingFocus, Typeahead, TypeaheadConfig,
};
use crate::composite::{self, Callback, Observers};
use crate::native_select::{NativeOption, NativeOptionGroup};
use crate::state::{Control, Value};

enum Entry {
    Option(NativeOption),
    Group(NativeOptionGroup),
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectOptionContent {
    pub leading: Option<String>,
    pub description: Option<String>,
    pub trailing: Option<String>,
}
impl SelectOptionContent {
    pub fn leading(mut self, value: impl Into<String>) -> Self {
        self.leading = Some(value.into());
        self
    }
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn trailing(mut self, value: impl Into<String>) -> Self {
        self.trailing = Some(value.into());
        self
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectCheckPosition {
    Hidden,
    #[default]
    Left,
    Right,
}
/// Read-only accepted values for passive content inside the real trigger Button.
#[derive(Clone)]
pub struct SelectTriggerContext {
    pub value: Vary<Option<String>>,
    pub label: Vary<String>,
    pub open: Vary<bool>,
}
pub struct Select {
    common: Common,
    key: String,
    entries: Vec<Entry>,
    count: usize,
    value: Value<Option<String>>,
    open: Value<bool>,
    changed: Option<Box<dyn FnMut(Option<String>)>>,
    opened: Option<Box<dyn FnMut(bool)>>,
    placeholder: String,
    clearable: bool,
    field: Option<crate::FieldControl>,
    form_name: Option<Vary<String>>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    loading: Vary<bool>,
    option_content: BTreeMap<String, SelectOptionContent>,
    check_position: SelectCheckPosition,
    max_dropdown_height: f64,
    popup_placement: glory::render::AnchorPlacement,
    trigger_content: Option<Box<dyn FnMut(SelectTriggerContext) -> Children>>,
    empty_message: String,
}
pub fn select(key: impl Into<String>) -> Select {
    Select {
        common: Common::default(),
        key: key.into(),
        entries: Vec::new(),
        count: 0,
        value: Value::default(),
        open: Value::default(),
        changed: None,
        opened: None,
        placeholder: "Choose an option".into(),
        clearable: false,
        field: None,
        form_name: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        loading: false.into(),
        option_content: BTreeMap::new(),
        check_position: SelectCheckPosition::Left,
        max_dropdown_height: 288.0,
        trigger_content: None,
        empty_message: "No options available".into(),
        popup_placement: glory::render::AnchorPlacement {
            match_width: true,
            ..glory::render::AnchorPlacement::new(
                glory::render::AnchorSide::Bottom,
                glory::render::AnchorAlign::Start,
            )
        },
    }
}
impl Select {
    common_methods!();
    /// Customize passive trigger content without replacing its intrinsic Button,
    /// listbox relationships, focus handling or accepted state.
    pub fn trigger_content<W: Widget>(
        mut self,
        mut factory: impl FnMut(SelectTriggerContext) -> W + 'static,
    ) -> Self {
        self.trigger_content = Some(Box::new(move |context| factory(context).into_children()));
        self
    }
    pub fn empty_message(mut self, value: impl Into<String>) -> Self {
        self.empty_message = value.into();
        self
    }
    pub fn label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(label.into());
        self
    }
    pub fn field(mut self, field: crate::FieldControl) -> Self {
        self.common.theme = field.theme.clone();
        self.common.size = field.size;
        self.common.unstyled = field.unstyled;
        self.field = Some(field);
        self
    }
    pub fn form_name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<Vary<Option<String>>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: Option<String>) -> Self {
        self.value.default = value;
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
    pub fn on_value_change(mut self, callback: impl FnMut(Option<String>) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn on_open_change(mut self, callback: impl FnMut(bool) + 'static) -> Self {
        self.opened = Some(Box::new(callback));
        self
    }
    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = value.into();
        self
    }
    pub fn clearable(mut self, value: bool) -> Self {
        self.clearable = value;
        self
    }
    pub fn required(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.required = value.into();
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn invalid(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.invalid = value.into();
        self
    }
    pub fn loading(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.loading = value.into();
        self
    }
    pub fn option_content(mut self, key: impl Into<String>, content: SelectOptionContent) -> Self {
        if self.option_content.len() <= 1024 {
            self.option_content.insert(key.into(), content);
        }
        self
    }
    pub fn check_position(mut self, value: SelectCheckPosition) -> Self {
        self.check_position = value;
        self
    }
    pub fn with_check_icon(mut self, value: bool) -> Self {
        self.check_position = if value {
            SelectCheckPosition::Left
        } else {
            SelectCheckPosition::Hidden
        };
        self
    }
    pub fn max_dropdown_height(mut self, value: f64) -> Self {
        self.max_dropdown_height = value;
        self
    }
    pub fn popup_placement(mut self, value: glory::render::AnchorPlacement) -> Self {
        self.popup_placement = value;
        self
    }
    pub fn option(mut self, value: NativeOption) -> Self {
        if self.count <= 1024 {
            self.entries.push(Entry::Option(value));
            self.count += 1;
        }
        self
    }
    pub fn options(mut self, values: impl IntoIterator<Item = NativeOption>) -> Self {
        for value in values
            .into_iter()
            .take(1025usize.saturating_sub(self.count))
        {
            self.entries.push(Entry::Option(value));
            self.count += 1;
        }
        self
    }
    pub fn group(mut self, value: NativeOptionGroup) -> Self {
        if self.count <= 1024 && self.entries.len() <= 1056 {
            self.count += value.options.len();
            self.entries.push(Entry::Group(value));
        }
        self
    }
}

struct State {
    owner: glory::core::InstanceId,
    value: Control<Option<String>>,
    selected: Cage<Option<String>>,
    open: Control<bool>,
    visible: Vary<bool>,
    editable: Vary<bool>,
    collection: Collection,
    cursor: RovingFocus,
    typeahead: RefCell<Typeahead>,
    config: Rc<SelectProps>,
    changed: Callback<Option<String>>,
    opened: Callback<bool>,
    activated: Observers,
}
impl State {
    fn opened(&self, open: bool) -> Result {
        if !self.owner.is_alive() {
            return Ok(());
        }
        if self.open.source.try_get()? != open {
            self.open.accept(&open)?;
            self.opened.emit(self.owner, open);
        }
        Ok(())
    }
    fn choose(&self, value: Option<String>) -> Result {
        if !self.editable.try_get()? {
            return Ok(());
        }
        if value.is_some() && !self.visible.try_get()? {
            return Ok(());
        }
        self.value
            .source
            .try_with(|current| validate(&self.config, current))??;
        validate(&self.config, &value)?;
        self.value.accept(&value)?;
        self.changed.emit(self.owner, value);
        if self.owner.is_alive() {
            self.opened(false)?;
        }
        Ok(())
    }
    fn current(&self) -> Result<Option<MountedItem>> {
        let current = self.cursor.current()?;
        Ok(self
            .collection
            .mounted()?
            .into_iter()
            .find(|item| Some(&item.member) == current.as_ref() && item.enabled))
    }
    fn set_current(&self, item: &MountedItem) -> Result {
        self.cursor.accept(item)
    }
    fn initial_cursor(&self) -> Result {
        if let Some(key) = self.selected.try_get()? {
            self.cursor.repair(Some(&key))
        } else {
            self.navigate(Navigation::First)
        }
    }
    fn navigate(&self, command: Navigation) -> Result {
        let items = self.collection.mounted()?;
        let destination = match command {
            Navigation::First => items.iter().find(|item| item.enabled).cloned(),
            Navigation::Last => items.iter().rev().find(|item| item.enabled).cloned(),
            _ => match self.current()? {
                Some(item) => self.cursor.destination(item.node, command)?,
                None => items.into_iter().find(|item| item.enabled),
            },
        };
        if let Some(item) = destination {
            self.set_current(&item)?;
        }
        Ok(())
    }
    fn command(&self, command: ChoiceCommand) -> Result {
        if command == ChoiceCommand::Close {
            return self.opened(false);
        }
        if !self.editable.try_get()? {
            return Ok(());
        }
        match command {
            ChoiceCommand::OpenFirst => {
                self.opened(true)?;
                self.initial_cursor()?;
            }
            ChoiceCommand::OpenLast => {
                self.opened(true)?;
                self.navigate(Navigation::Last)?;
            }
            ChoiceCommand::First => {
                self.opened(true)?;
                self.navigate(Navigation::First)?;
            }
            ChoiceCommand::Last => {
                self.opened(true)?;
                self.navigate(Navigation::Last)?;
            }
            ChoiceCommand::Previous => {
                if self.visible.try_get()? {
                    self.navigate(Navigation::Previous)?;
                } else {
                    self.opened(true)?;
                    self.navigate(Navigation::Last)?;
                }
            }
            ChoiceCommand::Next => {
                if self.visible.try_get()? {
                    self.navigate(Navigation::Next)?;
                } else {
                    self.opened(true)?;
                    self.initial_cursor()?;
                }
            }
            ChoiceCommand::Commit => {
                if self.visible.try_get()?
                    && let Some(item) = self.current()?
                {
                    self.choose(Some(item.key))?;
                } else if !self.visible.try_get()? {
                    self.opened(true)?;
                    self.initial_cursor()?;
                }
            }
            ChoiceCommand::Close => self.opened(false)?,
            ChoiceCommand::RemoveLast => {
                return Err(glory::render::Error::InvalidEvent(
                    "remove-last requires multiple selection",
                )
                .into());
            }
        }
        Ok(())
    }
    fn search(&self, text: &str, at_millis: u64) -> Result {
        if !self.editable.try_get()? {
            return Ok(());
        }
        let was_visible = self.visible.try_get()?;
        self.opened(true)?;
        if was_visible {
            self.cursor.repair(None)?;
        } else {
            self.initial_cursor()?;
        }
        if let Some(current) = self.current()? {
            let result = self.typeahead.borrow_mut().search(
                &self.collection,
                current.node,
                text,
                at_millis,
            )?;
            if let Some(item) = result {
                self.set_current(&item)?;
            }
        }
        Ok(())
    }
}
fn validate(config: &SelectProps, value: &Option<String>) -> Result {
    if let Some(value) = value
        && (value.len() > 64
            || !config
                .options()
                .any(|(option, _)| option.key.as_str() == value))
    {
        return Err(glory::render::Error::InvalidProps(
            "select value must be a known bounded option key",
        )
        .into());
    }
    Ok(())
}

impl Widget for Select {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.trigger_content.is_some() && self.common.name.is_none() && self.field.is_none() {
            return Err(glory::render::Error::InvalidProps(
                "custom select trigger requires an explicit name or Field association",
            )
            .into());
        }
        if self.count > 1024
            || self.entries.len() > 1056
            || self.option_content.len() > 1024
            || self.placeholder.len() > 256
            || self.placeholder.contains(['\0', '\r'])
            || self.empty_message.len() > 1024
            || self.empty_message.contains(['\0', '\r'])
            || !self.max_dropdown_height.is_finite()
            || !(80.0..=4096.0).contains(&self.max_dropdown_height)
        {
            return Err(
                glory::render::Error::InvalidProps("select configuration exceeds budget").into(),
            );
        }
        let trigger_id = self
            .field
            .as_ref()
            .map(|field| Ok(field.id.clone()))
            .unwrap_or_else(|| composite::id("select", &self.key, "trigger", "root"))?;
        let panel_id = composite::id("select", &self.key, "listbox", "root")?;
        let proxy_id = composite::id("select", &self.key, "proxy", "root")?;
        let entries: Result<Vec<_>> = std::mem::take(&mut self.entries)
            .into_iter()
            .map(|entry| match entry {
                Entry::Option(value) => Ok(SelectEntry::Option(value.lower()?)),
                Entry::Group(value) => Ok(SelectEntry::Group(SelectGroup {
                    label: value.label,
                    disabled: value.disabled,
                    options: value
                        .options
                        .into_iter()
                        .map(NativeOption::lower)
                        .collect::<Result<Vec<_>>>()?,
                })),
            })
            .collect();
        let config = Rc::new(SelectProps {
            entries: Rc::new(entries?),
            ..Default::default()
        });
        config.validate()?;
        for (key, content) in &self.option_content {
            let known = config
                .options()
                .any(|(option, _)| option.key.as_str() == key);
            let valid_text = |value: &Option<String>| {
                value.as_ref().is_none_or(|value| {
                    !value.is_empty() && value.len() <= 256 && !value.contains(['\0', '\r', '\n'])
                })
            };
            if !known
                || !valid_text(&content.leading)
                || !valid_text(&content.description)
                || !valid_text(&content.trailing)
            {
                return Err(glory::render::Error::InvalidProps(
                    "select option content requires a known key and bounded single-line text",
                )
                .into());
            }
        }
        let form_name = composite::form_name(scope, &self.form_name)?;
        let value = std::mem::take(&mut self.value).mount(scope);
        let check = config.clone();
        let selected =
            composite::validated(scope, &value.source, move |value| validate(&check, value))?;
        let open = std::mem::take(&mut self.open).mount(scope);
        let enabled = self
            .field
            .as_ref()
            .map(|field| field.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let metadata = crate::field::control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            self.readonly.clone(),
            self.invalid.clone(),
        );
        let input_meta = metadata.clone();
        let enabled_for_edit = enabled.clone();
        let editable: Vary<bool> = scope
            .bond(move || enabled_for_edit.get() && !input_meta.get().is_some_and(|s| s.readonly))
            .into();
        let source = open.source.clone();
        let enabled_for_panel = enabled.clone();
        let visible: Vary<bool> = scope
            .bond(move || source.get() && enabled_for_panel.get())
            .into();
        let collection = Collection::new(scope, CollectionLimits::default())?;
        let cursor = RovingFocus::new(scope, collection.clone(), true)?;
        let activated = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default(),
        );
        let state = Rc::new(State {
            owner: scope.id(),
            value,
            selected,
            open,
            visible: visible.clone(),
            editable,
            collection,
            cursor,
            typeahead: RefCell::new(Typeahead::new(TypeaheadConfig::default())?),
            config: config.clone(),
            changed: Callback::new(self.changed.take()),
            opened: Callback::new(self.opened.take()),
            activated,
        });
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                state.changed.clear();
                state.opened.clear();
                state.activated.clear();
            }
        })?;
        let trigger_ref = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let display = config.clone();
        let placeholder = std::mem::take(&mut self.placeholder);
        let current = selected;
        let label = scope.bond(move || {
            let selected = current.get();
            display
                .options()
                .find(|(option, _)| Some(option.key.as_str()) == selected.as_deref())
                .map_or_else(|| placeholder.clone(), |(option, _)| option.label.clone())
        });
        let trigger_meta = metadata.clone();
        let button_state = state.clone();
        let trigger_name = trigger_id.clone();
        let listbox_name = panel_id.clone();
        let key = self.key.clone();
        let loading_state = self.loading.clone();
        let trigger_metadata = scope.bond(move || {
            let mut value = trigger_meta.get().unwrap_or_default();
            value.id = Some(trigger_name.clone());
            value.form_control = None;
            value.combobox = Some(ComboboxKind::Select);
            value.popup = Some(PopupKind::Listbox);
            value.controls = Some(listbox_name.clone());
            value.expanded = Some(button_state.visible.get());
            value.busy = Some(loading_state.get());
            value.typeahead = true;
            value.active_descendant = if button_state.visible.get() {
                button_state
                    .current()
                    .ok()
                    .flatten()
                    .and_then(|item| composite::id("select", &key, "option", &item.key).ok())
            } else {
                None
            };
            Some(value)
        });
        let own_label_style = self.common.style.clone();
        let label_enabled = enabled.clone();
        let muted = self.common.theme.muted;
        let themed = !self.common.unstyled;
        let label_style = scope.bond(move || {
            let mut style = Style {
                shrink: Some(1.),
                ..Style::new()
                    .min_width(Length::Px(0.))
                    .white_space(WhiteSpace::NoWrap)
                    .overflow_x(Overflow::Hidden)
            };
            if themed && label_enabled.get() && selected.with(Option::is_none) {
                style.color = Some(own_label_style.with(|own| own.color).unwrap_or(muted));
            }
            style
        });
        let content = crate::input_actions::frame(scope, &self.common, "sapid-select-shell")?;
        let style = crate::input_actions::display_style(
            scope,
            &self.common,
            enabled.clone(),
            trigger_metadata.into(),
            if self.clearable { 2 } else { 1 },
        );
        self.common.style = scope
            .bond(move || {
                let mut style = style.get();
                style.align.get_or_insert(Align::Center);
                style.justify.get_or_insert(Justify::Start);
                style.display.get_or_insert(Display::Flex);
                style.position.get_or_insert(Position::Relative);
                style
            })
            .into();
        let mut trigger = configure!(
            self.common,
            glory::ui::button(),
            scope,
            "sapid-select",
            Style::new()
        )
        .label("")
        .enabled(enabled.clone())
        .semantics(trigger_metadata)
        .node_ref(trigger_ref);
        trigger = if let Some(mut factory) = self.trigger_content.take() {
            trigger.fill(factory(SelectTriggerContext {
                value: selected.into(),
                label: label.into(),
                open: visible.clone(),
            }))
        } else {
            trigger.fill(
                glory::ui::text()
                    .class("sapid-select-value")
                    .content(label)
                    .selectable(false)
                    .style(label_style),
            )
        };
        let loading_state = self.loading.clone();
        let loading_text = scope.bond(move || {
            if loading_state.get() {
                "…".to_owned()
            } else {
                String::new()
            }
        });
        let side = crate::input_actions::side(&self.common);
        let arrow_style = Style {
            shrink: Some(0.),
            ..Style::new()
                .display(Display::Block)
                .pointer_events(PointerEvents::None)
                .position(Position::Absolute)
                .width(Length::Px(16.))
                .height(Length::Px(16.))
                .right(Length::Px((side - 16.) / 2.))
                .top(Length::Percent(50.))
                .margin(Edges {
                    top: -8.,
                    bottom: 0.,
                    left: 0.,
                    right: 0.,
                })
        };
        let arrow_loading = self.loading.clone();
        trigger = trigger.fill(
            crate::input_actions::glyph(crate::Icon::ChevronDown, self.common.theme.muted)
                .class("sapid-select-chevron")
                .style(scope.bond(move || {
                    let mut style = arrow_style.clone();
                    if arrow_loading.get() {
                        style.display = Some(Display::None);
                    }
                    style
                })),
        );
        trigger = trigger.fill(
            glory::ui::text()
                .class("sapid-select-loading")
                .content(loading_text)
                .style(crate::input_actions::action_style(&self.common, 0)),
        );
        let controller = state.clone();
        let weak = scope.downgrade();
        trigger = trigger.on(EventKind::Activate, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if let Err(error) = (|| -> Result {
                if controller.editable.try_get()? {
                    controller.opened(!controller.visible.try_get()?)?;
                    controller.initial_cursor()?;
                }
                Ok(())
            })() {
                scope.report_error(error);
                return;
            }
            controller.activated.emit(scope.id(), event);
        });
        let controller = state.clone();
        let weak = scope.downgrade();
        let choices = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Choice)
                .unwrap_or_default(),
        );
        trigger = trigger.on(EventKind::Choice, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if let Event::Choice { command, .. } = &event
                && let Err(error) = controller.command(*command)
            {
                scope.report_error(error);
                return;
            }
            choices.emit(scope.id(), event);
        });
        let controller = state.clone();
        let weak = scope.downgrade();
        let searches = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Typeahead)
                .unwrap_or_default(),
        );
        trigger = trigger.on(EventKind::Typeahead, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if let Event::Typeahead {
                text, at_millis, ..
            } = &event
                && let Err(error) = controller.search(text, *at_millis)
            {
                scope.report_error(error);
                return;
            }
            searches.emit(scope.id(), event);
        });
        let controller = state.clone();
        let weak = scope.downgrade();
        let focus_observers = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::FocusChanged)
                .unwrap_or_default(),
        );
        trigger = trigger.on(EventKind::FocusChanged, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if event == Event::FocusChanged(false)
                && let Err(error) = controller.opened(false)
            {
                scope.report_error(error);
                return;
            }
            focus_observers.emit(scope.id(), event);
        });
        for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
            let handlers = Observers::new(handlers);
            let owner = scope.id();
            trigger = trigger.on(kind, move |event| handlers.emit(owner, event));
        }
        trigger.show_in(&content)?;
        let panel_label = trigger_id.clone();
        let panel_metadata = metadata.clone();
        let placement = self.popup_placement;
        let layer = Layer {
            modal: false,
            focus: LayerFocus::Preserve,
            placement: LayerPlacement::Anchor(placement),
            anchor: Some(trigger_id),
            initial_focus: None,
            return_focus: None,
            ..Default::default()
        };
        let panel_visible = visible.clone();
        let listbox_name = panel_id.clone();
        let panel_meta = scope.bond(move || {
            Some(Box::new(Semantics {
                id: Some(listbox_name.clone()),
                labelled_by: panel_metadata
                    .get()
                    .and_then(|metadata| metadata.labelled_by)
                    .or_else(|| Some(panel_label.clone())),
                navigation: Some(Orientation::Vertical),
                hidden: !panel_visible.get(),
                layer: Some(layer.clone()),
                ..Default::default()
            }))
        });
        let panel_ref = scope.cage(None);
        let controller = state.clone();
        let weak = scope.downgrade();
        let mut panel_style = Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .min_width(Length::Px(if self.popup_placement.match_width {
                0.
            } else {
                180.
            }))
            .max_height(Limit::Px(self.max_dropdown_height))
            .overflow_y(Overflow::Auto);
        if !self.common.unstyled {
            panel_style.background = Some(self.common.theme.surface);
            panel_style = panel_style
                .padding(Edges::all(4.0))
                .border_width(Edges::all(1.0))
                .border_style(BorderStyle::Solid)
                .border_color(self.common.theme.border)
                .radius(6.0);
        }
        glory::ui::view()
            .role(Role::Listbox)
            .semantics(panel_meta)
            .node_ref(panel_ref)
            .class(format!(
                "sapid-select-listbox {}",
                self.common.theme.class()
            ))
            .style(panel_style)
            .on(EventKind::Dismiss, move |_| {
                if let Some(scope) = weak.upgrade()
                    && let Err(error) = controller.opened(false)
                {
                    scope.report_error(error);
                }
            })
            .show_in(&content)?;
        let panel_node = panel_ref.get().ok_or(glory::ui::Error::Gone)?;
        let items_scope = scope.child_in(panel_node)?;
        if config.options().next().is_none() {
            glory::ui::text()
                .class("sapid-select-empty")
                .content(self.empty_message.clone())
                .style(Style {
                    color: Some(self.common.theme.muted),
                    ..Style::new().padding(Edges::all(8.))
                })
                .show_in(&items_scope)?;
        }
        for entry in config.entries.iter() {
            match entry {
                SelectEntry::Option(option) => mount_option(
                    &items_scope,
                    &state,
                    option,
                    false,
                    OptionPresentation {
                        key: &self.key,
                        owner: &panel_id,
                        common: &self.common,
                        content: self.option_content.get(option.key.as_str()),
                        check_position: self.check_position,
                    },
                )?,
                SelectEntry::Group(group) => {
                    let reference = scope.cage(None);
                    glory::ui::view()
                        .role(Role::Group)
                        .name(group.label.clone())
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column),
                        )
                        .node_ref(reference)
                        .fill(glory::ui::text().content(group.label.clone()))
                        .show_in(&items_scope)?;
                    let group_scope =
                        scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
                    for option in &group.options {
                        mount_option(
                            &group_scope,
                            &state,
                            option,
                            group.disabled,
                            OptionPresentation {
                                key: &self.key,
                                owner: &panel_id,
                                common: &self.common,
                                content: self.option_content.get(option.key.as_str()),
                                check_position: self.check_position,
                            },
                        )?;
                    }
                    items_scope.retain(group_scope)?;
                }
            }
        }
        content.retain(items_scope)?;
        let form_meta = metadata;
        let proxy_meta = scope.bond(move || {
            let value = form_meta.get();
            Some(Box::new(Semantics {
                id: Some(proxy_id.clone()),
                form_control: form_name
                    .as_ref()
                    .map(|name| FormControl {
                        name: name.get(),
                        value: None,
                    })
                    .or_else(|| value.and_then(|s| s.form_control)),
                ..Default::default()
            }))
        });
        let hidden = scope.cage(None);
        glory::ui::view()
            .semantics(Some(Box::new(Semantics {
                hidden: true,
                ..Default::default()
            })))
            .node_ref(hidden)
            .show_in(&content)?;
        let proxy_scope = scope.child_in(hidden.get().ok_or(glory::ui::Error::Gone)?)?;
        let selection = scope.bond(move || {
            selected
                .get()
                .into_iter()
                .map(|key| SemanticId::new(key).expect("validated option key"))
                .collect::<Vec<_>>()
        });
        glory::ui::select()
            .configuration((*config).clone())
            .selected(selection)
            .enabled(enabled)
            .semantics(proxy_meta)
            .show_in(&proxy_scope)?;
        content.retain(proxy_scope)?;
        if self.clearable {
            let controller = state.clone();
            let weak = scope.downgrade();
            let editable = state.editable.clone();
            let can_clear = scope.bond(move || editable.get() && selected.with(Option::is_some));
            let style = crate::input_actions::action_style(&self.common, 1);
            glory::ui::button()
                .label("")
                .name("Clear selection")
                .class("sapid-select-clear")
                .fill(crate::input_actions::glyph(
                    crate::Icon::Close,
                    self.common.theme.muted,
                ))
                .style(scope.bond(move || {
                    let mut style = style.clone();
                    if selected.with(Option::is_none) {
                        style.display = Some(Display::None);
                    }
                    style
                }))
                .enabled(can_clear)
                .on(EventKind::Activate, move |event| {
                    let Some(scope) = weak.upgrade() else { return };
                    if let Err(error) = controller.choose(None) {
                        scope.report_error(error);
                        return;
                    }
                    if controller.owner.is_alive()
                        && let Err(error) = composite::focus_node(&scope, trigger_ref.get())
                    {
                        scope.report_error(error);
                        return;
                    }
                    controller.activated.emit(scope.id(), event);
                })
                .show_in(&content)?;
        }
        let controller = state.clone();
        let weak = scope.downgrade();
        let mut previous_selected = selected.get();
        scope.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let current = selected.try_get()?;
                let changed = current != previous_selected;
                previous_selected = current;
                if !controller.visible.try_get()? {
                    controller.typeahead.borrow_mut().clear();
                    controller.initial_cursor()?;
                } else if changed || controller.current()?.is_none() {
                    controller.initial_cursor()?;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        let controller = state.clone();
        let weak = scope.downgrade();
        let mut revealed = None;
        scope.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let current = if controller.visible.try_get()? {
                    controller.current()?.map(|item| item.node)
                } else {
                    None
                };
                if current != revealed {
                    revealed = current;
                    if let Some(node) = current
                        && scope.capabilities()?.scroll
                    {
                        scope.action(glory::render::Action::RevealOption { node })?;
                    }
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        scope.retain(content)?;
        scope.retain(state)?;
        Ok(())
    }
}

struct OptionPresentation<'a> {
    key: &'a str,
    owner: &'a SemanticId,
    common: &'a Common,
    content: Option<&'a SelectOptionContent>,
    check_position: SelectCheckPosition,
}

fn mount_option(
    scope: &Scope,
    state: &Rc<State>,
    option: &glory::render::SelectOption,
    group_disabled: bool,
    presentation: OptionPresentation<'_>,
) -> Result {
    let OptionPresentation {
        key,
        owner,
        common,
        content,
        check_position,
    } = presentation;
    let parent = scope;
    let item_scope = parent.child()?;
    let scope = &item_scope;
    let id = composite::id("select", key, "option", option.key.as_str())?;
    let reference = scope.cage(None);
    let enabled = !option.disabled && !group_disabled;
    let member = state.collection.register(
        scope,
        option.key.as_str(),
        reference,
        enabled,
        option.label.clone(),
    )?;
    let selected = state.selected;
    let option_key = option.key.as_str().to_owned();
    let check_key = option_key.clone();
    let semantic_owner = owner.clone();
    let metadata = scope.bond(move || {
        Some(Box::new(Semantics {
            id: Some(id.clone()),
            option: Some(OptionSemantics {
                owner: semantic_owner.clone(),
            }),
            selected: Some(selected.get().as_deref() == Some(&option_key)),
            ..Default::default()
        }))
    });
    let controller = state.clone();
    let weak = scope.downgrade();
    let choose_key = option.key.as_str().to_owned();
    let pointer = state.clone();
    let weak_pointer = scope.downgrade();
    let pointer_member = member.clone();
    let mut base = if common.unstyled {
        Style::new()
    } else {
        common.theme.control(common.size)
    };
    if !common.unstyled {
        base.border_width = Some(Edges::all(0.0));
        base.background = Some(Color(0, 0, 0, 0));
    }
    let styled = state.clone();
    let styled_member = member.clone();
    let accent = common.theme.accent_soft;
    let foreground = common.theme.accent_text;
    let plain = common.unstyled;
    let style = scope.bond(move || {
        let mut value = base.clone();
        if !plain && styled.cursor.current().ok().flatten().as_ref() == Some(&styled_member) {
            value.background = Some(accent);
            value.color = Some(foreground);
        }
        value
    });
    let mut option_button = glory::ui::button()
        .label(option.label.clone())
        .focus(FocusMode::Programmatic)
        .class(format!("sapid-select-option {}", common.theme.class()))
        .style(style)
        .semantics(metadata)
        .node_ref(reference)
        .enabled(enabled)
        .on(EventKind::Activate, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if let Err(error) = controller.choose(Some(choose_key.clone())) {
                scope.report_error(error);
                return;
            }
            controller.activated.emit(scope.id(), event);
        })
        .on(EventKind::PointerWithin, move |event| {
            let Some(scope) = weak_pointer.upgrade() else {
                return;
            };
            if matches!(
                event,
                Event::PointerWithin {
                    phase: glory::render::PointerPhase::Move,
                    ..
                }
            ) && let Err(error) = pointer.cursor.observe(&pointer_member)
            {
                scope.report_error(error);
            }
        });
    if content.is_some() || check_position != SelectCheckPosition::Hidden {
        let checked = scope.bond(move || {
            if selected.get().as_deref() == Some(check_key.as_str()) {
                "✓".to_owned()
            } else {
                String::new()
            }
        });
        let check = || {
            glory::ui::text()
                .class("sapid-select-option-check")
                .content(checked)
                .style(Style::new().width(Length::Px(16.0)))
        };
        let mut row: Vec<Box<dyn Widget>> = Vec::with_capacity(4);
        if check_position == SelectCheckPosition::Left {
            row.push(Box::new(check()));
        }
        if let Some(value) = content.and_then(|value| value.leading.as_ref()) {
            row.push(Box::new(
                glory::ui::text()
                    .class("sapid-select-option-leading")
                    .content(value.clone()),
            ));
        }
        row.push(Box::new(
            glory::ui::text()
                .class("sapid-select-option-row sapid-select-option-label")
                .content(option.label.clone()),
        ));
        if let Some(value) = content.and_then(|value| value.description.as_ref()) {
            row.push(Box::new(
                glory::ui::text()
                    .class("sapid-select-option-description")
                    .content(value.clone()),
            ));
        }
        if let Some(value) = content.and_then(|value| value.trailing.as_ref()) {
            row.push(Box::new(
                glory::ui::text()
                    .class("sapid-select-option-trailing")
                    .content(value.clone()),
            ));
        }
        if check_position == SelectCheckPosition::Right {
            row.push(Box::new(check()));
        }
        option_button = option_button
            .label("")
            .name(option.label.clone())
            .fill(glory::ui::group().fill(row));
    }
    option_button.show_in(scope)?;
    parent.retain(item_scope)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use glory::render::NodeId;
    use glory::render::Props;
    use glory::ui::App;
    fn nodes(app: &App) -> Vec<NodeId> {
        let tree = app.tree();
        let mut pending = tree.roots().to_vec();
        let mut result = Vec::new();
        while let Some(id) = pending.pop() {
            result.push(id);
            pending.extend(tree.node(id).unwrap().children.iter().copied());
        }
        result
    }
    fn trigger(app: &App) -> NodeId {
        nodes(app)
            .into_iter()
            .find(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .semantics
                    .as_ref()
                    .is_some_and(|s| s.combobox.is_some())
            })
            .unwrap()
    }
    fn option_node(app: &App, key: &str) -> NodeId {
        nodes(app)
            .into_iter()
            .find(|id| {
                let tree = app.tree();
                let d = tree.node(*id).unwrap().descriptor.view();
                d.semantics.as_ref().is_some_and(|s| s.option.is_some())
                    && (matches!(d.props,Props::Button(button) if button.label==key)
                        || d.name.as_deref() == Some(key))
            })
            .unwrap()
    }
    #[test]
    fn passive_option_descendants_update_activity_in_select_and_editable_choices() {
        for kind in 0..4 {
            let app = App::memory();
            let options = [
                crate::select_option("a", "Alpha"),
                crate::select_option("b", "Beta"),
            ];
            let content = SelectOptionContent::default().description("Second option");
            if kind == 0 {
                app.mount(
                    select("passive-hover")
                        .default_open(true)
                        .options(options)
                        .option_content("b", content),
                )
                .unwrap();
            } else {
                let widget = crate::combobox("passive-hover")
                    .default_open(true)
                    .options(options)
                    .option_content("b", content);
                app.mount(match kind {
                    2 => widget.autocomplete(),
                    3 => widget.multiple(),
                    _ => widget,
                })
                .unwrap();
            }
            let trigger = trigger(&app);
            let beta = option_node(&app, "Beta");
            let child = nodes(&app)
                .into_iter()
                .find(|id| {
                    matches!(app.tree().node(*id).unwrap().descriptor.view().props,
                        Props::Text(text) if text.content == "Second option")
                })
                .unwrap();
            app.dispatch(
                child,
                Event::Pointer {
                    from: child,
                    related: None,
                    phase: glory::render::PointerPhase::Move,
                },
            )
            .unwrap();
            assert_eq!(
                app.tree().resolve_active_descendant(trigger).unwrap(),
                Some(beta),
                "kind {kind}: movement over a passive description belongs to its option"
            );
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn custom_trigger_content_reads_accepted_state_once_and_retains_real_choice_semantics() {
        use std::cell::Cell;
        let app = App::memory();
        let selected = app.scope().cage(None::<String>);
        let accept = app.scope().cage(false);
        let caption = app.scope().cage(None);
        let calls = Rc::new(Cell::new(0));
        let mounted = calls.clone();
        app.mount(
            select("custom-target")
                .name("Framework")
                .placeholder("Choose framework")
                .value(selected)
                .on_value_change(move |value| {
                    if accept.get() {
                        selected.revise(|old| *old = value)
                    }
                })
                .options([
                    crate::select_option("glory", "Glory"),
                    crate::select_option("rust", "Rust"),
                ])
                .trigger_content(move |context| {
                    mounted.set(mounted.get() + 1);
                    glory::ui::group()
                        .fill(glory::ui::text().node_ref(caption).content(context.label))
                }),
        )
        .unwrap();
        let button = trigger(&app);
        let text = caption.get().unwrap();
        for accepted in [false, true] {
            accept.revise(|v| *v = accepted);
            app.flush().unwrap();
            for command in [ChoiceCommand::OpenLast, ChoiceCommand::Commit] {
                app.dispatch(
                    button,
                    Event::Choice {
                        from: button,
                        command,
                    },
                )
                .unwrap();
            }
            let expected = if accepted { "Rust" } else { "Choose framework" };
            assert!(
                matches!(app.tree().node(text).unwrap().descriptor.view().props,Props::Text(p)if p.content==expected)
            );
        }
        selected.revise(|v| *v = Some("glory".into()));
        app.flush().unwrap();
        assert!(
            matches!(app.tree().node(text).unwrap().descriptor.view().props,Props::Text(p)if p.content=="Glory")
        );
        assert_eq!(calls.get(), 1);
        assert_eq!(trigger(&app), button);
        assert_eq!(
            app.tree()
                .node(button)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .combobox,
            Some(ComboboxKind::Select)
        );
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.runtime_stats().tasks, 0);
    }
    #[test]
    fn empty_listbox_has_a_bounded_passive_message_and_custom_trigger_rejects_nested_controls() {
        let app = App::memory();
        app.mount(
            select("empty-target")
                .default_open(true)
                .empty_message("No frameworks available"),
        )
        .unwrap();
        assert!(nodes(&app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p)if p.content=="No frameworks available")));
        app.clear().unwrap();
        assert!(
            app.mount(
                select("nested-target")
                    .name("Nested target")
                    .trigger_content(|_| crate::button().label("Invalid nested action"))
            )
            .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
        assert!(
            app.mount(select("large-empty").empty_message("x".repeat(1025)))
                .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
    #[test]
    fn listbox_keeps_trigger_focus_contract_and_selection_distinct_from_activity() {
        let app = App::memory();
        app.mount(select("local").options([
            crate::select_option("a", "A"),
            crate::select_option("b", "B"),
            crate::select_option("c", "C").disabled(true),
        ]))
        .unwrap();
        let trigger = trigger(&app);
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::OpenLast,
            },
        )
        .unwrap();
        let tree = app.tree();
        let metadata = tree
            .node(trigger)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap();
        assert_eq!(metadata.expanded, Some(true));
        assert!(
            metadata
                .active_descendant
                .as_ref()
                .unwrap()
                .as_str()
                .ends_with("1_b")
        );
        drop(tree);
        let b = option_node(&app, "B");
        assert_eq!(
            app.tree()
                .node(b)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(false)
        );
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::Commit,
            },
        )
        .unwrap();
        assert_eq!(
            app.tree()
                .node(b)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(true)
        );
        let tree = app.tree();
        let metadata = tree
            .node(trigger)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap();
        assert_eq!(metadata.expanded, Some(false));
        assert_eq!(metadata.active_descendant, None);
    }
    #[test]
    fn controlled_rejection_does_not_write_sources_or_leave_stale_active_relations() {
        let app = App::memory();
        let value = app.scope().cage(Some("a".to_owned()));
        let open = app.scope().cage(true);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let typed = calls.clone();
        app.mount(
            select("controlled")
                .value(value)
                .open(open)
                .options([
                    crate::select_option("a", "A"),
                    crate::select_option("b", "B"),
                ])
                .on_value_change(move |value| typed.borrow_mut().push(value)),
        )
        .unwrap();
        let trigger = trigger(&app);
        let b = option_node(&app, "B");
        app.dispatch(b, Event::Activate).unwrap();
        assert_eq!(value.get(), Some("a".into()));
        assert!(open.get());
        assert_eq!(*calls.borrow(), [Some("b".into())]);
        assert_eq!(
            app.tree()
                .node(b)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(false)
        );
        open.revise(|value| *value = false);
        app.flush().unwrap();
        assert_eq!(
            app.tree()
                .node(trigger)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .active_descendant,
            None
        );
    }
    #[test]
    fn field_required_is_on_the_visible_trigger_and_proxy_only_carries_successful_values() {
        let snapshot = glory::ssr::snapshot(
            crate::field("field-choice")
                .label("Plan")
                .required(true)
                .form_name("plan")
                .control(|field| {
                    select("plan")
                        .field(field)
                        .default_value(Some("a".into()))
                        .option(crate::select_option("a", "A").value("business"))
                }),
        )
        .unwrap();
        assert!(
            snapshot.html.contains("role=\"combobox\"")
                && snapshot.html.contains("aria-required=\"true\"")
                && snapshot.html.contains("role=\"listbox\"")
                && snapshot.html.contains("role=\"option\"")
        );
        assert!(
            !snapshot.html.contains(" required=\"\"")
                && snapshot.html.contains("name=\"plan\"")
                && snapshot.html.contains("value=\"business\" selected")
        );
    }
    #[test]
    fn bounded_configuration_and_initial_controlled_sources_fail_before_nodes() {
        let app = App::memory();
        let seen = Rc::new(std::cell::Cell::new(0));
        let count = seen.clone();
        let widget = select("bounded").options((0..1_000_000).map(move |index| {
            count.set(count.get() + 1);
            crate::select_option(format!("k{index}"), "Label")
        }));
        assert_eq!(seen.get(), 1025);
        assert!(app.mount(widget).is_err());
        assert_eq!(app.stats().nodes, 0);
        assert!(
            app.mount(select("large").value(Some("x".repeat(1_000_000))))
                .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }

    #[test]
    fn readonly_after_open_still_allows_escape_without_value_interaction() {
        let app = App::memory();
        let readonly = app.scope().cage(false);
        app.mount(
            select("readonly-open")
                .readonly(readonly)
                .default_open(true)
                .default_value(Some("a".into()))
                .options([
                    crate::select_option("a", "A"),
                    crate::select_option("b", "B"),
                ]),
        )
        .unwrap();
        readonly.revise(|value| *value = true);
        app.flush().unwrap();
        let trigger = trigger(&app);
        app.dispatch(option_node(&app, "B"), Event::Activate)
            .unwrap();
        assert_eq!(
            app.tree()
                .node(option_node(&app, "A"))
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(true)
        );
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::Close,
            },
        )
        .unwrap();
        let tree = app.tree();
        let semantics = tree
            .node(trigger)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap();
        assert_eq!(semantics.expanded, Some(false));
        assert_eq!(semantics.active_descendant, None);
    }

    #[test]
    fn unstyled_retains_layout_without_panel_or_clear_button_appearance() {
        let app = App::memory();
        app.mount(
            select("unstyled")
                .unstyled()
                .clearable(true)
                .default_value(Some("a".into()))
                .option(crate::select_option("a", "A")),
        )
        .unwrap();
        let tree = app.tree();
        for id in nodes(&app) {
            let descriptor = tree.node(id).unwrap().descriptor.view();
            if descriptor.role == Some(Role::Listbox)
                || descriptor.name.as_deref() == Some("Clear selection")
                || descriptor
                    .classes
                    .iter()
                    .any(|class| class == "sapid-select")
                || matches!(descriptor.props, Props::Button(button) if button.label == "A")
            {
                assert_eq!(descriptor.style.background, None);
                assert_eq!(descriptor.style.border_width, None);
                assert_eq!(descriptor.style.border_color, None);
            }
        }
    }

    #[test]
    fn invalid_live_source_preserves_last_valid_selection_and_recovers() {
        let app = App::memory();
        let value = app.scope().cage(Some("a".to_owned()));
        app.mount(select("recover").value(value).options([
            crate::select_option("a", "A"),
            crate::select_option("b", "B"),
        ]))
        .unwrap();
        value.revise(|value| *value = Some("x".repeat(1_000_000)));
        assert!(app.flush().is_err());
        let a = option_node(&app, "A");
        assert_eq!(
            app.tree()
                .node(a)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(true)
        );
        value.revise(|value| *value = Some("b".into()));
        app.flush().unwrap();
        let b = option_node(&app, "B");
        assert_eq!(
            app.tree()
                .node(b)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected,
            Some(true)
        );
    }

    #[test]
    fn repeated_typeahead_cycles_activity_without_reselecting_the_controlled_value() {
        let app = App::memory();
        let selected = app.scope().cage(Some("a".to_owned()));
        let open = app.scope().cage(true);
        let readonly = app.scope().cage(false);
        let enabled = app.scope().cage(true);
        app.mount(
            select("typeahead-cycle")
                .value(selected)
                .open(open)
                .readonly(readonly)
                .enabled(enabled)
                .options([
                    crate::select_option("a", "Alpha"),
                    crate::select_option("b", "Beta"),
                    crate::select_option("c", "Bravo"),
                ]),
        )
        .unwrap();
        let trigger = trigger(&app);
        let search = |at_millis| Event::Typeahead {
            from: trigger,
            text: "b".into(),
            at_millis,
        };
        app.dispatch(trigger, search(100)).unwrap();
        assert_eq!(
            app.tree().resolve_active_descendant(trigger).unwrap(),
            Some(option_node(&app, "Beta"))
        );
        app.dispatch(trigger, search(101)).unwrap();
        assert_eq!(
            app.tree().resolve_active_descendant(trigger).unwrap(),
            Some(option_node(&app, "Bravo"))
        );
        assert_eq!(selected.get(), Some("a".into()));
        readonly.revise(|value| *value = true);
        app.flush().unwrap();
        app.dispatch(trigger, search(102)).unwrap();
        assert_eq!(
            app.tree().resolve_active_descendant(trigger).unwrap(),
            Some(option_node(&app, "Bravo"))
        );
        enabled.revise(|value| *value = false);
        app.flush().unwrap();
        app.dispatch(trigger, search(103)).unwrap();
        assert_eq!(app.tree().resolve_active_descendant(trigger).unwrap(), None);
        assert_eq!(selected.get(), Some("a".into()));
        enabled.revise(|value| *value = true);
        readonly.revise(|value| *value = false);
        open.revise(|value| *value = false);
        app.flush().unwrap();
        app.dispatch(trigger, search(104)).unwrap();
        assert_eq!(app.tree().resolve_active_descendant(trigger).unwrap(), None);
        assert!(!open.get());
        assert_eq!(selected.get(), Some("a".into()));
    }

    #[test]
    fn native_keys_in_one_drained_tick_use_live_open_state_instead_of_old_dom_metadata() {
        use glory::render::{
            Capabilities, Error, Input, KeyboardInput, Mutation, Query, QueryToken, Receipt,
            Render, RuntimeId, memory::Memory,
        };
        struct Queued {
            memory: Memory,
            input: Rc<RefCell<Vec<Input>>>,
        }
        impl Render for Queued {
            fn style_profile(&self) -> glory::style::Profile {
                glory::style::Profile::RECORD
            }
            fn capabilities(&self) -> Capabilities {
                Capabilities::default()
            }
            fn apply(&mut self, mutations: &[Mutation]) -> std::result::Result<Receipt, Error> {
                self.memory.apply(mutations)
            }
            fn request(
                &mut self,
                query: Query,
                token: QueryToken,
            ) -> std::result::Result<(), Error> {
                self.memory.request(query, token)
            }
            fn drain(&mut self, inputs: &mut Vec<Input>) {
                inputs.append(&mut self.input.borrow_mut());
                self.memory.drain(inputs);
            }
        }
        let input = Rc::new(RefCell::new(Vec::new()));
        let queue = input.clone();
        let app = App::new(move |runtime: RuntimeId| Queued {
            memory: Memory::new(runtime),
            input: queue,
        });
        let selected = app.scope().cage(Some("a".to_owned()));
        let proposals = Rc::new(RefCell::new(Vec::new()));
        let out = proposals.clone();
        app.mount(
            select("rapid")
                .value(selected)
                .on_value_change(move |value| {
                    out.borrow_mut().push(value.clone());
                    selected.revise(|current| *current = value);
                })
                .options([
                    crate::select_option("a", "Alpha"),
                    crate::select_option("b", "Beta"),
                ]),
        )
        .unwrap();
        let trigger = trigger(&app);
        for key in ["ArrowDown", "ArrowDown", "Enter"] {
            let command = KeyboardInput {
                key,
                ..Default::default()
            }
            .choice(ComboboxKind::Select, false)
            .unwrap();
            input.borrow_mut().push(Input::Event {
                node: trigger,
                event: Event::Choice {
                    from: trigger,
                    command,
                },
            });
        }
        app.pump().unwrap();
        assert_eq!(selected.get(), Some("b".into()));
        assert_eq!(*proposals.borrow(), [Some("b".into())]);
        assert_eq!(app.tree().resolve_active_descendant(trigger).unwrap(), None);
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::Next,
            },
        )
        .unwrap();
        input.borrow_mut().extend([
            Input::Event {
                node: trigger,
                event: Event::Choice {
                    from: trigger,
                    command: ChoiceCommand::Close,
                },
            },
            Input::Event {
                node: option_node(&app, "Alpha"),
                event: Event::Activate,
            },
        ]);
        app.pump().unwrap();
        assert_eq!(selected.get(), Some("b".into()));
        assert_eq!(*proposals.borrow(), [Some("b".into())]);
    }

    #[test]
    fn finite_option_content_loading_check_position_and_dropdown_geometry_are_real() {
        let app = App::memory();
        app.mount(
            select("presented")
                .default_open(true)
                .default_value(Some("a".into()))
                .loading(true)
                .check_position(SelectCheckPosition::Right)
                .max_dropdown_height(180.0)
                .popup_placement(glory::render::AnchorPlacement::new(
                    glory::render::AnchorSide::Top,
                    glory::render::AnchorAlign::End,
                ))
                .options([
                    crate::select_option("a", "Alpha"),
                    crate::select_option("b", "Beta"),
                ])
                .option_content(
                    "a",
                    SelectOptionContent::default()
                        .leading("A")
                        .description("First option")
                        .trailing("⌘1"),
                ),
        )
        .unwrap();
        let class_count = |class: &str| {
            nodes(&app)
                .into_iter()
                .filter(|id| {
                    app.tree()
                        .node(*id)
                        .unwrap()
                        .descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|value| value == class)
                })
                .count()
        };
        assert_eq!(class_count("sapid-select-loading"), 1);
        assert_eq!(class_count("sapid-select-option-leading"), 1);
        assert_eq!(class_count("sapid-select-option-description"), 1);
        assert_eq!(class_count("sapid-select-option-trailing"), 1);
        assert_eq!(class_count("sapid-select-option-check"), 2);
        let panel = nodes(&app)
            .into_iter()
            .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Listbox))
            .unwrap();
        let tree = app.tree();
        let view = tree.node(panel).unwrap().descriptor.view();
        assert_eq!(view.style.max_height, Some(Limit::Px(180.0)));
        let LayerPlacement::Anchor(placement) = &view
            .semantics
            .as_ref()
            .unwrap()
            .layer
            .as_ref()
            .unwrap()
            .placement
        else {
            panic!("anchor placement")
        };
        assert_eq!(placement.side, glory::render::AnchorSide::Top);
        assert_eq!(placement.align, glory::render::AnchorAlign::End);
    }

    #[test]
    fn option_content_and_dropdown_height_reject_invalid_configuration_before_nodes() {
        let app = App::memory();
        assert!(
            app.mount(
                select("unknown-content")
                    .option(crate::select_option("a", "Alpha"))
                    .option_content("missing", SelectOptionContent::default().description("No")),
            )
            .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
        assert!(
            app.mount(
                select("bad-height")
                    .option(crate::select_option("a", "Alpha"))
                    .max_dropdown_height(20.0),
            )
            .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
