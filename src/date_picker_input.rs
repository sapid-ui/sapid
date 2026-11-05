//! Readonly successful text control with an independent host-owned Dialog trigger.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, ContentPolicy};
use crate::state::Value;
use crate::{
    CivilDate, DateInputFormat, DatePickerLevel, DatePickerMode, DatePickerPreset, DateSelection,
    DateSelectionError, DatesProvider, FieldControl, Weekday, YearMonth,
};
use glory::render::{Direction, FormControl, NodeId};
use glory::style::Display;
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: DateSelection,
    provider: DatesProvider,
}
/// A display Input and a separate real Button open a modal date selection Dialog.
/// The display is readonly in every mode; `readonly(true)` also locks selection.
pub struct DatePickerInput {
    common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    mode: DatePickerMode,
    value: Value<DateSelection>,
    default_set: bool,
    open: Value<bool>,
    readonly: Vary<bool>,
    required: Vary<bool>,
    invalid: Vary<bool>,
    field: Option<FieldControl>,
    form_name: Option<Vary<String>>,
    direction: Direction,
    excluded: Vec<CivilDate>,
    weekdays: u8,
    deselect: bool,
    presets: Vec<DatePickerPreset>,
    columns: u8,
    columns_to_scroll: Option<u8>,
    hide_outside_dates: bool,
    hide_weekdays: bool,
    week_numbers: bool,
    month: Option<Vary<YearMonth>>,
    default_month: Option<YearMonth>,
    level: Option<Vary<DatePickerLevel>>,
    default_level: DatePickerLevel,
    max_level: DatePickerLevel,
    display_format: DateInputFormat,
    placeholder: Vary<String>,
    clearable: bool,
    clear_label: String,
    trigger_label: Vary<String>,
    title: Vary<String>,
    close_label: Vary<String>,
    trigger_ref: Option<Cage<Option<NodeId>>>,
    panel_ref: Option<Cage<Option<NodeId>>>,
    calendar_ref: Option<Cage<Option<NodeId>>>,
    changed: Option<Box<dyn FnMut(DateSelection)>>,
    rejected: Option<Box<dyn FnMut(DateSelectionError)>>,
    open_changed: Option<Box<dyn FnMut(bool)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}
