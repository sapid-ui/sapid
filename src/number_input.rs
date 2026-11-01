//! Exact decimal editing over an actual text spinbutton.
use crate::appearance::{Common, common_methods, configure};
use crate::composite::{self, Callback, Observers};
use crate::field::control_metadata;
use crate::number_model::NumberDomain;
use crate::state::{Control, Dirty, Value};
use crate::{Decimal, FieldControl, NumberError, NumberLocale, NumberParse, NumberValue};
use glory::render::{
    Event, EventKind, GraphicPoint, GraphicProps, GraphicStroke, PathCommand, Purpose,
    RangeCommand, Semantics, SpinValue, StrokeCap,
};
use glory::style::{
    Align, BorderStyle, Display, Edges, FlexDirection, Justify, Length, Overflow, PointerEvents,
    Style,
};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid bounded number input state").into()
}
fn chevron(increase: bool, color: glory::style::Color) -> GraphicProps {
    let (start, middle, end) = if increase {
        ((6., 15.), (12., 9.), (18., 15.))
    } else {
        ((6., 9.), (12., 15.), (18., 9.))
    };
    GraphicProps {
        commands: vec![
            PathCommand::Move(GraphicPoint::new(start.0, start.1)),
            PathCommand::Line(GraphicPoint::new(middle.0, middle.1)),
            PathCommand::Line(GraphicPoint::new(end.0, end.1)),
        ],
        stroke: Some(GraphicStroke {
            color,
            width: 1.8,
            cap: StrokeCap::Round,
        }),
        ..Default::default()
    }
}
#[derive(Clone, PartialEq)]
struct Snapshot {
    value: NumberValue,
    domain: NumberDomain,
    locale: NumberLocale,
    grouping: bool,
    enabled: bool,
    metadata: Semantics,
}
impl Snapshot {
    fn parse(&self) -> NumberParse {
        self.value.parse(self.locale)
    }
    fn text(&self) -> String {
        self.value
            .text(self.locale, self.grouping)
            .expect("validated number value")
    }
    fn writable(&self) -> bool {
        self.enabled && !self.metadata.readonly
    }
    fn semantics(&self) -> Semantics {
        let mut metadata = self.metadata.clone();
        let parsed = self.parse();
        let value = match parsed {
            NumberParse::Complete(value) => Some(value),
            _ => None,
        };
        metadata.disabled = false;
        metadata.invalid |= matches!(
            parsed,
            NumberParse::Invalid(_) | NumberParse::Incomplete { .. }
        ) || value.is_some_and(|v| self.domain.outside(v));
        metadata.spinbutton = Some(SpinValue {
            min: self.domain.min.map(Decimal::to_f64),
            max: self.domain.max.map(Decimal::to_f64),
            value: value.map(Decimal::to_f64),
            step: self.domain.step.to_f64(),
            page_step: self.domain.page.to_f64(),
            value_text: Some(self.text()),
            can_increase: self.writable() && self.domain.increment(parsed, true, false).is_some(),
            can_decrease: self.writable() && self.domain.increment(parsed, false, false).is_some(),
        });
        metadata
    }
}
type Stamp = (NumberValue, Option<u64>);
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<NumberValue>,
    local: bool,
    accepted: Cage<Snapshot>,
    dirty: Dirty,
    changed: Callback<NumberValue>,
    committed: Callback<NumberValue>,
    rejected: Callback<NumberError>,
    confirmed: RefCell<Option<Stamp>>,
    composing: Cell<bool>,
    draft: RefCell<Option<NumberValue>>,
    composition_ended: Cell<bool>,
    composition_echo: RefCell<Option<(u64, NumberValue)>>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn stamp(&self) -> Result<Option<Stamp>> {
        let value = self
            .control
            .source
            .try_with(|v| v.validate().ok().map(|()| v.clone()))?;
        let version = match self.control.source {
            Vary::Cage(c) => Some(c.try_version()?),
            _ => None,
        };
        Ok(value.map(|v| (v, version)))
    }
    fn current(&self, state: &Snapshot) -> Result<NumberValue> {
        let stamp = self.stamp()?;
        let confirmed = self.confirmed.borrow().clone();
        Ok(if self.local || stamp.is_some() && stamp == confirmed {
            stamp.map(|v| v.0).unwrap_or_else(|| state.value.clone())
        } else {
            state.value.clone()
        })
    }
    fn propose(&self, value: NumberValue, commit: bool) -> Result {
        self.ensure()?;
        if !self.accepted.try_with(Snapshot::writable)? {
            return Ok(());
        }
        let before = self.stamp()?;
        self.confirmed.borrow_mut().take();
        self.control.accept(&value)?;
        self.dirty.touch()?;
        if commit {
            self.committed.emit(self.owner, value.clone());
        } else {
            self.changed.emit(self.owner, value.clone());
        }
        if self.ensure().is_ok() {
            let after = self.stamp()?;
            if after != before && after.as_ref().is_some_and(|v| v.0 == value) {
                *self.confirmed.borrow_mut() = after;
            }
        }
        Ok(())
    }
    fn text(&self, value: &str, commit: bool) -> Result {
        if value.len() > crate::MAX_NUMBER_INPUT_BYTES {
            self.dirty.touch()?;
            self.rejected.emit(self.owner, NumberError::InputTooLong);
            return Ok(());
        }
        let value = if value.is_empty() {
            NumberValue::Empty
        } else {
            NumberValue::Text(value.into())
        };
        if let Err(error) = value.validate() {
            self.dirty.touch()?;
            self.rejected.emit(self.owner, error);
            return Ok(());
        }
        if self.composing.get() {
            *self.draft.borrow_mut() = Some(value);
            return Ok(());
        }
        let state = self.accepted.try_get()?;
        let proposal = if commit {
            match value.parse(state.locale) {
                NumberParse::Complete(v) | NumberParse::Incomplete { candidate: Some(v) } => {
                    NumberValue::Decimal(state.domain.clamp(v))
                }
                _ => value,
            }
        } else {
            value
        };
        self.propose(proposal, commit)
    }
    fn spin(&self, command: RangeCommand) -> Result {
        self.ensure()?;
        let state = self.accepted.try_get()?;
        if !state.writable() || self.composing.get() {
            return Ok(());
        }
        let current = self.current(&state)?;
        let parsed = current.parse(state.locale);
        let next = match command {
            RangeCommand::Increase => state.domain.increment(parsed, true, false),
            RangeCommand::Decrease => state.domain.increment(parsed, false, false),
            RangeCommand::PageIncrease => state.domain.increment(parsed, true, true),
            RangeCommand::PageDecrease => state.domain.increment(parsed, false, true),
            RangeCommand::Minimum => state.domain.min,
            RangeCommand::Maximum => state.domain.max,
            RangeCommand::Set(value) => {
                if !value.is_finite() {
                    return Err(invalid());
                }
                let text = format!("{value:.18}");
                Some(
                    state
                        .domain
                        .clamp(Decimal::parse(&text).map_err(|_| invalid())?),
                )
            }
        };
        if let Some(next) = next {
            let value = NumberValue::Decimal(next);
            self.propose(value.clone(), false)?;
            if self.ensure().is_ok() {
                let before = self.stamp()?;
                self.committed.emit(self.owner, value.clone());
                if self.ensure().is_ok() {
                    let after = self.stamp()?;
                    if after != before && after.as_ref().is_some_and(|s| s.0 == value) {
                        *self.confirmed.borrow_mut() = after;
                    }
                }
            }
        }
        Ok(())
    }
    fn input_event(&self, event: &Event) -> Result {
        match event {
            Event::ValueChanged {
                value,
                composing,
                revision,
            } => {
                if *composing {
                    self.composing.set(true);
                }
                let echo = self.composition_echo.borrow_mut().take();
                if !composing
                    && echo.as_ref().is_some_and(|(r, v)| {
                        r == revision
                            && match v {
                                NumberValue::Empty => value.is_empty(),
                                NumberValue::Text(text) => text == value,
                                _ => false,
                            }
                    })
                {
                    return Ok(());
                }
                let ended = !composing
                    && (self.composition_ended.replace(false) || self.composing.replace(false));
                if ended {
                    self.draft.borrow_mut().take();
                }
                self.text(value, false)?;
                if ended && value.len() <= crate::MAX_NUMBER_INPUT_BYTES && self.ensure().is_ok() {
                    *self.composition_echo.borrow_mut() = Some((
                        *revision,
                        if value.is_empty() {
                            NumberValue::Empty
                        } else {
                            NumberValue::Text(value.clone())
                        },
                    ));
                    // A callback may have explicitly flushed before the echo
                    // was installed. Rearm a tick-end cleanup after it returns.
                    self.dirty.touch()?;
                }
                Ok(())
            }
            Event::Commit { value, .. } => self.text(value, true),
            Event::Spin { command, .. } => self.spin(*command),
            Event::Composition { active } => {
                self.composing.set(*active);
                if *active {
                    self.composition_ended.set(false);
                    self.composition_echo.borrow_mut().take();
                    Ok(())
                } else {
                    self.composition_ended.set(true);
                    self.dirty.touch()
                }
            }
            _ => Ok(()),
        }
    }
    fn report(&self, result: Result) {
        if let Err(e) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(e)
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.changed.clear();
        self.committed.clear();
        self.rejected.clear();
        self.draft.borrow_mut().take();
        self.composition_echo.borrow_mut().take();
        self.confirmed.borrow_mut().take();
    }
}
pub struct NumberInput {
    common: Common,
    value: Value<NumberValue>,
    min: Vary<Option<Decimal>>,
    max: Vary<Option<Decimal>>,
    step: Vary<Decimal>,
    page: Vary<Decimal>,
    locale: NumberLocale,
    grouping: bool,
    controls: bool,
    field: Option<FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    form_name: Option<Vary<String>>,
    changed: Option<Box<dyn FnMut(NumberValue)>>,
    committed: Option<Box<dyn FnMut(NumberValue)>>,
    rejected: Option<Box<dyn FnMut(NumberError)>>,
}
pub fn number_input() -> NumberInput {
    NumberInput {
        common: Common::default(),
        value: Value::default(),
        min: None.into(),
        max: None.into(),
        step: Decimal::new(1, 0).unwrap().into(),
        page: Decimal::new(10, 0).unwrap().into(),
        locale: NumberLocale::EnUs,
        grouping: false,
        controls: true,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        form_name: None,
        changed: None,
        committed: None,
        rejected: None,
    }
}
impl NumberInput {
    common_methods!();
    pub fn value(mut self, value: impl Into<Vary<NumberValue>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: impl Into<NumberValue>) -> Self {
        self.value.default = value.into();
        self
    }
    pub fn min(mut self, value: impl Into<Vary<Option<Decimal>>>) -> Self {
        self.min = value.into();
        self
    }
    pub fn max(mut self, value: impl Into<Vary<Option<Decimal>>>) -> Self {
        self.max = value.into();
        self
    }
    pub fn step(mut self, value: impl Into<Vary<Decimal>>) -> Self {
        self.step = value.into();
        self
    }
    pub fn page_step(mut self, value: impl Into<Vary<Decimal>>) -> Self {
        self.page = value.into();
        self
    }
    pub fn locale(mut self, value: NumberLocale) -> Self {
        self.locale = value;
        self
    }
    pub fn grouping(mut self, value: bool) -> Self {
        self.grouping = value;
        self
    }
    pub fn controls(mut self, value: bool) -> Self {
        self.controls = value;
        self
    }
    pub fn field(mut self, value: FieldControl) -> Self {
        self.common.theme = value.theme.clone();
        self.common.size = value.size;
        self.common.unstyled = value.unstyled;
        self.field = Some(value);
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
    pub fn form_name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(value.into());
        self
    }
    pub fn on_value_change(mut self, callback: impl FnMut(NumberValue) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn on_commit(mut self, callback: impl FnMut(NumberValue) + 'static) -> Self {
        self.committed = Some(Box::new(callback));
        self
    }
    pub fn on_reject(mut self, callback: impl FnMut(NumberError) + 'static) -> Self {
        self.rejected = Some(Box::new(callback));
        self
    }
}
impl Widget for NumberInput {
    fn build(&mut self, scope: &Scope) -> Result {
        // Validate constant text before even cloning its Vary handle.
        if let Some(source) = &self.value.source {
            source.try_with(|v| v.validate())?.map_err(|_| invalid())?;
        } else {
            self.value.default.validate().map_err(|_| invalid())?;
        }
        let local = self.value.source.is_none();
        let control = std::mem::take(&mut self.value).mount(scope);
        let raw = composite::validated(scope, &control.source, |v| {
            v.validate().map_err(|_| invalid())
        })?;
        let enabled = self
            .field
            .as_ref()
            .map(|f| f.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let metadata = control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            self.readonly.clone(),
            self.invalid.clone(),
        );
        let form_name = composite::form_name(scope, &self.form_name)?;
        let min = self.min.clone();
        let max = self.max.clone();
        let step = self.step.clone();
        let page = self.page.clone();
        let locale = self.locale;
        let grouping = self.grouping;
        let snapshot = scope.bond(move || {
            let mut metadata = metadata.get().map(|v| *v).unwrap_or_default();
            if let Some(name) = form_name {
                metadata.form_control = Some(glory::render::FormControl {
                    name: name.get(),
                    value: None,
                });
            }
            Snapshot {
                value: raw.get(),
                domain: NumberDomain {
                    min: min.get(),
                    max: max.get(),
                    step: step.get(),
                    page: page.get(),
                },
                locale,
                grouping,
                enabled: enabled.get(),
                metadata,
            }
        });
        let accepted = composite::validated(scope, &snapshot.into(), |s: &Snapshot| {
            s.domain.validate().map_err(|_| invalid())?;
            s.value.validate().map_err(|_| invalid())?;
            Ok(())
        })?;
        let dirty = Dirty::new(scope);
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            local,
            accepted,
            dirty: dirty.clone(),
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
            rejected: Callback::new(self.rejected.take()),
            confirmed: RefCell::new(None),
            composing: Cell::new(false),
            draft: RefCell::new(None),
            composition_ended: Cell::new(false),
            composition_echo: RefCell::new(None),
        });
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(s) = weak.upgrade() {
                s.retire()
            }
        })?;
        let initial = accepted.try_with(Snapshot::text)?;
        let mut base = self
            .common
            .theme
            .control(self.common.size)
            .width(Length::Px(180.));
        if self.controls && !self.common.unstyled {
            base.border_width = Some(Edges::all(0.));
            base.background = Some(glory::style::Color(0, 0, 0, 0));
            base.radius = Some(0.);
        }
        let overrides = self.common.style.clone();
        let height = base.min_height.unwrap_or(Length::Px(36.));
        let themed = !self.common.unstyled;
        let danger = self.common.theme.danger;
        self.common.style = scope
            .bond(move || {
                let mut style = overrides.get();
                style.width.get_or_insert(Length::Px(180.));
                style.min_height.get_or_insert(height);
                if themed && accepted.with(|s| s.semantics().invalid) {
                    style.border_color.get_or_insert(danger);
                }
                style
            })
            .into();

        let mut input = configure!(
            self.common,
            glory::ui::input(),
            scope,
            "sapid-number-input",
            base
        )
        .purpose(Purpose::Number)
        .value(initial.clone())
        .node_ref(reference)
        .semantics(scope.bond(move || composite::metadata(accepted.with(Snapshot::semantics))));
        for kind in [
            EventKind::ValueChanged,
            EventKind::Commit,
            EventKind::Composition,
            EventKind::Spin,
        ] {
            let s = state.clone();
            let observers = Observers::new(self.common.handlers.remove(&kind).unwrap_or_default());
            let cleanup = observers.clone();
            scope.on_cleanup(move || cleanup.clear())?;
            input=input.on(kind,move |event|{
                if s.ensure().is_err(){return}
                s.report(s.input_event(&event));
                if !matches!(&event,Event::ValueChanged{value,..}|Event::Commit{value,..} if value.len()>crate::MAX_NUMBER_INPUT_BYTES){observers.emit(s.owner,event)}
            });
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            input = input.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        let row_ref = scope.cage(None);
        let focused = scope.cage(false);
        let integrated = self.controls && !self.common.unstyled;
        let row_input = self.common.theme.input;
        let row_disabled = self.common.theme.disabled;
        let row_border = self.common.theme.border;
        let input_focus = self.common.theme.input_focus;
        let row_shape = Style {
            display: Some(Display::Flex),
            flex_direction: Some(FlexDirection::Row),
            gap: Some(0.),
            padding: Some(Edges::all(0.)),
            border_width: Some(Edges::all(if integrated { 1. } else { 0. })),
            border_style: integrated.then_some(BorderStyle::Solid),
            border_color: integrated.then_some(row_border),
            background: integrated.then_some(row_input),
            radius: integrated.then_some(self.common.theme.radius),
            overflow_x: integrated.then_some(Overflow::Hidden),
            overflow_y: integrated.then_some(Overflow::Hidden),
            ..Style::new()
        };
        let row_style: Vary<Style> = if integrated {
            scope
                .bond(move || {
                    let current = accepted.get();
                    let mut style = row_shape.clone();
                    style.border_color = Some(if current.semantics().invalid {
                        danger
                    } else if focused.get() {
                        input_focus
                    } else {
                        row_border
                    });
                    style.background = Some(if current.enabled {
                        row_input
                    } else {
                        row_disabled
                    });
                    style
                })
                .into()
        } else {
            row_shape.into()
        };
        let mut row = glory::ui::view()
            .field_part(glory::render::FieldPart::Fieldset)
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    field_part: Some(glory::render::FieldPart::Fieldset),
                    disabled: !accepted.with(|s| s.enabled),
                    ..Default::default()
                })
            }))
            .class("sapid-number-input-row")
            .style(row_style);
        if self.controls {
            row = row.on(EventKind::FocusWithin, move |event| {
                if let Event::FocusWithin { focused: next, .. } = event {
                    focused.revise(|current| *current = next)
                }
            });
        }
        row.node_ref(row_ref).show_in(scope)?;
        let container = scope.child_in(row_ref.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        input.show_in(&container)?;
        let node = reference.try_get()?.ok_or(glory::ui::Error::Gone)?;
        if self.controls {
            let panel_ref = scope.cage(None);
            let themed = !self.common.unstyled;
            let panel_style = Style {
                display: Some(Display::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(Length::Px(28.)),
                height: Some(height),
                min_height: Some(height),
                padding: Some(Edges::all(0.)),
                border_width: Some(if themed {
                    Edges {
                        left: 1.,
                        ..Edges::all(0.)
                    }
                } else {
                    Edges::all(0.)
                }),
                border_style: themed.then_some(BorderStyle::Solid),
                border_color: themed.then_some(self.common.theme.border),
                background: Some(glory::style::Color(0, 0, 0, 0)),
                radius: Some(0.),
                ..Style::new()
            };
            glory::ui::view()
                .class("sapid-number-input-controls")
                .style(panel_style)
                .node_ref(panel_ref)
                .show_in(&container)?;
            let controls = scope.child_in(panel_ref.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
            for (label, increase) in [("Increase", true), ("Decrease", false)] {
                let s = state.clone();
                let enabled = scope.bond(move || {
                    accepted.with(|v| {
                        v.writable() && v.domain.increment(v.parse(), increase, false).is_some()
                    })
                });
                let muted = self.common.theme.muted;
                let disabled_text = self.common.theme.disabled_text;
                let border = self.common.theme.border;
                let shape = Style {
                    display: Some(Display::Flex),
                    width: Some(Length::Percent(100.)),
                    height: Some(Length::Percent(50.)),
                    min_height: Some(Length::Px(0.)),
                    grow: Some(1.),
                    shrink: Some(1.),
                    align: Some(Align::Center),
                    justify: Some(Justify::Center),
                    padding: Some(Edges::all(0.)),
                    font_size: Some(11.),
                    line_height: Some(1.),
                    border_width: Some(if themed && increase {
                        Edges {
                            bottom: 1.,
                            ..Edges::all(0.)
                        }
                    } else {
                        Edges::all(0.)
                    }),
                    border_style: themed.then_some(BorderStyle::Solid),
                    border_color: themed.then_some(border),
                    background: Some(glory::style::Color(0, 0, 0, 0)),
                    radius: Some(0.),
                    ..Style::new()
                };
                let style_enabled = enabled;
                let styles = scope.bond(move || {
                    let mut style = shape.clone();
                    if themed {
                        style.color = Some(if style_enabled.get() {
                            muted
                        } else {
                            disabled_text
                        });
                    }
                    style
                });
                let graphic_enabled = enabled;
                let drawing = scope.bond(move || {
                    chevron(
                        increase,
                        if graphic_enabled.get() {
                            muted
                        } else {
                            disabled_text
                        },
                    )
                });
                let mut button = crate::button()
                    .name(label)
                    .label("")
                    .fill(
                        glory::ui::graphic()
                            .class("sapid-number-input-chevron")
                            .drawing(drawing)
                            .style(
                                Style::new()
                                    .width(Length::Px(16.))
                                    .height(Length::Px(16.))
                                    .pointer_events(PointerEvents::None),
                            ),
                    )
                    .class(if increase {
                        "sapid-number-input-control sapid-number-input-increase"
                    } else {
                        "sapid-number-input-control sapid-number-input-decrease"
                    })
                    .theme(self.common.theme.clone())
                    .variant(crate::ButtonVariant::Subtle)
                    .size(self.common.size)
                    .style(styles)
                    .enabled(enabled)
                    .on_press(move || {
                        s.report(s.spin(if increase {
                            RangeCommand::Increase
                        } else {
                            RangeCommand::Decrease
                        }))
                    });
                if self.common.unstyled {
                    button = button.unstyled();
                }
                button.show_in(&controls)?;
            }
            scope.retain(controls)?;
        }
        scope.retain(container)?;
        let weak = Rc::downgrade(&state);
        let owner = scope.downgrade();
        let mut previous = initial;
        scope.effect(move || {
            let (Some(state), Some(scope)) = (weak.upgrade(), owner.upgrade()) else {
                return;
            };
            let result = (|| -> Result {
                let edited = dirty.take();
                if state.composition_ended.replace(false) {
                    let draft = state.draft.borrow_mut().take();
                    if let Some(value) = draft {
                        state.propose(value, false)?;
                        return Ok(());
                    }
                }
                let text = accepted.try_with(Snapshot::text)?;
                if !state.composing.get() && (edited || text != previous) {
                    scope.set_input_value(node, text.clone())?;
                    previous = text;
                    state.confirmed.borrow_mut().take();
                    state.composition_echo.borrow_mut().take();
                }
                Ok(())
            })();
            state.report(result)
        });
        scope.retain(state)?;
        Ok(())
    }
}

impl Default for NumberInput {
    fn default() -> Self {
        number_input()
    }
}
