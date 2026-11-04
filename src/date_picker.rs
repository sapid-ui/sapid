//! Civil date selection on a real Grid. No routing, transport or timezone inference.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback};
use crate::state::{Control, Dirty, Value};
use crate::{
    CivilDate, CivilYear, DatePickerMode, DateSelection, DateSelectionError, DatesProvider,
    PeriodSelection, Weekday, YearMonth,
};
use glory::render::{Direction, Error};
use glory::style::{Display, FlexDirection, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};

pub const MAX_DATE_PICKER_PRESETS: usize = 32;
pub const MAX_DATE_PICKER_COLUMNS: u8 = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatePickerLevel {
    #[default]
    Month,
    Year,
    Decade,
}

impl DatePickerLevel {
    const fn rank(self) -> u8 {
        match self {
            Self::Month => 0,
            Self::Year => 1,
            Self::Decade => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatePickerPreset {
    pub label: String,
    pub value: DateSelection,
}

impl DatePickerPreset {
    pub fn new(label: impl Into<String>, value: DateSelection) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}
fn invalid() -> glory::ui::Error {
    Error::InvalidProps("invalid finite date picker selection").into()
}
#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    value: DateSelection,
    provider: DatesProvider,
    enabled: bool,
    readonly: bool,
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<DateSelection>,
    accepted: Cage<Snapshot>,
    cursor: Cell<Option<DateSelection>>,
    dirty: Dirty,
    changed: Callback<DateSelection>,
    rejected: Callback<DateSelectionError>,
    month: Control<YearMonth>,
    level: Control<DatePickerLevel>,
    accepted_level: Cage<DatePickerLevel>,
    max_level: DatePickerLevel,
    month_changed: Callback<YearMonth>,
    level_changed: Callback<DatePickerLevel>,
}

struct MonthColumns {
    picker: Rc<Controller>,
    months: Vec<Cage<YearMonth>>,
}

impl MonthColumns {
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.picker.weak.upgrade()
        {
            scope.report_error(error);
        }
    }

    fn sync(&self) -> Result {
        let anchor = self.picker.month.source.try_get()?;
        for (offset, month) in self.months.iter().enumerate() {
            let value = anchor
                .checked_add(i32::try_from(offset).map_err(|_| invalid())?)
                .map_err(|_| invalid())?;
            if month.try_get()? != value {
                month.try_revise(|current| *current = value)?;
            }
        }
        Ok(())
    }

    fn propose(&self, column: usize, displayed: YearMonth) -> Result {
        let offset = i32::try_from(column).map_err(|_| invalid())?;
        let anchor = displayed.checked_add(-offset).map_err(|_| invalid())?;
        self.picker.propose_month(anchor)?;
        self.sync()
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
    fn select(&self, date: CivilDate, deselect: bool) -> Result {
        self.ensure()?;
        let s = self.accepted.try_get()?;
        if !s.enabled || s.readonly {
            return Ok(());
        }
        let current = self
            .cursor
            .get()
            .filter(|v| self.control.source.try_get().ok() == Some(*v))
            .unwrap_or(s.value);
        let next = match current.activate(date, deselect) {
            Ok(next) => next,
            Err(error) => {
                self.rejected.emit(self.owner, error);
                return Ok(());
            }
        };
        next.validate(s.value.mode(), s.provider.date_bounds())
            .map_err(|_| invalid())?;
        self.control.accept(&next)?;
        self.changed.emit(self.owner, next);
        self.ensure()?;
        self.cursor.set(Some(next));
        self.dirty.touch()
    }
    fn choose(&self, next: DateSelection) -> Result {
        self.ensure()?;
        let snapshot = self.accepted.try_get()?;
        if !snapshot.enabled || snapshot.readonly {
            return Ok(());
        }
        if let Err(error) = next.validate(snapshot.value.mode(), snapshot.provider.date_bounds()) {
            self.rejected.emit(self.owner, error);
            return Ok(());
        }
        self.control.accept(&next)?;
        self.changed.emit(self.owner, next);
        self.ensure()?;
        self.cursor.set(Some(next));
        self.dirty.touch()
    }
    fn propose_month(&self, next: YearMonth) -> Result {
        self.ensure()?;
        if !self
            .accepted
            .try_with(|snapshot| snapshot.provider.date_bounds().month_allowed(next))?
        {
            return Ok(());
        }
        self.month.accept(&next)?;
        self.month_changed.emit(self.owner, next);
        self.ensure()
    }
    fn propose_level(&self, next: DatePickerLevel) -> Result {
        self.ensure()?;
        if next.rank() > self.max_level.rank() {
            return Ok(());
        }
        self.level.accept(&next)?;
        self.level_changed.emit(self.owner, next);
        self.ensure()
    }
    fn choose_month(&self, next: YearMonth) -> Result {
        self.propose_month(next)?;
        if self.month.source.try_get()? == next {
            self.propose_level(DatePickerLevel::Month)?;
        }
        Ok(())
    }
    fn choose_year(&self, next: CivilYear) -> Result {
        let current = self.month.source.try_get()?;
        let month = YearMonth::new(next.value(), current.month()).map_err(|_| invalid())?;
        self.propose_month(month)?;
        if self.month.source.try_get()? == month {
            self.propose_level(DatePickerLevel::Year)?;
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
        self.cursor.set(None);
        self.changed.clear();
        self.rejected.clear();
        self.month_changed.clear();
        self.level_changed.clear();
    }
}
/// Inline civil date picker. Mode is fixed; changing a controlled selection must retain it.
pub struct DatePicker {
    pub(crate) common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    mode: DatePickerMode,
    value: Value<DateSelection>,
    default_set: bool,
    month: Option<Vary<YearMonth>>,
    default_month: Option<YearMonth>,
    level: Value<DatePickerLevel>,
    max_level: DatePickerLevel,
    readonly: Vary<bool>,
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
    changed: Option<Box<dyn FnMut(DateSelection)>>,
    rejected: Option<Box<dyn FnMut(DateSelectionError)>>,
    month_changed: Option<Box<dyn FnMut(YearMonth)>>,
    level_changed: Option<Box<dyn FnMut(DatePickerLevel)>>,
}
pub fn date_picker(key: impl Into<String>, provider: impl Into<Vary<DatesProvider>>) -> DatePicker {
    DatePicker {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        mode: DatePickerMode::Single,
        value: Value::default(),
        default_set: false,
        month: None,
        default_month: None,
        level: Value::default(),
        max_level: DatePickerLevel::Decade,
        readonly: false.into(),
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
        changed: None,
        rejected: None,
        month_changed: None,
        level_changed: None,
    }
}
impl DatePicker {
    common_methods!();
    pub fn mode(mut self, value: DatePickerMode) -> Self {
        self.mode = value;
        self
    }
    pub fn value(mut self, value: impl Into<Vary<DateSelection>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: DateSelection) -> Self {
        self.value.default = value;
        self.default_set = true;
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
        self.level.source = Some(value.into());
        self
    }
    pub fn default_level(mut self, value: DatePickerLevel) -> Self {
        self.level.default = value;
        self
    }
    pub fn max_level(mut self, value: DatePickerLevel) -> Self {
        self.max_level = value;
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn allow_deselect(mut self, value: bool) -> Self {
        self.deselect = value;
        self
    }
    pub fn exclude_dates(mut self, value: impl IntoIterator<Item = CivilDate>) -> Self {
        self.excluded = value
            .into_iter()
            .take(crate::MAX_CALENDAR_EXCLUDED_DATES + 1)
            .collect();
        self
    }
    pub fn exclude_date(mut self, value: CivilDate) -> Self {
        if self.excluded.len() <= crate::MAX_CALENDAR_EXCLUDED_DATES {
            self.excluded.push(value);
        }
        self
    }
    pub fn disable_weekday(mut self, value: Weekday) -> Self {
        self.weekdays |= 1 << value.index();
        self
    }
    pub fn presets(mut self, values: impl IntoIterator<Item = DatePickerPreset>) -> Self {
        self.presets = values
            .into_iter()
            .take(MAX_DATE_PICKER_PRESETS + 1)
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
    pub fn on_value_change(mut self, callback: impl FnMut(DateSelection) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn on_reject(mut self, callback: impl FnMut(DateSelectionError) + 'static) -> Self {
        self.rejected = Some(Box::new(callback));
        self
    }
    pub fn on_month_change(mut self, callback: impl FnMut(YearMonth) + 'static) -> Self {
        self.month_changed = Some(Box::new(callback));
        self
    }
    pub fn on_level_change(mut self, callback: impl FnMut(DatePickerLevel) + 'static) -> Self {
        self.level_changed = Some(Box::new(callback));
        self
    }
}
impl Widget for DatePicker {
    fn build(&mut self, scope: &Scope) -> Result {
        let columns_to_scroll = self.columns_to_scroll.unwrap_or(self.columns);
        if self.columns == 0
            || self.columns > MAX_DATE_PICKER_COLUMNS
            || columns_to_scroll == 0
            || columns_to_scroll > MAX_DATE_PICKER_COLUMNS
            || self.presets.len() > MAX_DATE_PICKER_PRESETS
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
        if self.value.source.is_none() && !self.default_set {
            self.value.default = DateSelection::empty(self.mode);
        }
        let control = std::mem::take(&mut self.value).mount(scope);
        let provider = self.provider.clone();
        let value = control.source.clone();
        let enabled = self.common.enabled.clone();
        let readonly = self.readonly.clone();
        let source = scope.bond(move || Snapshot {
            value: value.get(),
            provider: provider.get(),
            enabled: enabled.get(),
            readonly: readonly.get(),
        });
        let mode = self.mode;
        let accepted = composite::validated(scope, &source.into(), move |s: &Snapshot| {
            s.value
                .validate(mode, s.provider.date_bounds())
                .map_err(|_| invalid())
        })?;
        let initial_provider = accepted.try_with(|snapshot| snapshot.provider)?;
        let default_month = self.default_month.unwrap_or_else(|| {
            initial_provider
                .date_bounds()
                .clamp(initial_provider.reference_date())
                .year_month()
        });
        let month = Value {
            source: self.month.take(),
            default: default_month,
        }
        .mount(scope);
        let level = std::mem::take(&mut self.level).mount(scope);
        let level_source = level.source.clone();
        let max_level = self.max_level;
        let accepted_level = composite::validated(scope, &level_source, move |value| {
            if value.rank() <= max_level.rank() {
                Ok(())
            } else {
                Err(invalid())
            }
        })?;
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            accepted,
            cursor: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            rejected: Callback::new(self.rejected.take()),
            month,
            level,
            accepted_level,
            max_level,
            month_changed: Callback::new(self.month_changed.take()),
            level_changed: Callback::new(self.level_changed.take()),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(s) = cleanup.upgrade() {
                s.retire();
            }
        })?;
        let weak = Rc::downgrade(&state);
        let dirty = state.dirty.clone();
        scope.effect(move || {
            if dirty.take()
                && let Some(s) = weak.upgrade()
            {
                s.cursor.set(None);
            }
        });
        let s = state.clone();
        let deselect = self.deselect;
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        let configured_style = self.common.style.clone();
        let visible_level = state.accepted_level;
        self.common.style = scope
            .bond(move || {
                let mut style = configured_style.get();
                if visible_level.get() != DatePickerLevel::Month {
                    style.display = Some(Display::None);
                }
                style
            })
            .into();
        let key = std::mem::take(&mut self.key);
        let excluded = std::mem::take(&mut self.excluded);
        if self.columns == 1 {
            let month_state = state.clone();
            let mut calendar = crate::calendar(
                key.clone(),
                scope.bond(move || accepted.with(|s| s.provider)),
            )
            .selection(scope.bond(move || accepted.with(|s| s.value)))
            .readonly(scope.bond(move || accepted.with(|s| s.readonly)))
            .direction(self.direction)
            .hide_outside_dates(self.hide_outside_dates)
            .hide_weekdays(self.hide_weekdays)
            .with_week_numbers(self.week_numbers)
            .exclude_dates(excluded)
            .month(state.month.source.clone())
            .on_value_change(move |date| {
                if let Some(date) = date {
                    s.report(s.select(date, deselect));
                }
            })
            .on_month_change(move |month| {
                month_state.report(month_state.propose_month(month));
            });
            if self.max_level.rank() > DatePickerLevel::Month.rank() {
                let level = state.clone();
                calendar = calendar.on_level_control(move || {
                    level.report(level.propose_level(DatePickerLevel::Year));
                });
            }
            calendar.common = std::mem::take(&mut self.common);
            calendar.common.enabled = scope.bond(move || accepted.with(|s| s.enabled)).into();
            for day in Weekday::ALL {
                if self.weekdays & (1 << day.index()) != 0 {
                    calendar = calendar.disable_weekday(day);
                }
            }
            calendar.show_in(scope)?;
        } else {
            let mut months = Vec::with_capacity(usize::from(self.columns));
            for offset in 0..self.columns {
                months.push(
                    scope.cage(
                        state
                            .month
                            .source
                            .try_get()?
                            .checked_add(i32::from(offset))
                            .map_err(|_| invalid())?,
                    ),
                );
            }
            let columns = Rc::new(MonthColumns {
                picker: state.clone(),
                months,
            });
            let weak_columns = Rc::downgrade(&columns);
            scope.effect(move || {
                if let Some(columns) = weak_columns.upgrade() {
                    columns.picker.month.source.get();
                    columns.report(columns.sync());
                }
            });
            let mut calendars: Vec<Box<dyn Widget>> = Vec::with_capacity(usize::from(self.columns));
            for column in 0..self.columns {
                let select = state.clone();
                let month_state = columns.clone();
                let mut calendar = crate::calendar(
                    format!("{key}-{column}"),
                    scope.bond(move || accepted.with(|s| s.provider)),
                )
                .theme(theme.clone())
                .size(size)
                .selection(scope.bond(move || accepted.with(|s| s.value)))
                .readonly(scope.bond(move || accepted.with(|s| s.readonly)))
                .enabled(scope.bond(move || accepted.with(|s| s.enabled)))
                .direction(self.direction)
                .hide_outside_dates(true)
                .hide_weekdays(self.hide_weekdays)
                .with_week_numbers(self.week_numbers)
                .exclude_dates(excluded.iter().copied())
                .month(columns.months[usize::from(column)])
                .navigation_controls(column == 0, column + 1 == self.columns)
                .navigation_step(columns_to_scroll)
                .on_value_change(move |date| {
                    if let Some(date) = date {
                        select.report(select.select(date, deselect));
                    }
                })
                .on_month_change(move |month| {
                    month_state.report(month_state.propose(usize::from(column), month));
                });
                if column == 0 && self.max_level.rank() > DatePickerLevel::Month.rank() {
                    let level = state.clone();
                    calendar = calendar.on_level_control(move || {
                        level.report(level.propose_level(DatePickerLevel::Year));
                    });
                }
                if unstyled {
                    calendar = calendar.unstyled();
                }
                for day in Weekday::ALL {
                    if self.weekdays & (1 << day.index()) != 0 {
                        calendar = calendar.disable_weekday(day);
                    }
                }
                calendars.push(Box::new(calendar));
            }
            let (classes, appearance) = self.common.appearance(
                scope,
                "sapid-date-picker-columns",
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Row)
                    .gap(16.),
            );
            let outer = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
            let mut surface = glory::ui::view()
                .class(classes)
                .style(appearance)
                .focus(self.common.focus)
                .node_ref(outer);
            if let Some(name) = self.common.name.take() {
                surface = surface.name(name);
            }
            for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
                let owner = scope.id();
                surface = surface.on(kind, move |event| {
                    crate::appearance::observe(&mut handlers, owner, event)
                });
            }
            surface.show_in(scope)?;
            let content = scope.child_in(outer.get().ok_or(glory::ui::Error::Gone)?)?;
            crate::group().gap(16.).fill(calendars).show_in(&content)?;
            scope.retain(content)?;
            scope.retain(columns)?;
        }
        let period_enabled = state.accepted;
        let month_level = state.accepted_level;
        let month_style = scope.bond(move || {
            if month_level.get() == DatePickerLevel::Year {
                Style::new()
            } else {
                Style::new().display(Display::None)
            }
        });
        let month_source = state.month.source.clone();
        let month_value = scope.bond(move || PeriodSelection::Single(Some(month_source.get())));
        let month_source = state.month.source.clone();
        let month_year = scope.bond(move || {
            CivilYear::new(month_source.get().year())
                .expect("validated YearMonth always has a civil year")
        });
        let choose_month = state.clone();
        let change_month_year = state.clone();
        let mut month_picker =
            crate::month_picker(format!("{key}-level-months"), self.provider.clone())
                .theme(theme.clone())
                .size(size)
                .class("sapid-date-picker-year-level")
                .style(month_style)
                .value(month_value)
                .year(month_year)
                .enabled(scope.bond(move || {
                    period_enabled.with(|snapshot| snapshot.enabled)
                        && month_level.get() == DatePickerLevel::Year
                }))
                .readonly(scope.bond(move || period_enabled.with(|snapshot| snapshot.readonly)))
                .direction(self.direction)
                .on_value_change(move |selection| {
                    if let PeriodSelection::Single(Some(month)) = selection {
                        choose_month.report(choose_month.choose_month(month));
                    }
                })
                .on_year_change(move |year| {
                    let current = change_month_year.month.source.get();
                    match YearMonth::new(year.value(), current.month()) {
                        Ok(month) => {
                            change_month_year.report(change_month_year.propose_month(month))
                        }
                        Err(_) => change_month_year.report(Err(invalid())),
                    }
                });
        if self.max_level.rank() > DatePickerLevel::Year.rank() {
            let level = state.clone();
            month_picker = month_picker.on_level_control(move || {
                level.report(level.propose_level(DatePickerLevel::Decade));
            });
        }
        if unstyled {
            month_picker = month_picker.unstyled();
        }
        month_picker.show_in(scope)?;

        let year_level = state.accepted_level;
        let year_style = scope.bond(move || {
            if year_level.get() == DatePickerLevel::Decade {
                Style::new()
            } else {
                Style::new().display(Display::None)
            }
        });
        let month_source = state.month.source.clone();
        let year_value = scope.bond(move || {
            PeriodSelection::Single(Some(
                CivilYear::new(month_source.get().year())
                    .expect("validated YearMonth always has a civil year"),
            ))
        });
        let month_source = state.month.source.clone();
        let year_page = scope.bond(move || {
            CivilYear::new(month_source.get().year())
                .expect("validated YearMonth always has a civil year")
        });
        let choose_year = state.clone();
        let change_year_page = state.clone();
        let mut year_picker =
            crate::year_picker(format!("{key}-level-years"), self.provider.clone())
                .theme(theme.clone())
                .size(size)
                .class("sapid-date-picker-decade-level")
                .style(year_style)
                .value(year_value)
                .year(year_page)
                .enabled(scope.bond(move || {
                    period_enabled.with(|snapshot| snapshot.enabled)
                        && year_level.get() == DatePickerLevel::Decade
                }))
                .readonly(scope.bond(move || period_enabled.with(|snapshot| snapshot.readonly)))
                .direction(self.direction)
                .on_value_change(move |selection| {
                    if let PeriodSelection::Single(Some(year)) = selection {
                        choose_year.report(choose_year.choose_year(year));
                    }
                })
                .on_year_change(move |year| {
                    let current = change_year_page.month.source.get();
                    match YearMonth::new(year.value(), current.month()) {
                        Ok(month) => change_year_page.report(change_year_page.propose_month(month)),
                        Err(_) => change_year_page.report(Err(invalid())),
                    }
                });
        if unstyled {
            year_picker = year_picker.unstyled();
        }
        year_picker.show_in(scope)?;
        if !self.presets.is_empty() {
            let mut buttons: Vec<Box<dyn Widget>> = Vec::with_capacity(self.presets.len());
            for preset in std::mem::take(&mut self.presets) {
                let selection = preset.value;
                let choose = state.clone();
                let selected = state
                    .accepted
                    .map(move |snapshot| snapshot.value == selection);
                let enabled = state
                    .accepted
                    .map(|snapshot| snapshot.enabled && !snapshot.readonly);
                let mut button = crate::toggle()
                    .theme(theme.clone())
                    .size(size)
                    .class("sapid-date-picker-preset")
                    .label(preset.label)
                    .pressed(selected)
                    .enabled(enabled)
                    .on_pressed_change(move |pressed| {
                        if pressed {
                            choose.report(choose.choose(selection));
                        }
                    });
                if unstyled {
                    button = button.unstyled();
                }
                buttons.push(Box::new(button));
            }
            crate::group()
                .theme(theme)
                .class("sapid-date-picker-presets")
                .gap(6.)
                .fill(buttons)
                .show_in(scope)?;
        }
        scope.retain(state)
    }
}
