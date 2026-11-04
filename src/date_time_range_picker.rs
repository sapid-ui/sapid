//! Atomic civil date/time ranges with one calendar and two segmented time editors.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, ContentPolicy};
use crate::state::{Control, Dirty, Value};
use crate::{
    DatePickerLevel, DatePickerMode, DateRangeSelection, DateSelection, DateTimeBounds,
    DateTimeDisplayFormat, DateTimeOutcome, DateTimeParts, DateTimePolicy, DateTimeStatus,
    DatesProvider, FieldControl, LocalDateTime, LocalTime, TimeFormat, TimePickerDraft,
    TimePickerStatus, YearMonth,
};
use glory::render::{Direction, FormControl, NodeId, Role};
use glory::style::{Align, Display, FlexDirection, Length, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};

pub const MAX_DATE_TIME_RANGE_PRESETS: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateTimeRangeValue {
    pub start: Option<LocalDateTime>,
    pub end: Option<LocalDateTime>,
}
impl DateTimeRangeValue {
    pub fn new(
        start: Option<LocalDateTime>,
        end: Option<LocalDateTime>,
    ) -> std::result::Result<Self, crate::DateError> {
        if start.zip(end).is_some_and(|(start, end)| start > end)
            || start.is_none() && end.is_some()
        {
            Err(crate::DateError::OutOfRange)
        } else {
            Ok(Self { start, end })
        }
    }
    pub const fn is_empty(self) -> bool {
        self.start.is_none()
    }
    pub const fn is_complete(self) -> bool {
        self.start.is_some() && self.end.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTimeRangePreset {
    pub label: String,
    pub value: DateTimeRangeValue,
}
impl DateTimeRangePreset {
    pub fn new(label: impl Into<String>, value: DateTimeRangeValue) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateTimeRangeStatus {
    Empty,
    Start(DateTimeOutcome),
    Complete {
        start: DateTimeOutcome,
        end: DateTimeOutcome,
    },
    Incomplete,
    Reversed,
    InvalidStart(DateTimeStatus),
    InvalidEnd(DateTimeStatus),
}
impl DateTimeRangeStatus {
    pub fn proposal(self) -> Option<DateTimeRangeValue> {
        match self {
            Self::Empty => Some(DateTimeRangeValue::default()),
            Self::Start(start) => Some(DateTimeRangeValue {
                start: Some(start.local),
                end: None,
            }),
            Self::Complete { start, end } => Some(DateTimeRangeValue {
                start: Some(start.local),
                end: Some(end.local),
            }),
            _ => None,
        }
    }
    pub const fn is_error(self) -> bool {
        matches!(
            self,
            Self::Reversed | Self::InvalidStart(_) | Self::InvalidEnd(_)
        )
    }
}

fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid finite date/time range picker").into()
}

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: DateTimeRangeValue,
    provider: DatesProvider,
    bounds: DateTimeBounds,
    enabled: bool,
    readonly: bool,
    revision: u64,
}

struct RangeController {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<DateTimeRangeValue>,
    accepted: Cage<Snapshot>,
    parts: [Cage<DateTimeParts>; 2],
    time_status: [Cell<TimePickerStatus>; 2],
    policy: DateTimePolicy,
    default_times: [Option<LocalTime>; 2],
    status: Cage<DateTimeRangeStatus>,
    pending: Cell<Option<DateTimeRangeValue>>,
    dirty: Dirty,
    changed: Callback<DateTimeRangeValue>,
    drafted: Callback<DateTimeRangeStatus>,
    committed: Callback<DateTimeRangeValue>,
    rejected: Callback<DateTimeRangeStatus>,
}
impl RangeController {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn writable(&self) -> Result<bool> {
        Ok(self
            .accepted
            .try_with(|value| value.enabled && !value.readonly)?)
    }
    fn classify(&self) -> Result<DateTimeRangeStatus> {
        let snapshot = self.accepted.try_get()?;
        let mut statuses = [DateTimeStatus::Empty; 2];
        for (index, output) in statuses.iter_mut().enumerate() {
            *output = match self.time_status[index].get() {
                TimePickerStatus::Empty | TimePickerStatus::Valid(_) => self.parts[index]
                    .try_get()?
                    .classify(snapshot.bounds, &self.policy),
                TimePickerStatus::Incomplete => DateTimeStatus::Incomplete,
                status => DateTimeStatus::InvalidTime(status),
            };
        }
        Ok(match statuses {
            [DateTimeStatus::Empty, DateTimeStatus::Empty] => DateTimeRangeStatus::Empty,
            [DateTimeStatus::Valid(start), DateTimeStatus::Empty] => {
                DateTimeRangeStatus::Start(start)
            }
            [DateTimeStatus::Valid(start), DateTimeStatus::Valid(end)]
                if start.local <= end.local =>
            {
                DateTimeRangeStatus::Complete { start, end }
            }
            [DateTimeStatus::Valid(_), DateTimeStatus::Valid(_)] => DateTimeRangeStatus::Reversed,
            [DateTimeStatus::Incomplete, _] | [DateTimeStatus::Empty, DateTimeStatus::Valid(_)] => {
                DateTimeRangeStatus::Incomplete
            }
            [DateTimeStatus::Valid(_), DateTimeStatus::Incomplete]
            | [DateTimeStatus::Empty, DateTimeStatus::Incomplete] => {
                DateTimeRangeStatus::Incomplete
            }
            [start, DateTimeStatus::Empty | DateTimeStatus::Incomplete] if start.is_error() => {
                DateTimeRangeStatus::InvalidStart(start)
            }
            [DateTimeStatus::Valid(_) | DateTimeStatus::Empty, end] => {
                DateTimeRangeStatus::InvalidEnd(end)
            }
            [start, _] => DateTimeRangeStatus::InvalidStart(start),
        })
    }
    fn publish(&self) -> Result {
        let status = self.classify()?;
        if self.status.try_get()? != status {
            self.status.try_revise(|current| *current = status)?;
            self.drafted.emit(self.owner, status);
        }
        if let Some(proposal) = status.proposal() {
            self.pending.set(Some(proposal));
            self.control.accept(&proposal)?;
            self.changed.emit(self.owner, proposal);
            self.dirty.touch()?;
        }
        Ok(())
    }
    fn calendar(&self, selection: DateSelection) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        let DateSelection::Range(range) = selection else {
            return Err(invalid());
        };
        match range {
            DateRangeSelection::Empty => {
                for index in 0..2 {
                    self.parts[index].try_revise(|parts| *parts = DateTimeParts::default())?;
                    self.time_status[index].set(TimePickerStatus::Empty);
                }
            }
            DateRangeSelection::Start(date) => {
                self.parts[0].try_revise(|parts| {
                    parts.date = Some(date);
                    if parts.time.is_none() {
                        parts.time = self.default_times[0];
                    }
                })?;
                self.time_status[0].set(
                    self.default_times[0].map_or(TimePickerStatus::Empty, TimePickerStatus::Valid),
                );
                self.parts[1].try_revise(|parts| *parts = DateTimeParts::default())?;
                self.time_status[1].set(TimePickerStatus::Empty);
            }
            DateRangeSelection::Complete(range) => {
                for (index, date) in [range.start(), range.end()].into_iter().enumerate() {
                    self.parts[index].try_revise(|parts| {
                        parts.date = Some(date);
                        if parts.time.is_none() {
                            parts.time = self.default_times[index];
                        }
                    })?;
                    if self.time_status[index].get() == TimePickerStatus::Empty {
                        self.time_status[index].set(
                            self.default_times[index]
                                .map_or(TimePickerStatus::Empty, TimePickerStatus::Valid),
                        );
                    }
                }
            }
        }
        self.publish()
    }
    fn time_draft(&self, index: usize, draft: TimePickerDraft) -> Result {
        self.ensure()?;
        if self.writable()? {
            self.time_status[index].set(draft.status);
            if !matches!(
                draft.status,
                TimePickerStatus::Valid(_) | TimePickerStatus::Empty
            ) {
                self.publish()?;
            }
        }
        Ok(())
    }
    fn time(&self, index: usize, value: Option<LocalTime>) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.parts[index].try_revise(|parts| parts.time = value)?;
        self.time_status[index].set(value.map_or(TimePickerStatus::Empty, TimePickerStatus::Valid));
        self.publish()
    }
    fn choose(&self, value: DateTimeRangeValue) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.set_parts(value)?;
        self.publish()
    }
    fn set_parts(&self, value: DateTimeRangeValue) -> Result {
        for (index, endpoint) in [value.start, value.end].into_iter().enumerate() {
            self.parts[index].try_revise(|parts| *parts = DateTimeParts::from_local(endpoint))?;
            self.time_status[index].set(endpoint.map_or(TimePickerStatus::Empty, |value| {
                TimePickerStatus::Valid(value.time())
            }));
        }
        Ok(())
    }
    fn done(&self) -> Result {
        let status = self.classify()?;
        if matches!(
            status,
            DateTimeRangeStatus::Empty | DateTimeRangeStatus::Complete { .. }
        ) {
            self.committed.emit(self.owner, status.proposal().unwrap());
        } else {
            self.rejected.emit(self.owner, status);
        }
        Ok(())
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
        self.changed.clear();
        self.drafted.clear();
        self.committed.clear();
        self.rejected.clear();
    }
}

