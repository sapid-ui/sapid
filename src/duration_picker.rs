//! Segmented duration input. Durations are not wall-clock times and may exceed 24 hours.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback};
use crate::state::{Control, Dirty, Value};
use crate::{Decimal, NumberValue, TimePrecision};
use glory::render::{NodeId, Role};
use glory::style::{Display, FlexDirection, Length, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};

pub const MAX_DURATION_HOURS: u32 = 999_999;
pub const MAX_DURATION_HOUR_DIGITS: u8 = 6;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDuration {
    seconds: u64,
}
impl TimeDuration {
    pub fn new(
        hours: u32,
        minutes: u8,
        seconds: u8,
    ) -> std::result::Result<Self, glory::render::Error> {
        if hours > MAX_DURATION_HOURS || minutes > 59 || seconds > 59 {
            return Err(glory::render::Error::InvalidProps(
                "invalid finite duration",
            ));
        }
        Ok(Self {
            seconds: u64::from(hours) * 3600 + u64::from(minutes) * 60 + u64::from(seconds),
        })
    }
    pub const fn hours(self) -> u32 {
        (self.seconds / 3600) as u32
    }
    pub const fn minutes(self) -> u8 {
        ((self.seconds / 60) % 60) as u8
    }
    pub const fn seconds(self) -> u8 {
        (self.seconds % 60) as u8
    }
    pub const fn total_seconds(self) -> u64 {
        self.seconds
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DurationParts {
    pub hours: Option<u32>,
    pub minutes: Option<u8>,
    pub seconds: Option<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurationPickerStatus {
    Empty,
    Incomplete,
    Valid(TimeDuration),
    Invalid,
    TooLong,
}
impl DurationPickerStatus {
    fn proposal(self) -> Option<Option<TimeDuration>> {
        match self {
            Self::Empty => Some(None),
            Self::Valid(value) => Some(Some(value)),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DurationPickerDraft {
    pub parts: DurationParts,
    pub status: DurationPickerStatus,
}

fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid finite segmented duration picker").into()
}
fn decimal(value: u32) -> Decimal {
    Decimal::new(i128::from(value), 0).unwrap()
}
fn display(value: u32, digits: u8) -> NumberValue {
    NumberValue::Text(format!("{value:0width$}", width = usize::from(digits)))
}
fn parse(
    value: &NumberValue,
    max_digits: usize,
) -> std::result::Result<Option<u32>, DurationPickerStatus> {
    match value {
        NumberValue::Empty => Ok(None),
        NumberValue::Decimal(value)
            if value.scale() == 0
                && (0..=i128::from(MAX_DURATION_HOURS)).contains(&value.coefficient()) =>
        {
            Ok(Some(value.coefficient() as u32))
        }
        NumberValue::Text(value) if value.is_empty() => Ok(None),
        NumberValue::Text(value) if value.len() > max_digits => Err(DurationPickerStatus::TooLong),
        NumberValue::Text(value) if value.bytes().all(|byte| byte.is_ascii_digit()) => value
            .parse::<u32>()
            .ok()
            .filter(|value| *value <= MAX_DURATION_HOURS)
            .map(Some)
            .ok_or(DurationPickerStatus::Invalid),
        _ => Err(DurationPickerStatus::Invalid),
    }
}
fn classify(parts: DurationParts, precision: TimePrecision) -> DurationPickerStatus {
    let visible = if precision == TimePrecision::Second {
        3
    } else {
        2
    };
    let values = [
        parts.hours.is_some(),
        parts.minutes.is_some(),
        parts.seconds.is_some(),
    ];
    if values[..visible].iter().all(|value| !value) {
        return DurationPickerStatus::Empty;
    }
    let (Some(hours), Some(minutes)) = (parts.hours, parts.minutes) else {
        return DurationPickerStatus::Incomplete;
    };
    let seconds = if precision == TimePrecision::Second {
        let Some(seconds) = parts.seconds else {
            return DurationPickerStatus::Incomplete;
        };
        seconds
    } else {
        0
    };
    TimeDuration::new(hours, minutes, seconds)
        .map(DurationPickerStatus::Valid)
        .unwrap_or(DurationPickerStatus::Invalid)
}

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: Option<TimeDuration>,
    enabled: bool,
    readonly: bool,
}
#[derive(Clone, Copy)]
struct Pending {
    baseline: Option<TimeDuration>,
    proposal: Option<Option<TimeDuration>>,
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<Option<TimeDuration>>,
    accepted: Cage<Snapshot>,
    precision: TimePrecision,
    min_hours_digits: u8,
    parts: [Cage<NumberValue>; 3],
    status: Cage<DurationPickerStatus>,
    pending: Cell<Option<Pending>>,
    dirty: Dirty,
    changed: Callback<Option<TimeDuration>>,
    committed: Callback<Option<TimeDuration>>,
    draft_changed: Callback<DurationPickerDraft>,
    rejected: Callback<DurationPickerStatus>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn parts(&self) -> Result<DurationParts> {
        Ok(DurationParts {
            hours: self.parts[0]
                .try_with(|value| parse(value, usize::from(MAX_DURATION_HOUR_DIGITS)))?
                .map_err(|_| invalid())?,
            minutes: self.parts[1]
                .try_with(|value| parse(value, 2))?
                .map_err(|_| invalid())?
                .map(|value| value as u8),
            seconds: if self.precision == TimePrecision::Second {
                self.parts[2]
                    .try_with(|value| parse(value, 2))?
                    .map_err(|_| invalid())?
                    .map(|value| value as u8)
            } else {
                None
            },
        })
    }
    fn set_parts(&self, value: Option<TimeDuration>) -> Result {
        let values = value.map(|value| {
            [
                value.hours(),
                u32::from(value.minutes()),
                u32::from(value.seconds()),
            ]
        });
        for index in 0..3 {
            let next = values.map_or(NumberValue::Empty, |values| {
                display(
                    values[index],
                    if index == 0 { self.min_hours_digits } else { 2 },
                )
            });
            if self.parts[index].try_with(|old| *old != next)? {
                self.parts[index].try_revise(|old| *old = next)?;
            }
        }
        let status = value.map_or(DurationPickerStatus::Empty, DurationPickerStatus::Valid);
        if self.status.get() != status {
            self.status.try_revise(|old| *old = status)?;
        }
        Ok(())
    }
    fn segment(&self, index: usize, value: NumberValue, committing: bool) -> Result {
        self.ensure()?;
        if !self
            .accepted
            .try_with(|value| value.enabled && !value.readonly)?
        {
            return Ok(());
        }
        let max_digits = if index == 0 {
            usize::from(MAX_DURATION_HOUR_DIGITS)
        } else {
            2
        };
        let parsed = match parse(&value, max_digits) {
            Ok(value) => value,
            Err(status) => {
                self.rejected.emit(self.owner, status);
                return self.dirty.touch();
            }
        };
        if index > 0 && parsed.is_some_and(|value| value > 59) {
            self.rejected
                .emit(self.owner, DurationPickerStatus::Invalid);
            return self.dirty.touch();
        }
        let next = if committing || matches!(value, NumberValue::Decimal(_)) {
            parsed.map_or(NumberValue::Empty, |value| {
                display(value, if index == 0 { self.min_hours_digits } else { 2 })
            })
        } else {
            value
        };
        if self.parts[index].try_with(|old| *old != next)? {
            self.parts[index].try_revise(|old| *old = next)?;
        }
        let parts = self.parts()?;
        let status = classify(parts, self.precision);
        if self.status.get() != status {
            self.status.try_revise(|old| *old = status)?;
        }
        self.draft_changed
            .emit(self.owner, DurationPickerDraft { parts, status });
        self.ensure()?;
        let baseline = self.accepted.try_with(|value| value.value)?;
        let proposal = status.proposal();
        self.pending.set(Some(Pending { baseline, proposal }));
        if let Some(next) = proposal {
            self.control.accept(&next)?;
            if !committing || next != baseline {
                self.changed.emit(self.owner, next);
                self.ensure()?;
            }
            if committing {
                self.committed.emit(self.owner, next);
                self.ensure()?;
            }
        } else if committing {
            self.rejected.emit(self.owner, status);
        }
        self.dirty.touch()
    }
    fn choose(&self, value: Option<TimeDuration>) -> Result {
        self.ensure()?;
        if !self
            .accepted
            .try_with(|value| value.enabled && !value.readonly)?
        {
            return Ok(());
        }
        let baseline = self.accepted.try_with(|value| value.value)?;
        self.set_parts(value)?;
        self.control.accept(&value)?;
        self.changed.emit(self.owner, value);
        self.pending.set(Some(Pending {
            baseline,
            proposal: Some(value),
        }));
        self.dirty.touch()
    }
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error);
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.pending.set(None);
        self.changed.clear();
        self.committed.clear();
        self.draft_changed.clear();
        self.rejected.clear();
    }
}

pub struct DurationPicker {
    pub(crate) common: Common,
    key: String,
    precision: TimePrecision,
    value: Value<Option<TimeDuration>>,
    readonly: Vary<bool>,
    min_hours_digits: u8,
    clearable: bool,
    clear_label: String,
    placeholders: [Option<Vary<String>>; 3],
    labels: [Vary<String>; 3],
    references: [Option<Cage<Option<NodeId>>>; 3],
    steps: [u32; 3],
    changed: Option<Box<dyn FnMut(Option<TimeDuration>)>>,
    committed: Option<Box<dyn FnMut(Option<TimeDuration>)>>,
    draft_changed: Option<Box<dyn FnMut(DurationPickerDraft)>>,
    rejected: Option<Box<dyn FnMut(DurationPickerStatus)>>,
}
pub fn duration_picker(key: impl Into<String>) -> DurationPicker {
    DurationPicker {
        common: Common::default(),
        key: key.into(),
        precision: TimePrecision::Minute,
        value: Value::default(),
        readonly: false.into(),
        min_hours_digits: 2,
        clearable: false,
        clear_label: "Clear duration".into(),
        placeholders: [None, Some("--".into()), Some("--".into())],
        labels: ["Hours".into(), "Minutes".into(), "Seconds".into()],
        references: [None; 3],
        steps: [1; 3],
        changed: None,
        committed: None,
        draft_changed: None,
        rejected: None,
    }
}
impl DurationPicker {
    common_methods!();
    pub fn precision(mut self, value: TimePrecision) -> Self {
        self.precision = value;
        self
    }
    pub fn value(mut self, value: impl Into<Vary<Option<TimeDuration>>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: Option<TimeDuration>) -> Self {
        self.value.default = value;
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn min_hours_digits(mut self, value: u8) -> Self {
        self.min_hours_digits = value;
        self
    }
    pub fn clearable(mut self, value: bool) -> Self {
        self.clearable = value;
        self
    }
    pub fn clear_label(mut self, value: impl Into<String>) -> Self {
        self.clear_label = value.into();
        self
    }
    pub fn hour_placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholders[0] = Some(value.into());
        self
    }
    pub fn minute_placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholders[1] = Some(value.into());
        self
    }
    pub fn second_placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholders[2] = Some(value.into());
        self
    }
    pub fn hour_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.labels[0] = value.into();
        self
    }
    pub fn minute_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.labels[1] = value.into();
        self
    }
    pub fn second_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.labels[2] = value.into();
        self
    }
    pub fn hour_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.references[0] = Some(value);
        self
    }
    pub fn minute_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.references[1] = Some(value);
        self
    }
    pub fn second_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.references[2] = Some(value);
        self
    }
    pub fn hour_step(mut self, value: u32) -> Self {
        self.steps[0] = value;
        self
    }
    pub fn minute_step(mut self, value: u8) -> Self {
        self.steps[1] = u32::from(value);
        self
    }
    pub fn second_step(mut self, value: u8) -> Self {
        self.steps[2] = u32::from(value);
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(Option<TimeDuration>) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_commit(mut self, value: impl FnMut(Option<TimeDuration>) + 'static) -> Self {
        self.committed = Some(Box::new(value));
        self
    }
    pub fn on_draft_change(mut self, value: impl FnMut(DurationPickerDraft) + 'static) -> Self {
        self.draft_changed = Some(Box::new(value));
        self
    }
    pub fn on_reject(mut self, value: impl FnMut(DurationPickerStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(value));
        self
    }
}
impl Widget for DurationPicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.is_empty()
            || self.key.len() > 56
            || !self
                .key
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            || !(1..=MAX_DURATION_HOUR_DIGITS).contains(&self.min_hours_digits)
            || self.steps[0] == 0
            || self.steps[0] > MAX_DURATION_HOURS
            || self.steps[1] == 0
            || self.steps[1] > 59
            || self.steps[2] == 0
            || self.steps[2] > 59
            || self.clear_label.trim().is_empty()
            || self.clear_label.len() > 120
            || self.clear_label.contains(['\0', '\r', '\n'])
        {
            return Err(invalid());
        }
        let control = std::mem::take(&mut self.value).mount(scope);
        let enabled = self.common.enabled.clone();
        let readonly = self.readonly.clone();
        let source = control.source.clone();
        let accepted = composite::validated(
            scope,
            &scope
                .bond(move || Snapshot {
                    value: source.get(),
                    enabled: enabled.get(),
                    readonly: readonly.get(),
                })
                .into(),
            |snapshot: &Snapshot| {
                if snapshot
                    .value
                    .is_none_or(|value| value.hours() <= MAX_DURATION_HOURS)
                {
                    Ok(())
                } else {
                    Err(invalid())
                }
            },
        )?;
        let initial = accepted.with(|value| value.value);
        let initial_parts = initial.map_or(DurationParts::default(), |value| DurationParts {
            hours: Some(value.hours()),
            minutes: Some(value.minutes()),
            seconds: (self.precision == TimePrecision::Second).then_some(value.seconds()),
        });
        let initial_status = classify(initial_parts, self.precision);
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            accepted,
            precision: self.precision,
            min_hours_digits: self.min_hours_digits,
            parts: [
                scope.cage(initial.map_or(NumberValue::Empty, |value| {
                    display(value.hours(), self.min_hours_digits)
                })),
                scope.cage(initial.map_or(NumberValue::Empty, |value| {
                    display(u32::from(value.minutes()), 2)
                })),
                scope.cage(initial.map_or(NumberValue::Empty, |value| {
                    display(u32::from(value.seconds()), 2)
                })),
            ],
            status: scope.cage(initial_status),
            pending: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
            draft_changed: Callback::new(self.draft_changed.take()),
            rejected: Callback::new(self.rejected.take()),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = cleanup.upgrade() {
                state.retire()
            }
        })?;
        let root = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let (class, appearance) = self.common.appearance(
            scope,
            "sapid-time-picker sapid-duration-picker",
            Style::new()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Column)
                .gap(8.),
        );
        let mut widget = glory::ui::view()
            .role(Role::Group)
            .class(class)
            .style(appearance)
            .node_ref(root)
            .focus(self.common.focus);
        if let Some(name) = self.common.name.take() {
            widget = widget.name(name);
        } else {
            widget = widget.name("Duration");
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            widget = widget.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        widget.show_in(scope)?;
        let content = scope.child_in(root.get().ok_or(glory::ui::Error::Gone)?)?;
        let visible_parts = state.parts;
        let visible_count = if self.precision == TimePrecision::Second {
            3
        } else {
            2
        };
        let visible = scope.bond(move || {
            visible_parts[..visible_count]
                .iter()
                .any(|part| !matches!(part.get(), NumberValue::Empty))
        });
        let clearable = self.clearable;
        let actions_visible = scope.bond(move || clearable && visible.get());
        let (editors, actions) = crate::input_actions::segmented_row(
            &content,
            "sapid-time-segments sapid-duration-segments",
            actions_visible.into(),
        )?;
        for index in 0..if self.precision == TimePrecision::Second {
            3
        } else {
            2
        } {
            if index > 0 {
                glory::ui::text().content(":").show_in(&editors)?;
            }
            let changed = state.clone();
            let committed = state.clone();
            let rejected = state.clone();
            let width: Vary<Style> = if index == 0 {
                let part = state.parts[0];
                let minimum = self.min_hours_digits;
                scope
                    .bond(move || {
                        let digits = match part.get() {
                            NumberValue::Text(value) => value.len(),
                            NumberValue::Decimal(value) => value.coefficient().to_string().len(),
                            NumberValue::Empty => usize::from(minimum),
                        };
                        Style::new().width(Length::Px(
                            28. + 10. * digits.max(usize::from(minimum)) as f64,
                        ))
                    })
                    .into()
            } else {
                Style::new().width(Length::Px(72.)).into()
            };
            let mut input = crate::number_input()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .name(self.labels[index].clone())
                .class(match index {
                    0 => "sapid-duration-hour",
                    1 => "sapid-duration-minute",
                    _ => "sapid-duration-second",
                })
                .style(width)
                .value(state.parts[index])
                .min(Some(decimal(0)))
                .max(Some(decimal(if index == 0 {
                    MAX_DURATION_HOURS
                } else {
                    59
                })))
                .step(decimal(self.steps[index]))
                .page_step(decimal(
                    self.steps[index].saturating_mul(10).min(MAX_DURATION_HOURS),
                ))
                .controls(false)
                .enabled(accepted.map(|value| value.enabled))
                .readonly(accepted.map(|value| value.readonly))
                .on_value_change(move |value| changed.report(changed.segment(index, value, false)))
                .on_commit(move |value| committed.report(committed.segment(index, value, true)))
                .on_reject(move |_| {
                    rejected.report(rejected.segment(
                        index,
                        NumberValue::Text("invalid".into()),
                        true,
                    ))
                });
            if self.common.unstyled {
                input = input.unstyled();
            }
            if let Some(reference) = self.references[index] {
                input = input.node_ref(reference);
            }
            let placeholder = if index == 0 && self.placeholders[0].is_none() {
                Some("-".repeat(usize::from(self.min_hours_digits)).into())
            } else {
                self.placeholders[index].take()
            };
            if let Some(placeholder) = placeholder {
                input = input.placeholder(placeholder);
            }
            input.show_in(&editors)?;
        }
        if self.clearable {
            let clear = state.clone();
            let label = self.clear_label.clone();
            let theme = self.common.theme.clone();
            let size = self.common.size;
            let unstyled = self.common.unstyled;
            let action_style = crate::input_actions::flow_action_style(&self.common);
            glory::ui::when(visible, move || {
                let clear = clear.clone();
                let mut button = crate::button()
                    .theme(theme.clone())
                    .size(size)
                    .variant(crate::ButtonVariant::Subtle)
                    .compact(true)
                    .class("sapid-duration-picker-clear")
                    .name(label.clone())
                    .style(action_style.clone())
                    .fill(crate::input_actions::glyph(crate::Icon::Close, theme.muted))
                    .on_press(move || clear.report(clear.choose(None)));
                if unstyled {
                    button = button.unstyled();
                }
                button
            })
            .show_in(&actions)?;
        }
        let weak = Rc::downgrade(&state);
        let mut previous = initial;
        scope.effect(move || {
            let Some(state) = weak.upgrade() else { return };
            let dirty = state.dirty.take();
            let value = accepted.with(|snapshot| snapshot.value);
            if dirty || value != previous {
                let pending = state.pending.take();
                let keep = pending.is_some_and(|pending| {
                    pending.proposal == Some(value)
                        || pending.proposal.is_none() && pending.baseline == value
                });
                if !keep {
                    state.report(state.set_parts(value));
                }
                previous = value;
            }
        });
        scope.retain(actions)?;
        scope.retain(editors)?;
        scope.retain(content)?;
        scope.retain(state)
    }
}
