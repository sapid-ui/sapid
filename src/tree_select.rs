//! A select-only intrinsic combobox whose popup contains real TreeItems.
use crate::{
    appearance::{Common, common_methods, configure},
    composite::{self, Callback, Observers},
    state::{Control, Dirty, Value},
    tree::TreePopup,
    tree_choice_model::*,
    tree_model::*,
};
use glory::render::{
    AnchorPlacement, ChoiceCommand, ComboboxKind, Direction, Event, EventKind, FormControl, Layer,
    LayerFocus, LayerPlacement, PopupKind, Props, Semantics,
};
use glory::style::{BorderStyle, Edges, Length, Limit, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
    rc::Rc,
};

fn error(error: TreeError) -> glory::ui::Error {
    glory::render::Error::InvalidProps(error.message()).into()
}
#[derive(Clone)]
struct Accepted {
    model: Rc<TreeChoiceModel>,
    values: TreeChoices,
    expanded: BTreeSet<String>,
}
fn admit(
    scope: &Scope,
    nodes: Vary<Vec<TreeNode>>,
    values: Vary<TreeChoices>,
    policy: TreeChoicePolicy,
    multiple: bool,
    columns: Option<Vary<BTreeSet<String>>>,
) -> Result<Cage<Accepted>> {
    fn read(
        nodes: &Vary<Vec<TreeNode>>,
        values: &Vary<TreeChoices>,
        policy: TreeChoicePolicy,
        multiple: bool,
        previous: Option<&Accepted>,
        columns: &Option<Vary<BTreeSet<String>>>,
    ) -> Result<Accepted> {
        let model = nodes.try_with(|nodes| {
            TreeModel::validate(nodes).map_err(error)?;
            if let Some(old) = previous.filter(|old| old.model.tree().nodes() == nodes) {
                Ok(old.model.clone())
            } else {
                TreeChoiceModel::new(
                    TreeModel::new(nodes.clone()).map_err(error)?,
                    policy,
                    multiple,
                )
                .map(Rc::new)
                .map_err(error)
            }
        })??;
        let values = values.try_get()?;
        model.validate(&values).map_err(error)?;
        let expanded = columns
            .as_ref()
            .map(|source| source.try_get())
            .transpose()?
            .unwrap_or_default();
        if columns.is_some() {
            model.validate_columns(&expanded).map_err(error)?;
        }
        Ok(Accepted {
            model,
            values,
            expanded,
        })
    }
    let accepted = scope.cage(read(&nodes, &values, policy, multiple, None, &columns)?);
    let weak = scope.downgrade();
    scope.structural(move || {
        let Some(scope) = weak.upgrade() else { return };
        let old = accepted.get_untracked();
        match read(&nodes, &values, policy, multiple, Some(&old), &columns) {
            Ok(next)
                if !Rc::ptr_eq(&old.model, &next.model)
                    || old.values != next.values
                    || old.expanded != next.expanded =>
            {
                accepted.revise(|old| *old = next)
            }
            Ok(_) => {}
            Err(error) => scope.report_error(error),
        }
    });
    Ok(accepted)
}

fn visible_nodes(
    nodes: &[TreeNode],
    query: &str,
    choices: &TreeChoices,
    hide_picked: bool,
) -> Result<Vec<TreeNode>> {
    let filtered = filter_tree_nodes(nodes, query).map_err(error)?;
    if !hide_picked || choices.is_empty() {
        return Ok(filtered);
    }
    let hidden = choices
        .paths()
        .iter()
        .map(TreePath::leaf)
        .collect::<BTreeSet<_>>();
    let retained = filtered
        .iter()
        .filter(|node| {
            let mut current = Some(node.key.as_str());
            while let Some(key) = current {
                if hidden.contains(key) {
                    return false;
                }
                current = filtered
                    .iter()
                    .find(|candidate| candidate.key == key)
                    .and_then(|candidate| candidate.parent.as_deref());
            }
            true
        })
        .cloned()
        .collect::<Vec<_>>();
    TreeModel::validate(&retained).map_err(error)?;
    Ok(retained)
}

