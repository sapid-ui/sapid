//! Component-owned menu search over the shared editor and command protocol.
use super::*;

pub const MAX_MENU_SEARCH_BYTES: usize = 4096;

pub struct MenuSearch {
    common: Common,
    value: Value<String>,
    changed: Option<Box<dyn FnMut(String)>>,
    placeholder: Vary<String>,
    clear_on_close: bool,
}

pub fn menu_search() -> MenuSearch {
    MenuSearch {
        common: Common::default(),
        value: Value::default(),
        changed: None,
        placeholder: "Search items".into(),
        clear_on_close: true,
    }
}

impl MenuSearch {
    common_methods!();
    pub fn value(mut self, value: impl Into<Vary<String>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.value.default = value.into();
        self
    }
    pub fn on_value_change(mut self, changed: impl FnMut(String) + 'static) -> Self {
        self.changed = Some(Box::new(changed));
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<Vary<String>>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn clear_on_close(mut self, clear: bool) -> Self {
        self.clear_on_close = clear;
        self
    }
    pub(super) fn validate(&self) -> Result {
        if self.common.focus != FocusMode::Auto {
            return Err(invalid("menu owns its search editor's focus"));
        }
        query(
            &self
                .value
                .source
                .as_ref()
                .map_or_else(|| Ok(self.value.default.clone()), Vary::try_get)?,
        )
    }
    pub(super) fn prepare(
        &mut self,
        scope: &Scope,
        namespace: &str,
        path: &[String],
    ) -> Result<Rc<SearchState>> {
        self.validate()?;
        let value = std::mem::take(&mut self.value).mount(scope);
        let accepted = composite::validated(scope, &value.source, |value| query(value))?;
        Ok(Rc::new(SearchState {
            value,
            accepted,
            changed: Callback::new(self.changed.take()),
            editor: self.common.node_ref.unwrap_or_else(|| scope.cage(None)),
            id: id(namespace, path, "search")?,
            highlighted: scope.cage(None),
            clear_on_close: self.clear_on_close,
        }))
    }
    pub(super) fn show(
        mut self,
        scope: &Scope,
        parent: &Common,
        menu_id: SemanticId,
        state: Rc<MenuState>,
    ) -> Result {
        let editor = state
            .search_editor
            .as_ref()
            .ok_or(glory::ui::Error::Gone)?
            .clone();
        self.common.theme = parent.theme.clone();
        self.common.size = parent.size;
        self.common.unstyled |= parent.unstyled;
        self.common.name.get_or_insert_with(|| "Search menu".into());
        self.common.node_ref = Some(editor.editor);
        let root_enabled = state.enabled.clone();
        let enabled = self.common.enabled.clone();
        self.common.enabled = scope
            .bond(move || root_enabled.get() && enabled.get())
            .into();
        let own = self.common.style.clone();
        self.common.style = scope
            .bond(move || {
                crate::appearance::merge(
                    glory::style::Style::new()
                        .width(Length::Percent(100.))
                        .min_width(Length::Px(0.)),
                    own.get(),
                )
            })
            .into();
        let source = editor.accepted;
        let live = state.clone();
        let active: Vary<Option<SemanticId>> = scope.bond(move || live.active_search_name()).into();
        let weak = scope.downgrade();
        let availability = self.common.enabled.clone();
        if let Some(mut observers) = self.common.handlers.remove(&EventKind::Menu) {
            let commands = state.clone();
            let weak = scope.downgrade();
            let owner = scope.id();
            self.common.handlers.insert(
                EventKind::Menu,
                vec![Box::new(move |event| {
                    let Some(scope) = weak.upgrade() else { return };
                    if let Event::Menu { from, command } = &event
                        && let Err(error) = commands.command(&scope, *from, *command)
                    {
                        scope.report_error(error);
                        return;
                    }
                    observe(&mut observers, owner, event);
                })],
            );
        }
        let mut input = crate::text_field()
            .purpose(glory::render::Purpose::Search)
            .value(source)
            .placeholder(self.placeholder)
            .in_menu(editor.id.clone(), menu_id, active)
            .on_value_change(move |value| {
                let Some(scope) = weak.upgrade() else { return };
                if !state.owner.is_alive() || !state.visible.get() || !availability.get() {
                    return;
                }
                let result = (|| -> Result {
                    query(&value)?;
                    editor.highlighted.try_revise(|current| *current = None)?;
                    editor.value.accept(&value)?;
                    editor.changed.emit(state.owner, value);
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            });
        input.common = self.common;
        input.widget_class = "sapid-text-field sapid-menu-search";
        input.show_in(scope)
    }
}

fn query(value: &str) -> Result {
    if value.len() > MAX_MENU_SEARCH_BYTES {
        Err(invalid("menu search exceeds 4096 UTF-8 bytes"))
    } else {
        Ok(())
    }
}

pub(super) struct SearchState {
    value: Control<String>,
    accepted: Cage<String>,
    changed: Callback<String>,
    pub editor: Cage<Option<NodeId>>,
    pub id: SemanticId,
    pub highlighted: Cage<Option<NodeId>>,
    clear_on_close: bool,
}
impl SearchState {
    pub fn close(&self, owner: glory::core::InstanceId) -> Result {
        if self.clear_on_close
            && owner.is_alive()
            && self.value.source.try_with(|value| !value.is_empty())?
        {
            self.value.accept(&String::new())?;
            self.changed.emit(owner, String::new());
        }
        Ok(())
    }
}

type Handler = RefCell<Box<dyn FnMut(Event)>>;
#[derive(Clone)]
pub(super) struct Activation {
    pub node: NodeId,
    pub name: SemanticId,
    pub kind: MenuItemKind,
    pub checked: Option<Vary<bool>>,
    pub callback: Weak<Handler>,
}
impl Activation {
    fn invoke(self) -> Result {
        let Some(callback) = self.callback.upgrade() else {
            return Ok(());
        };
        let event = match self.kind {
            MenuItemKind::Action => Event::Activate,
            MenuItemKind::Radio => {
                if self
                    .checked
                    .ok_or(invalid("radio menu action has no current value"))?
                    .try_get()?
                {
                    return Ok(());
                }
                Event::CheckedChanged(true)
            }
            MenuItemKind::Checkbox => Event::CheckedChanged(
                !self
                    .checked
                    .ok_or(invalid("checkbox menu action has no current value"))?
                    .try_get()?,
            ),
        };
        callback.borrow_mut()(event);
        Ok(())
    }
}

pub(super) fn handler(callback: impl FnMut(Event) + 'static) -> Rc<Handler> {
    Rc::new(RefCell::new(Box::new(callback)))
}

pub(super) fn highlight_style(
    scope: &Scope,
    state: &MenuState,
    common: &mut Common,
    reference: Cage<Option<NodeId>>,
) {
    let Some(editor) = &state.search_editor else {
        return;
    };
    let highlighted = editor.highlighted;
    let own = common.style.clone();
    let theme = common.theme.clone();
    let enabled = common.enabled.clone();
    let plain = common.unstyled;
    common.style = scope
        .bond(move || {
            let paint = if !plain
                && enabled.get()
                && reference
                    .get()
                    .is_some_and(|node| highlighted.get() == Some(node))
            {
                glory::style::Style {
                    background: Some(theme.accent_soft),
                    color: Some(theme.accent_text),
                    ..Default::default()
                }
            } else {
                glory::style::Style::new()
            };
            crate::appearance::merge(paint, own.get())
        })
        .into();
}

impl MenuState {
    fn active_search_name(&self) -> Option<SemanticId> {
        if !self.visible.get() {
            return None;
        }
        let current = self.search_editor.as_ref()?.highlighted.get()?;
        self.activations
            .borrow()
            .iter()
            .find(|item| item.node == current)
            .map(|item| item.name.clone())
    }
    pub(super) fn search_command(
        &self,
        scope: &Scope,
        from: NodeId,
        command: MenuCommand,
    ) -> Result {
        let Some(editor) = &self.search_editor else {
            return Ok(());
        };
        if !self.visible.try_get()?
            || !self.enabled.try_get()?
            || editor.editor.try_get()? != Some(from)
        {
            return Ok(());
        }
        self.cancel_pending();
        let items = self.collection.mounted()?;
        let current = editor.highlighted.try_get()?;
        if command == MenuCommand::SearchCommit {
            if let Some(current) = current
                && items
                    .iter()
                    .any(|item| item.node == current && item.enabled)
            {
                let activation = self
                    .activations
                    .borrow()
                    .iter()
                    .find(|item| item.node == current)
                    .cloned();
                if let Some(activation) = activation {
                    activation.invoke()?;
                }
            }
            return Ok(());
        }
        let target = if let Some(current) = current.filter(|current| {
            items
                .iter()
                .any(|item| item.node == *current && item.enabled)
        }) {
            self.roving.destination(
                current,
                if command == MenuCommand::SearchNext {
                    Navigation::Next
                } else {
                    Navigation::Previous
                },
            )?
        } else if command == MenuCommand::SearchPrevious {
            items.into_iter().rev().find(|item| item.enabled)
        } else {
            items.into_iter().find(|item| item.enabled)
        };
        if let Some(item) = target {
            self.close_children(None)?;
            if self.owner.is_alive() {
                self.focus_item(scope, item)?;
            }
        }
        Ok(())
    }
}
