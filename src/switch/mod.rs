//! A native checkbox control with independent switch semantics and Field support.
mod visual;
use glory::Scope;
use glory::core::Vary;
use glory::render::{Change, Event, EventKind, Patch};
use glory::ui::{Result, Widget};

use crate::appearance::{Common, common_methods, configure, observe};
use crate::state::{Dirty, Value};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwitchLabelPosition {
    Before,
    #[default]
    After,
}

pub struct Switch {
    common: Common,
    value: Value<bool>,
    label: Vary<String>,
    on_change: Option<Box<dyn FnMut(bool)>>,
    field: Option<crate::FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    label_position: SwitchLabelPosition,
    marks: bool,
}

pub fn switch() -> Switch {
    Switch {
        common: Common::default(),
        value: Value::default(),
        label: "".into(),
        on_change: None,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        label_position: SwitchLabelPosition::After,
        marks: false,
    }
}

impl Switch {
    common_methods!();
    pub fn label_position(mut self, value: SwitchLabelPosition) -> Self {
        self.label_position = value;
        self
    }
    /// Show passive check/cross marks within the same track.
    pub fn with_marks(mut self, value: bool) -> Self {
        self.marks = value;
        self
    }
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

    pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.label = value.into();
        self
    }

    /// A controlled source: events propose changes without writing this source.
    pub fn checked(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.value.source = Some(value.into());
        self
    }

    pub fn default_checked(mut self, value: bool) -> Self {
        self.value.default = value;
        self
    }

    pub fn on_checked_change(mut self, handler: impl FnMut(bool) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Widget for Switch {
    fn build(&mut self, scope: &Scope) -> Result {
        let control = std::mem::take(&mut self.value).mount(scope);
        let initial = control.source.try_get()?;
        let dirty = Dirty::new(scope);
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        if self.common.name.is_none() && self.field.as_ref().is_none_or(|field| !field.has_label())
        {
            self.common.name = Some(self.label.clone());
        }
        let base = self.common.theme.checkbox(self.common.size);
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
        let focused = scope.cage(false);
        let decoration = visual::Visual::new(
            scope,
            self,
            control.source.clone(),
            enabled.clone(),
            metadata.clone(),
            focused,
        );
        self.common.class = "".into();
        let custom = self.common.style.clone();
        let label_position = self.label_position;
        let visible_label = self.label.clone();
        self.common.style = scope
            .bond(move || {
                let mut overlay = glory::style::Style::new()
                    .position(glory::style::Position::Absolute)
                    .left(glory::style::Length::Px(0.))
                    .right(glory::style::Length::Px(0.))
                    .top(glory::style::Length::Px(0.))
                    .bottom(glory::style::Length::Px(0.))
                    .display(glory::style::Display::Flex)
                    .opacity(0.);
                overlay.align = Some(glory::style::Align::Center);
                overlay.justify = Some(
                    if label_position == SwitchLabelPosition::Before
                        && !visible_label.with(String::is_empty)
                    {
                        glory::style::Justify::End
                    } else {
                        glory::style::Justify::Start
                    },
                );
                overlay.padding = custom.get().padding;
                overlay
            })
            .into();
        let mut inner = configure!(
            self.common,
            glory::ui::check(),
            scope,
            "sapid-switch-control",
            base
        )
        .mode(glory::render::CheckMode::Switch)
        .label("")
        .checked(initial)
        .enabled(enabled.clone())
        .semantics(metadata.clone())
        .node_ref(reference);

        let mut on_change = self.on_change.take();
        let mut handlers = self
            .common
            .handlers
            .remove(&EventKind::CheckedChanged)
            .unwrap_or_default();
        let accept = control.clone();
        let pulse = dirty.clone();
        let weak = scope.downgrade();
        let owner = scope.id();
        inner = inner.on(EventKind::CheckedChanged, move |event| {
            let Some(scope) = weak.upgrade() else { return };
            if let Event::CheckedChanged(value) = &event {
                match crate::field::check_available(&enabled, &metadata) {
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
                if let Some(handler) = &mut on_change {
                    handler(*value);
                }
            }
            observe(&mut handlers, owner, event);
        });
        let mut focus_observers = self
            .common
            .handlers
            .remove(&EventKind::FocusChanged)
            .unwrap_or_default();
        let weak = scope.downgrade();
        inner = inner.on(EventKind::FocusChanged, move |event| {
            if let Event::FocusChanged(value) = event
                && let Err(error) = focused.try_revise(|old| *old = value)
                && let Some(scope) = weak.upgrade()
            {
                scope.report_error(error);
            }
            observe(&mut focus_observers, owner, event);
        });
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            inner = inner.on(kind, move |event| observe(&mut handlers, owner, event));
        }
        decoration.show(scope, inner)?;
        let node = reference.try_get()?.ok_or(glory::ui::Error::Gone)?;
        let weak = scope.downgrade();
        let mut previous = initial;
        scope.effect(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let edited = dirty.take();
                let value = control.source.try_get()?;
                if edited || value != previous {
                    // An equal patch restores a rejected native toggle too.
                    scope.patch(node, Patch::Checked(Change::Set(value)))?;
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

impl Default for Switch {
    fn default() -> Self {
        switch()
    }
}