struct State {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    accepted: Cage<Accepted>,
    value: Control<TreeChoices>,
    open: Control<bool>,
    search: Control<String>,
    visible: Vary<bool>,
    editable: Vary<bool>,
    pending: RefCell<Option<TreeChoices>>,
    dirty: Dirty,
    changed: Callback<TreeChoices>,
    searched: Callback<String>,
    opened: Callback<bool>,
    within: Cell<Option<bool>>,
    max_values: usize,
}
impl State {
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error)
        }
    }
    fn open(&self, value: bool) -> Result {
        if !self.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        if value && !self.editable.get() {
            return Ok(());
        }
        self.open.accept(&value)?;
        self.opened.emit(self.owner, value);
        if !value && !self.search.source.get().is_empty() {
            self.search.accept(&String::new())?;
            self.searched.emit(self.owner, String::new());
        }
        Ok(())
    }
    fn edit_search(&self, value: String) -> Result {
        if !self.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        if !self.editable.get() {
            return Ok(());
        }
        if value.len() > MAX_TREE_LABEL_BYTES || value.contains(['\0', '\r']) {
            return Err(error(TreeError::State));
        }
        self.search.accept(&value)?;
        self.searched.emit(self.owner, value);
        self.open(true)
    }
    fn toggle(&self, key: &str) -> Result {
        if !self.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        if !self.editable.get() || !self.visible.get() {
            return Ok(());
        }
        let accepted = self.accepted.get_untracked();
        let current = self
            .pending
            .borrow()
            .clone()
            .unwrap_or_else(|| accepted.values.clone());
        let next = accepted.model.toggle(&current, key).map_err(error)?;
        if next.paths().len() > self.max_values && next.paths().len() > current.paths().len() {
            return Ok(());
        }
        if next == current {
            let node = accepted
                .model
                .tree()
                .index(key)
                .map(|index| &accepted.model.tree().nodes()[index]);
            if !accepted.model.multiple()
                && node.is_some_and(|node| {
                    !node.disabled
                        && (node.kind == TreeKind::Leaf
                            || accepted.model.policy() == TreeChoicePolicy::AnyNode)
                })
            {
                self.open(false)?;
            }
            return Ok(());
        }
        self.pending.replace(Some(next.clone()));
        self.value.accept(&next)?;
        self.dirty.touch()?;
        self.changed.emit(self.owner, next);
        if self.owner.is_alive() && !accepted.model.multiple() {
            self.open(false)?
        }
        Ok(())
    }
    fn clear(&self) -> Result {
        if !self.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        if !self.editable.get() {
            return Ok(());
        }
        let next = TreeChoices::default();
        let current = self
            .pending
            .borrow()
            .clone()
            .unwrap_or_else(|| self.accepted.with_untracked(|a| a.values.clone()));
        if next == current {
            return Ok(());
        }
        self.pending.replace(Some(next.clone()));
        self.value.accept(&next)?;
        self.dirty.touch()?;
        self.changed.emit(self.owner, next);
        Ok(())
    }
}

type NodeFactory = Rc<RefCell<Box<dyn FnMut(Vary<TreeNode>) -> Box<dyn Widget>>>>;

