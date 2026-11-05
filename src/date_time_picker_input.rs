//! One successful civil ISO Input and a separate host-owned Dialog.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, ContentPolicy};
use crate::state::Value;
use crate::{
    DatePickerLevel, DateTimeBounds, DateTimeDraft, DateTimeOutcome, DateTimePolicy,
    DateTimePreset, DateTimeStatus, DatesProvider, FieldControl, LocalDateTime, LocalTime,
    TimeFormat, TimeGridSpec, YearMonth,
};
use glory::render::{Direction, FormControl, NodeId};
use glory::style::Display;
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateTimeDisplayFormat {
    #[default]
    Iso,
    DayMonthYear,
    MonthDayYear,
}
impl DateTimeDisplayFormat {
    pub(crate) fn format(self, value: LocalDateTime, time: TimeFormat) -> String {
        if self == Self::Iso {
            return crate::format_local_date_time(value);
        }
        let date = value.date();
        let time = time
            .format(value.time())
            .expect("validated date/time display precision");
        match self {
            Self::Iso => unreachable!(),
            Self::DayMonthYear => {
                format!(
                    "{:02}/{:02}/{} {time}",
                    date.day(),
                    date.month(),
                    date.year()
                )
            }
            Self::MonthDayYear => {
                format!(
                    "{:02}/{:02}/{} {time}",
                    date.month(),
                    date.day(),
                    date.year()
                )
            }
        }
    }
}
/// Complete valid edits publish immediately. Done commits/closes; Escape keeps
/// accepted edits. A draft or unresolved timezone gap never replaces the ISO value.
pub struct DateTimePicker {
    common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    policy: DateTimePolicy,
    value: Value<Option<LocalDateTime>>,
    open: Value<bool>,
    bounds: Vary<DateTimeBounds>,
    revision: Vary<u64>,
    format: TimeFormat,
    display_format: DateTimeDisplayFormat,
    placeholder: Vary<String>,
    default_time: Option<LocalTime>,
    presets: Vec<DateTimePreset>,
    time_presets: Option<crate::TimePickerPresets>,
    time_dropdown: bool,
    time_reverse_controls: bool,
    time_max_dropdown_height: f64,
    time_steps: [u8; 3],
    clearable: bool,
    clear_label: String,
    trigger_label: Vary<String>,
    title: Vary<String>,
    close_label: Vary<String>,
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
    grid: Option<TimeGridSpec>,
    content_policy: ContentPolicy,
    references: [Option<Cage<Option<NodeId>>>; 3],
    trigger_ref: Option<Cage<Option<NodeId>>>,
    panel_ref: Option<Cage<Option<NodeId>>>,
    calendar_ref: Option<Cage<Option<NodeId>>>,
    changed: Option<Box<dyn FnMut(Option<LocalDateTime>)>>,
    committed: Option<Box<dyn FnMut(Option<DateTimeOutcome>)>>,
    drafted: Option<Box<dyn FnMut(DateTimeDraft)>>,
    resolved: Option<Box<dyn FnMut(DateTimeOutcome)>>,
    rejected: Option<Box<dyn FnMut(DateTimeStatus)>>,
    opened: Option<Box<dyn FnMut(bool)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}