pub struct InlineDateTimeRangePicker {
    common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    policy: DateTimePolicy,
    value: Value<DateTimeRangeValue>,
    bounds: Vary<DateTimeBounds>,
    revision: Vary<u64>,
    format: TimeFormat,
    default_times: [Option<LocalTime>; 2],
    readonly: Vary<bool>,
    field: Option<FieldControl>,
    direction: Direction,
    month: Option<Vary<YearMonth>>,
    default_month: Option<YearMonth>,
    level: Option<Vary<DatePickerLevel>>,
    default_level: DatePickerLevel,
    max_level: DatePickerLevel,
    columns: u8,
    columns_to_scroll: Option<u8>,
    hide_outside_dates: bool,
    hide_weekdays: bool,
    week_numbers: bool,
    presets: Vec<DateTimeRangePreset>,
    refs: [Option<Cage<Option<NodeId>>>; 4],
    changed: Option<Box<dyn FnMut(DateTimeRangeValue)>>,
    drafted: Option<Box<dyn FnMut(DateTimeRangeStatus)>>,
    committed: Option<Box<dyn FnMut(DateTimeRangeValue)>>,
    rejected: Option<Box<dyn FnMut(DateTimeRangeStatus)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}
pub fn inline_date_time_range_picker(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
    policy: DateTimePolicy,
) -> InlineDateTimeRangePicker {
    InlineDateTimeRangePicker {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        policy,
        value: Value::default(),
        bounds: DateTimeBounds::default().into(),
        revision: 0.into(),
        format: TimeFormat::default(),
        default_times: [None; 2],
        readonly: false.into(),
        field: None,
        direction: Direction::Ltr,
        month: None,
        default_month: None,
        level: None,
        default_level: DatePickerLevel::Month,
        max_level: DatePickerLevel::Decade,
        columns: 1,
        columns_to_scroll: None,
        hide_outside_dates: false,
        hide_weekdays: false,
        week_numbers: false,
        presets: Vec::new(),
        refs: [None; 4],
        changed: None,
        drafted: None,
        committed: None,
        rejected: None,
        month_changed: None,
        level_changed: None,
    }
}
impl InlineDateTimeRangePicker {
    common_methods!();
    pub fn value(mut self, value: impl Into<Vary<DateTimeRangeValue>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: DateTimeRangeValue) -> Self {
        self.value.default = value;
        self
    }
    pub fn bounds(mut self, value: impl Into<Vary<DateTimeBounds>>) -> Self {
        self.bounds = value.into();
        self
    }
    pub fn reset_revision(mut self, value: impl Into<Vary<u64>>) -> Self {
        self.revision = value.into();
        self
    }
    pub fn format(mut self, value: TimeFormat) -> Self {
        self.format = value;
        self
    }
    pub fn default_times(mut self, start: LocalTime, end: LocalTime) -> Self {
        self.default_times = [Some(start), Some(end)];
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn field(mut self, value: FieldControl) -> Self {
        self.common.theme = value.theme.clone();
        self.common.size = value.size;
        self.common.unstyled = value.unstyled;
        self.field = Some(value);
        self
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn month(mut self, value: impl Into<Vary<YearMonth>>) -> Self {
        self.month = Some(value.into());
        self
    }
    pub fn default_month(mut self, value: YearMonth) -> Self {
        self.default_month = Some(value);
        self
    }
    pub fn level(mut self, value: impl Into<Vary<DatePickerLevel>>) -> Self {
        self.level = Some(value.into());
        self
    }
    pub fn default_level(mut self, value: DatePickerLevel) -> Self {
        self.default_level = value;
        self
    }
    pub fn max_level(mut self, value: DatePickerLevel) -> Self {
        self.max_level = value;
        self
    }
    pub fn number_of_columns(mut self, value: u8) -> Self {
        self.columns = value;
        self
    }
    pub fn columns_to_scroll(mut self, value: u8) -> Self {
        self.columns_to_scroll = Some(value);
        self
    }
    pub fn hide_outside_dates(mut self, value: bool) -> Self {
        self.hide_outside_dates = value;
        self
    }
    pub fn hide_weekdays(mut self, value: bool) -> Self {
        self.hide_weekdays = value;
        self
    }
    pub fn with_week_numbers(mut self, value: bool) -> Self {
        self.week_numbers = value;
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DateTimeRangePreset>) -> Self {
        self.presets = values
            .into_iter()
            .take(MAX_DATE_TIME_RANGE_PRESETS + 1)
            .collect();
        self
    }
    pub fn start_hour_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.refs[0] = Some(value);
        self
    }
    pub fn start_minute_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.refs[1] = Some(value);
        self
    }
    pub fn end_hour_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.refs[2] = Some(value);
        self
    }
    pub fn end_minute_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.refs[3] = Some(value);
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(DateTimeRangeValue) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_draft_change(mut self, value: impl FnMut(DateTimeRangeStatus) + 'static) -> Self {
        self.drafted = Some(Box::new(value));
        self
    }
    pub fn on_commit(mut self, value: impl FnMut(DateTimeRangeValue) + 'static) -> Self {
        self.committed = Some(Box::new(value));
        self
    }
    pub fn on_reject(mut self, value: impl FnMut(DateTimeRangeStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(value));
        self
    }
    pub fn on_month_change(mut self, value: impl FnMut(YearMonth) + 'static) -> Self {
        self.month_changed = Some(Box::new(value));
        self
    }
    pub fn on_level_change(mut self, value: impl FnMut(DatePickerLevel) + 'static) -> Self {
        self.level_changed = Some(Box::new(value));
        self
    }
}