pub struct TreeSelect {
    common: Common,
    key: String,
    nodes: Option<Vary<Vec<TreeNode>>>,
    value: Value<TreeChoices>,
    open: Value<bool>,
    search: Value<String>,
    expanded: Value<BTreeSet<String>>,
    changed: Option<Box<dyn FnMut(TreeChoices)>>,
    opened: Option<Box<dyn FnMut(bool)>>,
    searched: Option<Box<dyn FnMut(String)>>,
    expansion: Option<Box<dyn FnMut(BTreeSet<String>)>>,
    load: Option<Box<dyn FnMut(String)>>,
    policy: TreeChoicePolicy,
    multiple: bool,
    direction: Direction,
    placeholder: String,
    clearable: bool,
    searchable: bool,
    hide_picked: bool,
    max_values: usize,
    nothing_found: String,
    with_lines: bool,
    max_dropdown_height: f64,
    popup_width: f64,
    popup_placement: AnchorPlacement,
    render_node: Option<NodeFactory>,
    render_value: Option<NodeFactory>,
    field: Option<crate::FieldControl>,
    form_name: Option<Vary<String>>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    columns: bool,
}
pub fn tree_select(key: impl Into<String>) -> TreeSelect {
    TreeSelect {
        common: Common::default(),
        key: key.into(),
        nodes: None,
        value: Value::default(),
        open: Value::default(),
        search: Value::default(),
        expanded: Value::default(),
        changed: None,
        opened: None,
        searched: None,
        expansion: None,
        load: None,
        policy: TreeChoicePolicy::Leaves,
        multiple: false,
        direction: Direction::Ltr,
        placeholder: "Choose an item".into(),
        clearable: false,
        searchable: false,
        hide_picked: false,
        max_values: MAX_TREE_CHOICES,
        nothing_found: "Nothing found".into(),
        with_lines: true,
        max_dropdown_height: 288.,
        popup_width: 320.,
        popup_placement: AnchorPlacement::default(),
        render_node: None,
        render_value: None,
        field: None,
        form_name: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        columns: false,
    }
}
impl TreeSelect {
    common_methods!();
    pub(crate) fn cascading_columns(mut self) -> Self {
        self.columns = true;
        self
    }
    pub fn nodes(mut self, value: impl Into<Vary<Vec<TreeNode>>>) -> Self {
        self.nodes = Some(value.into());
        self
    }
    pub fn value(mut self, value: impl Into<Vary<TreeChoices>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: TreeChoices) -> Self {
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
    pub fn search(mut self, value: impl Into<Vary<String>>) -> Self {
        self.search.source = Some(value.into());
        self
    }
    pub fn default_search(mut self, value: impl Into<String>) -> Self {
        self.search.default = value.into();
        self
    }
    pub fn expanded(mut self, value: impl Into<Vary<BTreeSet<String>>>) -> Self {
        self.expanded.source = Some(value.into());
        self
    }
    pub fn default_expanded(mut self, value: BTreeSet<String>) -> Self {
        self.expanded.default = value;
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(TreeChoices) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_open_change(mut self, value: impl FnMut(bool) + 'static) -> Self {
        self.opened = Some(Box::new(value));
        self
    }
    pub fn on_search_change(mut self, value: impl FnMut(String) + 'static) -> Self {
        self.searched = Some(Box::new(value));
        self
    }
    pub fn on_expanded_change(mut self, value: impl FnMut(BTreeSet<String>) + 'static) -> Self {
        self.expansion = Some(Box::new(value));
        self
    }
    pub fn on_load_children(mut self, value: impl FnMut(String) + 'static) -> Self {
        self.load = Some(Box::new(value));
        self
    }
    pub fn policy(mut self, value: TreeChoicePolicy) -> Self {
        self.policy = value;
        self
    }
    pub fn multiple(mut self, value: bool) -> Self {
        self.multiple = value;
        self
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
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
    pub fn searchable(mut self, value: bool) -> Self {
        self.searchable = value;
        self
    }
    pub fn hide_picked(mut self, value: bool) -> Self {
        self.hide_picked = value;
        self
    }
    pub fn max_values(mut self, value: usize) -> Self {
        self.max_values = value;
        self
    }
    pub fn nothing_found_message(mut self, value: impl Into<String>) -> Self {
        self.nothing_found = value.into();
        self
    }
    pub fn with_lines(mut self, value: bool) -> Self {
        self.with_lines = value;
        self
    }
    pub fn max_dropdown_height(mut self, value: f64) -> Self {
        self.max_dropdown_height = value;
        self
    }
    pub fn popup_width(mut self, value: f64) -> Self {
        self.popup_width = value;
        self
    }
    pub fn popup_placement(mut self, value: AnchorPlacement) -> Self {
        self.popup_placement = value;
        self
    }
    pub fn render_node<W: Widget>(
        mut self,
        mut render: impl FnMut(Vary<TreeNode>) -> W + 'static,
    ) -> Self {
        self.render_node = Some(Rc::new(RefCell::new(Box::new(move |node| {
            Box::new(render(node))
        }))));
        self
    }
    pub fn render_value<W: Widget>(
        mut self,
        mut render: impl FnMut(Vary<TreeNode>) -> W + 'static,
    ) -> Self {
        self.render_value = Some(Rc::new(RefCell::new(Box::new(move |node| {
            Box::new(render(node))
        }))));
        self
    }
    pub fn field(mut self, value: crate::FieldControl) -> Self {
        self.common.theme = value.theme.clone();
        self.common.size = value.size;
        self.common.unstyled = value.unstyled;
        self.field = Some(value);
        self
    }
    pub fn form_name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(value.into());
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
}
impl Widget for TreeSelect {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.placeholder.len() > 256
            || self.placeholder.contains(['\0', '\r', '\n'])
            || self.nothing_found.len() > 256
            || self.nothing_found.contains(['\0', '\r'])
            || self.max_values == 0
            || self.max_values > MAX_TREE_CHOICES
            || !self.max_dropdown_height.is_finite()
            || !(80.0..=4096.0).contains(&self.max_dropdown_height)
            || !self.popup_width.is_finite()
            || !(160.0..=4096.0).contains(&self.popup_width)
        {
            return Err(error(TreeError::Label));
        }
        self.popup_placement.validate()?;
        let trigger_id = self
            .field
            .as_ref()
            .map(|field| field.id.clone())
            .map(Ok)
            .unwrap_or_else(|| composite::id("tree-select", &self.key, "trigger", "root"))?;
        let panel_id = composite::id("tree-select", &self.key, "popup", "root")?;
        let value = std::mem::take(&mut self.value).mount(scope);
        let expanded = std::mem::take(&mut self.expanded).mount(scope);
        let accepted = admit(
            scope,
            self.nodes.take().ok_or_else(|| error(TreeError::Nodes))?,
            value.source.clone(),
            self.policy,
            self.multiple,
            self.columns.then(|| expanded.source.clone()),
        )?;
        let open = std::mem::take(&mut self.open).mount(scope);
        let search = std::mem::take(&mut self.search).mount(scope);
        let accepted_search = composite::validated(scope, &search.source, |value| {
            if value.len() > MAX_TREE_LABEL_BYTES || value.contains(['\0', '\r']) {
                Err(error(TreeError::State))
            } else {
                Ok(())
            }
        })?;
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
        let meta = metadata.clone();
        let enabled_for_edit = enabled.clone();
        let editable: Vary<bool> = scope
            .bond(move || enabled_for_edit.get() && !meta.get().is_some_and(|s| s.readonly))
            .into();
        let open_source = open.source.clone();
        let edit = editable.clone();
        let visible: Vary<bool> = scope.bond(move || open_source.get() && edit.get()).into();
        let state = Rc::new(State {
            owner: scope.id(),
            weak: scope.downgrade(),
            accepted,
            value,
            open,
            search,
            visible: visible.clone(),
            editable: editable.clone(),
            pending: RefCell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            searched: Callback::new(self.searched.take()),
            opened: Callback::new(self.opened.take()),
            within: Cell::new(None),
            max_values: self.max_values,
        });
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                state.changed.clear();
                state.searched.clear();
                state.opened.clear();
                state.pending.take();
            }
        })?;
        let weak = Rc::downgrade(&state);
        let dirty = state.dirty.clone();
        scope.effect(move || {
            if dirty.take()
                && let Some(state) = weak.upgrade()
            {
                state.pending.take();
                // Moving between two live items queues loss then gain in one
                // input tick. Close only after its final focus observation.
                if state.within.get() == Some(false) && state.visible.get() {
                    state.report(state.open(false));
                }
            }
        });
        let label = accepted.map(move |a| {
            a.values
                .paths()
                .iter()
                .map(|path| {
                    let key = path.leaf();
                    a.model
                        .tree()
                        .index(key)
                        .map(|i| a.model.tree().nodes()[i].label.clone())
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join(", ")
        });
        let button_placeholder = self.placeholder.clone();
        let button_label = scope.bond(move || {
            let value = label.get();
            if value.is_empty() {
                button_placeholder.clone()
            } else {
                value
            }
        });
        let name = trigger_id.clone();
        let panel = panel_id.clone();
        let shown = visible.clone();
        let form_meta = metadata.clone();
        let semantic_placeholder = self.placeholder.clone();
        let semantic_searchable = self.searchable;
        let semantic_kind = if self.searchable {
            ComboboxKind::Editable
        } else {
            ComboboxKind::Select
        };
        let trigger_meta = scope.bond(move || {
            let mut s = form_meta.get().unwrap_or_default();
            s.id = Some(name.clone());
            s.form_control = None;
            s.combobox = Some(semantic_kind);
            s.popup = Some(PopupKind::Tree);
            s.controls = Some(panel.clone());
            s.expanded = Some(shown.get());
            if semantic_searchable {
                s.placeholder = Some(semantic_placeholder.clone());
            }
            Some(s)
        });
        let root = scope.create(glory::render::Descriptor::new(Props::Group))?;
        let content = scope.child_in(root)?;
        let control = self.common.theme.control(self.common.size);
        let padding = control.padding.unwrap_or(Edges::all(0.));
        let base = Style {
            display: Some(glory::style::Display::Flex),
            align: Some(glory::style::Align::Center),
            grow: Some(1.),
            min_width: Some(Length::Px(0.)),
            justify: Some(glory::style::Justify::Start),
            padding: Some(Edges {
                right: 36.,
                ..padding
            }),
            ..control
        };
        let activation = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default(),
        );
        let clear = activation.clone();
        scope.on_cleanup(move || clear.clear())?;
        let choices = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Choice)
                .unwrap_or_default(),
        );
        let clear = choices.clone();
        scope.on_cleanup(move || clear.clear())?;
        let trigger: Box<dyn Widget> = if self.searchable {
            let search_visible = visible.clone();
            let display = scope.bond(move || {
                if search_visible.get() {
                    accepted_search.get()
                } else {
                    label.get()
                }
            });
            let mut trigger = configure!(
                self.common,
                glory::ui::input(),
                scope,
                "sapid-tree-select sapid-tree-select-searchable",
                base
            )
            .value(display)
            .enabled(enabled.clone())
            .semantics(trigger_meta);
            for kind in [EventKind::ValueChanged, EventKind::Commit] {
                let s = state.clone();
                let observers =
                    Observers::new(self.common.handlers.remove(&kind).unwrap_or_default());
                let clear = observers.clone();
                scope.on_cleanup(move || clear.clear())?;
                trigger = trigger.on(kind, move |event| {
                    if let Event::ValueChanged { value, .. } | Event::Commit { value, .. } = &event
                    {
                        s.report(s.edit_search(value.clone()));
                    }
                    observers.emit(s.owner, event);
                });
            }
            let s = state.clone();
            trigger = trigger.on(EventKind::FocusChanged, move |event| {
                if matches!(event, Event::FocusChanged(true)) {
                    s.report(s.open(true));
                }
            });
            let s = state.clone();
            trigger = trigger.on(EventKind::Choice, move |event| {
                if let Event::Choice { command, .. } = &event {
                    match command {
                        ChoiceCommand::Close => s.report(s.open(false)),
                        _ => s.report(s.open(true)),
                    }
                }
                choices.emit(s.owner, event);
            });
            for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
                let observers = Observers::new(handlers);
                let clear = observers.clone();
                scope.on_cleanup(move || clear.clear())?;
                let owner = scope.id();
                trigger = trigger.on(kind, move |event| observers.emit(owner, event));
            }
            Box::new(trigger)
        } else {
            let mut trigger = configure!(
                self.common,
                glory::ui::button(),
                scope,
                "sapid-tree-select",
                base
            )
            .label(button_label)
            .enabled(enabled.clone())
            .semantics(trigger_meta);
            if let Some(renderer) = self.render_value.clone() {
                let values = accepted.map(|a| {
                    a.values
                        .paths()
                        .iter()
                        .filter_map(|path| {
                            a.model
                                .tree()
                                .index(path.leaf())
                                .map(|index| a.model.tree().nodes()[index].clone())
                        })
                        .collect::<Vec<_>>()
                });
                trigger = trigger.fill(
                    glory::ui::each(values)
                        .key(|node| node.key.clone())
                        .children(move |entry| (renderer.borrow_mut())(entry.value.into())),
                );
            }
            let s = state.clone();
            trigger = trigger.on(EventKind::Activate, move |event| {
                s.report(s.open(!s.visible.get()));
                activation.emit(s.owner, event);
            });
            let s = state.clone();
            trigger = trigger.on(EventKind::Choice, move |event| {
                if let Event::Choice { command, .. } = &event {
                    match command {
                        ChoiceCommand::Close => s.report(s.open(false)),
                        _ => s.report(s.open(true)),
                    }
                }
                choices.emit(s.owner, event);
            });
            for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
                let observers = Observers::new(handlers);
                let clear = observers.clone();
                scope.on_cleanup(move || clear.clear())?;
                let owner = scope.id();
                trigger = trigger.on(kind, move |event| observers.emit(owner, event));
            }
            Box::new(trigger)
        };
        let hidden = visible.clone();
        let s = state.clone();
        let focus_state = state.clone();
        let hide_picked = self.hide_picked;
        let nodes = scope.bond(move || {
            accepted.with(|a| {
                visible_nodes(
                    a.model.tree().nodes(),
                    &accepted_search.get(),
                    &a.values,
                    hide_picked,
                )
                .expect("accepted tree-select filtering")
            })
        });
        let selection = accepted.map(|a| {
            a.values
                .paths()
                .iter()
                .map(|path| path.leaf().to_owned())
                .collect::<BTreeSet<_>>()
        });
        let mut tree = crate::tree(&self.key)
            .theme(self.common.theme.clone())
            .name("Choices")
            .class("sapid-tree-select-popup")
            .height(self.max_dropdown_height)
            .with_lines(self.with_lines)
            .empty_message(self.nothing_found.clone())
            .nodes(nodes)
            .selected(selection)
            .selection_mode(if self.multiple {
                TreeSelectionMode::Multiple
            } else {
                TreeSelectionMode::Single
            })
            .direction(self.direction)
            .enabled(editable)
            .on_selection_intent(move |key| s.report(s.toggle(&key)))
            .popup_root(TreePopup {
                id: panel_id,
                labelled_by: trigger_id.clone(),
                hidden: scope.bond(move || !hidden.get()).into(),
                layer: Layer {
                    modal: false,
                    focus: if self.searchable {
                        LayerFocus::Preserve
                    } else {
                        LayerFocus::Manage
                    },
                    placement: LayerPlacement::Anchor(self.popup_placement),
                    anchor: Some(trigger_id.clone()),
                    return_focus: (!self.searchable).then_some(trigger_id),
                    ..Default::default()
                },
            })
            .on(EventKind::FocusWithin, move |event| {
                if let Event::FocusWithin { focused, .. } = event {
                    focus_state.within.set(Some(focused));
                    focus_state.report(focus_state.dirty.touch());
                }
            });
        let mut popup_style = Style::new()
            .min_width(Length::Px(180.))
            .width(Length::Px(if self.columns {
                560.
            } else {
                self.popup_width
            }))
            .max_width(Limit::Percent(95.));
        if !self.common.unstyled {
            popup_style = popup_style
                .border_width(Edges::all(1.))
                .border_style(BorderStyle::Solid)
                .border_color(self.common.theme.border);
        }
        tree = tree.style(popup_style);
        if self.common.unstyled {
            tree = tree.unstyled();
        }
        if self.columns {
            tree = tree.column_layout();
        }
        if let Some(renderer) = self.render_node.take() {
            tree = tree.render_label(move |node| (renderer.borrow_mut())(node));
        }
        if self.multiple {
            tree = tree
                .checked(accepted.map(|a| a.model.checked(&a.values).expect("admitted choices")));
        }
        tree = tree.expanded(expanded.source.clone());
        let expansion = Callback::new(self.expansion.take());
        let cleanup = expansion.clone();
        scope.on_cleanup(move || cleanup.clear())?;
        let owner = scope.id();
        let weak = scope.downgrade();
        tree = tree.on_expanded_change(move |value| {
            if let Err(error) = expanded.accept(&value) {
                if let Some(scope) = weak.upgrade() {
                    scope.report_error(error)
                }
                return;
            }
            expansion.emit(owner, value);
        });
        if let Some(handler) = self.load.take() {
            tree = tree.on_load_children(handler)
        }
        let s = state.clone();
        tree.on(EventKind::Dismiss, move |_| s.report(s.open(false)))
            .show_in(&content)?;
        let editable = state.editable.clone();
        let clear_visible: Vary<bool> = if self.clearable {
            scope
                .bond(move || editable.get() && !accepted.with(|a| a.values.is_empty()))
                .into()
        } else {
            false.into()
        };
        let action_color = self.common.theme.muted;
        let mut controls: Vec<Box<dyn Widget>> = vec![trigger];
        if self.clearable {
            let s = state.clone();
            let clear_style = clear_visible.clone();
            controls.push(Box::new(
                crate::close_button()
                    .theme(self.common.theme.clone())
                    .size(crate::Size::Small)
                    .unstyled()
                    .class("sapid-tree-select-clear")
                    .name("Clear tree selection")
                    .enabled(clear_visible.clone())
                    .style(scope.bond(move || Style {
                        position: Some(glory::style::Position::Absolute),
                        top: Some(Length::Px(0.)),
                        right: Some(Length::Px(4.)),
                        display: Some(if clear_style.get() {
                            glory::style::Display::Flex
                        } else {
                            glory::style::Display::None
                        }),
                        align: Some(glory::style::Align::Center),
                        justify: Some(glory::style::Justify::Center),
                        width: Some(Length::Px(28.)),
                        height: Some(Length::Percent(100.)),
                        padding: Some(Edges::all(0.)),
                        border_width: Some(Edges::all(0.)),
                        background: Some(glory::style::Color(0, 0, 0, 0)),
                        color: Some(action_color),
                        ..Style::new()
                    }))
                    .on_press(move || s.report(s.clear())),
            ));
        }
        controls.push(Box::new(
            crate::theme_icon(crate::Icon::ChevronDown)
                .theme(self.common.theme.clone())
                .size(crate::Size::ExtraSmall)
                .unstyled()
                .color(action_color)
                .class("sapid-tree-select-chevron")
                .style(scope.bond(move || Style {
                    position: Some(glory::style::Position::Absolute),
                    top: Some(Length::Px(0.)),
                    right: Some(Length::Px(10.)),
                    display: Some(if clear_visible.get() {
                        glory::style::Display::None
                    } else {
                        glory::style::Display::Flex
                    }),
                    align: Some(glory::style::Align::Center),
                    justify: Some(glory::style::Justify::Center),
                    width: Some(Length::Px(16.)),
                    height: Some(Length::Percent(100.)),
                    pointer_events: Some(glory::style::PointerEvents::None),
                    ..Style::new()
                })),
        ));
        glory::ui::view()
            .class("sapid-tree-select-control")
            .style(Style {
                position: Some(glory::style::Position::Relative),
                display: Some(glory::style::Display::Flex),
                align: Some(glory::style::Align::Center),
                width: Some(Length::Percent(100.)),
                ..Style::new()
            })
            .fill(controls)
            .show_in(&content)?;
        // Only current accepted values are successful. Real readonly Inputs
        // live under a hidden semantic View; no programmatic file/DOM injection.
        let form_name = composite::form_name(scope, &self.form_name)?;
        let hidden_ref = scope.cage(None);
        glory::ui::view()
            .node_ref(hidden_ref)
            .semantics(Some(Box::new(Semantics {
                hidden: true,
                ..Default::default()
            })))
            .show_in(&content)?;
        let proxies = scope.child_in(hidden_ref.get().ok_or(glory::ui::Error::Gone)?)?;
        let form_metadata = metadata;
        glory::ui::each(accepted.map(|a| a.values.paths().to_vec()))
            .key(|path| path.leaf().to_owned())
            .children(move |entry| {
                let path = entry.value;
                let meta = form_metadata.clone();
                let name = form_name;
                let enabled = enabled.clone();
                glory::ui::widget(move |scope| {
                    let meta = meta.clone();
                    glory::ui::input()
                        .value(path.map(|path| path.keys().join("/")))
                        .enabled(enabled.clone())
                        .semantics(scope.bond(move || {
                            let control = name
                                .map(|name| FormControl {
                                    name: name.get(),
                                    value: None,
                                })
                                .or_else(|| meta.get().and_then(|s| s.form_control));
                            Some(Box::new(Semantics {
                                readonly: true,
                                form_control: control,
                                ..Default::default()
                            }))
                        }))
                        .show_in(scope)
                })
            })
            .show_in(&proxies)?;
        content.retain(proxies)?;
        scope.retain(content)
    }
}
