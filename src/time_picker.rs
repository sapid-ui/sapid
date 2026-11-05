//! Segmented real spinbutton editors over one typed wall-clock value.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback};
use crate::state::{Control, Dirty, Value};
use crate::{
    Decimal, FieldControl, HourCycle, LocalTime, MAX_DISABLED_TIMES, NumberValue, TimeBounds,
    TimeFormat, TimeGridSpec, TimeParts, TimePickerDraft, TimePickerStatus, TimePrecision,
};
use glory::render::{Event, EventKind, NodeId, PopupKind, Role, Semantics};
use glory::style::{Display, FlexDirection, Length, Limit, Overflow, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};
pub const MAX_TIME_PICKER_PRESETS: usize = 96;
pub const MAX_TIME_PICKER_PRESET_GROUPS: usize = 16;
pub const MAX_TIME_PICKER_PASTE_BYTES: usize = 32;
type PasteSplit = Rc<dyn Fn(&str) -> Option<LocalTime>>;

fn default_paste_split(value: &str) -> Option<LocalTime> {
    LocalTime::parse_iso(value).ok()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePickerPresetGroup {
    pub label: String,
    pub values: Vec<LocalTime>,
}
impl TimePickerPresetGroup {
    pub fn new(label: impl Into<String>, values: impl IntoIterator<Item = LocalTime>) -> Self {
        Self {
            label: label.into(),
            values: values.into_iter().collect(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimePickerPresets {
    Values(Vec<LocalTime>),
    Groups(Vec<TimePickerPresetGroup>),
}
impl From<Vec<LocalTime>> for TimePickerPresets {
    fn from(value: Vec<LocalTime>) -> Self {
        Self::Values(value)
    }
}
impl From<Vec<TimePickerPresetGroup>> for TimePickerPresets {
    fn from(value: Vec<TimePickerPresetGroup>) -> Self {
        Self::Groups(value)
    }
}
fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid finite segmented time picker").into()
}
fn decimal(v: u8) -> Decimal {
    Decimal::new(i128::from(v), 0).unwrap()
}
fn number(v: Option<u8>) -> NumberValue {
    v.map_or(NumberValue::Empty, |v| NumberValue::Text(format!("{v:02}")))
}
fn part(v: &NumberValue) -> std::result::Result<Option<u8>, TimePickerStatus> {
    match v {
        NumberValue::Empty => Ok(None),
        NumberValue::Decimal(v) if v.scale() == 0 && (0..=99).contains(&v.coefficient()) => {
            Ok(Some(v.coefficient() as u8))
        }
        NumberValue::Text(v) if v.is_empty() => Ok(None),
        NumberValue::Text(v) if v.len() > 2 => Err(TimePickerStatus::TooLong),
        NumberValue::Text(v) if v.bytes().all(|b| b.is_ascii_digit()) => {
            Ok(Some(v.parse().map_err(|_| TimePickerStatus::Invalid)?))
        }
        _ => Err(TimePickerStatus::Invalid),
    }
}
#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: Option<LocalTime>,
    bounds: TimeBounds,
    enabled: bool,
    readonly: bool,
    reset_epoch: u64,
}
#[derive(Clone, Copy)]
struct Pending {
    baseline: Option<LocalTime>,
    proposal: Option<Option<LocalTime>>,
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<Option<LocalTime>>,
    accepted: Cage<Snapshot>,
    format: TimeFormat,
    parts: [Cage<NumberValue>; 3],
    afternoon: Cage<bool>,
    status: Cage<TimePickerStatus>,
    last: Cell<Option<TimePickerDraft>>,
    disabled: Rc<Vec<LocalTime>>,
    pending: Cell<Option<Pending>>,
    dirty: Dirty,
    changed: Callback<Option<LocalTime>>,
    committed: Callback<Option<LocalTime>>,
    draft_changed: Callback<TimePickerDraft>,
    rejected: Callback<TimePickerStatus>,
}

#[derive(Clone)]
struct PopupControl {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    control: Control<bool>,
    changed: Callback<bool>,
}
impl PopupControl {
    fn propose(&self, value: bool) -> Result {
        if !self.owner.is_alive() {
            return Err(glory::ui::Error::Gone);
        }
        self.control.accept(&value)?;
        self.changed.emit(self.owner, value);
        Ok(())
    }
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error);
        }
    }
}

#[derive(Clone)]
struct PresetContent {
    format: TimeFormat,
    accepted: Cage<Snapshot>,
    state: Rc<Controller>,
    popup: PopupControl,
    close_on_select: bool,
    theme: crate::Theme,
    size: crate::Size,
    unstyled: bool,
}