impl Widget for InlineDateTimeRangePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.is_empty()
            || self.key.len() > 48
            || !self
                .key
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            || self.presets.len() > MAX_DATE_TIME_RANGE_PRESETS
            || self.columns == 0
            || self.columns > crate::MAX_DATE_PICKER_COLUMNS
            || self
                .columns_to_scroll
                .is_some_and(|value| value == 0 || value > crate::MAX_DATE_PICKER_COLUMNS)
            || self.presets.iter().any(|preset| {
                preset.label.trim().is_empty()
                    || preset.label.len() > 120
                    || preset.label.contains(['\0', '\r', '\n'])
                    || !preset.value.is_complete()
            })
            || self.presets.iter().enumerate().any(|(index, preset)| {
                self.presets[..index].iter().any(|previous| {
                    previous.label == preset.label || previous.value == preset.value
                })
            })
        {
            return Err(invalid());
        }
        for value in self.default_times.into_iter().flatten() {
            self.format.validate(value).map_err(|_| invalid())?;
        }
        let enabled = self
            .field
            .as_ref()
            .map(|field| field.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let readonly = self
            .field
            .as_ref()
            .map(|field| field.readonly_state(scope, self.readonly.clone()))
            .unwrap_or_else(|| self.readonly.clone());
        let control = std::mem::take(&mut self.value).mount(scope);
        let value = control.source.clone();
        let provider = self.provider.clone();
        let bounds = self.bounds.clone();
        let revision = self.revision.clone();
        let source = scope.bond(move || Snapshot {
            value: value.get(),
            provider: provider.get(),
            bounds: bounds.get(),
            enabled: enabled.get(),
            readonly: readonly.get(),
            revision: revision.get(),
        });
        let format = self.format;
        let accepted = composite::validated(scope, &source.into(), move |snapshot: &Snapshot| {
            DateTimeRangeValue::new(snapshot.value.start, snapshot.value.end)
                .map_err(|_| invalid())?;
            let provider = snapshot
                .bounds
                .provider(snapshot.provider)
                .map_err(|_| invalid())?;
            for endpoint in [snapshot.value.start, snapshot.value.end]
                .into_iter()
                .flatten()
            {
                if !snapshot.bounds.contains(endpoint)
                    || !provider.date_bounds().contains(endpoint.date())
                    || format.validate(endpoint.time()).is_err()
                {
                    return Err(invalid());
                }
            }
            Ok(())
        })?;
        let initial = accepted.get();
        let state = Rc::new(RangeController {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            accepted,
            parts: [
                scope.cage(DateTimeParts::from_local(initial.value.start)),
                scope.cage(DateTimeParts::from_local(initial.value.end)),
            ],
            time_status: [
                Cell::new(
                    initial
                        .value
                        .start
                        .map_or(TimePickerStatus::Empty, |value| {
                            TimePickerStatus::Valid(value.time())
                        }),
                ),
                Cell::new(initial.value.end.map_or(TimePickerStatus::Empty, |value| {
                    TimePickerStatus::Valid(value.time())
                })),
            ],
            policy: self.policy.clone(),
            default_times: self.default_times,
            status: scope.cage(DateTimeRangeStatus::Empty),
            pending: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            drafted: Callback::new(self.drafted.take()),
            committed: Callback::new(self.committed.take()),
            rejected: Callback::new(self.rejected.take()),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = cleanup.upgrade() {
                state.retire();
            }
        })?;
        let root = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let (class, appearance) = self.common.appearance(
            scope,
            "sapid-inline-date-time-range-picker",
            Style::new()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Column)
                .gap(8.),
        );
        let mut root_widget = glory::ui::view()
            .role(Role::Group)
            .name(
                self.common
                    .name
                    .take()
                    .unwrap_or_else(|| "Date and time range".into()),
            )
            .class(class)
            .style(appearance)
            .node_ref(root)
            .focus(self.common.focus);
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            root_widget = root_widget.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        root_widget.show_in(scope)?;
        let content = scope.child_in(root.get().ok_or(glory::ui::Error::Gone)?)?;
        let parts = state.parts;
        let calendar_value = scope.bond(move || {
            match (
                parts[0].with(|parts| parts.date),
                parts[1].with(|parts| parts.date),
            ) {
                (None, None) => DateSelection::Range(DateRangeSelection::Empty),
                (Some(start), None) => DateSelection::Range(DateRangeSelection::Start(start)),
                (Some(start), Some(end)) => DateSelection::Range(DateRangeSelection::Complete(
                    crate::CivilRange::new(start.min(end), start.max(end)).unwrap(),
                )),
                (None, Some(_)) => DateSelection::Range(DateRangeSelection::Empty),
            }
        });
        let choose_date = state.clone();
        let mut calendar = crate::date_picker(
            format!("{}-date", self.key),
            accepted.map(|snapshot| snapshot.bounds.provider(snapshot.provider).unwrap()),
        )
        .mode(DatePickerMode::Range)
        .value(calendar_value)
        .enabled(accepted.map(|snapshot| snapshot.enabled))
        .readonly(accepted.map(|snapshot| snapshot.readonly))
        .direction(self.direction)
        .theme(self.common.theme.clone())
        .size(self.common.size)
        .name("Date range")
        .default_level(self.default_level)
        .max_level(self.max_level)
        .number_of_columns(self.columns)
        .hide_outside_dates(self.hide_outside_dates)
        .hide_weekdays(self.hide_weekdays)
        .with_week_numbers(self.week_numbers)
        .on_value_change(move |value| choose_date.report(choose_date.calendar(value)));
        if let Some(value) = self.columns_to_scroll {
            calendar = calendar.columns_to_scroll(value);
        }
        if let Some(value) = self.month.take() {
            calendar = calendar.month(value);
        }
        if let Some(value) = self.default_month {
            calendar = calendar.default_month(value);
        }
        if let Some(value) = self.level.take() {
            calendar = calendar.level(value);
        }
        if let Some(callback) = self.month_changed.take() {
            calendar = calendar.on_month_change(callback);
        }
        if let Some(callback) = self.level_changed.take() {
            calendar = calendar.on_level_change(callback);
        }
        if self.common.unstyled {
            calendar = calendar.unstyled();
        }
        calendar.show_in(&content)?;
        if !self.presets.is_empty() {
            let mut buttons: Vec<Box<dyn Widget>> = Vec::with_capacity(self.presets.len());
            for preset in std::mem::take(&mut self.presets) {
                let value = preset.value;
                let choose = state.clone();
                let selected = accepted.map(move |snapshot| snapshot.value == value);
                let enabled = accepted.map(|snapshot| snapshot.enabled && !snapshot.readonly);
                let mut button = crate::toggle()
                    .theme(self.common.theme.clone())
                    .size(self.common.size)
                    .class("sapid-date-time-range-preset")
                    .label(preset.label)
                    .pressed(selected)
                    .enabled(enabled)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.choose(value));
                        }
                    });
                if self.common.unstyled {
                    button = button.unstyled();
                }
                buttons.push(Box::new(button));
            }
            crate::group()
                .theme(self.common.theme.clone())
                .class("sapid-date-time-range-presets")
                .gap(6.)
                .fill(buttons)
                .show_in(&content)?;
        }
        let mut time_widgets: Vec<Box<dyn Widget>> = Vec::with_capacity(2);
        for index in 0..2 {
            let on_time = state.clone();
            let on_draft = state.clone();
            let mut picker = crate::time_picker(format!("{}-time-{index}", self.key))
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .name(if index == 0 { "Start time" } else { "End time" })
                .format(format)
                .value(state.parts[index].map(|parts| parts.time))
                .enabled(accepted.map(|snapshot| snapshot.enabled))
                .readonly(accepted.map(|snapshot| snapshot.readonly))
                .on_draft_change(move |draft| on_draft.report(on_draft.time_draft(index, draft)))
                .on_value_change(move |value| on_time.report(on_time.time(index, value)));
            if let Some(reference) = self.refs[index * 2] {
                picker = picker.hour_ref(reference);
            }
            if let Some(reference) = self.refs[index * 2 + 1] {
                picker = picker.minute_ref(reference);
            }
            if self.common.unstyled {
                picker = picker.unstyled();
            }
            time_widgets.push(Box::new(picker));
        }
        crate::group()
            .theme(self.common.theme.clone())
            .class("sapid-date-time-range-times")
            .gap(16.)
            .fill(time_widgets)
            .show_in(&content)?;
        let status = state.status;
        glory::ui::text()
            .class("sapid-date-time-range-status")
            .content(status.map(|status| match status {
                DateTimeRangeStatus::Incomplete => "Complete both date/time endpoints".into(),
                DateTimeRangeStatus::Reversed => "End must not precede start".into(),
                DateTimeRangeStatus::InvalidStart(_) => "Start date/time is invalid".into(),
                DateTimeRangeStatus::InvalidEnd(_) => "End date/time is invalid".into(),
                _ => String::new(),
            }))
            .show_in(&content)?;
        for (clear, label) in [(true, "Clear date and time range"), (false, "Done")] {
            let action = state.clone();
            crate::button()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .label(label)
                .enabled(accepted.map(|snapshot| snapshot.enabled && !snapshot.readonly))
                .on_press(move || {
                    action.report(if clear {
                        action.choose(DateTimeRangeValue::default())
                    } else {
                        action.done()
                    })
                })
                .show_in(&content)?;
        }
        let weak = Rc::downgrade(&state);
        let mut previous = initial;
        scope.effect(move || {
            let Some(state) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let dirty = state.dirty.take();
                let next = accepted.get();
                let changed = next.value != previous.value || next.revision != previous.revision;
                if dirty || changed {
                    let pending = state.pending.take();
                    if pending != Some(next.value) {
                        state.set_parts(next.value)?;
                    }
                    previous = next;
                }
                let status = state.classify()?;
                if state.status.try_get()? != status {
                    state.status.try_revise(|current| *current = status)?;
                }
                Ok(())
            })();
            state.report(result);
        });
        scope.retain(content)?;
        scope.retain(state)
    }
}

