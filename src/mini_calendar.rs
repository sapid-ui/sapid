//! A bounded, navigable strip of civil dates, independent of the monthly Calendar.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, Observers};
use crate::state::{Control, Dirty, Value};
use crate::{CivilDate, DateLocale, DatesProvider, Theme, Weekday};
use glory::render::{
    Current, Direction, Event, EventKind, FocusMode, GridCommand, NodeId, Role, Semantics,
    TableMetadata,
};
use glory::style::{
    Align, BorderStyle, Display, Edges, FlexDirection, GridPlacement, GridTrack, GridTracks,
    Length, Limit, Overflow, Style,
};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, rc::Rc};

pub const MAX_MINI_CALENDAR_DAYS: usize = 31;
fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid finite mini calendar").into()
}
fn start_at(date: CivilDate, count: usize) -> CivilDate {
    date.min(CivilDate::MAX.checked_add_days(1 - count as i32).unwrap())
}
#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    provider: DatesProvider,
    start: CivilDate,
    value: Option<CivilDate>,
    enabled: bool,
    readonly: bool,
}
#[derive(Clone, Copy)]
struct Cursor {
    from: NodeId,
    start: CivilDate,
    provider: DatesProvider,
    date: CivilDate,
}
struct State {
    weak: glory::ui::WeakScope,
    provider: Vary<DatesProvider>,
    value: Control<Option<CivilDate>>,
    date: Control<CivilDate>,
    enabled: Vary<bool>,
    readonly: Vary<bool>,
    accepted: Cage<Snapshot>,
    count: usize,
    excluded: Vec<CivilDate>,
    weekdays: u8,
    deselect: bool,
    references: Vec<Cage<Option<NodeId>>>,
    focused: Cage<Option<CivilDate>>,
    physical: Cage<Option<NodeId>>,
    cursor: Cell<Option<Cursor>>,
    intent: Cell<Option<CivilDate>>,
    dirty: Dirty,
    changed: Callback<Option<CivilDate>>,
    date_changed: Callback<CivilDate>,
}
impl State {
    fn snapshot(&self) -> Result<Snapshot> {
        let s = Snapshot {
            provider: self.provider.try_get()?,
            start: start_at(self.date.source.try_get()?, self.count),
            value: self.value.source.try_get()?,
            enabled: self.enabled.try_get()?,
            readonly: self.readonly.try_get()?,
        };
        validate(&s)?;
        Ok(s)
    }
    fn allowed(&self, s: Snapshot, date: CivilDate) -> bool {
        s.enabled
            && s.provider.date_bounds().contains(date)
            && self.weekdays & (1 << date.weekday().index()) == 0
            && self.excluded.binary_search(&date).is_err()
    }
    fn at(&self, s: Snapshot, i: usize) -> CivilDate {
        s.start.checked_add_days(i as i32).unwrap()
    }
    fn nearest(&self, s: Snapshot, preferred: Option<CivilDate>) -> Option<CivilDate> {
        (0..self.count)
            .map(|i| self.at(s, i))
            .filter(|date| self.allowed(s, *date))
            .min_by_key(|date| preferred.map_or(0, |p| p.days_until(*date).unsigned_abs()))
    }
    fn report(&self, result: Result) {
        if let Err(e) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(e)
        }
    }
    fn range(&self, proposal: CivilDate) -> Result {
        let Some(scope) = self.weak.upgrade() else {
            return Ok(());
        };
        let proposal = start_at(proposal, self.count);
        if start_at(self.date.source.try_get()?, self.count) != proposal {
            self.date.accept(&proposal)?;
            self.date_changed.emit(scope.id(), proposal);
        }
        self.dirty.touch()
    }
    fn page(&self, delta: i32) -> Result {
        let s = self.snapshot()?;
        if !s.enabled {
            return Ok(());
        }
        let min = s.provider.date_bounds().min().unwrap_or(CivilDate::MIN);
        let max = start_at(
            s.provider.date_bounds().max().unwrap_or(CivilDate::MAX),
            self.count,
        )
        .max(min);
        let next = s
            .start
            .checked_add_days(delta * self.count as i32)
            .unwrap_or(if delta < 0 {
                CivilDate::MIN
            } else {
                CivilDate::MAX
            })
            .max(min)
            .min(max);
        self.cursor.set(None);
        self.range(next)
    }
    fn navigate(&self, from: NodeId, command: GridCommand) -> Result {
        let s = self.snapshot()?;
        let Some(at) = self.references.iter().position(|r| r.get() == Some(from)) else {
            return Ok(());
        };
        if !self.allowed(s, self.at(s, at)) {
            return Ok(());
        }
        let current = self
            .cursor
            .get()
            .filter(|c| c.from == from && c.start == s.start && c.provider == s.provider)
            .map_or(self.at(s, at), |c| c.date);
        let (target, delta, steps) = match command {
            GridCommand::Left | GridCommand::Up => (
                current.checked_add_days(-1).ok(),
                -1,
                crate::MAX_CALENDAR_SEARCH_DAYS,
            ),
            GridCommand::Right | GridCommand::Down => (
                current.checked_add_days(1).ok(),
                1,
                crate::MAX_CALENDAR_SEARCH_DAYS,
            ),
            GridCommand::FirstInRow | GridCommand::PreviousLargePage => {
                (Some(s.start), 1, self.count)
            }
            GridCommand::LastInRow | GridCommand::NextLargePage => {
                (Some(self.at(s, self.count - 1)), -1, self.count)
            }
            GridCommand::PreviousPage => (
                current.checked_add_days(-(self.count as i32)).ok(),
                -1,
                crate::MAX_CALENDAR_SEARCH_DAYS,
            ),
            GridCommand::NextPage => (
                current.checked_add_days(self.count as i32).ok(),
                1,
                crate::MAX_CALENDAR_SEARCH_DAYS,
            ),
        };
        let Some(mut target) = target else {
            return Ok(());
        };
        for _ in 0..steps {
            if self.allowed(s, target) {
                if target < s.start || target > self.at(s, self.count - 1) {
                    let next = if target < s.start {
                        target
                    } else {
                        target.checked_add_days(1 - self.count as i32).unwrap()
                    };
                    self.range(next)?;
                }
                let now = self.snapshot()?;
                if target < now.start || target > self.at(now, self.count - 1) {
                    return Ok(());
                }
                self.cursor.set(Some(Cursor {
                    from,
                    start: now.start,
                    provider: now.provider,
                    date: target,
                }));
                self.focused.try_revise(|old| *old = Some(target))?;
                self.intent.set(Some(target));
                return self.dirty.touch();
            }
            let Ok(next) = target.checked_add_days(delta) else {
                break;
            };
            target = next;
        }
        Ok(())
    }
    fn select(&self, index: usize) -> Result {
        let Some(scope) = self.weak.upgrade() else {
            return Ok(());
        };
        let s = self.snapshot()?;
        let date = self
            .cursor
            .get()
            .filter(|c| {
                self.references[index].get() == Some(c.from)
                    && c.start == s.start
                    && c.provider == s.provider
            })
            .map_or(self.at(s, index), |c| c.date);
        if s.readonly || !self.allowed(s, date) {
            return Ok(());
        }
        let next = if self.deselect && s.value == Some(date) {
            None
        } else {
            Some(date)
        };
        if s.value != next {
            self.value.accept(&next)?;
            self.changed.emit(scope.id(), next);
        }
        if scope.id().is_alive() {
            self.dirty.touch()?;
        }
        Ok(())
    }
}
fn validate(s: &Snapshot) -> Result {
    if s.value
        .is_some_and(|v| !s.provider.date_bounds().contains(v))
    {
        Err(invalid())
    } else {
        Ok(())
    }
}
type DayStyle = Rc<dyn Fn(CivilDate) -> Style>;
pub struct MiniCalendar {
    common: Common,
    key: String,
    provider: Vary<DatesProvider>,
    value: Value<Option<CivilDate>>,
    date: Option<Vary<CivilDate>>,
    default_date: Option<CivilDate>,
    count: usize,
    readonly: Vary<bool>,
    direction: Direction,
    excluded: Vec<CivilDate>,
    weekdays: u8,
    deselect: bool,
    day_style: Option<DayStyle>,
    changed: Option<Box<dyn FnMut(Option<CivilDate>)>>,
    date_changed: Option<Box<dyn FnMut(CivilDate)>>,
}
pub fn mini_calendar(
    key: impl Into<String>,
    provider: impl Into<Vary<DatesProvider>>,
) -> MiniCalendar {
    MiniCalendar {
        common: Common::default(),
        key: key.into(),
        provider: provider.into(),
        value: Value::default(),
        date: None,
        default_date: None,
        count: 7,
        readonly: false.into(),
        direction: Direction::Ltr,
        excluded: Vec::new(),
        weekdays: 0,
        deselect: false,
        day_style: None,
        changed: None,
        date_changed: None,
    }
}
impl MiniCalendar {
    common_methods!();
    pub fn value(mut self, value: impl Into<Vary<Option<CivilDate>>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: Option<CivilDate>) -> Self {
        self.value.default = value;
        self
    }
    pub fn date(mut self, value: impl Into<Vary<CivilDate>>) -> Self {
        self.date = Some(value.into());
        self
    }
    pub fn default_date(mut self, value: CivilDate) -> Self {
        self.default_date = Some(value);
        self
    }
    pub fn number_of_days(mut self, value: usize) -> Self {
        self.count = value;
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
    pub fn day_style(mut self, value: impl Fn(CivilDate) -> Style + 'static) -> Self {
        self.day_style = Some(Rc::new(value));
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(Option<CivilDate>) + 'static) -> Self {
        self.changed = Some(Box::new(value));
        self
    }
    pub fn on_date_change(mut self, value: impl FnMut(CivilDate) + 'static) -> Self {
        self.date_changed = Some(Box::new(value));
        self
    }
}
fn child(scope: &Scope, reference: Cage<Option<NodeId>>) -> Result<Scope> {
    scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)
}
impl Widget for MiniCalendar {
    fn build(&mut self, scope: &Scope) -> Result {
        let id = composite::id("mini-calendar", &self.key, "grid", "days")?;
        if !(1..=MAX_MINI_CALENDAR_DAYS).contains(&self.count)
            || self.excluded.len() > crate::MAX_CALENDAR_EXCLUDED_DATES
        {
            return Err(invalid());
        }
        let count = self.count;
        let provider = self.provider.clone();
        let default = self
            .default_date
            .unwrap_or(provider.try_get()?.reference_date());
        let date = Value {
            source: self.date.take(),
            default,
        }
        .mount(scope);
        let value = std::mem::take(&mut self.value).mount(scope);
        let p = provider.clone();
        let d = date.source.clone();
        let v = value.source.clone();
        let enabled = self.common.enabled.clone();
        let readonly = self.readonly.clone();
        let snapshot = scope.bond(move || Snapshot {
            provider: p.get(),
            start: start_at(d.get(), count),
            value: v.get(),
            enabled: enabled.get(),
            readonly: readonly.get(),
        });
        let accepted = composite::validated(scope, &snapshot.into(), validate)?;
        let mut excluded = std::mem::take(&mut self.excluded);
        excluded.sort_unstable();
        excluded.dedup();
        let state = Rc::new(State {
            weak: scope.downgrade(),
            provider,
            value,
            date,
            enabled: self.common.enabled.clone(),
            readonly: self.readonly.clone(),
            accepted,
            count,
            excluded,
            weekdays: self.weekdays,
            deselect: self.deselect,
            references: (0..count).map(|_| scope.cage(None)).collect(),
            focused: scope.cage(None),
            physical: scope.cage(None),
            cursor: Cell::new(None),
            intent: Cell::new(None),
            dirty: Dirty::new(scope),
            changed: Callback::new(self.changed.take()),
            date_changed: Callback::new(self.date_changed.take()),
        });
        let initial = accepted.get();
        state
            .focused
            .revise(|old| *old = state.nearest(initial, initial.value));
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(s) = cleanup.upgrade() {
                s.changed.clear();
                s.date_changed.clear();
                s.cursor.set(None);
                s.intent.set(None);
            }
        })?;
        let theme = self.common.theme.clone();
        let themed = !self.common.unstyled;
        let width = match theme.control(self.common.size).min_height {
            Some(Length::Px(v)) => v.max(28.),
            _ => 36.,
        };
        let base = Style {
            align: Some(Align::Center),
            ..Style::new()
        }
        .display(Display::Flex)
        .gap(4.)
        .width(Length::Percent(100.))
        .max_width(Limit::Px((count as f64 + 2.) * (width + 4.)))
        .min_width(Length::Px(0.))
        .overflow_x(Overflow::Auto);
        let outer = scope.cage(None);
        let layout = base.clone();
        let (classes, appearance) = self.common.appearance(scope, "sapid-mini-calendar", base);
        let style = scope.bond(move || crate::appearance::merge(layout.clone(), appearance.get()));
        glory::ui::view()
            .class(classes)
            .style(style)
            .node_ref(outer)
            .show_in(scope)?;
        let content = child(scope, outer)?;
        let activation = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default(),
        );
        let grid_events = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::GridNavigate)
                .unwrap_or_default(),
        );
        let observers = (activation.clone(), grid_events.clone());
        scope.on_cleanup(move || {
            observers.0.clear();
            observers.1.clear();
        })?;
        control(
            &content,
            state.clone(),
            -1,
            width,
            &theme,
            themed,
            self.direction,
        )?;
        let root = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let direction = self.direction;
        let s = state.clone();
        let mut grid = glory::ui::view()
            .role(Role::Grid)
            .name(
                self.common
                    .name
                    .take()
                    .unwrap_or_else(|| "Choose a date".into()),
            )
            .class("sapid-mini-calendar-grid")
            .focus(self.common.focus)
            .node_ref(root)
            .style(
                Style {
                    grow: Some(1.),
                    shrink: Some(1.),
                    ..Style::new()
                }
                .min_width(Length::Px(count as f64 * 30.)),
            )
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    readonly: accepted.with(|snapshot| snapshot.readonly),
                    grid_navigation: Some(direction),
                    table: Some(TableMetadata::Table {
                        rows: Some(1),
                        columns: Some(count as u16),
                    }),
                    ..Default::default()
                })
            }))
            .on(EventKind::GridNavigate, move |e| {
                if let Event::GridNavigate { from, command } = &e {
                    s.report(s.navigate(*from, *command));
                }
                if let Some(scope) = s.weak.upgrade() {
                    grid_events.emit(scope.id(), e)
                }
            });
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            grid = grid.on(kind, move |e| {
                crate::appearance::observe(&mut handlers, owner, e)
            });
        }
        grid.show_in(&content)?;
        let grid_scope = child(scope, root)?;
        let row = scope.cage(None);
        glory::ui::view()
            .role(Role::Row)
            .node_ref(row)
            .style(
                Style::new()
                    .display(Display::Grid)
                    .grid_columns(GridTracks::Tracks(vec![GridTrack::Fr(1.); count]))
                    .gap(2.),
            )
            .show_in(&grid_scope)?;
        let row_scope = child(scope, row)?;
        for index in 0..count {
            let physical = if direction == Direction::Rtl {
                count - 1 - index
            } else {
                index
            };
            let cell = scope.cage(None);
            let s = state.clone();
            glory::ui::view()
                .role(Role::GridCell)
                .node_ref(cell)
                .style(
                    Style::new()
                        .grid_column(GridPlacement::At {
                            start: physical as u16 + 1,
                            span: 1,
                        })
                        .grid_row(GridPlacement::At { start: 1, span: 1 }),
                )
                .semantics(scope.bond(move || {
                    let snap = accepted.get();
                    composite::metadata(Semantics {
                        selected: Some(snap.value == Some(s.at(snap, index))),
                        table: Some(TableMetadata::Cell {
                            row: 0,
                            column: index as u16,
                            row_span: 1,
                            column_span: 1,
                            headers: vec![],
                        }),
                        ..Default::default()
                    })
                }))
                .show_in(&row_scope)?;
            let cell_scope = child(scope, cell)?;
            Day {
                state: state.clone(),
                index,
                theme: theme.clone(),
                themed,
                width,
                day_style: self.day_style.clone(),
                observers: activation.clone(),
            }
            .show_in(&cell_scope)?;
            scope.retain(cell_scope)?;
        }
        control(&content, state.clone(), 1, width, &theme, themed, direction)?;
        let weak = Rc::downgrade(&state);
        scope.effect(move || {
            let Some(s) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let edited = s.dirty.take();
                let snap = accepted.get();
                if s.focused.get().is_none_or(|date| {
                    date < snap.start || date > s.at(snap, count - 1) || !s.allowed(snap, date)
                }) {
                    let next = s.nearest(snap, snap.value);
                    if next != s.focused.get() {
                        s.focused.try_revise(|old| *old = next)?;
                    }
                }
                if edited {
                    s.cursor.set(None);
                }
                if let Some(date) = s.intent.take()
                    && date >= snap.start
                    && date <= s.at(snap, count - 1)
                    && let Some(scope) = s.weak.upgrade()
                {
                    let node = s.references[snap.start.days_until(date) as usize].get();
                    composite::focus_node(&scope, node)?;
                    if let Some(node) = node
                        && scope.capabilities()?.scroll
                    {
                        scope.action(glory::render::Action::ScrollIntoView(node))?;
                    }
                }
                Ok(())
            })();
            s.report(result);
        });
        scope.retain(row_scope)?;
        scope.retain(grid_scope)?;
        scope.retain(content)?;
        Ok(())
    }
}
fn control(
    scope: &Scope,
    s: Rc<State>,
    delta: i32,
    width: f64,
    theme: &Theme,
    themed: bool,
    direction: Direction,
) -> Result {
    let enabled = s.clone();
    let action = s.clone();
    let icon = if (delta < 0) == (direction == Direction::Ltr) {
        crate::Icon::ChevronLeft
    } else {
        crate::Icon::ChevronRight
    };
    let mut button = glory::ui::button()
        .name(if delta < 0 {
            "Previous range"
        } else {
            "Next range"
        })
        .class("sapid-mini-calendar-control")
        .enabled(scope.bond(move || {
            let snap = enabled.accepted.get();
            snap.enabled
                && if delta < 0 {
                    snap.start > snap.provider.date_bounds().min().unwrap_or(CivilDate::MIN)
                } else {
                    enabled.at(snap, enabled.count - 1)
                        < snap.provider.date_bounds().max().unwrap_or(CivilDate::MAX)
                }
        }))
        .fill(
            glory::ui::graphic()
                .drawing(icon.drawing(theme.foreground))
                .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
        )
        .on(EventKind::Activate, move |_| {
            action.report(action.page(delta))
        });
    let mut style = Style {
        shrink: Some(0.),
        align: Some(Align::Center),
        ..Style::new()
    }
    .width(Length::Px(width.min(28.)))
    .height(Length::Px(width.min(32.)))
    .padding(Edges::all(0.));
    if themed {
        style.background = Some(glory::style::Color(0, 0, 0, 0));
        style.color = Some(theme.foreground);
        style = style
            .radius(theme.radius)
            .border_width(Edges::all(0.))
            .border_style(BorderStyle::None);
    }
    button = button.style(style);
    button.show_in(scope)
}
struct Day {
    state: Rc<State>,
    index: usize,
    theme: Theme,
    themed: bool,
    width: f64,
    day_style: Option<DayStyle>,
    observers: Observers,
}
impl Widget for Day {
    fn build(&mut self, scope: &Scope) -> Result {
        let index = self.index;
        let s = self.state.clone();
        let accepted = s.accepted;
        let focused = s.focused;
        let physical = s.physical;
        let reference = s.references[index];
        let date = scope.bond(move || {
            let snap = accepted.get();
            snap.start.checked_add_days(index as i32).unwrap()
        });
        let enabled = s.clone();
        let styling = s.clone();
        let selected = s.clone();
        let focus = s.clone();
        let theme = self.theme.clone();
        let themed = self.themed;
        let custom = self.day_style.clone();
        let width = self.width;
        let observers = self.observers.clone();
        glory::ui::button()
            .class("sapid-mini-calendar-day")
            .node_ref(reference)
            .name(scope.bond(move || {
                accepted.with(|snap| snap.provider.locale_value().date_label(date.get()))
            }))
            .enabled(scope.bond(move || enabled.allowed(accepted.get(), date.get())))
            .focus(scope.bond(move || {
                if focused.get() == Some(date.get()) {
                    FocusMode::TabStop
                } else {
                    FocusMode::Programmatic
                }
            }))
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    current: accepted.with(|snap| {
                        (snap.provider.today_date() == Some(date.get())).then_some(Current::Date)
                    }),
                    ..Default::default()
                })
            }))
            .style(scope.bond(move || {
                let snap = accepted.get();
                let day = date.get();
                let chosen = snap.value == Some(day);
                let active = reference.get().is_some_and(|id| physical.get() == Some(id));
                let mut style = Style::new()
                    .width(Length::Percent(100.))
                    .min_width(Length::Px(28.))
                    .height(Length::Px(width * 1.5 + 2.))
                    .padding(Edges::all(2.))
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(2.);
                if themed {
                    style = style
                        .radius(theme.radius)
                        .border_width(Edges::all(1.))
                        .border_style(BorderStyle::Solid)
                        .border_color(if active { theme.accent } else { theme.border });
                    style.background = Some(if chosen { theme.accent } else { theme.surface });
                    style.color = Some(if !styling.allowed(snap, day) {
                        theme.disabled_text
                    } else if chosen {
                        theme.on_accent
                    } else {
                        theme.foreground
                    });
                }
                if let Some(custom) = &custom {
                    style = crate::appearance::merge(style, custom(day));
                }
                style
            }))
            .fill(
                glory::ui::group().fill((
                    glory::ui::text()
                        .content(scope.bond(move || {
                            accepted.with(|snap| {
                                let d = date.get();
                                match snap.provider.locale_value() {
                                    DateLocale::En => {
                                        snap.provider.locale_value().month_name(d.year_month())[..3]
                                            .to_owned()
                                    }
                                    DateLocale::ZhCn => format!("{}月", d.month()),
                                }
                            })
                        }))
                        .style(Style {
                            font_size: Some(if width < 30. { 10. } else { 11. }),
                            ..Style::new()
                        }),
                    glory::ui::text()
                        .content(scope.bond(move || date.get().day().to_string()))
                        .style(Style {
                            font_size: Some(if width < 30. { 14. } else { 16. }),
                            ..Style::new()
                        }),
                )),
            )
            .on(EventKind::Activate, move |e| {
                selected.report(selected.select(index));
                if let Some(scope) = selected.weak.upgrade() {
                    observers.emit(scope.id(), e)
                }
            })
            .on(EventKind::FocusChanged, move |e| {
                if let Event::FocusChanged(active) = e {
                    let result = (|| -> Result {
                        if active {
                            focus.cursor.set(None);
                            focused.try_revise(|old| *old = Some(date.get()))?;
                            physical.try_revise(|old| *old = reference.get())?;
                        } else if physical.get() == reference.get() {
                            physical.try_revise(|old| *old = None)?;
                        }
                        Ok(())
                    })();
                    focus.report(result)
                }
            })
            .show_in(scope)
    }
}
