//! Calendar heatmap: dated values in week columns and weekday rows.
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, Observers};
use crate::dates::{CivilDate, DateLocale, Weekday};
use crate::heatmap_model::*;
use crate::state::Dirty;
use glory::render::{
    Event, EventKind, FocusMode, GridCommand, NodeId, Role, Semantics, TableMetadata,
};
use glory::style::{
    Align, BorderStyle, Color, Display, Edges, FlexDirection, GridPlacement, GridTrack, GridTracks,
    Length, Limit, Overflow, Style, WhiteSpace,
};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};
pub const MAX_HEATMAP_COLORS: usize = 8;
pub const MAX_HEATMAP_KEY_BYTES: usize = 48;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatmapCell {
    pub date: CivilDate,
    pub value: Option<f64>,
}
type Label = Rc<dyn Fn(HeatmapCell) -> String>;
type CellStyle = Rc<dyn Fn(HeatmapCell) -> Style>;
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
#[derive(Clone, PartialEq)]
struct Model {
    range: HeatmapRange,
    data: HeatmapData,
    domain: HeatmapDomain,
    locale: DateLocale,
    labels: BTreeMap<CivilDate, String>,
}
fn admit(
    range: HeatmapRange,
    data: HeatmapData,
    domain: Option<HeatmapDomain>,
    locale: DateLocale,
    label: &Option<Label>,
) -> Result<Rc<Model>> {
    let mut labels = BTreeMap::new();
    let mut bytes = 0usize;
    for i in 0..range.len() {
        let date = range.date(i).unwrap();
        let cell = HeatmapCell {
            date,
            value: data.value(date),
        };
        let text = label.as_ref().map_or_else(
            || {
                let value = cell.value.map_or_else(
                    || match locale {
                        DateLocale::En => "No data".into(),
                        DateLocale::ZhCn => "无数据".into(),
                    },
                    |value| match locale {
                        DateLocale::En => {
                            format!("{value} contribution{}", if value == 1. { "" } else { "s" })
                        }
                        DateLocale::ZhCn => format!("{value} 次贡献"),
                    },
                );
                format!("{}: {value}", locale.date_label(date))
            },
            |label| label(cell),
        );
        if text.is_empty() || text.len() > 256 || text.contains('\0') {
            return Err(invalid("heatmap labels require 1–256 UTF-8 bytes"));
        }
        bytes += text.len();
        if bytes > crate::chart_model::MAX_CHART_TEXT_BYTES {
            return Err(invalid("heatmap label text exceeds 64 KiB"));
        }
        labels.insert(date, text);
    }
    let domain = domain.unwrap_or_else(|| data.domain(range));
    Ok(Rc::new(Model {
        range,
        data,
        domain,
        locale,
        labels,
    }))
}
struct State {
    weak: glory::ui::WeakScope,
    model: Cage<Rc<Model>>,
    range: Vary<HeatmapRange>,
    data: Vary<HeatmapData>,
    enabled: Vary<bool>,
    first: Weekday,
    references: RefCell<BTreeMap<CivilDate, Cage<Option<NodeId>>>>,
    focused: Cage<Option<CivilDate>>,
    physical: Cage<Option<NodeId>>,
    hint: Option<Cage<Option<CivilDate>>>,
    cursor: Cell<Option<(NodeId, HeatmapRange, CivilDate)>>,
    intent: Cell<Option<CivilDate>>,
    dirty: Dirty,
    clicked: Callback<HeatmapCell>,
    activate_observers: Observers,
    navigation_observers: Observers,
}
impl State {
    fn hint(&self, date: CivilDate, open: bool) -> Result {
        if self.weak.upgrade().is_none() {
            return Ok(());
        }
        let Some(hint) = self.hint else { return Ok(()) };
        if open {
            if self.enabled.try_get()?
                && self.range.try_get()?.contains(date)
                && hint.try_get()? != Some(date)
            {
                hint.try_revise(|current| *current = Some(date))?;
            }
        } else if hint.try_get()? == Some(date) {
            hint.try_revise(|current| *current = None)?;
        }
        Ok(())
    }
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error)
        }
    }
    fn activate(&self, date: CivilDate, event: Event) -> Result {
        let Some(scope) = self.weak.upgrade() else {
            return Ok(());
        };
        let range = self.range.try_get()?;
        let origin = self
            .references
            .borrow()
            .get(&date)
            .and_then(|reference| reference.get());
        let date = self
            .cursor
            .get()
            .filter(|(node, old, _)| Some(*node) == origin && *old == range)
            .map_or(date, |(_, _, date)| date);
        if !self.enabled.try_get()? || !range.contains(date) {
            return Ok(());
        }
        let cell = self.data.try_with(|data| HeatmapCell {
            date,
            value: data.value(date),
        })?;
        self.clicked.emit(scope.id(), cell);
        self.activate_observers.emit(scope.id(), event);
        Ok(())
    }
    fn navigate(&self, from: NodeId, command: GridCommand) -> Result {
        if self.weak.upgrade().is_none() || !self.enabled.try_get()? {
            return Ok(());
        }
        let range = self.range.try_get()?;
        let Some(origin) = self
            .references
            .borrow()
            .iter()
            .find(|(_, r)| r.get() == Some(from))
            .map(|(date, _)| *date)
        else {
            return Ok(());
        };
        let date = self
            .cursor
            .get()
            .filter(|(node, old, _)| *node == from && *old == range)
            .map_or(origin, |(_, _, date)| date);
        let (week, row) = range
            .position(date, self.first)
            .ok_or(invalid("retired heatmap keyboard origin"))?;
        let target = match command {
            GridCommand::Left => date.checked_add_days(-7).ok(),
            GridCommand::Right => date.checked_add_days(7).ok(),
            GridCommand::Up => {
                if row > 0 {
                    date.checked_add_days(-1).ok()
                } else {
                    None
                }
            }
            GridCommand::Down => {
                if row < 6 {
                    date.checked_add_days(1).ok()
                } else {
                    None
                }
            }
            GridCommand::FirstInRow => {
                (0..=week).find_map(|column| range.at(column, row, self.first))
            }
            GridCommand::LastInRow => (week..range.weeks(self.first))
                .rev()
                .find_map(|column| range.at(column, row, self.first)),
            GridCommand::PreviousPage => date
                .checked_add_days(-28)
                .ok()
                .map(|date| date.max(range.start())),
            GridCommand::NextPage => date
                .checked_add_days(28)
                .ok()
                .map(|date| date.min(range.end())),
            GridCommand::PreviousLargePage => Some(range.start()),
            GridCommand::NextLargePage => Some(range.end()),
        };
        if let Some(target) = target.filter(|date| range.contains(*date)) {
            self.cursor.set(Some((from, range, target)));
            self.focused.try_revise(|old| *old = Some(target))?;
            self.intent.set(Some(target));
            self.dirty.touch()?;
        }
        Ok(())
    }
}
pub struct Heatmap {
    common: Common,
    key: String,
    range: Option<Vary<HeatmapRange>>,
    data: Vary<HeatmapData>,
    domain: Vary<Option<HeatmapDomain>>,
    locale: Vary<DateLocale>,
    first: Weekday,
    colors: Vec<Color>,
    empty: Option<Color>,
    cell_size: f64,
    gap: f64,
    radius: f64,
    weekdays: bool,
    months: bool,
    legend: bool,
    tooltips: bool,
    weekday_labels: Option<[String; 7]>,
    month_labels: Option<[String; 12]>,
    label: Option<Label>,
    cell_style: Option<CellStyle>,
    clicked: Option<Box<dyn FnMut(HeatmapCell)>>,
}
pub fn heatmap(key: impl Into<String>) -> Heatmap {
    Heatmap {
        common: Common::default(),
        key: key.into(),
        range: None,
        data: HeatmapData::default().into(),
        domain: None.into(),
        locale: DateLocale::En.into(),
        first: Weekday::Sunday,
        colors: vec![],
        empty: None,
        cell_size: 12.,
        gap: 3.,
        radius: 2.,
        weekdays: false,
        months: false,
        legend: false,
        tooltips: false,
        weekday_labels: None,
        month_labels: None,
        label: None,
        cell_style: None,
        clicked: None,
    }
}
impl Heatmap {
    common_methods!();
    pub fn range(mut self, value: impl Into<Vary<HeatmapRange>>) -> Self {
        self.range = Some(value.into());
        self
    }
    pub fn data(mut self, value: impl Into<Vary<HeatmapData>>) -> Self {
        self.data = value.into();
        self
    }
    pub fn domain(mut self, value: impl Into<Vary<Option<HeatmapDomain>>>) -> Self {
        self.domain = value.into();
        self
    }
    pub fn locale(mut self, value: impl Into<Vary<DateLocale>>) -> Self {
        self.locale = value.into();
        self
    }
    pub fn week_start(mut self, value: Weekday) -> Self {
        self.first = value;
        self
    }
    pub fn colors(mut self, value: impl IntoIterator<Item = Color>) -> Self {
        self.colors = value.into_iter().take(MAX_HEATMAP_COLORS + 1).collect();
        self
    }
    pub fn empty_color(mut self, value: Color) -> Self {
        self.empty = Some(value);
        self
    }
    pub fn cell_size(mut self, value: f64) -> Self {
        self.cell_size = value;
        self
    }
    pub fn gap(mut self, value: f64) -> Self {
        self.gap = value;
        self
    }
    pub fn radius(mut self, value: f64) -> Self {
        self.radius = value;
        self
    }
    pub fn with_weekday_labels(mut self, value: bool) -> Self {
        self.weekdays = value;
        self
    }
    pub fn with_month_labels(mut self, value: bool) -> Self {
        self.months = value;
        self
    }
    pub fn with_legend(mut self, value: bool) -> Self {
        self.legend = value;
        self
    }
    pub fn with_tooltip(mut self, value: bool) -> Self {
        self.tooltips = value;
        self
    }
    /// Sunday through Saturday, independent of the configured display order.
    pub fn weekday_labels(mut self, value: [String; 7]) -> Self {
        self.weekday_labels = Some(value);
        self
    }
    /// January through December.
    pub fn month_labels(mut self, value: [String; 12]) -> Self {
        self.month_labels = Some(value);
        self
    }
    pub fn label_format(mut self, value: impl Fn(HeatmapCell) -> String + 'static) -> Self {
        self.label = Some(Rc::new(value));
        self
    }
    pub fn cell_style(mut self, value: impl Fn(HeatmapCell) -> Style + 'static) -> Self {
        self.cell_style = Some(Rc::new(value));
        self
    }
    pub fn on_day_click(mut self, value: impl FnMut(HeatmapCell) + 'static) -> Self {
        self.clicked = Some(Box::new(value));
        self
    }
}
fn child(scope: &Scope, reference: Cage<Option<NodeId>>) -> Result<Scope> {
    scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)
}
fn tracks(weeks: usize, size: f64, gap: f64) -> Style {
    Style::new()
        .display(Display::Grid)
        .grid_columns(GridTracks::Tracks(vec![GridTrack::Px(size); weeks]))
        .gap(gap)
        .width(Length::Px(weeks as f64 * (size + gap) - gap))
}
impl Widget for Heatmap {
    fn build(&mut self, scope: &Scope) -> Result {
        let id = composite::id("heatmap", &self.key, "calendar", "days")?;
        if self.key.len() > MAX_HEATMAP_KEY_BYTES {
            return Err(invalid("heatmap keys are limited to 48 bytes"));
        }
        if self.colors.len() > MAX_HEATMAP_COLORS
            || !self.cell_size.is_finite()
            || !(8.0..=32.0).contains(&self.cell_size)
            || !self.gap.is_finite()
            || !(0.0..=8.0).contains(&self.gap)
            || !self.radius.is_finite()
            || !(0.0..=self.cell_size / 2.).contains(&self.radius)
        {
            return Err(invalid("invalid finite heatmap colors or cell geometry"));
        }
        for label in self
            .weekday_labels
            .iter()
            .flatten()
            .chain(self.month_labels.iter().flatten())
        {
            if label.is_empty() || label.len() > 32 || label.contains('\0') {
                return Err(invalid("heatmap axis labels require 1–32 UTF-8 bytes"));
            }
        }
        let range = self
            .range
            .take()
            .ok_or(invalid("heatmap requires an explicit civil date range"))?;
        let range_source = range.clone();
        let data = self.data.clone();
        let domain = self.domain.clone();
        let locale = self.locale.clone();
        let label = self.label.clone();
        let read = move || -> Result<Rc<Model>> {
            // This private type is already bounded. Clone its current snapshot
            // once, then run the application's formatter outside its borrow.
            admit(
                range_source.try_get()?,
                data.try_get()?,
                domain.try_get()?,
                locale.try_get()?,
                &label,
            )
        };
        let model = scope.cage(glory::core::untracked(&read)?);
        let weak = scope.downgrade();
        scope.structural(move || {
            let Some(scope) = weak.upgrade() else { return };
            match read() {
                Ok(next) => {
                    if glory::core::untracked(|| model.with(|old| **old != *next)) {
                        model.revise(|old| *old = next)
                    }
                }
                Err(error) => scope.report_error(error),
            }
        });
        let first = self.first;
        let state = Rc::new(State {
            weak: scope.downgrade(),
            model,
            range,
            data: self.data.clone(),
            enabled: self.common.enabled.clone(),
            first,
            references: RefCell::new(BTreeMap::new()),
            focused: scope.cage(Some(model.with(|m| m.range.start()))),
            physical: scope.cage(None),
            hint: self.tooltips.then(|| scope.cage(None)),
            cursor: Cell::new(None),
            intent: Cell::new(None),
            dirty: Dirty::new(scope),
            clicked: Callback::new(self.clicked.take()),
            activate_observers: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Activate)
                    .unwrap_or_default(),
            ),
            navigation_observers: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::GridNavigate)
                    .unwrap_or_default(),
            ),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = cleanup.upgrade() {
                state.clicked.clear();
                state.activate_observers.clear();
                state.navigation_observers.clear();
                state.references.borrow_mut().clear();
                state.cursor.set(None);
                state.intent.set(None)
            }
        })?;
        let theme = self.common.theme.clone();
        let colors = Rc::new(if self.colors.is_empty() {
            vec![
                theme.accent_soft,
                theme.shades[4],
                theme.shades[6],
                theme.accent_text,
            ]
        } else {
            std::mem::take(&mut self.colors)
        });
        let size = self.cell_size;
        let gap = self.gap;
        let radius = self.radius;
        let empty = self.empty.unwrap_or(theme.surface);
        let themed = !self.common.unstyled;
        let tooltips = self.tooltips;
        let config = Rc::new(DayConfig {
            key: self.key.clone(),
            colors: colors.clone(),
            empty,
            size,
            radius,
            theme: theme.clone(),
            themed,
            tooltips,
            style: self.cell_style.clone(),
        });
        let root = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let layout = Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .gap(8.)
            .width(Length::Percent(100.))
            .min_width(Length::Px(0.))
            .max_width(Limit::Percent(100.));
        let base = Style {
            color: Some(theme.foreground),
            background: Some(theme.surface),
            ..layout.clone()
        };
        let (classes, appearance) = self.common.appearance(scope, "sapid-heatmap", base);
        let mut outer = glory::ui::view()
            .role(Role::Group)
            .focus(self.common.focus)
            .class(classes)
            .style(scope.bond(move || crate::appearance::merge(layout.clone(), appearance.get())))
            .node_ref(root);
        if let Some(name) = self.common.name.take() {
            outer = outer.name(name)
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            outer = outer.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        outer.show_in(scope)?;
        let area = child(scope, root)?;
        let scroller = scope.cage(None);
        glory::ui::view()
            .class("sapid-heatmap-scroll")
            .node_ref(scroller)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .min_width(Length::Px(0.))
                    .overflow_x(Overflow::Auto),
            )
            .show_in(&area)?;
        let viewport = child(scope, scroller)?;
        let chart = scope.cage(None);
        glory::ui::view()
            .node_ref(chart)
            .style(
                Style {
                    align: Some(Align::Start),
                    ..Style::new()
                }
                .display(Display::Flex)
                .gap(6.),
            )
            .show_in(&viewport)?;
        let chart_scope = child(scope, chart)?;
        if self.weekdays {
            let labels = scope.cage(None);
            let custom = self.weekday_labels.clone();
            let pad = if self.months { 20. } else { 0. };
            glory::ui::view()
                .class("sapid-heatmap-weekdays")
                .node_ref(labels)
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_direction(FlexDirection::Column)
                        .gap(gap)
                        .padding(Edges {
                            top: pad,
                            bottom: 0.,
                            left: 0.,
                            right: 0.,
                        }),
                )
                .show_in(&chart_scope)?;
            let labels_scope = child(scope, labels)?;
            for row in 0..7 {
                let day = Weekday::ALL[(first.index() as usize + row) % 7];
                let custom = custom.clone();
                glory::ui::text()
                    .content(scope.bond(move || {
                        custom.as_ref().map_or_else(
                            || model.with(|m| m.locale.weekday_short(day).to_owned()),
                            |labels| labels[(day.index() as usize + 1) % 7].clone(),
                        )
                    }))
                    .style(
                        Style {
                            font_size: Some(11.),
                            line_height: Some(size),
                            ..Style::new()
                        }
                        .height(Length::Px(size))
                        .white_space(WhiteSpace::NoWrap),
                    )
                    .show_in(&labels_scope)?;
            }
            scope.retain(labels_scope)?;
        }
        let body = scope.cage(None);
        glory::ui::view()
            .node_ref(body)
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column),
            )
            .show_in(&chart_scope)?;
        let body_scope = child(scope, body)?;
        if self.months {
            let header = scope.cage(None);
            glory::ui::view()
                .class("sapid-heatmap-months")
                .node_ref(header)
                .style(scope.bond(move || {
                    model.with(|m| tracks(m.range.weeks(first), size, gap).height(Length::Px(20.)))
                }))
                .show_in(&body_scope)?;
            let header_scope = child(scope, header)?;
            let columns =
                scope.bond(move || model.with(|m| (0..m.range.weeks(first)).collect::<Vec<_>>()));
            let custom = self.month_labels.clone();
            glory::ui::each(columns)
                .key(|column| *column)
                .children(move |entry| {
                    let col = entry.value;
                    let custom = custom.clone();
                    glory::ui::text()
                        .content(col.map(move |col| {
                            model.with(|m| {
                                let date = match (0..7).find_map(|row| m.range.at(*col, row, first))
                                {
                                    Some(date) => date,
                                    None => return String::new(),
                                };
                                let previous = col.checked_sub(1).and_then(|previous| {
                                    (0..7).find_map(|row| m.range.at(previous, row, first))
                                });
                                if previous.is_some_and(|previous| previous.month() == date.month())
                                {
                                    String::new()
                                } else {
                                    custom.as_ref().map_or_else(
                                        || match m.locale {
                                            DateLocale::En => {
                                                m.locale.month_name(date.year_month())[..3]
                                                    .to_owned()
                                            }
                                            DateLocale::ZhCn => format!("{}月", date.month()),
                                        },
                                        |labels| labels[date.month() as usize - 1].clone(),
                                    )
                                }
                            })
                        }))
                        .style(
                            Style {
                                font_size: Some(11.),
                                ..Style::new()
                            }
                            .white_space(WhiteSpace::NoWrap),
                        )
                })
                .show_in(&header_scope)?;
            scope.retain(header_scope)?;
        }
        let grid = scope.cage(None);
        let commands = state.clone();
        glory::ui::view()
            .role(Role::Grid)
            .name("Calendar contributions")
            .class("sapid-heatmap-grid")
            .node_ref(grid)
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    grid_navigation: Some(glory::render::Direction::Ltr),
                    table: Some(TableMetadata::Table {
                        rows: Some(7),
                        columns: Some(model.with(|m| m.range.weeks(first)) as u16),
                    }),
                    ..Default::default()
                })
            }))
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(gap),
            )
            .on(EventKind::GridNavigate, move |event| {
                if let Event::GridNavigate { from, command } = &event {
                    match commands.enabled.try_get() {
                        Ok(false) => return,
                        Err(error) => {
                            commands.report(Err(error.into()));
                            return;
                        }
                        Ok(true) => {}
                    }
                    if let Err(error) = commands.navigate(*from, *command) {
                        commands.report(Err(error));
                        return;
                    }
                    if let Some(scope) = commands.weak.upgrade() {
                        commands.navigation_observers.emit(scope.id(), event);
                    }
                }
            })
            .show_in(&body_scope)?;
        let grid_scope = child(scope, grid)?;
        for row in 0..7 {
            let reference = scope.cage(None);
            glory::ui::view()
                .role(Role::Row)
                .node_ref(reference)
                .style(scope.bond(move || {
                    model.with(|m| tracks(m.range.weeks(first), size, gap).height(Length::Px(size)))
                }))
                .show_in(&grid_scope)?;
            let row_scope = child(scope, reference)?;
            let dates = scope.bond(move || {
                model.with(|m| {
                    (0..m.range.len())
                        .filter_map(|i| m.range.date(i))
                        .filter(|date| date.weekday().index_from(first) as usize == row)
                        .collect::<Vec<_>>()
                })
            });
            let state = state.clone();
            let config = config.clone();
            glory::ui::each(dates)
                .key(|date| date.ordinal())
                .children(move |entry| Day {
                    date: entry.value.get_untracked(),
                    state: state.clone(),
                    config: config.clone(),
                })
                .show_in(&row_scope)?;
            scope.retain(row_scope)?;
        }
        scope.retain(grid_scope)?;
        scope.retain(body_scope)?;
        scope.retain(chart_scope)?;
        scope.retain(viewport)?;
        if self.legend {
            let legend = scope.cage(None);
            glory::ui::view()
                .class("sapid-heatmap-legend")
                .node_ref(legend)
                .style(
                    Style {
                        align: Some(Align::Center),
                        ..Style::new()
                    }
                    .display(Display::Flex)
                    .gap(4.),
                )
                .show_in(&area)?;
            let legend_scope = child(scope, legend)?;
            glory::ui::text()
                .content(scope.bond(move || model.with(|m| format!("{}", m.domain.min()))))
                .style(Style {
                    font_size: Some(11.),
                    ..Style::new()
                })
                .show_in(&legend_scope)?;
            for color in colors.iter() {
                glory::ui::graphic()
                    .drawing(crate::chart_aux::swatch(*color))
                    .style(Style::new().width(Length::Px(12.)).height(Length::Px(12.)))
                    .show_in(&legend_scope)?;
            }
            glory::ui::text()
                .content(scope.bond(move || model.with(|m| format!("{}", m.domain.max()))))
                .style(Style {
                    font_size: Some(11.),
                    ..Style::new()
                })
                .show_in(&legend_scope)?;
            scope.retain(legend_scope)?;
        }
        let weak = Rc::downgrade(&state);
        scope.effect(move || {
            let Some(state) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let edited = state.dirty.take();
                let range = model.with(|m| m.range);
                if let Some(hint) = state.hint
                    && hint
                        .try_get()?
                        .is_some_and(|date| !range.contains(date) || !state.enabled.get())
                {
                    hint.try_revise(|current| *current = None)?;
                }
                if state.focused.get().is_none_or(|date| !range.contains(date)) {
                    state.focused.try_revise(|old| *old = Some(range.start()))?;
                }
                if edited {
                    state.cursor.set(None)
                }
                if let Some(date) = state.intent.take()
                    && range.contains(date)
                    && let Some(scope) = state.weak.upgrade()
                {
                    let target = state
                        .references
                        .borrow()
                        .get(&date)
                        .and_then(|reference| reference.get());
                    composite::focus_node(&scope, target)?;
                    if let Some(target) = target
                        && scope.capabilities()?.scroll
                    {
                        scope.action(glory::render::Action::ScrollIntoView(target))?;
                    }
                }
                Ok(())
            })();
            state.report(result)
        });
        scope.retain(area)?;
        Ok(())
    }
}
struct DayConfig {
    key: String,
    colors: Rc<Vec<Color>>,
    empty: Color,
    size: f64,
    radius: f64,
    theme: crate::Theme,
    themed: bool,
    tooltips: bool,
    style: Option<CellStyle>,
}
struct Day {
    date: CivilDate,
    state: Rc<State>,
    config: Rc<DayConfig>,
}
impl Widget for Day {
    fn build(&mut self, scope: &Scope) -> Result {
        let date = self.date;
        let state = self.state.clone();
        let model = state.model;
        let first = state.first;
        let size = self.config.size;
        let cell = scope.cage(None);
        glory::ui::view()
            .role(Role::GridCell)
            .class("sapid-heatmap-cell")
            .node_ref(cell)
            .style(scope.bond(move || {
                model.with(|m| {
                    let Some((col, _)) = m.range.position(date, first) else {
                        return Style::new().display(Display::None);
                    };
                    Style::new()
                        .grid_column(GridPlacement::At {
                            start: col as u16 + 1,
                            span: 1,
                        })
                        .grid_row(GridPlacement::At { start: 1, span: 1 })
                        .width(Length::Px(size))
                        .height(Length::Px(size))
                })
            }))
            .semantics(scope.bond(move || {
                model.with(|m| {
                    let (col, row) = m.range.position(date, first)?;
                    composite::metadata(Semantics {
                        table: Some(TableMetadata::Cell {
                            row: row as u32,
                            column: col as u16,
                            row_span: 1,
                            column_span: 1,
                            headers: vec![],
                        }),
                        ..Default::default()
                    })
                })
            }))
            .show_in(scope)?;
        let area = child(scope, cell)?;
        let reference = scope.cage(None);
        state.references.borrow_mut().insert(date, reference);
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                if let (Ok(Some(current)), Ok(Some(node))) =
                    (state.physical.try_get(), reference.try_get())
                    && current == node
                {
                    let _ = state.physical.try_revise(|old| *old = None);
                }
                state.references.borrow_mut().remove(&date);
                if let Some(hint) = state.hint
                    && hint.try_get().ok() == Some(Some(date))
                {
                    let _ = hint.try_revise(|current| *current = None);
                }
            }
        })?;
        if self.config.tooltips {
            let state = self.state.clone();
            let config = self.config.clone();
            let visibility = self
                .state
                .hint
                .ok_or(invalid("missing heatmap tooltip state"))?;
            let controller = self.state.clone();
            crate::tooltip(format!("{}-{}", self.config.key, date))
                .theme(self.config.theme.clone())
                .text(
                    scope.bond(move || {
                        model.with(|m| m.labels.get(&date).cloned().unwrap_or_default())
                    }),
                )
                .enabled(self.state.enabled.clone())
                .open(scope.bond(move || visibility.get() == Some(date)))
                .on_open_change(move |open| controller.report(controller.hint(date, open)))
                .trigger_ref(reference)
                .target(move |target| DayButton {
                    date,
                    state: state.clone(),
                    config: config.clone(),
                    reference,
                    target: Some(target),
                })
                .show_in(&area)?;
        } else {
            DayButton {
                date,
                state: self.state.clone(),
                config: self.config.clone(),
                reference,
                target: None,
            }
            .show_in(&area)?;
        }
        scope.retain(area)
    }
}
struct DayButton {
    date: CivilDate,
    state: Rc<State>,
    config: Rc<DayConfig>,
    reference: Cage<Option<NodeId>>,
    target: Option<crate::FloatingTarget>,
}
impl Widget for DayButton {
    fn build(&mut self, scope: &Scope) -> Result {
        let date = self.date;
        let state = self.state.clone();
        let model = state.model;
        let focused = state.focused;
        let physical = state.physical;
        let config = self.config.clone();
        let reference = self.reference;
        let activate = state.clone();
        let focus = state.clone();
        let available = state.enabled.clone();
        let mut button = glory::ui::button()
            .name(
                scope
                    .bond(move || model.with(|m| m.labels.get(&date).cloned().unwrap_or_default())),
            )
            .class("sapid-heatmap-day")
            .node_ref(reference)
            .enabled(available)
            .focus(scope.bond(move || {
                if focused.get() == Some(date) {
                    FocusMode::TabStop
                } else {
                    FocusMode::Programmatic
                }
            }))
            .style(scope.bond(move || {
                model.with(|m| {
                    let value = m.data.value(date);
                    let color = value
                        .and_then(|value| m.domain.level(value, config.colors.len()))
                        .map_or(config.empty, |level| config.colors[level]);
                    let mut style = Style::new()
                        .display(Display::Block)
                        .width(Length::Px(config.size))
                        .height(Length::Px(config.size))
                        .min_width(Length::Px(config.size))
                        .min_height(Length::Px(config.size))
                        .padding(Edges::all(0.))
                        .radius(config.radius);
                    style.background = Some(color);
                    if config.themed {
                        style = style
                            .border_width(Edges::all(1.))
                            .border_style(BorderStyle::Solid)
                            .border_color(
                                if reference.get().is_some_and(|id| physical.get() == Some(id)) {
                                    config.theme.accent
                                } else {
                                    config.theme.border
                                },
                            );
                    }
                    if let Some(custom) = &config.style {
                        style =
                            crate::appearance::merge(style, custom(HeatmapCell { date, value }));
                    }
                    style
                })
            }))
            .on(EventKind::Activate, move |event| {
                activate.report(activate.activate(date, event))
            })
            .on(EventKind::FocusChanged, move |event| {
                if let Event::FocusChanged(active) = event {
                    let result = (|| -> Result {
                        if active {
                            focus.cursor.set(None);
                            focused.try_revise(|old| *old = Some(date))?;
                            physical.try_revise(|old| *old = reference.get())?;
                        } else if physical.get() == reference.get() {
                            physical.try_revise(|old| *old = None)?;
                        }
                        Ok(())
                    })();
                    focus.report(result);
                }
            });
        if let Some(target) = self.target.take() {
            button = target.button(button)
        }
        button.show_in(scope)
    }
}