/// Input-style date/time range picker backed by one atomic range value.
pub struct DateTimeRangePicker {
    common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    policy: DateTimePolicy,
    value: Value<DateTimeRangeValue>,
    open: Value<bool>,
    bounds: Vary<DateTimeBounds>,
    revision: Vary<u64>,
    format: TimeFormat,
    display_format: DateTimeDisplayFormat,
    placeholder: Vary<String>,
    separator: String,
    default_times: [Option<LocalTime>; 2],
    presets: Vec<DateTimeRangePreset>,
    clearable: bool,
    readonly: Vary<bool>,
    required: Vary<bool>,
    invalid: Vary<bool>,
    field: Option<FieldControl>,
    form_name: Option<Vary<String>>,
    direction: Direction,
    month: Option<Vary<YearMonth>>,
    default_month: Option<YearMonth>,
    level: Option<Vary<DatePickerLevel>>,
    default_level: DatePickerLevel,
    max_level: DatePickerLevel,
    columns: u8,
    columns_to_scroll: Option<u8>,
    hide_outside_dates: bool,
    hide_weekdays: bool,
    week_numbers: bool,
    content_policy: ContentPolicy,
    trigger_ref: Option<Cage<Option<NodeId>>>,
    panel_ref: Option<Cage<Option<NodeId>>>,
    changed: Option<Box<dyn FnMut(DateTimeRangeValue)>>,
    committed: Option<Box<dyn FnMut(DateTimeRangeValue)>>,
    rejected: Option<Box<dyn FnMut(DateTimeRangeStatus)>>,
    opened: Option<Box<dyn FnMut(bool)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}

