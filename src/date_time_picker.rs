//! Civil date/time drafts with explicit application-owned resolution.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback};
use crate::state::{Control, Dirty, Value};
use crate::{
    DatePickerLevel, DateSelection, DateTimeBounds, DateTimeDraft, DateTimeOutcome, DateTimeParts,
    DateTimePolicy, DateTimeStatus, DatesProvider, FieldControl, LocalDateTime, LocalTime,
    TimeFormat, TimeGridSpec, TimePickerDraft, TimePickerStatus, YearMonth,
};
use glory::render::{Direction, NodeId, Role};
use glory::style::{Display, FlexDirection};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};
pub const MAX_DATE_TIME_PRESETS: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTimePreset {
    pub label: String,
    pub value: LocalDateTime,
}
impl DateTimePreset {
    pub fn new(label: impl Into<String>, value: LocalDateTime) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}
fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid finite date/time picker configuration").into()
}
#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: Option<LocalDateTime>,
    provider: DatesProvider,
    bounds: DateTimeBounds,
    enabled: bool,
    readonly: bool,
    revision: u64,
}
#[derive(Clone, Copy)]
struct Pending {
    baseline: Option<LocalDateTime>,
    proposal: Option<Option<LocalDateTime>>,
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    epoch: Cell<u64>,
    control: Control<Option<LocalDateTime>>,
    accepted: Cage<Snapshot>,
    input: glory::Bond<Snapshot>,
    policy: DateTimePolicy,
    default_time: Option<LocalTime>,
    parts: Cage<DateTimeParts>,
    time_status: Cell<TimePickerStatus>,
    status: Cage<DateTimeStatus>,
    reset: Cage<u64>,
    last: Cell<Option<DateTimeDraft>>,
    pending: Cell<Option<Pending>>,
    dirty: Dirty,
    changed: Callback<Option<LocalDateTime>>,
    resolved: Callback<DateTimeOutcome>,
    drafted: Callback<DateTimeDraft>,
    committed: Callback<Option<DateTimeOutcome>>,
    rejected: Callback<DateTimeStatus>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn next(&self) -> Result<u64> {
        self.ensure()?;
        let n = self.epoch.get().checked_add(1).ok_or_else(invalid)?;
        self.epoch.set(n);
        Ok(n)
    }
    fn writable(&self) -> Result<bool> {
        Ok(self.accepted.try_with(|s| s.enabled && !s.readonly)?)
    }
    fn reset_parts(&self, value: Option<LocalDateTime>) -> Result {
        self.next()?;
        self.parts
            .try_revise(|p| *p = DateTimeParts::from_local(value))?;
        self.time_status
            .set(value.map_or(TimePickerStatus::Empty, |v| {
                TimePickerStatus::Valid(v.time())
            }));
        let reset = self.reset.try_get()?.checked_add(1).ok_or_else(invalid)?;
        self.reset.try_revise(|n| *n = reset)?;
        self.last.set(None);
        Ok(())
    }
    fn stamp(&self) -> Result<(Option<LocalDateTime>, Option<u64>)> {
        let version = match &self.control.source {
            Vary::Cage(c) => Some(c.try_version()?),
            _ => None,
        };
        Ok((self.control.source.try_get()?, version))
    }
    fn evaluate(&self) -> Result<Option<(u64, DateTimeDraft)>> {
        let epoch = self.epoch.get();
        let s = self.accepted.try_get()?;
        let parts = self.parts.try_get()?;
        let stamp = self.stamp()?;
        let configuration = glory::core::untracked(|| self.input.try_get())?;
        // Application rules may reenter, write sources or retire this Owner.
        // No Cage or callback registry is borrowed across their invocation.
        let status = match self.time_status.get() {
            TimePickerStatus::Empty | TimePickerStatus::Valid(_) => {
                glory::core::untracked(|| parts.classify(s.bounds, &self.policy))
            }
            TimePickerStatus::Incomplete => DateTimeStatus::Incomplete,
            status => DateTimeStatus::InvalidTime(status),
        };
        if !self.live.get() || !self.owner.is_alive() {
            return Ok(None);
        }
        if self.epoch.get() != epoch
            || self.accepted.try_get()? != s
            || self.parts.try_get()? != parts
            || self.stamp()? != stamp
            || glory::core::untracked(|| self.input.try_get())? != configuration
        {
            self.dirty.touch()?;
            return Ok(None);
        }
        Ok(Some((epoch, DateTimeDraft { parts, status })))
    }
    fn publish(&self, epoch: u64, draft: DateTimeDraft, notify: bool) -> Result<bool> {
        if self.status.try_get()? != draft.status {
            self.status.try_revise(|s| *s = draft.status)?;
        }
        let different = self.last.replace(Some(draft)) != Some(draft);
        if notify && different {
            self.drafted.emit(self.owner, draft);
        }
        Ok(self.live.get() && self.owner.is_alive() && self.epoch.get() == epoch)
    }
    fn propose(&self) -> Result {
        let baseline = self.control.source.try_get()?;
        let stamp = self.stamp()?;
        let configuration = glory::core::untracked(|| self.input.try_get())?;
        let Some((epoch, draft)) = self.evaluate()? else {
            return Ok(());
        };
        let proposal = draft.status.proposal();
        self.pending.set(Some(Pending { baseline, proposal }));
        if !self.publish(epoch, draft, true)? {
            return Ok(());
        }
        if self.stamp()? != stamp
            || glory::core::untracked(|| self.input.try_get())? != configuration
        {
            self.pending.set(None);
            self.dirty.touch()?;
            return Ok(());
        }
        if let Some(value) = proposal {
            self.control.accept(&value)?;
            self.changed.emit(self.owner, value);
            if !self.live.get() || !self.owner.is_alive() {
                return Ok(());
            }
            if self.epoch.get() == epoch
                && self.control.source.try_get()? == value
                && let DateTimeStatus::Valid(outcome) = draft.status
            {
                self.resolved.emit(self.owner, outcome);
            }
        }
        if self.live.get() && self.owner.is_alive() {
            self.dirty.touch()?;
        }
        Ok(())
    }
    fn date(&self, value: DateSelection) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        let DateSelection::Single(date) = value else {
            return Err(invalid());
        };
        self.next()?;
        self.parts.try_revise(|p| {
            p.date = date;
            if date.is_some() && p.time.is_none() {
                p.time = self.default_time;
            }
        })?;
        if date.is_some() && self.time_status.get() == TimePickerStatus::Empty {
            self.time_status.set(
                self.default_time
                    .map_or(TimePickerStatus::Empty, TimePickerStatus::Valid),
            );
        }
        self.propose()
    }
    fn preset(&self, value: LocalDateTime) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.reset_parts(Some(value))?;
        self.propose()
    }
    fn time_draft(&self, draft: TimePickerDraft) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.next()?;
        self.time_status.set(draft.status);
        if !matches!(
            draft.status,
            TimePickerStatus::Valid(_) | TimePickerStatus::Empty
        ) {
            self.propose()?;
        }
        Ok(())
    }
    fn time(&self, value: Option<LocalTime>) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.next()?;
        self.parts.try_revise(|p| p.time = value)?;
        self.time_status
            .set(value.map_or(TimePickerStatus::Empty, TimePickerStatus::Valid));
        self.propose()
    }
    fn clear(&self) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        self.reset_parts(None)?;
        self.propose()
    }
    fn done(&self) -> Result {
        self.ensure()?;
        if !self.writable()? {
            return Ok(());
        }
        let configuration = glory::core::untracked(|| self.input.try_get())?;
        let Some((epoch, draft)) = self.evaluate()? else {
            return Ok(());
        };
        if !self.publish(epoch, draft, true)? {
            return Ok(());
        }
        if glory::core::untracked(|| self.input.try_get())? != configuration {
            self.dirty.touch()?;
            return Ok(());
        }
        let result = match draft.status {
            DateTimeStatus::Empty if self.control.source.try_get()?.is_none() => Some(None),
            DateTimeStatus::Valid(v) if self.control.source.try_get()? == Some(v.local) => {
                Some(Some(v))
            }
            _ => None,
        };
        if let Some(value) = result {
            self.committed.emit(self.owner, value);
        } else {
            self.rejected.emit(self.owner, draft.status);
        }
        Ok(())
    }
    fn report(&self, result: Result) {
        if let Err(e) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(e);
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.pending.set(None);
        self.last.set(None);
        self.changed.clear();
        self.resolved.clear();
        self.drafted.clear();
        self.committed.clear();
        self.rejected.clear();
    }
}
/// Inline Calendar and real segmented spinbutton inputs. Policy is always explicit.
pub struct InlineDateTimePicker {
    pub(crate) common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    policy: DateTimePolicy,
    value: Value<Option<LocalDateTime>>,
    bounds: Vary<DateTimeBounds>,
    revision: Vary<u64>,
    format: TimeFormat,
    default_time: Option<LocalTime>,
    presets: Vec<DateTimePreset>,
    time_presets: Option<crate::TimePickerPresets>,
    time_dropdown: bool,
    time_reverse_controls: bool,
    time_max_dropdown_height: f64,
    time_steps: [u8; 3],
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
    grid: Option<TimeGridSpec>,
    refs: [Option<Cage<Option<NodeId>>>; 3],
    calendar_ref: Option<Cage<Option<NodeId>>>,
    changed: Option<Box<dyn FnMut(Option<LocalDateTime>)>>,
    resolved: Option<Box<dyn FnMut(DateTimeOutcome)>>,
    drafted: Option<Box<dyn FnMut(DateTimeDraft)>>,
    committed: Option<Box<dyn FnMut(Option<DateTimeOutcome>)>>,
    rejected: Option<Box<dyn FnMut(DateTimeStatus)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}