#[derive(Clone)]
struct TimeControlsContent {
    format: TimeFormat,
    accepted: Cage<Snapshot>,
    state: Rc<Controller>,
    steps: [u8; 3],
    period_labels: [String; 2],
    reverse: bool,
    max_height: f64,
    theme: crate::Theme,
    size: crate::Size,
    unstyled: bool,
}
impl TimeControlsContent {
    fn build(&self) -> Box<dyn Widget> {
        let displayed = if self.format.precision == TimePrecision::Second {
            3
        } else {
            2
        };
        let mut columns: Vec<Box<dyn Widget>> = Vec::with_capacity(displayed + 1);
        for index in 0..displayed {
            let (start, end) = if index == 0 && self.format.cycle == HourCycle::H12 {
                (1_u8, 12_u8)
            } else if index == 0 {
                (0, 23)
            } else {
                (0, 59)
            };
            let mut values = (start..=end)
                .filter(|value| (value - start) % self.steps[index] == 0)
                .collect::<Vec<_>>();
            if self.reverse {
                values.reverse();
            }
            let mut items: Vec<Box<dyn Widget>> = Vec::with_capacity(values.len());
            for value in values {
                let choose = self.state.clone();
                let selected = self.state.parts[index]
                    .map(move |current| part(current).ok().flatten() == Some(value));
                let ready = self
                    .accepted
                    .map(|snapshot| snapshot.enabled && !snapshot.readonly);
                let mut button = crate::toggle()
                    .theme(self.theme.clone())
                    .size(self.size)
                    .class("sapid-time-picker-control")
                    .label(format!("{value:02}"))
                    .pressed(selected)
                    .enabled(ready)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.segment(index, number(Some(value)), false));
                        }
                    });
                if self.unstyled {
                    button = button.unstyled();
                }
                items.push(Box::new(button));
            }
            columns.push(Box::new(
                crate::stack()
                    .theme(self.theme.clone())
                    .class("sapid-time-picker-control-column")
                    .gap(4.)
                    .fill(items),
            ));
        }
        if self.format.cycle == HourCycle::H12 {
            let mut periods: Vec<(bool, String)> = vec![
                (false, self.period_labels[0].clone()),
                (true, self.period_labels[1].clone()),
            ];
            if self.reverse {
                periods.reverse();
            }
            let mut items: Vec<Box<dyn Widget>> = Vec::with_capacity(2);
            for (afternoon, label) in periods {
                let choose = self.state.clone();
                let ready = self
                    .accepted
                    .map(|snapshot| snapshot.enabled && !snapshot.readonly);
                let mut button = crate::toggle()
                    .theme(self.theme.clone())
                    .size(self.size)
                    .class("sapid-time-picker-control sapid-time-picker-period-control")
                    .label(label)
                    .pressed(self.state.afternoon.map(move |value| *value == afternoon))
                    .enabled(ready)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.period(afternoon));
                        }
                    });
                if self.unstyled {
                    button = button.unstyled();
                }
                items.push(Box::new(button));
            }
            columns.push(Box::new(
                crate::stack()
                    .theme(self.theme.clone())
                    .class("sapid-time-picker-control-column sapid-time-picker-period-column")
                    .gap(4.)
                    .fill(items),
            ));
        }
        Box::new(
            crate::group()
                .theme(self.theme.clone())
                .class("sapid-time-picker-controls")
                .gap(6.)
                .style(
                    Style::new()
                        .max_height(Limit::Px(self.max_height))
                        .overflow_y(Overflow::Auto),
                )
                .fill(columns),
        )
    }
}
impl PresetContent {
    fn build(&self, groups: Vec<TimePickerPresetGroup>) -> Box<dyn Widget> {
        let mut sections: Vec<Box<dyn Widget>> = Vec::with_capacity(groups.len());
        for (group_index, group) in groups.into_iter().enumerate() {
            let mut buttons: Vec<Box<dyn Widget>> = Vec::with_capacity(group.values.len());
            for value in group.values {
                let label = self
                    .format
                    .format(value)
                    .expect("preset admission validates time precision");
                let selected = self
                    .accepted
                    .map(move |snapshot| snapshot.value == Some(value));
                let ready = self.accepted.map({
                    let disabled = self.state.disabled.clone();
                    move |snapshot| {
                        snapshot.enabled
                            && !snapshot.readonly
                            && snapshot.bounds.contains(value)
                            && disabled.binary_search(&value).is_err()
                    }
                });
                let choose = self.state.clone();
                let close = self.popup.clone();
                let close_on_select = self.close_on_select;
                let mut button = crate::toggle()
                    .theme(self.theme.clone())
                    .size(self.size)
                    .class("sapid-time-picker-preset")
                    .label(label)
                    .pressed(selected)
                    .enabled(ready)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.choose(Some(value)));
                            if close_on_select {
                                close.report(close.propose(false));
                            }
                        }
                    });
                if self.unstyled {
                    button = button.unstyled();
                }
                buttons.push(Box::new(button));
            }
            let columns = if self.format.precision == TimePrecision::Second {
                2
            } else {
                3
            };
            let items = buttons
                .into_iter()
                .enumerate()
                .map(|(index, button)| crate::grid_item(format!("preset-{index}")).fill(button));
            let grid = crate::simple_grid(format!("time-presets-{group_index}"))
                .theme(self.theme.clone())
                .size(self.size)
                .columns(columns)
                .gap(6.)
                .items(items);
            let mut children: Vec<Box<dyn Widget>> = Vec::new();
            if !group.label.is_empty() {
                children.push(Box::new(
                    crate::text()
                        .class("sapid-time-picker-preset-group-label")
                        .content(group.label),
                ));
            }
            children.push(Box::new(grid));
            let mut section = crate::stack()
                .theme(self.theme.clone())
                .class("sapid-time-picker-preset-group")
                .gap(6.)
                .fill(children);
            if self.unstyled {
                section = section.unstyled();
            }
            sections.push(Box::new(section));
        }
        let mut content = crate::stack()
            .theme(self.theme.clone())
            .class("sapid-time-picker-presets")
            .gap(12.)
            .fill(sections);
        if self.unstyled {
            content = content.unstyled();
        }
        Box::new(content)
    }
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn writable(&self) -> Result<bool> {
        self.accepted
            .try_with(|s| s.enabled && !s.readonly)
            .map_err(Into::into)
    }
    fn current(&self) -> Result<Option<LocalTime>> {
        let s = self.accepted.try_get()?;
        let value = self.control.source.try_get()?;
        Ok(
            if value.is_none_or(|v| s.bounds.contains(v) && self.format.validate(v).is_ok()) {
                value
            } else {
                s.value
            },
        )
    }
    fn parts(&self) -> Result<TimeParts> {
        Ok(TimeParts {
            hour: self.parts[0].try_with(part)?.map_err(|_| invalid())?,
            minute: self.parts[1].try_with(part)?.map_err(|_| invalid())?,
            second: if self.format.precision == TimePrecision::Second {
                self.parts[2].try_with(part)?.map_err(|_| invalid())?
            } else {
                None
            },
            afternoon: self.afternoon.get(),
        })
    }
    fn classify(&self, parts: TimeParts) -> Result<TimePickerStatus> {
        let status = parts.classify(self.format, self.accepted.try_with(|s| s.bounds)?);
        Ok(match status {
            TimePickerStatus::Valid(v) if self.disabled.binary_search(&v).is_ok() => {
                TimePickerStatus::Unavailable(v)
            }
            _ => status,
        })
    }
    fn publish_draft(&self, draft: TimePickerDraft) -> Result {
        if self.status.try_get()? != draft.status {
            self.status.try_revise(|v| *v = draft.status)?;
        }
        if self.last.replace(Some(draft)) != Some(draft) {
            self.draft_changed.emit(self.owner, draft);
            self.ensure()?;
        }
        Ok(())
    }
    fn reject(&self, status: TimePickerStatus) -> Result {
        self.ensure()?;
        self.publish_draft(TimePickerDraft {
            parts: self.parts()?,
            status,
        })?;
        self.rejected.emit(self.owner, status);
        self.ensure()?;
        self.pending.set(None);
        self.dirty.touch()
    }
    fn propose(&self, committing: bool) -> Result {
        let baseline = self.current()?;
        let parts = self.parts()?;
        let status = self.classify(parts)?;
        let proposal = status.proposal();
        self.pending.set(Some(Pending { baseline, proposal }));
        self.publish_draft(TimePickerDraft { parts, status })?;
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
            self.ensure()?;
        }
        self.dirty.touch()
    }
    fn segment(&self, index: usize, value: NumberValue, committing: bool) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        let parsed = match part(&value) {
            Ok(v) => v,
            Err(e) => return self.reject(e),
        };
        let value = if committing || matches!(value, NumberValue::Decimal(_)) {
            number(parsed)
        } else {
            value
        };
        if self.parts[index].try_with(|old| *old != value)? {
            self.parts[index].try_revise(|v| *v = value)?;
        }
        self.propose(committing)
    }
    fn period(&self, afternoon: bool) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.afternoon.try_revise(|v| *v = afternoon)?;
        self.propose(false)
    }
    fn set_parts(&self, value: Option<LocalTime>) -> Result {
        let parts = TimeParts::from_time(value, self.format).map_err(|_| invalid())?;
        for (index, part) in [parts.hour, parts.minute, parts.second]
            .into_iter()
            .enumerate()
        {
            let next = number(part);
            if self.parts[index].try_with(|old| *old != next)? {
                self.parts[index].try_revise(|v| *v = next)?;
            }
        }
        if self.afternoon.get() != parts.afternoon {
            self.afternoon.try_revise(|v| *v = parts.afternoon)?;
        }
        let status = parts.classify(self.format, self.accepted.try_with(|s| s.bounds)?);
        if self.status.get() != status {
            self.status.try_revise(|v| *v = status)?;
        }
        self.last.set(Some(TimePickerDraft { parts, status }));
        Ok(())
    }
    fn choose(&self, value: Option<LocalTime>) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        let s = self.accepted.try_get()?;
        if value.is_some_and(|v| {
            !s.bounds.contains(v)
                || self.format.validate(v).is_err()
                || self.disabled.binary_search(&v).is_ok()
        }) {
            return Ok(());
        }
        let baseline = self.current()?;
        self.set_parts(value)?;
        let draft = TimePickerDraft {
            parts: self.parts()?,
            status: self.status.get(),
        };
        self.last.set(None);
        self.publish_draft(draft)?;
        self.control.accept(&value)?;
        self.changed.emit(self.owner, value);
        self.ensure()?;
        self.pending.set(Some(Pending {
            baseline,
            proposal: Some(value),
        }));
        self.dirty.touch()
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
        self.pending.set(None);
        self.last.set(None);
        self.changed.clear();
        self.committed.clear();
        self.draft_changed.clear();
        self.rejected.clear();
    }
}
pub struct TimePicker {
    common: Common,
    key: String,
    format: TimeFormat,
    bounds: Vary<TimeBounds>,
    value: Value<Option<LocalTime>>,
    readonly: Vary<bool>,
    reset_epoch: Vary<u64>,
    required: Vary<bool>,
    invalid: Vary<bool>,
    field: Option<FieldControl>,
    form_name: Option<Vary<String>>,
    grid: Option<TimeGridSpec>,
    presets: Option<TimePickerPresets>,
    with_dropdown: bool,
    reverse_time_controls_list: bool,
    max_dropdown_content_height: f64,
    open: Value<bool>,
    close_on_preset_select: bool,
    popup_placement: crate::AnchorPlacement,
    popup_width: Option<f64>,
    clearable: bool,
    clear_label: String,
    disabled: Vec<LocalTime>,
    references: [Option<Cage<Option<NodeId>>>; 3],
    placeholders: [Option<Vary<String>>; 3],
    labels: [Vary<String>; 3],
    period_label: Vary<String>,
    period_labels: [String; 2],
    paste_split: Option<PasteSplit>,
    paste_requires_separator: bool,
    steps: [u8; 3],
    changed: Option<Box<dyn FnMut(Option<LocalTime>)>>,
    committed: Option<Box<dyn FnMut(Option<LocalTime>)>>,
    draft_changed: Option<Box<dyn FnMut(TimePickerDraft)>>,
    rejected: Option<Box<dyn FnMut(TimePickerStatus)>>,
    open_changed: Option<Box<dyn FnMut(bool)>>,
    focused: Option<Box<dyn FnMut(())>>,
    blurred: Option<Box<dyn FnMut(())>>,
}
pub fn time_picker(key: impl Into<String>) -> TimePicker {
    TimePicker {
        common: Common::default(),
        key: key.into(),
        format: TimeFormat::default(),
        bounds: TimeBounds::default().into(),
        value: Value::default(),
        readonly: false.into(),
        reset_epoch: 0.into(),
        required: false.into(),
        invalid: false.into(),
        field: None,
        form_name: None,
        grid: None,
        presets: None,
        with_dropdown: false,
        reverse_time_controls_list: false,
        max_dropdown_content_height: 240.,
        open: Value::default(),
        close_on_preset_select: false,
        popup_placement: crate::AnchorPlacement::default(),
        popup_width: None,
        clearable: false,
        clear_label: "Clear time".into(),
        disabled: vec![],
        references: [None; 3],
        placeholders: [Some("--".into()), Some("--".into()), Some("--".into())],
        labels: ["Hour".into(), "Minute".into(), "Second".into()],
        period_label: "AM/PM".into(),
        period_labels: ["AM".into(), "PM".into()],
        paste_split: Some(Rc::new(default_paste_split)),
        paste_requires_separator: true,
        steps: [1; 3],
        changed: None,
        committed: None,
        draft_changed: None,
        rejected: None,
        open_changed: None,
        focused: None,
        blurred: None,
    }
}
impl TimePicker {
    common_methods!();
    /// Switches to the typed duration variant. Call this before duration-specific builders.
    pub fn duration(self) -> crate::DurationPicker {
        let mut duration = crate::duration_picker(self.key).precision(self.format.precision);
        duration.common = self.common;
        duration
    }
    pub(crate) fn reset_epoch(mut self, v: impl Into<Vary<u64>>) -> Self {
        self.reset_epoch = v.into();
        self
    }
    pub fn format(mut self, v: TimeFormat) -> Self {
        self.format = v;
        self
    }
    pub fn bounds(mut self, v: impl Into<Vary<TimeBounds>>) -> Self {
        self.bounds = v.into();
        self
    }
    pub fn value(mut self, v: impl Into<Vary<Option<LocalTime>>>) -> Self {
        self.value.source = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: Option<LocalTime>) -> Self {
        self.value.default = v;
        self
    }
    pub fn readonly(mut self, v: impl Into<Vary<bool>>) -> Self {
        self.readonly = v.into();
        self
    }
    pub fn required(mut self, v: impl Into<Vary<bool>>) -> Self {
        self.required = v.into();
        self
    }
    pub fn invalid(mut self, v: impl Into<Vary<bool>>) -> Self {
        self.invalid = v.into();
        self
    }
    pub fn field(mut self, v: FieldControl) -> Self {
        self.common.theme = v.theme.clone();
        self.common.size = v.size;
        self.common.unstyled = v.unstyled;
        self.field = Some(v);
        self
    }
    /// Produces explicit `_hour`, `_minute` and optional `_second` successful names.
    pub fn form_name(mut self, v: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(v.into());
        self
    }
    pub fn grid(mut self, v: TimeGridSpec) -> Self {
        self.grid = Some(v);
        self
    }
    pub fn presets(mut self, value: impl Into<TimePickerPresets>) -> Self {
        self.presets = Some(value.into());
        self
    }
    pub fn with_dropdown(mut self, value: bool) -> Self {
        self.with_dropdown = value;
        self
    }
    pub fn reverse_time_controls_list(mut self, value: bool) -> Self {
        self.reverse_time_controls_list = value;
        self
    }
    pub fn max_dropdown_content_height(mut self, value: f64) -> Self {
        self.max_dropdown_content_height = value;
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
    pub fn close_on_preset_select(mut self, value: bool) -> Self {
        self.close_on_preset_select = value;
        self
    }
    pub fn popup_placement(mut self, value: crate::AnchorPlacement) -> Self {
        self.popup_placement = value;
        self
    }
    pub fn popup_width(mut self, value: f64) -> Self {
        self.popup_width = Some(value);
        self
    }
    pub fn on_open_change(mut self, value: impl FnMut(bool) + 'static) -> Self {
        self.open_changed = Some(Box::new(value));
        self
    }
    /// Shows a finite clear action while the accepted value is non-empty.
    pub fn clearable(mut self, value: bool) -> Self {
        self.clearable = value;
        self
    }
    /// Sets the accessible label of the optional clear action.
    pub fn clear_label(mut self, value: impl Into<String>) -> Self {
        self.clear_label = value.into();
        self
    }
    pub fn disable_time(mut self, v: LocalTime) -> Self {
        if self.disabled.len() <= MAX_DISABLED_TIMES {
            self.disabled.push(v)
        }
        self
    }
    pub fn disable_times(mut self, v: impl IntoIterator<Item = LocalTime>) -> Self {
        self.disabled = v.into_iter().take(MAX_DISABLED_TIMES + 1).collect();
        self
    }
    pub fn hour_step(mut self, value: u8) -> Self {
        self.steps[0] = value;
        self
    }
    pub fn minute_step(mut self, value: u8) -> Self {
        self.steps[1] = value;
        self
    }
    pub fn second_step(mut self, value: u8) -> Self {
        self.steps[2] = value;
        self
    }
    pub fn hour_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.references[0] = Some(v);
        self
    }
    pub fn minute_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.references[1] = Some(v);
        self
    }
    pub fn second_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.references[2] = Some(v);
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
    pub fn period_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.period_label = value.into();
        self
    }
    pub fn period_labels(mut self, am: impl Into<String>, pm: impl Into<String>) -> Self {
        self.period_labels = [am.into(), pm.into()];
        self
    }
    pub fn paste_split(mut self, value: impl Fn(&str) -> Option<LocalTime> + 'static) -> Self {
        self.paste_split = Some(Rc::new(value));
        self.paste_requires_separator = false;
        self
    }
    pub fn handle_paste(mut self, value: bool) -> Self {
        if !value {
            self.paste_split = None;
        } else if self.paste_split.is_none() {
            self.paste_split = Some(Rc::new(default_paste_split));
            self.paste_requires_separator = true;
        }
        self
    }
    pub fn on_value_change(mut self, v: impl FnMut(Option<LocalTime>) + 'static) -> Self {
        self.changed = Some(Box::new(v));
        self
    }
    pub fn on_commit(mut self, v: impl FnMut(Option<LocalTime>) + 'static) -> Self {
        self.committed = Some(Box::new(v));
        self
    }
    pub fn on_draft_change(mut self, v: impl FnMut(TimePickerDraft) + 'static) -> Self {
        self.draft_changed = Some(Box::new(v));
        self
    }
    pub fn on_reject(mut self, v: impl FnMut(TimePickerStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(v));
        self
    }
    pub fn on_focus(mut self, mut value: impl FnMut() + 'static) -> Self {
        self.focused = Some(Box::new(move |()| value()));
        self
    }
    pub fn on_blur(mut self, mut value: impl FnMut() + 'static) -> Self {
        self.blurred = Some(Box::new(move |()| value()));
        self
    }
}
impl Widget for TimePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.is_empty()
            || self.key.len() > 56
            || !self
                .key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || self.disabled.len() > MAX_DISABLED_TIMES
            || self.clear_label.is_empty()
            || self.clear_label.len() > 120
            || self.clear_label.contains(['\0', '\r', '\n'])
            || self
                .popup_width
                .is_some_and(|value| !value.is_finite() || !(80.0..=1200.0).contains(&value))
            || !self.max_dropdown_content_height.is_finite()
            || !(80.0..=1200.0).contains(&self.max_dropdown_content_height)
            || self.period_labels.iter().any(|value| {
                value.trim().is_empty() || value.len() > 40 || value.contains(['\0', '\r', '\n'])
            })
            || self
                .steps
                .iter()
                .enumerate()
                .any(|(at, step)| *step == 0 || *step > if at == 0 { 24 } else { 59 })
        {
            return Err(invalid());
        }
        self.popup_placement.validate().map_err(|_| invalid())?;
        let format = self.format;
        if let Some(spec) = self.grid {
            for at in 0..spec.len() {
                format
                    .validate(spec.at(at).unwrap())
                    .map_err(|_| invalid())?;
            }
        }
        if self.grid.is_some() && self.presets.is_some() {
            return Err(invalid());
        }
        let preset_groups = match self.presets.take() {
            None => None,
            Some(TimePickerPresets::Values(values)) => Some(vec![TimePickerPresetGroup {
                label: String::new(),
                values,
            }]),
            Some(TimePickerPresets::Groups(groups)) => Some(groups),
        };
        if let Some(groups) = &preset_groups {
            let count = groups.iter().map(|group| group.values.len()).sum::<usize>();
            let mut values = Vec::with_capacity(count);
            if groups.is_empty()
                || groups.len() > MAX_TIME_PICKER_PRESET_GROUPS
                || count == 0
                || count > MAX_TIME_PICKER_PRESETS
                || groups.iter().any(|group| {
                    group.values.is_empty()
                        || (!group.label.is_empty()
                            && (group.label.trim().is_empty()
                                || group.label.len() > 120
                                || group.label.contains(['\0', '\r', '\n'])))
                })
            {
                return Err(invalid());
            }
            for group in groups {
                for value in &group.values {
                    format.validate(*value).map_err(|_| invalid())?;
                    values.push(*value);
                }
            }
            values.sort_unstable();
            if values.windows(2).any(|pair| pair[0] == pair[1]) {
                return Err(invalid());
            }
        }
        let enabled = self
            .field
            .as_ref()
            .map(|f| f.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let readonly = self
            .field
            .as_ref()
            .map(|f| f.readonly_state(scope, self.readonly.clone()))
            .unwrap_or_else(|| self.readonly.clone());
        let control = std::mem::take(&mut self.value).mount(scope);
        let value = control.source.clone();
        let bounds = self.bounds.clone();
        let reset_epoch = self.reset_epoch.clone();
        let source = scope.bond(move || Snapshot {
            value: value.get(),
            bounds: bounds.get(),
            enabled: enabled.get(),
            readonly: readonly.get(),
            reset_epoch: reset_epoch.get(),
        });
        let accepted = composite::validated(scope, &source.into(), move |s: &Snapshot| {
            if s.value
                .is_none_or(|v| s.bounds.contains(v) && format.validate(v).is_ok())
            {
                Ok(())
            } else {
                Err(invalid())
            }
        })?;
        let form_name = if let Some(name) = &self.form_name {
            Some(composite::validated(scope, name, |s| {
                if s.len() <= 240 && !s.contains(['\0', '\r', '\n']) {
                    Ok(())
                } else {
                    Err(invalid())
                }
            })?)
        } else {
            None
        };
        let initial = accepted.with(|s| s.value);
        let initial_parts = TimeParts::from_time(initial, format).map_err(|_| invalid())?;
        let initial_status = initial_parts.classify(format, accepted.with(|s| s.bounds));
        let mut disabled = std::mem::take(&mut self.disabled);
        disabled.sort_unstable();
        disabled.dedup();
        let disabled = Rc::new(disabled);
        let popup_control = std::mem::take(&mut self.open).mount(scope);
        let popup_source = popup_control.source.clone();
        let popup = PopupControl {
            owner: scope.id(),
            weak: scope.downgrade(),
            control: popup_control,
            changed: Callback::new(self.open_changed.take()),
        };
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            accepted,
            format,
            parts: [
                scope.cage(number(initial_parts.hour)),
                scope.cage(number(initial_parts.minute)),
                scope.cage(number(initial_parts.second)),
            ],
            afternoon: scope.cage(initial_parts.afternoon),
            status: scope.cage(initial_status),
            last: Cell::new(Some(TimePickerDraft {
                parts: initial_parts,
                status: initial_status,
            })),
            disabled: disabled.clone(),
            pending: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
            draft_changed: Callback::new(self.draft_changed.take()),
            rejected: Callback::new(self.rejected.take()),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(s) = cleanup.upgrade() {
                s.retire()
            }
        })?;
        let root = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let (class, appearance) = self.common.appearance(
            scope,
            "sapid-time-picker",
            Style::new()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Column)
                .gap(8.),
        );
        let style = scope.bond(move || {
            let mut style = appearance.get();
            style.display.get_or_insert(Display::Flex);
            style.flex_direction.get_or_insert(FlexDirection::Column);
            style.gap.get_or_insert(8.);
            style
        });
        let metadata = crate::field::control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            self.readonly.clone(),
            self.invalid.clone(),
        );
        let anchor_id = composite::id("time-picker", &self.key, "trigger", "root")?;
        let panel_id = composite::id("popover", &format!("{}-presets", self.key), "panel", "root")?;
        let mut widget = glory::ui::view()
            .role(Role::Group)
            .class(class)
            .style(style)
            .node_ref(root)
            .focus(self.common.focus);
        let focused = Callback::new(self.focused.take());
        let blurred = Callback::new(self.blurred.take());
        let focus_owner = scope.id();
        let focus_callback = focused.clone();
        let blur_callback = blurred.clone();
        widget = widget.on(EventKind::FocusWithin, move |event| match event {
            Event::FocusWithin { focused: true, .. } => focus_callback.emit(focus_owner, ()),
            Event::FocusWithin { focused: false, .. } => blur_callback.emit(focus_owner, ()),
            _ => {}
        });
        if let Some(name) = self.common.name.take() {
            widget = widget.name(name);
        } else {
            widget = widget.name("Time");
        }
        let group_metadata = metadata.clone();
        widget = widget.semantics(scope.bond(move || {
            let mut value = Semantics::default();
            if let Some(metadata) = group_metadata.get() {
                value.labelled_by = metadata.labelled_by.clone();
                value.described_by = metadata.described_by.clone();
            }
            Some(Box::new(value))
        }));
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            widget = widget.on(kind, move |e| {
                crate::appearance::observe(&mut handlers, owner, e)
            });
        }
        widget.show_in(scope)?;
        let content = scope.child_in(root.get().ok_or(glory::ui::Error::Gone)?)?;
        let visible_parts = state.parts;
        let visible_count = if format.precision == TimePrecision::Second {
            3
        } else {
            2
        };
        let clear_visible = scope.bond(move || {
            visible_parts[..visible_count]
                .iter()
                .any(|part| match part.get() {
                    NumberValue::Empty => false,
                    NumberValue::Text(value) => !value.is_empty(),
                    _ => true,
                })
        });
        let show_chooser = preset_groups.is_some() || self.with_dropdown;
        let clearable = self.clearable;
        let actions_visible = scope.bond(move || show_chooser || clearable && clear_visible.get());
        let (editors, actions) = crate::input_actions::segmented_row(
            &content,
            "sapid-time-segments",
            actions_visible.into(),
        )?;
        let status = state.status;
        for index in 0..if format.precision == TimePrecision::Second {
            3
        } else {
            2
        } {
            if index > 0 {
                glory::ui::text().content(":").show_in(&editors)?;
            }
            let min = if index == 0 && format.cycle == HourCycle::H12 {
                1
            } else {
                0
            };
            let max = if index == 0 {
                if format.cycle == HourCycle::H12 {
                    12
                } else {
                    23
                }
            } else {
                59
            };
            let changed = state.clone();
            let committed = state.clone();
            let rejected = state.clone();
            let meta = metadata.clone();
            let errors = metadata.clone();
            let mut input = crate::number_input()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .name(self.labels[index].clone())
                .class(match index {
                    0 => "sapid-time-hour",
                    1 => "sapid-time-minute",
                    _ => "sapid-time-second",
                })
                .style(Style::new().width(Length::Px(72.)))
                .value(state.parts[index])
                .min(Some(decimal(min)))
                .max(Some(decimal(max)))
                .step(decimal(self.steps[index]))
                .page_step(Decimal::new(i128::from(self.steps[index]) * 10, 0).unwrap())
                .controls(false)
                .enabled(accepted.map(|s| s.enabled))
                .readonly(accepted.map(|s| s.readonly))
                .required(scope.bond(move || meta.get().is_some_and(|s| s.required)))
                .invalid(scope.bond(move || {
                    status.get().is_error() || errors.get().is_some_and(|s| s.invalid)
                }))
                .on_value_change(move |v| changed.report(changed.segment(index, v, false)))
                .on_commit(move |v| committed.report(committed.segment(index, v, true)))
                .on_reject(move |reason| {
                    rejected.report(rejected.reject(
                        if reason == crate::NumberError::InputTooLong {
                            TimePickerStatus::TooLong
                        } else {
                            TimePickerStatus::Invalid
                        },
                    ))
                });
            if index == 0
                && let Some(field) = &self.field
            {
                input = input.field(field.clone());
            }
            if self.common.unstyled {
                input = input.unstyled();
            }
            if let Some(reference) = self.references[index] {
                input = input.node_ref(reference);
            }
            if let Some(name) = form_name {
                let suffix = ["hour", "minute", "second"][index];
                input = input.form_name(scope.bond(move || format!("{}_{suffix}", name.get())));
            }
            if let Some(placeholder) = self.placeholders[index].take() {
                input = input.placeholder(placeholder);
            }
            if let Some(parser) = self.paste_split.clone() {
                let pasted = state.clone();
                let requires_separator = self.paste_requires_separator;
                input = input.capture_input_edits(MAX_TIME_PICKER_PASTE_BYTES, move |raw| {
                    if (!requires_separator || raw.contains(':'))
                        && let Some(value) = parser(raw)
                    {
                        pasted.report(pasted.choose(Some(value)));
                        true
                    } else {
                        false
                    }
                });
            }

            if preset_groups.is_some() || self.with_dropdown {
                let focus = popup.clone();
                input = input.on(EventKind::FocusChanged, move |event| {
                    if matches!(event, Event::FocusChanged(true)) {
                        focus.report(focus.propose(true));
                    }
                });
            }
            input.show_in(&editors)?;
        }
        if format.cycle == HourCycle::H12 {
            let s = state.clone();
            let period = state.afternoon;
            let [am, pm] = self.period_labels.clone();
            let visible_label =
                scope.bond(move || if period.get() { pm.clone() } else { am.clone() });
            let mut toggle = crate::toggle()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .name(self.period_label.clone())
                .label(visible_label)
                .pressed(state.afternoon)
                .enabled(accepted.map(|s| s.enabled && !s.readonly))
                .on_pressed_change(move |v| s.report(s.period(v)));
            if self.common.unstyled {
                toggle = toggle.unstyled();
            }
            toggle.show_in(&editors)?;
        }
        if self.clearable {
            let clear = state.clone();
            let clear_label = self.clear_label.clone();
            let clear_enabled = accepted.map(|s| s.enabled && !s.readonly);
            let theme = self.common.theme.clone();
            let size = self.common.size;
            let unstyled = self.common.unstyled;
            let action_style = crate::input_actions::flow_action_style(&self.common);
            glory::ui::when(clear_visible, move || {
                let clear = clear.clone();
                let mut button = crate::button()
                    .theme(theme.clone())
                    .size(size)
                    .variant(crate::ButtonVariant::Subtle)
                    .compact(true)
                    .class("sapid-time-picker-clear")
                    .name(clear_label.clone())
                    .style(action_style.clone())
                    .fill(crate::input_actions::glyph(crate::Icon::Close, theme.muted))
                    .enabled(clear_enabled)
                    .on_press(move || clear.report(clear.choose(None)));
                if unstyled {
                    button = button.unstyled();
                }
                button
            })
            .show_in(&actions)?;
        }
        if preset_groups.is_some() || self.with_dropdown {
            let trigger = popup.clone();
            let trigger_open = popup_source.clone();
            let trigger_id = anchor_id.clone();
            let trigger_panel = panel_id.clone();
            let mut button = glory::ui::button()
                .name("Choose time")
                .fill(crate::input_actions::glyph(
                    crate::Icon::ChevronDown,
                    self.common.theme.muted,
                ))
                .class("sapid-time-picker-preset-trigger")
                .enabled(accepted.map(|snapshot| snapshot.enabled && !snapshot.readonly))
                .semantics(scope.bond(move || {
                    Some(Box::new(Semantics {
                        id: Some(trigger_id.clone()),
                        controls: Some(trigger_panel.clone()),
                        expanded: Some(trigger_open.get()),
                        popup: Some(PopupKind::Dialog),
                        ..Default::default()
                    }))
                }))
                .on(EventKind::Activate, move |_| {
                    trigger.report(trigger.propose(!trigger.control.source.get()));
                });
            button = button.style(crate::input_actions::flow_action_style(&self.common));
            button.show_in(&actions)?;
        }
        if let Some(spec) = self.grid {
            let s = state.clone();
            let value = accepted.map(|s| s.value);
            let mut grid = crate::time_grid(format!("{}-slots", self.key))
                .spec(spec)
                .format(format)
                .bounds(accepted.map(|s| s.bounds))
                .value(value)
                .enabled(accepted.map(|s| s.enabled))
                .readonly(accepted.map(|s| s.readonly))
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .disable_times(disabled.iter().copied())
                .on_value_change(move |v| s.report(s.choose(v)));
            if self.common.unstyled {
                grid = grid.unstyled();
            }
            grid.show_in(&content)?;
        }
        if preset_groups.is_some() || self.with_dropdown {
            let popup_state = popup.clone();
            let popup_for_content = popup.clone();
            let picker_state = state.clone();
            let picker_accepted = accepted;
            let picker_theme = self.common.theme.clone();
            let content_theme = picker_theme.clone();
            let size = self.common.size;
            let unstyled = self.common.unstyled;
            let close_on_select = self.close_on_preset_select;
            let weak = scope.downgrade();
            let preset_content = PresetContent {
                format,
                accepted: picker_accepted,
                state: picker_state,
                popup: popup_for_content,
                close_on_select,
                theme: content_theme,
                size,
                unstyled,
            };
            let control_content = TimeControlsContent {
                format,
                accepted: picker_accepted,
                state: state.clone(),
                steps: self.steps,
                period_labels: self.period_labels.clone(),
                reverse: self.reverse_time_controls_list,
                max_height: self.max_dropdown_content_height,
                theme: self.common.theme.clone(),
                size,
                unstyled,
            };
            let with_dropdown = self.with_dropdown;
            let groups = preset_groups.clone();
            let mut layer = crate::popover(format!("{}-presets", self.key))
                .name("Choose time")
                .show_close(false)
                .theme(picker_theme)
                .size(size)
                .placement(self.popup_placement)
                .external_trigger(anchor_id)
                .open(popup_source)
                .on_open_change(move |value| popup_state.report(popup_state.propose(value)))
                .content(move || {
                    let mut sections: Vec<Box<dyn Widget>> = Vec::new();
                    if with_dropdown {
                        sections.push(control_content.build());
                    }
                    if let Some(groups) = groups.clone() {
                        sections.push(preset_content.build(groups));
                    }
                    Box::new(
                        crate::stack()
                            .class("sapid-time-picker-dropdown")
                            .gap(12.)
                            .fill(sections),
                    ) as Box<dyn Widget>
                });
            if let Some(width) = self.popup_width {
                layer = layer.style(Style::new().width(Length::Px(width)));
            }
            if self.common.unstyled {
                layer = layer.unstyled();
            }
            layer.show_in(&content).inspect_err(|error| {
                if let Some(scope) = weak.upgrade() {
                    scope.report_error(error.clone());
                }
            })?;
        }
        let weak = Rc::downgrade(&state);
        let mut previous = initial;
        let mut previous_reset = accepted.with(|s| s.reset_epoch);
        scope.effect(move || {
            let Some(s) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let dirty = s.dirty.take();
                let value = accepted.with(|s| s.value);
                let reset = accepted.with(|s| s.reset_epoch);
                if dirty || value != previous || reset != previous_reset {
                    let pending = s.pending.take();
                    let keep = reset == previous_reset
                        && pending.is_some_and(|p| {
                            p.proposal == Some(value) || p.proposal.is_none() && p.baseline == value
                        });
                    if !keep {
                        s.set_parts(value)?;
                    }
                    previous = value;
                    previous_reset = reset;
                }
                Ok(())
            })();
            s.report(result)
        });
        scope.retain(actions)?;
        scope.retain(editors)?;
        scope.retain(content)?;
        scope.retain((state, popup, focused, blurred))
    }
}