pub fn date_time_range_picker(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
    policy: DateTimePolicy,
) -> DateTimeRangePicker {
    DateTimeRangePicker {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        policy,
        value: Value::default(),
        open: Value::default(),
        bounds: DateTimeBounds::default().into(),
        revision: 0.into(),
        format: TimeFormat::default(),
        display_format: DateTimeDisplayFormat::default(),
        placeholder: "Pick date and time range".to_owned().into(),
        separator: " – ".into(),
        default_times: [None; 2],
        presets: Vec::new(),
        clearable: false,
        readonly: false.into(),
        required: false.into(),
        invalid: false.into(),
        field: None,
        form_name: None,
        direction: Direction::Ltr,
        month: None,
        default_month: None,
        level: None,
        default_level: DatePickerLevel::Month,
        max_level: DatePickerLevel::Decade,
        columns: 1,
        columns_to_scroll: None,
        hide_outside_dates: false,
        hide_weekdays: false,
        week_numbers: false,
        content_policy: ContentPolicy::KeepMounted,
        trigger_ref: None,
        panel_ref: None,
        changed: None,
        committed: None,
        rejected: None,
        opened: None,
        month_changed: None,
        level_changed: None,
    }
}

impl DateTimeRangePicker {
    common_methods!();
    pub fn value(mut self, value: impl Into<Vary<DateTimeRangeValue>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: DateTimeRangeValue) -> Self {
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
    pub fn bounds(mut self, value: impl Into<Vary<DateTimeBounds>>) -> Self {
        self.bounds = value.into();
        self
    }
    pub fn reset_revision(mut self, value: impl Into<Vary<u64>>) -> Self {
        self.revision = value.into();
        self
    }
    pub fn format(mut self, value: TimeFormat) -> Self {
        self.format = value;
        self
    }
    pub fn display_format(mut self, value: DateTimeDisplayFormat) -> Self {
        self.display_format = value;
        self
    }
    pub fn placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholder = value.into();
        self
    }
    pub fn separator(mut self, value: impl Into<String>) -> Self {
        self.separator = value.into();
        self
    }
    pub fn default_times(mut self, start: LocalTime, end: LocalTime) -> Self {
        self.default_times = [Some(start), Some(end)];
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DateTimeRangePreset>) -> Self {
        self.presets = values
            .into_iter()
            .take(MAX_DATE_TIME_RANGE_PRESETS + 1)
            .collect();
        self
    }
    pub fn clearable(mut self, value: bool) -> Self {
        self.clearable = value;
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn required(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.required = value.into();
        self
    }
    pub fn invalid(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.invalid = value.into();
        self
    }
    pub fn field(mut self, value: FieldControl) -> Self {
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
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn month(mut self, value: impl Into<Vary<YearMonth>>) -> Self {
        self.month = Some(value.into());
        self
    }
    pub fn default_month(mut self, value: YearMonth) -> Self {
        self.default_month = Some(value);
        self
    }
    pub fn level(mut self, value: impl Into<Vary<DatePickerLevel>>) -> Self {
        self.level = Some(value.into());
        self
    }
    pub fn default_level(mut self, value: DatePickerLevel) -> Self {
        self.default_level = value;
        self
    }
    pub fn max_level(mut self, value: DatePickerLevel) -> Self {
        self.max_level = value;
        self
    }
    pub fn number_of_columns(mut self, value: u8) -> Self {
        self.columns = value;
        self
    }
    pub fn columns_to_scroll(mut self, value: u8) -> Self {
        self.columns_to_scroll = Some(value);
        self
    }
    pub fn hide_outside_dates(mut self, value: bool) -> Self {
        self.hide_outside_dates = value;
        self
    }
    pub fn hide_weekdays(mut self, value: bool) -> Self {
        self.hide_weekdays = value;
        self
    }
    pub fn with_week_numbers(mut self, value: bool) -> Self {
        self.week_numbers = value;
        self
    }
    pub fn content_policy(mut self, value: ContentPolicy) -> Self {
        self.content_policy = value;
        self
    }
    pub fn trigger_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.trigger_ref = Some(value);
        self
    }
    pub fn panel_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.panel_ref = Some(value);
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(DateTimeRangeValue) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_commit(mut self, value: impl FnMut(DateTimeRangeValue) + 'static) -> Self {
        self.committed = Some(Box::new(value));
        self
    }
    pub fn on_reject(mut self, value: impl FnMut(DateTimeRangeStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(value));
        self
    }
    pub fn on_open_change(mut self, value: impl FnMut(bool) + 'static) -> Self {
        self.opened = Some(Box::new(value));
        self
    }
    pub fn on_month_change(mut self, value: impl FnMut(YearMonth) + 'static) -> Self {
        self.month_changed = Some(Box::new(value));
        self
    }
    pub fn on_level_change(mut self, value: impl FnMut(DatePickerLevel) + 'static) -> Self {
        self.level_changed = Some(Box::new(value));
        self
    }
}

impl Widget for DateTimeRangePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.separator.is_empty()
            || self.separator.len() > 16
            || self.separator.contains(['\0', '\r', '\n'])
            || self.columns == 0
            || self.columns > crate::MAX_DATE_PICKER_COLUMNS
            || self
                .columns_to_scroll
                .is_some_and(|value| value == 0 || value > crate::MAX_DATE_PICKER_COLUMNS)
        {
            return Err(invalid());
        }
        let control = std::mem::take(&mut self.value).mount(scope);
        let open = std::mem::take(&mut self.open).mount(scope);
        let enabled = self
            .field
            .as_ref()
            .map(|field| field.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let readonly = self
            .field
            .as_ref()
            .map(|field| field.readonly_state(scope, self.readonly.clone()))
            .unwrap_or_else(|| self.readonly.clone());
        let source = control.source.clone();
        let bounds_source = self.bounds.clone();
        let provider_source = self.provider.clone();
        let format = self.format;
        let snapshot =
            scope.bond(move || (source.get(), bounds_source.get(), provider_source.get()));
        let accepted = composite::validated(
            scope,
            &snapshot.into(),
            move |value: &(DateTimeRangeValue, DateTimeBounds, DatesProvider)| {
                DateTimeRangeValue::new(value.0.start, value.0.end).map_err(|_| invalid())?;
                let provider = value.1.provider(value.2).map_err(|_| invalid())?;
                for endpoint in [value.0.start, value.0.end].into_iter().flatten() {
                    if !value.1.contains(endpoint)
                        || !provider.date_bounds().contains(endpoint.date())
                        || format.validate(endpoint.time()).is_err()
                    {
                        return Err(invalid());
                    }
                }
                Ok(())
            },
        )?;
        let placeholder = composite::validated(scope, &self.placeholder, |value| {
            if value.len() <= 240 && !value.contains(['\0', '\r', '\n']) {
                Ok(())
            } else {
                Err(invalid())
            }
        })?;
        let owner = scope.id();
        let weak = scope.downgrade();
        let changed = Callback::new(self.changed.take());
        let committed = Callback::new(self.committed.take());
        let rejected = Callback::new(self.rejected.take());
        let opened = Callback::new(self.opened.take());
        let month_changed = Callback::new(self.month_changed.take());
        let level_changed = Callback::new(self.level_changed.take());
        let cleanup = (
            changed.clone(),
            committed.clone(),
            rejected.clone(),
            opened.clone(),
            month_changed.clone(),
            level_changed.clone(),
        );
        scope.on_cleanup(move || {
            cleanup.0.clear();
            cleanup.1.clear();
            cleanup.2.clear();
            cleanup.3.clear();
            cleanup.4.clear();
            cleanup.5.clear();
        })?;
        let metadata = crate::field::control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            true.into(),
            self.invalid.clone(),
        );
        let form_name = composite::form_name(scope, &self.form_name)?;
        let metadata = scope.bond(move || {
            let mut value = metadata.get().unwrap_or_default();
            value.placeholder = Some(placeholder.get());
            if let Some(name) = form_name {
                value.form_control = Some(FormControl {
                    name: name.get(),
                    value: None,
                });
            }
            Some(value)
        });
        let wrapper = scope.cage(None);
        glory::ui::view()
            .class("sapid-date-time-range-picker")
            .style(Style {
                align: Some(Align::Center),
                ..Style::new().display(Display::Flex).gap(8.)
            })
            .node_ref(wrapper)
            .show_in(scope)?;
        let content = scope.child_in(wrapper.get().ok_or(glory::ui::Error::Gone)?)?;
        let (class, appearance) = self.common.appearance(
            scope,
            "sapid-date-time-range-display",
            self.common.theme.control(self.common.size),
        );
        let style = scope.bond(move || {
            let mut value = appearance.get();
            value.width.get_or_insert(Length::Px(360.));
            value.min_height.get_or_insert(Length::Px(32.));
            value
        });
        let display_format = self.display_format;
        let separator = self.separator.clone();
        let mut input = glory::ui::input()
            .class(class)
            .style(style)
            .enabled(enabled.clone())
            .focus(self.common.focus)
            .value(
                accepted.map(move |value| match (value.0.start, value.0.end) {
                    (None, _) => String::new(),
                    (Some(start), None) => display_format.format(start, format),
                    (Some(start), Some(end)) => format!(
                        "{}{}{}",
                        display_format.format(start, format),
                        separator,
                        display_format.format(end, format)
                    ),
                }),
            )
            .semantics(metadata);
        if let Some(name) = self.common.name.take() {
            input = input.name(name);
        }
        if let Some(reference) = self.common.node_ref {
            input = input.node_ref(reference);
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            input = input.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        input.show_in(&content)?;
        if self.clearable {
            let clear_control = control.clone();
            let clear_changed = changed.clone();
            let clear_weak = scope.downgrade();
            crate::button()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .variant(crate::ButtonVariant::Subtle)
                .compact(true)
                .label("Clear range")
                .name("Clear date and time range")
                .enabled(scope.bond({
                    let enabled = enabled.clone();
                    let readonly = readonly.clone();
                    move || enabled.get() && !readonly.get()
                }))
                .on_press(
                    move || match clear_control.accept(&DateTimeRangeValue::default()) {
                        Ok(()) => clear_changed.emit(owner, DateTimeRangeValue::default()),
                        Err(error) => {
                            if let Some(scope) = clear_weak.upgrade() {
                                scope.report_error(error)
                            }
                        }
                    },
                )
                .show_in(&content)?;
        }
        let key = format!("{}-inline", self.key);
        let provider = self.provider.clone();
        let policy = self.policy.clone();
        let bounds = self.bounds.clone();
        let revision = self.revision.clone();
        let value_source = control.source.clone();
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        let direction = self.direction;
        let month = self.month.take();
        let default_month = self.default_month;
        let level = self.level.take();
        let default_level = self.default_level;
        let max_level = self.max_level;
        let columns = self.columns;
        let columns_to_scroll = self.columns_to_scroll;
        let hide_outside_dates = self.hide_outside_dates;
        let hide_weekdays = self.hide_weekdays;
        let week_numbers = self.week_numbers;
        let default_times = self.default_times;
        let presets = self.presets.clone();
        let close = open.clone();
        let close_opened = opened.clone();
        let range_control = control.clone();
        let focus = composite::id("dialog", &format!("{}-dialog", self.key), "close", "root")?;
        let mut dialog = crate::dialog(format!("{}-dialog", self.key))
            .label("Choose date and time range")
            .title("Date and time range")
            .close_label("Close date and time range")
            .initial_focus(focus)
            .theme(theme.clone())
            .size(size)
            .enabled(enabled)
            .open(open.source.clone())
            .content_policy(self.content_policy)
            .on_open_change(move |value| {
                if owner.is_alive() {
                    match open.accept(&value) {
                        Ok(()) => opened.emit(owner, value),
                        Err(error) => {
                            if let Some(scope) = weak.upgrade() {
                                scope.report_error(error)
                            }
                        }
                    }
                }
            })
            .content(move || {
                let control = range_control.clone();
                let changed = changed.clone();
                let committed = committed.clone();
                let rejected = rejected.clone();
                let close = close.clone();
                let opened = close_opened.clone();
                let month_changed = month_changed.clone();
                let level_changed = level_changed.clone();
                let accepted_source = value_source.clone();
                let mut picker =
                    inline_date_time_range_picker(key.clone(), provider.clone(), policy.clone())
                        .value(value_source.clone())
                        .bounds(bounds.clone())
                        .reset_revision(revision.clone())
                        .format(format)
                        .presets(presets.clone())
                        .theme(theme.clone())
                        .size(size)
                        .enabled(true)
                        .readonly(readonly.clone())
                        .direction(direction)
                        .default_level(default_level)
                        .max_level(max_level)
                        .number_of_columns(columns)
                        .hide_outside_dates(hide_outside_dates)
                        .hide_weekdays(hide_weekdays)
                        .with_week_numbers(week_numbers)
                        .on_value_change(move |value| {
                            if owner.is_alive() && control.accept(&value).is_ok() {
                                changed.emit(owner, value);
                            }
                        })
                        .on_reject(move |value| rejected.emit(owner, value))
                        .on_month_change(move |value| month_changed.emit(owner, value))
                        .on_level_change(move |value| level_changed.emit(owner, value))
                        .on_commit(move |value| {
                            committed.emit(owner, value);
                            if owner.is_alive()
                                && accepted_source.try_get().ok() == Some(value)
                                && close.accept(&false).is_ok()
                            {
                                opened.emit(owner, false);
                            }
                        });
                if let Some(value) = columns_to_scroll {
                    picker = picker.columns_to_scroll(value);
                }
                if let Some(value) = &month {
                    picker = picker.month(value.clone());
                }
                if let Some(value) = default_month {
                    picker = picker.default_month(value);
                }
                if let Some(value) = &level {
                    picker = picker.level(value.clone());
                }
                if let [Some(start), Some(end)] = default_times {
                    picker = picker.default_times(start, end);
                }
                if unstyled {
                    picker = picker.unstyled();
                }
                picker
            });
        if let Some(reference) = self.trigger_ref {
            dialog = dialog.trigger_ref(reference);
        }
        if let Some(reference) = self.panel_ref {
            dialog = dialog.panel_ref(reference);
        }
        if self.common.unstyled {
            dialog = dialog.unstyled();
        }
        dialog.show_in(&content)?;
        scope.retain(content)
    }
}
