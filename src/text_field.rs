use glory::Scope;
use glory::core::Vary;
use glory::render::{Event, EventKind, Purpose};
use glory::ui::{Children, IntoChildren};
use glory::ui::{Result, Widget};

use crate::appearance::{Common, common_methods, configure, observe};
use crate::state::{Dirty, Value};

pub struct TextField {
    inner: Option<glory::ui::Input>,
    pub(crate) common: Common,
    pub(crate) widget_class: &'static str,
    value: Value<String>,
    on_change: Option<Box<dyn FnMut(String)>>,
    on_commit: Option<Box<dyn FnMut(String)>>,
    pub(crate) field: Option<crate::FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    success: Vary<bool>,
    loading: Vary<bool>,
    loading_position: InputSectionPosition,
    left_section: Option<Children>,
    right_section: Option<Children>,
    left_section_width: f64,
    right_section_width: f64,
    extra_error: Option<(glory::render::SemanticId, Vary<String>)>,
    placeholder: Option<Vary<String>>,
    multiline: bool,
    semantic_id: Option<glory::render::SemanticId>,
    menu_search: Option<(
        glory::render::SemanticId,
        Vary<Option<glory::render::SemanticId>>,
    )>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSectionPosition {
    Left,
    #[default]
    Right,
}

pub fn text_field() -> TextField {
    TextField {
        inner: Some(glory::ui::input()),
        common: Common::default(),
        widget_class: "sapid-text-field",
        value: Value::default(),
        on_change: None,
        on_commit: None,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        success: false.into(),
        loading: false.into(),
        loading_position: InputSectionPosition::Right,
        left_section: None,
        right_section: None,
        left_section_width: 36.,
        right_section_width: 36.,
        extra_error: None,
        placeholder: None,
        multiline: false,
        semantic_id: None,
        menu_search: None,
    }
}

impl TextField {
    pub(crate) fn in_menu(
        mut self,
        id: glory::render::SemanticId,
        owner: glory::render::SemanticId,
        active: Vary<Option<glory::render::SemanticId>>,
    ) -> Self {
        self.semantic_id = Some(id);
        self.menu_search = Some((owner, active));
        self
    }
    pub(crate) fn has_right_section(&self) -> bool {
        self.right_section.is_some()
    }
    pub(crate) fn right_section_loading(&self) -> Option<Vary<bool>> {
        (self.loading_position == InputSectionPosition::Right).then(|| self.loading.clone())
    }
    pub(crate) fn semantic_id(mut self, value: glory::render::SemanticId) -> Self {
        self.semantic_id = Some(value);
        self
    }
    pub fn placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    pub(crate) fn additional_error(
        mut self,
        id: glory::render::SemanticId,
        message: Vary<String>,
    ) -> Self {
        self.extra_error = Some((id, message));
        self
    }
    common_methods!();
    pub fn field(mut self, field: crate::FieldControl) -> Self {
        self.common.theme = field.theme.clone();
        self.common.size = field.size;
        self.common.unstyled = field.unstyled;
        self.field = Some(field);
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
    pub fn success(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.success = value.into();
        self
    }
    pub fn loading(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.loading = value.into();
        self
    }
    pub fn loading_position(mut self, value: InputSectionPosition) -> Self {
        self.loading_position = value;
        self
    }
    pub fn left_section(mut self, value: impl IntoChildren) -> Self {
        self.left_section = Some(value.into_children());
        self
    }
    pub fn right_section(mut self, value: impl IntoChildren) -> Self {
        self.right_section = Some(value.into_children());
        self
    }
    pub fn left_section_width(mut self, value: f64) -> Self {
        self.left_section_width = value;
        self
    }
    pub fn right_section_width(mut self, value: f64) -> Self {
        self.right_section_width = value;
        self
    }

    /// Controlled text. Native edits call on_value_change; the source decides
    /// which value is displayed after that host tick.
    pub fn value(mut self, value: impl Into<Vary<String>>) -> Self {
        self.value.source = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.value.default = value.into();
        self
    }

    pub fn purpose(mut self, value: impl Into<Vary<Purpose>>) -> Self {
        self.inner = self.inner.take().map(|inner| inner.purpose(value));
        self
    }

    pub fn multiline(mut self, value: bool) -> Self {
        self.multiline = value;
        self.inner = self.inner.take().map(|inner| inner.multiline(value));
        self
    }

    /// Includes composing edits. Observe EventKind::Composition separately when
    /// validation or submission should wait for composition to end.
    pub fn on_value_change(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Commit carries the host's final text. It updates local state and invokes
    /// this callback, without synthesizing an additional on_value_change call.
    pub fn on_commit(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.on_commit = Some(Box::new(handler));
        self
    }
}

impl Widget for TextField {
    fn build(&mut self, scope: &Scope) -> Result {
        if !self.left_section_width.is_finite()
            || !self.right_section_width.is_finite()
            || !(20.0..=200.0).contains(&self.left_section_width)
            || !(20.0..=200.0).contains(&self.right_section_width)
        {
            return Err(glory::render::Error::InvalidProps(
                "input section widths require finite 20..=200 CSS pixels",
            )
            .into());
        }
        let placeholder =
            crate::text_input::placeholder(scope, self.placeholder.take(), self.multiline)?;
        let control = std::mem::take(&mut self.value).mount(scope);
        let initial = control.source.try_get()?;
        let dirty = Dirty::new(scope);
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let base = self.common.theme.control(self.common.size);
        let success = self.field.as_ref().map_or_else(
            || self.success.clone(),
            |field| field.success_state(scope, self.success.clone()),
        );
        let loading_left = self.loading_position == InputSectionPosition::Left;
        let has_loading = self.loading.is_reactive() || self.loading.get();
        let has_left = self.left_section.is_some() || (has_loading && loading_left);
        let has_right = self.right_section.is_some() || (has_loading && !loading_left);
        if has_left || has_right {
            let styles = self.common.style.clone();
            let mut fallback = base.padding.unwrap_or_else(|| glory::style::Edges::all(0.));
            if has_left {
                fallback.left = fallback.left.max(self.left_section_width);
            }
            if has_right {
                fallback.right = fallback.right.max(self.right_section_width);
            }
            let left = self.left_section_width;
            let right = self.right_section_width;
            self.common.style = scope
                .bond(move || {
                    let mut style = styles.get();
                    let mut padding = style.padding.unwrap_or(fallback);
                    if has_left {
                        padding.left = padding.left.max(left);
                    }
                    if has_right {
                        padding.right = padding.right.max(right);
                    }
                    style.padding = Some(padding);
                    style
                })
                .into();
        }
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
        let metadata = if let Some(id) = self.semantic_id.take() {
            scope
                .bond(move || {
                    let mut state = metadata.get().unwrap_or_default();
                    state.id = Some(id.clone());
                    Some(state)
                })
                .into()
        } else {
            metadata
        };
        let metadata = if let Some((owner, active)) = self.menu_search.take() {
            scope
                .bond(move || {
                    let mut state = metadata.get().unwrap_or_default();
                    state.menu_search = Some(owner.clone());
                    state.controls = Some(owner.clone());
                    state.active_descendant = active.get();
                    Some(state)
                })
                .into()
        } else {
            metadata
        };
        let metadata = if let Some((id, message)) = self.extra_error.take() {
            scope
                .bond(move || {
                    let mut state = metadata.get().unwrap_or_default();
                    let error = message.with(|value| !value.is_empty());
                    state.invalid |= error;
                    if error && state.error_message.is_none() {
                        state.error_message = Some(id.clone())
                    }
                    crate::composite::metadata(*state)
                })
                .into()
        } else {
            metadata
        };
        let metadata = if let Some(placeholder) = placeholder {
            scope
                .bond(move || {
                    let mut state = metadata.get().unwrap_or_default();
                    state.placeholder = Some(placeholder.get());
                    Some(state)
                })
                .into()
        } else {
            metadata
        };
        if metadata.is_reactive()
            || enabled.is_reactive()
            || !enabled.try_get()?
            || self.required.get()
            || self.readonly.get()
            || self.invalid.get()
            || success.is_reactive()
            || success.get()
        {
            let state = metadata.clone();
            let styles = self.common.style.clone();
            let theme = self.common.theme.clone();
            let unstyled = self.common.unstyled;
            let active = enabled.clone();
            let success_color = theme.success;
            let successful = success.clone();
            self.common.style = scope
                .bond(move || {
                    let mut style = styles.get();
                    if !unstyled {
                        if !active.get() {
                            style.background.get_or_insert(theme.disabled);
                            style.color.get_or_insert(theme.disabled_text);
                        }
                        if let Some(state) = state.get() {
                            if state.invalid && style.border_color.is_none() {
                                style.border_color = Some(theme.danger);
                            } else if successful.get() && style.border_color.is_none() {
                                style.border_color = Some(success_color);
                            }
                            if state.readonly && style.background.is_none() {
                                style.background = Some(theme.disabled);
                            }
                        }
                    }
                    style
                })
                .into();
        }
        let mut inner = configure!(
            self.common,
            self.inner.take().ok_or(glory::ui::Error::Gone)?,
            scope,
            self.widget_class,
            base
        )
        .value(initial.clone())
        .enabled(enabled.clone())
        .semantics(metadata.clone())
        .node_ref(reference);

        for (kind, mut callback) in [
            (EventKind::ValueChanged, self.on_change.take()),
            (EventKind::Commit, self.on_commit.take()),
        ] {
            let mut handlers = self.common.handlers.remove(&kind).unwrap_or_default();
            let accept = control.clone();
            let pulse = dirty.clone();
            let weak = scope.downgrade();
            let owner = scope.id();
            let available = enabled.clone();
            let states = metadata.clone();
            inner = inner.on(kind, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                if let Event::ValueChanged { value, .. } | Event::Commit { value, .. } = &event {
                    match crate::field::check_available(&available, &states) {
                        Ok(true) => {}
                        Ok(false) => {
                            if let Err(error) = pulse.touch() {
                                scope.report_error(error);
                            }
                            return;
                        }
                        Err(error) => {
                            scope.report_error(error);
                            return;
                        }
                    }
                    if let Err(error) = accept.accept(value).and_then(|()| pulse.touch()) {
                        scope.report_error(error);
                        return;
                    }
                    if let Some(handler) = &mut callback {
                        handler(value.clone());
                    }
                }
                observe(&mut handlers, owner, event);
            });
        }
        let owner = scope.id();
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            inner = inner.on(kind, move |event| observe(&mut handlers, owner, event));
        }
        let decorated = self.left_section.is_some() || self.right_section.is_some() || has_loading;
        if decorated {
            let section_style = |left: bool, width: f64| {
                let mut style = glory::style::Style::new()
                    .position(glory::style::Position::Absolute)
                    .top(glory::style::Length::Px(8.))
                    .width(glory::style::Length::Px(width));
                if left {
                    style = style.left(glory::style::Length::Px(0.));
                } else {
                    style = style.right(glory::style::Length::Px(0.));
                }
                style
            };
            let mut editor = glory::ui::view()
                .class("sapid-text-field-editor")
                .style(glory::style::Style::new().position(glory::style::Position::Relative))
                .fill(inner);
            let section_container = |left, width| glory::style::Style {
                align: Some(glory::style::Align::Center),
                justify: Some(glory::style::Justify::Center),
                ..section_style(left, width)
                    .top(glory::style::Length::Px(0.))
                    .bottom(glory::style::Length::Px(0.))
                    .display(glory::style::Display::Flex)
            };
            if let Some(children) = self.left_section.take() {
                editor = editor.fill(
                    glory::ui::view()
                        .class("sapid-input-left-section")
                        .style(section_container(true, self.left_section_width))
                        .fill(children),
                );
            }
            if let Some(children) = self.right_section.take() {
                editor = editor.fill(
                    glory::ui::view()
                        .class("sapid-input-right-section")
                        .style(section_container(false, self.right_section_width))
                        .fill(children),
                );
            }
            editor = editor.fill(
                crate::spinner()
                    .theme(self.common.theme.clone())
                    .diameter(18.)
                    .thickness(2.)
                    .visible(self.loading.clone())
                    .class(if loading_left {
                        "sapid-input-loading is-left"
                    } else {
                        "sapid-input-loading is-right"
                    })
                    .style(section_style(
                        loading_left,
                        if loading_left {
                            self.left_section_width
                        } else {
                            self.right_section_width
                        },
                    )),
            );
            editor.show_in(scope)?;
        } else {
            inner.show_in(scope)?;
        }
        let node = reference.try_get()?.ok_or(glory::ui::Error::Gone)?;
        let weak = scope.downgrade();
        let mut previous = initial;
        scope.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let edited = dirty.take();
                let value = control.source.try_get()?;
                if edited || value != previous {
                    scope.set_input_value(node, value.clone())?;
                    previous = value;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        Ok(())
    }
}

impl Default for TextField {
    fn default() -> Self {
        text_field()
    }
}