pub fn inline_date_time_picker(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
    policy: DateTimePolicy,
) -> InlineDateTimePicker {
    InlineDateTimePicker {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        policy,
        value: Value::default(),
        bounds: DateTimeBounds::default().into(),
        revision: 0.into(),
        format: TimeFormat::default(),
        default_time: None,
        presets: Vec::new(),
        time_presets: None,
        time_dropdown: false,
        time_reverse_controls: false,
        time_max_dropdown_height: 240.,
        time_steps: [1; 3],
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
        grid: None,
        refs: [None; 3],
        calendar_ref: None,
        changed: None,
        resolved: None,
        drafted: None,
        committed: None,
        rejected: None,
        month_changed: None,
        level_changed: None,
    }
}
impl InlineDateTimePicker {
    common_methods!();
    pub fn value(mut self, v: impl Into<Vary<Option<LocalDateTime>>>) -> Self {
        self.value.source = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: Option<LocalDateTime>) -> Self {
        self.value.default = v;
        self
    }
    pub fn bounds(mut self, v: impl Into<Vary<DateTimeBounds>>) -> Self {
        self.bounds = v.into();
        self
    }
    /// An explicit application revision resets drafts, even for an equal civil value,
    /// and reevaluates resolver rules. Applications must not wrap this counter.
    pub fn reset_revision(mut self, v: impl Into<Vary<u64>>) -> Self {
        self.revision = v.into();
        self
    }
    pub fn format(mut self, v: TimeFormat) -> Self {
        self.format = v;
        self
    }
    pub fn default_time(mut self, value: LocalTime) -> Self {
        self.default_time = Some(value);
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DateTimePreset>) -> Self {
        self.presets = values.into_iter().take(MAX_DATE_TIME_PRESETS + 1).collect();
        self
    }
    pub fn time_presets(mut self, value: impl Into<crate::TimePickerPresets>) -> Self {
        self.time_presets = Some(value.into());
        self
    }
    pub fn time_dropdown(mut self, value: bool) -> Self {
        self.time_dropdown = value;
        self
    }
    pub fn reverse_time_controls(mut self, value: bool) -> Self {
        self.time_reverse_controls = value;
        self
    }
    pub fn max_time_dropdown_height(mut self, value: f64) -> Self {
        self.time_max_dropdown_height = value;
        self
    }
    pub fn time_steps(mut self, hour: u8, minute: u8, second: u8) -> Self {
        self.time_steps = [hour, minute, second];
        self
    }
    pub fn readonly(mut self, v: impl Into<Vary<bool>>) -> Self {
        self.readonly = v.into();
        self
    }
    pub fn field(mut self, v: FieldControl) -> Self {
        self.common.theme = v.theme.clone();
        self.common.size = v.size;
        self.common.unstyled = v.unstyled;
        self.field = Some(v);
        self
    }
    pub fn direction(mut self, v: Direction) -> Self {
        self.direction = v;
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
    pub fn grid(mut self, v: TimeGridSpec) -> Self {
        self.grid = Some(v);
        self
    }
    pub fn hour_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.refs[0] = Some(v);
        self
    }
    pub fn minute_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.refs[1] = Some(v);
        self
    }
    pub fn second_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.refs[2] = Some(v);
        self
    }
    pub fn calendar_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.calendar_ref = Some(v);
        self
    }
    pub fn on_value_change(mut self, v: impl FnMut(Option<LocalDateTime>) + 'static) -> Self {
        self.changed = Some(Box::new(v));
        self
    }
    pub fn on_resolved(mut self, v: impl FnMut(DateTimeOutcome) + 'static) -> Self {
        self.resolved = Some(Box::new(v));
        self
    }
    pub fn on_draft_change(mut self, v: impl FnMut(DateTimeDraft) + 'static) -> Self {
        self.drafted = Some(Box::new(v));
        self
    }
    pub fn on_commit(mut self, v: impl FnMut(Option<DateTimeOutcome>) + 'static) -> Self {
        self.committed = Some(Box::new(v));
        self
    }
    pub fn on_reject(mut self, v: impl FnMut(DateTimeStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(v));
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
impl Widget for InlineDateTimePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.is_empty()
            || self.key.len() > 48
            || !self
                .key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || self.presets.len() > MAX_DATE_TIME_PRESETS
            || self.columns == 0
            || self.columns > crate::MAX_DATE_PICKER_COLUMNS
            || self
                .columns_to_scroll
                .is_some_and(|value| value == 0 || value > crate::MAX_DATE_PICKER_COLUMNS)
            || self.presets.iter().any(|preset| {
                preset.label.trim().is_empty()
                    || preset.label.len() > 120
                    || preset.label.contains(['\0', '\r', '\n'])
            })
            || !self.time_max_dropdown_height.is_finite()
            || !(80.0..=1200.0).contains(&self.time_max_dropdown_height)
            || self
                .time_steps
                .iter()
                .enumerate()
                .any(|(index, step)| *step == 0 || *step > if index == 0 { 24 } else { 59 })
        {
            return Err(invalid());
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
        let source = control.source.clone();
        let provider = self.provider.clone();
        let bounds = self.bounds.clone();
        let revision = self.revision.clone();
        let source = scope.bond(move || Snapshot {
            value: source.get(),
            provider: provider.get(),
            bounds: bounds.get(),
            enabled: enabled.get(),
            readonly: readonly.get(),
            revision: revision.get(),
        });
        let format = self.format;
        if let Some(value) = self.default_time {
            format.validate(value).map_err(|_| invalid())?;
        }
        for preset in &self.presets {
            if format.validate(preset.value.time()).is_err() {
                return Err(invalid());
            }
        }
        let mut preset_values = self
            .presets
            .iter()
            .map(|preset| preset.value)
            .collect::<Vec<_>>();
        preset_values.sort_unstable();
        if preset_values.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid());
        }
        let accepted = composite::validated(scope, &source.into(), move |s: &Snapshot| {
            let provider = s.bounds.provider(s.provider).map_err(|_| invalid())?;
            if s.value.is_some_and(|v| {
                !s.bounds.contains(v)
                    || !provider.date_bounds().contains(v.date())
                    || format.validate(v.time()).is_err()
            }) {
                return Err(invalid());
            }
            Ok(())
        })?;
        let initial = accepted.get();
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            epoch: Cell::new(0),
            control,
            accepted,
            input: source,
            policy: self.policy.clone(),
            default_time: self.default_time,
            parts: scope.cage(DateTimeParts::from_local(initial.value)),
            time_status: Cell::new(initial.value.map_or(TimePickerStatus::Empty, |v| {
                TimePickerStatus::Valid(v.time())
            })),
            status: scope.cage(DateTimeStatus::Empty),
            reset: scope.cage(0),
            last: Cell::new(None),
            pending: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            resolved: Callback::new(self.resolved.take()),
            drafted: Callback::new(self.drafted.take()),
            committed: Callback::new(self.committed.take()),
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
            "sapid-inline-date-time-picker",
            composite::column(8.),
        );
        let style = scope.bond(move || {
            let mut s = appearance.get();
            s.display.get_or_insert(Display::Flex);
            s.flex_direction.get_or_insert(FlexDirection::Column);
            s.gap.get_or_insert(8.);
            s
        });
        let mut widget = glory::ui::view()
            .role(Role::Group)
            .class(class)
            .style(style)
            .node_ref(root)
            .focus(self.common.focus);
        widget = widget.name(
            self.common
                .name
                .take()
                .unwrap_or_else(|| "Date and time".to_owned().into()),
        );
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            widget = widget.on(kind, move |e| {
                crate::appearance::observe(&mut handlers, owner, e)
            });
        }
        widget.show_in(scope)?;
        let content = scope.child_in(root.get().ok_or(glory::ui::Error::Gone)?)?;
        let parts = state.parts;
        let calendar_value = scope.bond(move || {
            let s = accepted.get();
            let bounds = s.bounds.provider(s.provider).unwrap().date_bounds();
            DateSelection::Single(parts.with(|p| p.date.filter(|d| bounds.contains(*d))))
        });
        let s = state.clone();
        let mut calendar = crate::date_picker(
            format!("{}-date", self.key),
            accepted.map(|s| s.bounds.provider(s.provider).unwrap()),
        )
        .value(calendar_value)
        .enabled(accepted.map(|s| s.enabled))
        .readonly(accepted.map(|s| s.readonly))
        .direction(self.direction)
        .theme(self.common.theme.clone())
        .size(self.common.size)
        .name("Date")
        .allow_deselect(true)
        .default_level(self.default_level)
        .max_level(self.max_level)
        .number_of_columns(self.columns)
        .hide_outside_dates(self.hide_outside_dates)
        .hide_weekdays(self.hide_weekdays)
        .with_week_numbers(self.week_numbers)
        .on_value_change(move |v| s.report(s.date(v)));
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
        if let Some(r) = self.calendar_ref {
            calendar = calendar.node_ref(r);
        }
        if self.common.unstyled {
            calendar = calendar.unstyled();
        }
        calendar.show_in(&content)?;
        if !self.presets.is_empty() {
            let mut buttons: Vec<Box<dyn Widget>> = Vec::with_capacity(self.presets.len());
            for preset in std::mem::take(&mut self.presets) {
                let choose = state.clone();
                let label = preset.label;
                let value = preset.value;
                let selected = accepted.map(move |snapshot| snapshot.value == Some(value));
                let enabled = accepted.map(move |snapshot| {
                    snapshot.enabled && !snapshot.readonly && snapshot.bounds.contains(value)
                });
                let mut button = crate::toggle()
                    .theme(self.common.theme.clone())
                    .size(self.common.size)
                    .class("sapid-date-time-preset")
                    .label(label)
                    .pressed(selected)
                    .enabled(enabled)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.preset(value));
                        }
                    });
                if self.common.unstyled {
                    button = button.unstyled();
                }
                buttons.push(Box::new(button));
            }
            crate::group()
                .theme(self.common.theme.clone())
                .class("sapid-date-time-presets")
                .gap(6.)
                .fill(buttons)
                .show_in(&content)?;
        }
        let parts = state.parts;
        let ontime = state.clone();
        let drafted = state.clone();
        let mut time = crate::time_picker(format!("{}-time", self.key))
            .format(format)
            .value(parts.map(|p| p.time))
            .reset_epoch(state.reset)
            .enabled(accepted.map(|s| s.enabled))
            .readonly(accepted.map(|s| s.readonly))
            .theme(self.common.theme.clone())
            .size(self.common.size)
            .on_draft_change(move |v| drafted.report(drafted.time_draft(v)))
            .on_value_change(move |v| ontime.report(ontime.time(v)));
        time = time
            .with_dropdown(self.time_dropdown)
            .reverse_time_controls_list(self.time_reverse_controls)
            .max_dropdown_content_height(self.time_max_dropdown_height)
            .hour_step(self.time_steps[0])
            .minute_step(self.time_steps[1])
            .second_step(self.time_steps[2]);
        if let Some(value) = self.time_presets.take() {
            time = time.presets(value);
        }
        if let Some(v) = self.grid {
            time = time.grid(v);
        }
        if let Some(v) = &self.field {
            time = time.field(v.clone());
        }
        if let Some(r) = self.refs[0] {
            time = time.hour_ref(r);
        }
        if let Some(r) = self.refs[1] {
            time = time.minute_ref(r);
        }
        if let Some(r) = self.refs[2] {
            time = time.second_ref(r);
        }
        if self.common.unstyled {
            time = time.unstyled();
        }
        time.show_in(&content)?;
        let status = state.status;
        glory::ui::text()
            .class("sapid-date-time-status")
            .content(scope.bond(move || {
                match status.get() {
                    DateTimeStatus::OutOfBounds(_) => "Date/time is outside the allowed interval",
                    DateTimeStatus::Ambiguous { .. } => {
                        "Date/time is ambiguous; choose an explicit application policy"
                    }
                    DateTimeStatus::Gap => {
                        "Date/time does not exist in the application timezone rules"
                    }
                    DateTimeStatus::InvalidResolver => {
                        "Application timezone resolver returned invalid candidates"
                    }
                    DateTimeStatus::InvalidTime(_) => "Complete a valid time",
                    _ => "",
                }
                .to_owned()
            }))
            .show_in(&content)?;
        for (clear, label) in [(true, "Clear date and time"), (false, "Done")] {
            let s = state.clone();
            let mut button = crate::button()
                .label(label)
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .enabled(accepted.map(|v| v.enabled && !v.readonly))
                .on_press(move || s.report(if clear { s.clear() } else { s.done() }));
            if self.common.unstyled {
                button = button.unstyled();
            }
            button.show_in(&content)?;
        }
        let weak = Rc::downgrade(&state);
        let mut previous = initial;
        scope.effect(move || {
            let Some(s) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let dirty = s.dirty.take();
                let next = accepted.get();
                let external = next.value != previous.value
                    || next.provider != previous.provider
                    || next.bounds != previous.bounds
                    || next.revision != previous.revision;
                if dirty || external {
                    let pending = s.pending.take();
                    let keep = next.revision == previous.revision
                        && pending.is_some_and(|p| {
                            p.proposal == Some(next.value)
                                || p.proposal.is_none() && p.baseline == next.value
                        });
                    if !keep {
                        s.reset_parts(next.value)?;
                    }
                    previous = next;
                }
                if let Some((epoch, draft)) = s.evaluate()? {
                    s.publish(epoch, draft, false)?;
                }
                Ok(())
            })();
            s.report(result)
        });
        scope.retain(content)?;
        scope.retain(state)
    }
}