pub fn date_time_picker(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
    policy: DateTimePolicy,
) -> DateTimePicker {
    DateTimePicker {
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
        placeholder: "Pick date and time".to_owned().into(),
        default_time: None,
        presets: Vec::new(),
        time_presets: None,
        time_dropdown: false,
        time_reverse_controls: false,
        time_max_dropdown_height: 240.,
        time_steps: [1; 3],
        clearable: false,
        clear_label: "Clear date and time".into(),
        trigger_label: "Choose date and time".into(),
        title: "Date and time".into(),
        close_label: "Close date and time".into(),
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
        grid: None,
        content_policy: ContentPolicy::KeepMounted,
        references: [None; 3],
        trigger_ref: None,
        panel_ref: None,
        calendar_ref: None,
        changed: None,
        committed: None,
        drafted: None,
        resolved: None,
        rejected: None,
        opened: None,
        month_changed: None,
        level_changed: None,
    }
}
impl DateTimePicker {
    common_methods!();
    pub fn value(mut self, v: impl Into<Vary<Option<LocalDateTime>>>) -> Self {
        self.value.source = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: Option<LocalDateTime>) -> Self {
        self.value.default = v;
        self
    }
    pub fn open(mut self, v: impl Into<Vary<bool>>) -> Self {
        self.open.source = Some(v.into());
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.open.default = v;
        self
    }
    pub fn bounds(mut self, v: impl Into<Vary<DateTimeBounds>>) -> Self {
        self.bounds = v.into();
        self
    }
    pub fn reset_revision(mut self, v: impl Into<Vary<u64>>) -> Self {
        self.revision = v.into();
        self
    }
    pub fn format(mut self, v: TimeFormat) -> Self {
        self.format = v;
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
    pub fn default_time(mut self, value: LocalTime) -> Self {
        self.default_time = Some(value);
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DateTimePreset>) -> Self {
        self.presets = values
            .into_iter()
            .take(crate::MAX_DATE_TIME_PRESETS + 1)
            .collect();
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
    pub fn clearable(mut self, value: bool) -> Self {
        self.clearable = value;
        self
    }
    pub fn clear_label(mut self, value: impl Into<String>) -> Self {
        self.clear_label = value.into();
        self
    }
    pub fn trigger_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.trigger_label = value.into();
        self
    }
    pub fn title(mut self, value: impl Into<Vary<String>>) -> Self {
        self.title = value.into();
        self
    }
    pub fn close_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.close_label = value.into();
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
    pub fn form_name(mut self, v: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(v.into());
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
    pub fn content_policy(mut self, v: ContentPolicy) -> Self {
        self.content_policy = v;
        self
    }
    pub fn trigger_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.trigger_ref = Some(v);
        self
    }
    pub fn panel_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.panel_ref = Some(v);
        self
    }
    pub fn calendar_ref(mut self, v: Cage<Option<NodeId>>) -> Self {
        self.calendar_ref = Some(v);
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
    pub fn on_value_change(mut self, v: impl FnMut(Option<LocalDateTime>) + 'static) -> Self {
        self.changed = Some(Box::new(v));
        self
    }
    pub fn on_commit(mut self, v: impl FnMut(Option<DateTimeOutcome>) + 'static) -> Self {
        self.committed = Some(Box::new(v));
        self
    }
    pub fn on_draft_change(mut self, v: impl FnMut(DateTimeDraft) + 'static) -> Self {
        self.drafted = Some(Box::new(v));
        self
    }
    pub fn on_resolved(mut self, v: impl FnMut(DateTimeOutcome) + 'static) -> Self {
        self.resolved = Some(Box::new(v));
        self
    }
    pub fn on_reject(mut self, v: impl FnMut(DateTimeStatus) + 'static) -> Self {
        self.rejected = Some(Box::new(v));
        self
    }
    pub fn on_open_change(mut self, v: impl FnMut(bool) + 'static) -> Self {
        self.opened = Some(Box::new(v));
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
impl Widget for DateTimePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.is_empty()
            || self.key.len() > 40
            || !self
                .key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || self.clear_label.trim().is_empty()
            || self.clear_label.len() > 120
            || self.clear_label.contains(['\0', '\r', '\n'])
            || self.columns == 0
            || self.columns > crate::MAX_DATE_PICKER_COLUMNS
            || self
                .columns_to_scroll
                .is_some_and(|value| value == 0 || value > crate::MAX_DATE_PICKER_COLUMNS)
            || !self.time_max_dropdown_height.is_finite()
            || !(80.0..=1200.0).contains(&self.time_max_dropdown_height)
            || self
                .time_steps
                .iter()
                .enumerate()
                .any(|(index, step)| *step == 0 || *step > if index == 0 { 24 } else { 59 })
        {
            return Err(glory::render::Error::InvalidProps("invalid date/time dialog key").into());
        }
        let control = std::mem::take(&mut self.value).mount(scope);
        let open = std::mem::take(&mut self.open).mount(scope);
        let format = self.format;
        let display_format = self.display_format;
        let value_source = control.source.clone();
        let bounds_source = self.bounds.clone();
        let provider_source = self.provider.clone();
        let snapshot = scope.bond(move || {
            (
                value_source.get(),
                bounds_source.get(),
                provider_source.get(),
            )
        });
        let accepted = composite::validated(
            scope,
            &snapshot.into(),
            move |v: &(Option<LocalDateTime>, DateTimeBounds, DatesProvider)| {
                let provider = v
                    .1
                    .provider(v.2)
                    .map_err(|_| glory::render::Error::InvalidProps("empty date/time interval"))?;
                if v.0.is_some_and(|value| {
                    format.validate(value.time()).is_err()
                        || !v.1.contains(value)
                        || !provider.date_bounds().contains(value.date())
                }) {
                    return Err(
                        glory::render::Error::InvalidProps("date/time precision loss").into(),
                    );
                }
                Ok(())
            },
        )?;
        let owner = scope.id();
        let weak = scope.downgrade();
        let changed = Callback::new(self.changed.take());
        let committed = Callback::new(self.committed.take());
        let drafted = Callback::new(self.drafted.take());
        let resolved = Callback::new(self.resolved.take());
        let rejected = Callback::new(self.rejected.take());
        let opened = Callback::new(self.opened.take());
        let month_changed = Callback::new(self.month_changed.take());
        let level_changed = Callback::new(self.level_changed.take());
        let cleanup = (
            changed.clone(),
            committed.clone(),
            drafted.clone(),
            resolved.clone(),
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
            cleanup.6.clear();
            cleanup.7.clear();
        })?;
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
        let metadata = crate::field::control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            true.into(),
            self.invalid.clone(),
        );
        let form_name = composite::form_name(scope, &self.form_name)?;
        let placeholder = composite::validated(scope, &self.placeholder, |value| {
            if value.len() <= 240 && !value.contains(['\0', '\r', '\n']) {
                Ok(())
            } else {
                Err(glory::render::Error::InvalidProps("invalid date/time placeholder").into())
            }
        })?;
        let metadata = scope.bond(move || {
            let mut s = metadata.get().unwrap_or_default();
            s.placeholder = Some(placeholder.get());
            if let Some(name) = form_name {
                s.form_control = Some(FormControl {
                    name: name.get(),
                    value: None,
                });
            }
            Some(s)
        });
        let content = crate::input_actions::frame(scope, &self.common, "sapid-date-time-picker")?;
        let class = self.common.classes(scope, "sapid-date-time-display");
        let style = crate::input_actions::display_style(
            scope,
            &self.common,
            enabled.clone(),
            metadata.into(),
            if self.clearable { 2 } else { 1 },
        );
        let display_ref = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let mut input = glory::ui::input()
            .class(class)
            .style(style)
            .enabled(enabled.clone())
            .focus(self.common.focus)
            .node_ref(display_ref)
            .value(accepted.map(move |v| {
                v.0.map(|value| display_format.format(value, format))
                    .unwrap_or_default()
            }))
            .semantics(crate::input_actions::display_metadata(
                scope,
                metadata.into(),
            ));
        if let Some(v) = self.common.name.take() {
            input = input.name(v);
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            input = input.on(kind, move |e| {
                crate::appearance::observe(&mut handlers, owner, e)
            });
        }
        input.show_in(&content)?;
        crate::input_actions::submission(
            &content,
            metadata.into(),
            accepted
                .map(|value| {
                    value
                        .0
                        .map(crate::format_local_date_time)
                        .unwrap_or_default()
                })
                .into(),
            enabled.clone(),
        )?;
        if self.clearable {
            let clear_enabled = enabled.clone();
            let clear_readonly = readonly.clone();
            let available = scope.bond(move || {
                clear_enabled.get()
                    && !clear_readonly.get()
                    && accepted.with(|value| value.0.is_some())
            });
            let style = crate::input_actions::action_style(&self.common, 1);
            let control = control.clone();
            let callback = changed.clone();
            let weak = scope.downgrade();
            glory::ui::button()
                .name(self.clear_label.clone())
                .class("sapid-date-time-clear")
                .fill(crate::input_actions::glyph(
                    crate::Icon::Close,
                    self.common.theme.muted,
                ))
                .style(scope.bond(move || {
                    let mut style = style.clone();
                    if !accepted.with(|value| value.0.is_some()) {
                        style.display = Some(Display::None);
                    }
                    style
                }))
                .enabled(available)
                .on(glory::render::EventKind::Activate, move |_| {
                    if !available.get() {
                        return;
                    }
                    let Some(scope) = weak.upgrade() else { return };
                    let result = (|| -> Result {
                        control.accept(&None)?;
                        callback.emit(owner, None);
                        if owner.is_alive() {
                            composite::focus_node(&scope, display_ref.get())?;
                        }
                        Ok(())
                    })();
                    if let Err(error) = result {
                        scope.report_error(error);
                    }
                })
                .show_in(&content)?;
        }
        let key = format!("{}-inline", self.key);
        let provider = self.provider.clone();
        let policy = self.policy.clone();
        let bounds = self.bounds.clone();
        let revision = self.revision.clone();
        let source = control.source.clone();
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
        let grid = self.grid;
        let default_time = self.default_time;
        let presets = self.presets.clone();
        let time_presets = self.time_presets.clone();
        let time_dropdown = self.time_dropdown;
        let time_reverse_controls = self.time_reverse_controls;
        let time_max_dropdown_height = self.time_max_dropdown_height;
        let time_steps = self.time_steps;
        let refs = self.references;
        let calendar_ref = self.calendar_ref;
        let active = enabled.clone();
        let close_open = open.clone();
        let close_opened = opened.clone();
        let changed_control = control.clone();
        let focus = composite::id("dialog", &format!("{}-dialog", self.key), "close", "root")?;
        let mut dialog = crate::dialog(format!("{}-dialog", self.key))
            .label(self.trigger_label.clone())
            .title(self.title.clone())
            .close_label(self.close_label.clone())
            .trigger_style(crate::input_actions::action_style(&self.common, 0))
            .trigger_content({
                let color = self.common.theme.muted;
                move || crate::input_actions::glyph(crate::Icon::Calendar, color)
            })
            .initial_focus(focus)
            .theme(theme.clone())
            .size(size)
            .enabled(enabled)
            .open(open.source.clone())
            .content_policy(self.content_policy)
            .on_open_change(move |v| {
                if !owner.is_alive() {
                    return;
                }
                if let Err(e) = open.accept(&v) {
                    if let Some(s) = weak.upgrade() {
                        s.report_error(e);
                    }
                    return;
                }
                opened.emit(owner, v);
            })
            .content(move || {
                let c = changed_control.clone();
                let changed = changed.clone();
                let committed = committed.clone();
                let drafted = drafted.clone();
                let resolved = resolved.clone();
                let rejected = rejected.clone();
                let close = close_open.clone();
                let opened = close_opened.clone();
                let month_changed = month_changed.clone();
                let level_changed = level_changed.clone();
                let accepted_source = source.clone();
                let mut picker =
                    crate::inline_date_time_picker(key.clone(), provider.clone(), policy.clone())
                        .value(source.clone())
                        .bounds(bounds.clone())
                        .reset_revision(revision.clone())
                        .format(format)
                        .presets(presets.clone());
                picker = picker
                    .time_dropdown(time_dropdown)
                    .reverse_time_controls(time_reverse_controls)
                    .max_time_dropdown_height(time_max_dropdown_height)
                    .time_steps(time_steps[0], time_steps[1], time_steps[2]);
                picker = picker
                    .theme(theme.clone())
                    .size(size)
                    .enabled(active.clone())
                    .readonly(readonly.clone())
                    .direction(direction);
                picker = picker
                    .default_level(default_level)
                    .max_level(max_level)
                    .number_of_columns(columns)
                    .hide_outside_dates(hide_outside_dates)
                    .hide_weekdays(hide_weekdays)
                    .with_week_numbers(week_numbers);
                picker = picker
                    .on_value_change(move |v| {
                        if owner.is_alive() && c.accept(&v).is_ok() {
                            changed.emit(owner, v);
                        }
                    })
                    .on_draft_change(move |v| drafted.emit(owner, v))
                    .on_resolved(move |v| resolved.emit(owner, v))
                    .on_reject(move |v| rejected.emit(owner, v))
                    .on_month_change(move |value| month_changed.emit(owner, value))
                    .on_level_change(move |value| level_changed.emit(owner, value))
                    .on_commit(move |v| {
                        committed.emit(owner, v);
                        if owner.is_alive()
                            && accepted_source.try_get().ok() == Some(v.map(|v| v.local))
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
                if let Some(value) = time_presets.clone() {
                    picker = picker.time_presets(value);
                }
                if let Some(value) = default_time {
                    picker = picker.default_time(value);
                }
                if let Some(g) = grid {
                    picker = picker.grid(g);
                }
                if let Some(r) = calendar_ref {
                    picker = picker.calendar_ref(r);
                }
                if let Some(r) = refs[0] {
                    picker = picker.hour_ref(r);
                }
                if let Some(r) = refs[1] {
                    picker = picker.minute_ref(r);
                }
                if let Some(r) = refs[2] {
                    picker = picker.second_ref(r);
                }
                if unstyled {
                    picker = picker.unstyled();
                }
                picker
            });
        if let Some(r) = self.trigger_ref {
            dialog = dialog.trigger_ref(r);
        }
        if let Some(r) = self.panel_ref {
            dialog = dialog.panel_ref(r);
        }
        if self.common.unstyled {
            dialog = dialog.unstyled();
        }
        dialog.show_in(&content)?;
        scope.retain(content)
    }
}