pub fn date_picker_input(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
) -> DatePickerInput {
    DatePickerInput {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        mode: DatePickerMode::Single,
        value: Value::default(),
        default_set: false,
        open: Value::default(),
        readonly: false.into(),
        required: false.into(),
        invalid: false.into(),
        field: None,
        form_name: None,
        direction: Direction::Ltr,
        excluded: Vec::new(),
        weekdays: 0,
        deselect: false,
        presets: Vec::new(),
        columns: 1,
        columns_to_scroll: None,
        hide_outside_dates: false,
        hide_weekdays: false,
        week_numbers: false,
        month: None,
        default_month: None,
        level: None,
        default_level: DatePickerLevel::Month,
        max_level: DatePickerLevel::Decade,
        display_format: DateInputFormat::Iso,
        placeholder: "Choose dates".into(),
        clearable: false,
        clear_label: "Clear dates".into(),
        trigger_label: "Choose dates".into(),
        title: "Choose dates".into(),
        close_label: "Close date picker".into(),
        trigger_ref: None,
        panel_ref: None,
        calendar_ref: None,
        changed: None,
        rejected: None,
        open_changed: None,
        month_changed: None,
        level_changed: None,
    }
}
impl DatePickerInput {
    common_methods!();
    pub fn mode(mut self, v: DatePickerMode) -> Self {
        self.mode = v;
        self
    }
    pub fn value(mut self, v: impl Into<Vary<DateSelection>>) -> Self {
        self.value.source = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: DateSelection) -> Self {
        self.value.default = v;
        self.default_set = true;
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
    /// Explicit ISO successful-control name, independent of the accessible name.
    pub fn form_name(mut self, v: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(v.into());
        self
    }
    pub fn direction(mut self, v: Direction) -> Self {
        self.direction = v;
        self
    }
    pub fn exclude_dates(mut self, v: impl IntoIterator<Item = CivilDate>) -> Self {
        self.excluded = v
            .into_iter()
            .take(crate::MAX_CALENDAR_EXCLUDED_DATES + 1)
            .collect();
        self
    }
    pub fn exclude_date(mut self, v: CivilDate) -> Self {
        if self.excluded.len() <= crate::MAX_CALENDAR_EXCLUDED_DATES {
            self.excluded.push(v);
        }
        self
    }
    pub fn disable_weekday(mut self, v: Weekday) -> Self {
        self.weekdays |= 1 << v.index();
        self
    }
    pub fn allow_deselect(mut self, v: bool) -> Self {
        self.deselect = v;
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DatePickerPreset>) -> Self {
        self.presets = values
            .into_iter()
            .take(crate::MAX_DATE_PICKER_PRESETS + 1)
            .collect();
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
    pub fn month(mut self, v: impl Into<Vary<YearMonth>>) -> Self {
        self.month = Some(v.into());
        self
    }
    pub fn default_month(mut self, v: YearMonth) -> Self {
        self.default_month = Some(v);
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
    /// Selects one of the bounded, locale-independent readonly display formats.
    /// The controlled value remains a typed [`DateSelection`].
    pub fn display_format(mut self, value: DateInputFormat) -> Self {
        self.display_format = value;
        self
    }
    pub fn placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholder = value.into();
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
    pub fn trigger_label(mut self, v: impl Into<Vary<String>>) -> Self {
        self.trigger_label = v.into();
        self
    }
    pub fn title(mut self, v: impl Into<Vary<String>>) -> Self {
        self.title = v.into();
        self
    }
    pub fn close_label(mut self, v: impl Into<Vary<String>>) -> Self {
        self.close_label = v.into();
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
    pub fn on_value_change(mut self, v: impl FnMut(DateSelection) + 'static) -> Self {
        self.changed = Some(Box::new(v));
        self
    }
    pub fn on_reject(mut self, v: impl FnMut(DateSelectionError) + 'static) -> Self {
        self.rejected = Some(Box::new(v));
        self
    }
    pub fn on_open_change(mut self, v: impl FnMut(bool) + 'static) -> Self {
        self.open_changed = Some(Box::new(v));
        self
    }
    pub fn on_month_change(mut self, v: impl FnMut(YearMonth) + 'static) -> Self {
        self.month_changed = Some(Box::new(v));
        self
    }
    pub fn on_level_change(mut self, value: impl FnMut(DatePickerLevel) + 'static) -> Self {
        self.level_changed = Some(Box::new(value));
        self
    }
}
impl Widget for DatePickerInput {
    fn build(&mut self, scope: &Scope) -> Result {
        // Overlay and Calendar both append names. Validate before formatting any derived key.
        if self.key.is_empty()
            || self.key.len() > 57
            || !self
                .key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || self.excluded.len() > crate::MAX_CALENDAR_EXCLUDED_DATES
            || self.clear_label.trim().is_empty()
            || self.clear_label.len() > 120
            || self.clear_label.contains(['\0', '\r', '\n'])
            || self.presets.len() > crate::MAX_DATE_PICKER_PRESETS
            || self.columns == 0
            || self.columns > crate::MAX_DATE_PICKER_COLUMNS
            || self
                .columns_to_scroll
                .is_some_and(|value| value == 0 || value > crate::MAX_DATE_PICKER_COLUMNS)
        {
            return Err(glory::render::Error::InvalidProps(
                "invalid date picker input key or exclusions",
            )
            .into());
        }
        if self.value.source.is_none() && !self.default_set {
            self.value.default = DateSelection::empty(self.mode);
        }
        let value = std::mem::take(&mut self.value).mount(scope);
        let open = std::mem::take(&mut self.open).mount(scope);
        let mode = self.mode;
        let source = value.source.clone();
        let provider = self.provider.clone();
        let snapshot = scope.bond(move || Snapshot {
            value: source.get(),
            provider: provider.get(),
        });
        let accepted = composite::validated(scope, &snapshot.into(), move |s: &Snapshot| {
            s.value
                .validate(mode, s.provider.date_bounds())
                .map_err(|_| {
                    glory::render::Error::InvalidProps("invalid date picker input selection").into()
                })
        })?;
        let owner = scope.id();
        let weak = scope.downgrade();
        let changed = Callback::new(self.changed.take());
        let rejected = Callback::new(self.rejected.take());
        let opened = Callback::new(self.open_changed.take());
        let month_changed = Callback::new(self.month_changed.take());
        let level_changed = Callback::new(self.level_changed.take());
        let callbacks = (
            changed.clone(),
            rejected.clone(),
            opened.clone(),
            month_changed.clone(),
            level_changed.clone(),
        );
        scope.on_cleanup(move || {
            callbacks.0.clear();
            callbacks.1.clear();
            callbacks.2.clear();
            callbacks.3.clear();
            callbacks.4.clear();
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
                Err(glory::render::Error::InvalidProps("invalid picker placeholder").into())
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
        let content = crate::input_actions::frame(scope, &self.common, "sapid-date-picker-input")?;
        let classes = self.common.classes(scope, "sapid-date-picker-display");
        let style = crate::input_actions::display_style(
            scope,
            &self.common,
            enabled.clone(),
            metadata.into(),
            if self.clearable { 2 } else { 1 },
        );
        let display_ref = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let display_format = self.display_format;
        let mut input = glory::ui::input()
            .class(classes)
            .style(style)
            .enabled(enabled.clone())
            .focus(self.common.focus)
            .node_ref(display_ref)
            .value(scope.bond(move || accepted.with(|s| s.value.format_display(display_format))))
            .semantics(crate::input_actions::display_metadata(
                scope,
                metadata.into(),
            ));
        if let Some(name) = self.common.name.take() {
            input = input.name(name);
        }
        for (kind, mut callbacks) in std::mem::take(&mut self.common.handlers) {
            input = input.on(kind, move |e| {
                crate::appearance::observe(&mut callbacks, owner, e)
            });
        }
        input.show_in(&content)?;
        crate::input_actions::submission(
            &content,
            metadata.into(),
            accepted.map(|s| s.value.format_iso()).into(),
            enabled.clone(),
        )?;
        if self.clearable {
            let clear_enabled = enabled.clone();
            let clear_readonly = readonly.clone();
            let available = scope.bond(move || {
                clear_enabled.get()
                    && !clear_readonly.get()
                    && accepted.with(|s| s.value.anchor().is_some())
            });
            let style = crate::input_actions::action_style(&self.common, 1);
            let control = value.clone();
            let callback = changed.clone();
            let weak = scope.downgrade();
            glory::ui::button()
                .name(self.clear_label.clone())
                .class("sapid-date-picker-clear")
                .fill(crate::input_actions::glyph(
                    crate::Icon::Close,
                    self.common.theme.muted,
                ))
                .style(scope.bond(move || {
                    let mut style = style.clone();
                    if accepted.with(|value| value.value.anchor().is_none()) {
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
                        let empty = DateSelection::empty(mode);
                        control.accept(&empty)?;
                        callback.emit(owner, empty);
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
        let key = format!("{}-calendar", self.key);
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        let direction = self.direction;
        let excluded = std::mem::take(&mut self.excluded);
        let weekdays = self.weekdays;
        let deselect = self.deselect;
        let presets = std::mem::take(&mut self.presets);
        let columns = self.columns;
        let columns_to_scroll = self.columns_to_scroll;
        let hide_outside_dates = self.hide_outside_dates;
        let hide_weekdays = self.hide_weekdays;
        let week_numbers = self.week_numbers;
        let month = self.month.take();
        let default_month = self.default_month;
        let level = self.level.take();
        let default_level = self.default_level;
        let max_level = self.max_level;
        let calendar_ref = self.calendar_ref;
        let calendar_source = value.source.clone();
        let callback_value = value.clone();
        let callback_open = open.clone();
        let callback_opened = opened.clone();
        let active = enabled.clone();
        let initial_focus =
            composite::id("dialog", &format!("{}-dialog", self.key), "close", "root")?;
        let mut dialog = crate::dialog(format!("{}-dialog", self.key))
            .initial_focus(initial_focus)
            .label(self.trigger_label.clone())
            .trigger_style(crate::input_actions::action_style(&self.common, 0))
            .trigger_content({
                let color = self.common.theme.muted;
                move || crate::input_actions::glyph(crate::Icon::Calendar, color)
            })
            .title(self.title.clone())
            .close_label(self.close_label.clone())
            .theme(theme.clone())
            .size(size)
            .enabled(enabled)
            .open(open.source.clone())
            .content_policy(ContentPolicy::KeepMounted)
            .on_open_change(move |v| {
                if !owner.is_alive() {
                    return;
                }
                let result = open.accept(&v);
                if let Err(e) = result {
                    if let Some(scope) = weak.upgrade() {
                        scope.report_error(e);
                    }
                    return;
                }
                opened.emit(owner, v);
            })
            .content(move || {
                let callback_value = callback_value.clone();
                let callback_open = callback_open.clone();
                let changed = changed.clone();
                let opened = callback_opened.clone();
                let rejected = rejected.clone();
                let month_changed = month_changed.clone();
                let level_changed = level_changed.clone();
                let mut picker = crate::date_picker(key.clone(), accepted.map(|s| s.provider))
                    .mode(mode)
                    .value(calendar_source.clone())
                    .enabled(active.clone())
                    .readonly(readonly.clone())
                    .direction(direction)
                    .allow_deselect(deselect)
                    .presets(presets.clone())
                    .number_of_columns(columns)
                    .default_level(default_level)
                    .max_level(max_level)
                    .hide_outside_dates(hide_outside_dates)
                    .hide_weekdays(hide_weekdays)
                    .with_week_numbers(week_numbers)
                    .exclude_dates(excluded.iter().copied())
                    .theme(theme.clone())
                    .size(size)
                    .on_value_change(move |next| {
                        if !owner.is_alive() {
                            return;
                        }
                        if callback_value.accept(&next).is_err() {
                            return;
                        }
                        changed.emit(owner, next);
                        if owner.is_alive()
                            && mode != DatePickerMode::Multiple
                            && next.is_complete()
                            && callback_value.source.try_get().ok() == Some(next)
                            && callback_open.accept(&false).is_ok()
                        {
                            opened.emit(owner, false);
                        }
                    })
                    .on_reject(move |e| rejected.emit(owner, e))
                    .on_month_change(move |m| month_changed.emit(owner, m))
                    .on_level_change(move |next| level_changed.emit(owner, next));
                if let Some(value) = columns_to_scroll {
                    picker = picker.columns_to_scroll(value);
                }
                if let Some(month) = &month {
                    picker = picker.month(month.clone());
                }
                if let Some(month) = default_month {
                    picker = picker.default_month(month);
                }
                if let Some(level) = &level {
                    picker = picker.level(level.clone());
                }
                if let Some(reference) = calendar_ref {
                    picker = picker.node_ref(reference);
                }
                if unstyled {
                    picker = picker.unstyled();
                }
                for day in Weekday::ALL {
                    if weekdays & (1 << day.index()) != 0 {
                        picker = picker.disable_weekday(day);
                    }
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
