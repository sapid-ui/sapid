//! Portable finite geometry for Cartesian charts. Every layer is one existing
//! validated Graphic, shared by HTML/SVG and native rendering.
use crate::chart_model::*;
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, PathCommand as Cmd, StrokeCap,
};
use glory::style::Color;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CartesianKind {
    #[default]
    Line,
    Area,
    Bar,
    Composite,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartFrame {
    pub width: f64,
    pub height: f64,
}
impl Default for ChartFrame {
    fn default() -> Self {
        Self {
            width: 640.,
            height: 320.,
        }
    }
}
impl ChartFrame {
    pub fn validate(self) -> Result<(), ChartError> {
        if !self.width.is_finite()
            || !self.height.is_finite()
            || !(160.0..=16384.0).contains(&self.width)
            || !(120.0..=16384.0).contains(&self.height)
        {
            return Err(ChartError::Geometry);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartTheme {
    pub series: [Color; MAX_CHART_SERIES],
    pub grid: Color,
}
impl ChartTheme {
    pub fn from_theme(theme: &crate::Theme) -> Self {
        Self {
            series: theme.series,
            grid: theme.border,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CartesianOptions {
    pub kind: CartesianKind,
    pub stacked: bool,
    pub hidden: BTreeSet<String>,
    pub brush: Option<ChartBrush>,
    pub domain: Option<ChartDomain>,
    pub reference_lines: Vec<ChartReferenceLine>,
    pub reference_areas: Vec<ChartReferenceArea>,
    pub reference_dots: Vec<ChartReferenceDot>,
}
pub const MAX_CHART_REFERENCES: usize = 16;
#[derive(Clone, Debug, PartialEq)]
pub enum ChartReferenceAxis {
    Category(String),
    Value(f64),
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartReferenceLine {
    pub axis: ChartReferenceAxis,
    pub label: Option<String>,
    pub color: Option<Color>,
}
impl ChartReferenceLine {
    pub fn category(key: impl Into<String>) -> Self {
        Self {
            axis: ChartReferenceAxis::Category(key.into()),
            label: None,
            color: None,
        }
    }
    pub fn value(value: f64) -> Self {
        Self {
            axis: ChartReferenceAxis::Value(value),
            label: None,
            color: None,
        }
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartReferenceArea {
    pub categories: Option<(String, String)>,
    pub values: Option<(f64, f64)>,
    pub label: Option<String>,
    pub color: Option<Color>,
}
impl ChartReferenceArea {
    pub fn new() -> Self {
        Self {
            categories: None,
            values: None,
            label: None,
            color: None,
        }
    }
    pub fn categories(mut self, first: impl Into<String>, last: impl Into<String>) -> Self {
        self.categories = Some((first.into(), last.into()));
        self
    }
    pub fn values(mut self, min: f64, max: f64) -> Self {
        self.values = Some((min, max));
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
}
impl Default for ChartReferenceArea {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartReferenceDot {
    pub category: String,
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<Color>,
}
impl ChartReferenceDot {
    pub fn new(category: impl Into<String>, value: f64) -> Self {
        Self {
            category: category.into(),
            value,
            label: None,
            color: None,
        }
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartPath {
    pub key: String,
    pub series: Option<usize>,
    pub drawing: GraphicProps,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartLocation {
    pub series: usize,
    pub category: usize,
    pub value: f64,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartTick {
    pub value: f64,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartLabel {
    pub category: usize,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChartReferenceLabel {
    pub key: String,
    pub text: String,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CartesianScene {
    pub frame: ChartFrame,
    pub domain: ChartDomain,
    pub paths: Vec<ChartPath>,
    pub locations: Vec<ChartLocation>,
    pub ticks: Vec<ChartTick>,
    pub labels: Vec<ChartLabel>,
    pub reference_labels: Vec<ChartReferenceLabel>,
    pub clipped_values: usize,
}
fn path(
    frame: ChartFrame,
    key: String,
    series: Option<usize>,
    commands: Vec<Cmd>,
    fill: Option<Color>,
    stroke: Option<GraphicStroke>,
) -> Result<ChartPath, ChartError> {
    let drawing = GraphicProps {
        width: frame.width,
        height: frame.height,
        commands,
        fill,
        stroke,
        ..Default::default()
    };
    drawing.validate().map_err(|_| ChartError::Geometry)?;
    Ok(ChartPath {
        key,
        series,
        drawing,
    })
}
fn stroke(color: Color, width: f64) -> GraphicStroke {
    GraphicStroke {
        color,
        width,
        cap: StrokeCap::Round,
    }
}
fn rectangle(x: f64, y: f64, width: f64, height: f64) -> [Cmd; 5] {
    [
        Cmd::Move(Point::new(x, y)),
        Cmd::Line(Point::new(x + width, y)),
        Cmd::Line(Point::new(x + width, y + height)),
        Cmd::Line(Point::new(x, y + height)),
        Cmd::Close,
    ]
}
impl CartesianScene {
    /// Export the finite drawing layers. Labels and exact values are exported
    /// separately in `ticks`, `labels` and the widget's intrinsic data table.
    /// This geometry export does not claim a captured browser/native frame.
    pub fn to_svg(&self) -> Result<String, ChartError> {
        use std::fmt::Write;
        self.frame.validate()?;
        if self.paths.len() > MAX_CHART_SERIES * 3 + 1 {
            return Err(ChartError::Budget);
        }
        for path in &self.paths {
            path.drawing.validate().map_err(|_| ChartError::Geometry)?;
            if path.drawing.width != self.frame.width
                || path.drawing.height != self.frame.height
                || path.drawing.animation.is_some()
                || path.drawing.transition.is_some()
            {
                return Err(ChartError::Geometry);
            }
        }
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">",
            self.frame.width, self.frame.height
        );
        let color = |c: Color| format!("rgba({},{},{},{})", c.0, c.1, c.2, f64::from(c.3) / 255.);
        for path in &self.paths {
            svg.push_str("<path d=\"");
            for command in &path.drawing.commands {
                match command {
                    Cmd::Move(p) => write!(svg, "M{} {}", p.x, p.y),
                    Cmd::Line(p) => write!(svg, "L{} {}", p.x, p.y),
                    Cmd::Cubic { first, second, end } => write!(
                        svg,
                        "C{} {} {} {} {} {}",
                        first.x, first.y, second.x, second.y, end.x, end.y
                    ),
                    Cmd::Close => write!(svg, "Z"),
                }
                .expect("String writing");
            }
            write!(
                svg,
                "\" fill=\"{}\"",
                path.drawing.fill.map_or_else(|| "none".into(), color)
            )
            .expect("String writing");
            if let Some(stroke) = path.drawing.stroke {
                let cap = match stroke.cap {
                    StrokeCap::Butt => "butt",
                    StrokeCap::Round => "round",
                    StrokeCap::Square => "square",
                };
                write!(
                    svg,
                    " stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"{}\"",
                    color(stroke.color),
                    stroke.width,
                    cap
                )
                .expect("String writing");
            }
            svg.push_str("/>");
        }
        svg.push_str("</svg>");
        Ok(svg)
    }
    pub fn build(
        data: &ChartData,
        frame: ChartFrame,
        options: &CartesianOptions,
        theme: &ChartTheme,
    ) -> Result<Self, ChartError> {
        frame.validate()?;
        data.validate()?;
        // Validate hidden keys even when the caller supplies an explicit domain.
        let natural = data.domain(options.stacked, &options.hidden)?;
        let domain = options.domain.unwrap_or(natural);
        if let Some(brush) = options.brush {
            brush.validate(data.categories.len())?;
        }
        let first = options.brush.map_or(0, |brush| brush.first);
        let end = options
            .brush
            .map_or(data.categories.len(), |brush| brush.last + 1);
        let count = end - first;
        let left = 52.;
        let top = 12.;
        let right = frame.width - 16.;
        let bottom = frame.height - 36.;
        let y = |value: f64| -> Result<f64, ChartError> {
            Ok(bottom - domain.fraction(value)?.clamp(0., 1.) * (bottom - top))
        };
        let band = (right - left) / count.max(1) as f64;
        let x = |index: usize| left + (index - first) as f64 * band + band / 2.;
        let reference_count = options.reference_lines.len()
            + options.reference_areas.len()
            + options.reference_dots.len();
        if reference_count > MAX_CHART_REFERENCES {
            return Err(ChartError::Budget);
        }
        let category = |key: &str| {
            data.categories
                .iter()
                .position(|entry| entry.key == key)
                .ok_or(ChartError::Key)
        };
        let valid_label = |value: &Option<String>| {
            if value.as_ref().is_some_and(|value| !label(value)) {
                Err(ChartError::Label)
            } else {
                Ok(())
            }
        };
        let ticks = domain
            .ticks(5)?
            .into_iter()
            .map(|value| {
                Ok(ChartTick {
                    value,
                    x: left - 6.,
                    y: y(value)?,
                })
            })
            .collect::<Result<Vec<_>, ChartError>>()?;
        let mut grid = Vec::new();
        for tick in &ticks {
            grid.extend([
                Cmd::Move(Point::new(left, tick.y)),
                Cmd::Line(Point::new(right, tick.y)),
            ]);
        }
        let mut paths = vec![path(
            frame,
            "grid".into(),
            None,
            grid,
            None,
            Some(stroke(theme.grid, 1.)),
        )?];
        let mut reference_labels = Vec::new();
        for (index, area) in options.reference_areas.iter().enumerate() {
            valid_label(&area.label)?;
            if area.categories.is_none() && area.values.is_none() {
                return Err(ChartError::Schema);
            }
            let (x1, x2) = if let Some((start, finish)) = &area.categories {
                let start = category(start)?;
                let finish = category(finish)?;
                if start > finish {
                    return Err(ChartError::Schema);
                }
                let start = start.max(first);
                let finish = finish.min(end.saturating_sub(1));
                if start > finish {
                    continue;
                }
                (x(start) - band / 2., x(finish) + band / 2.)
            } else {
                (left, right)
            };
            let (y1, y2) = if let Some((min, max)) = area.values {
                if !min.is_finite()
                    || !max.is_finite()
                    || min.abs() > MAX_CHART_VALUE
                    || max.abs() > MAX_CHART_VALUE
                    || min > max
                {
                    return Err(ChartError::Values);
                }
                (y(max)?, y(min)?)
            } else {
                (top, bottom)
            };
            let mut color = area.color.unwrap_or(theme.series[0]);
            color.3 = color.3.min(64);
            paths.push(path(
                frame,
                format!("reference-area-{index}"),
                None,
                rectangle(x1, y1, x2 - x1, y2 - y1).to_vec(),
                Some(color),
                None,
            )?);
            if let Some(text) = &area.label {
                reference_labels.push(ChartReferenceLabel {
                    key: format!("area-{index}"),
                    text: text.clone(),
                    x: x1 + 4.,
                    y: y1 + 4.,
                });
            }
        }
        for (index, line) in options.reference_lines.iter().enumerate() {
            valid_label(&line.label)?;
            let (commands, label_x, label_y) = match &line.axis {
                ChartReferenceAxis::Category(key) => {
                    let category = category(key)?;
                    if category < first || category >= end {
                        continue;
                    }
                    let x = x(category);
                    (
                        vec![
                            Cmd::Move(Point::new(x, top)),
                            Cmd::Line(Point::new(x, bottom)),
                        ],
                        x + 4.,
                        top + 4.,
                    )
                }
                ChartReferenceAxis::Value(value) => {
                    if !value.is_finite() || value.abs() > MAX_CHART_VALUE {
                        return Err(ChartError::Values);
                    }
                    let y = y(*value)?;
                    (
                        vec![
                            Cmd::Move(Point::new(left, y)),
                            Cmd::Line(Point::new(right, y)),
                        ],
                        left + 4.,
                        y + 4.,
                    )
                }
            };
            paths.push(path(
                frame,
                format!("reference-line-{index}"),
                None,
                commands,
                None,
                Some(stroke(line.color.unwrap_or(theme.grid), 2.)),
            )?);
            if let Some(text) = &line.label {
                reference_labels.push(ChartReferenceLabel {
                    key: format!("line-{index}"),
                    text: text.clone(),
                    x: label_x,
                    y: label_y,
                });
            }
        }
        let mut reference_dot_paths = Vec::new();
        for (index, dot) in options.reference_dots.iter().enumerate() {
            valid_label(&dot.label)?;
            let category = category(&dot.category)?;
            if !dot.value.is_finite() || dot.value.abs() > MAX_CHART_VALUE {
                return Err(ChartError::Values);
            }
            if category < first || category >= end {
                continue;
            }
            let x = x(category);
            let y = y(dot.value)?;
            let color = dot.color.unwrap_or(theme.series[0]);
            reference_dot_paths.push(path(
                frame,
                format!("reference-dot-{index}"),
                None,
                rectangle(x - 4., y - 4., 8., 8.).to_vec(),
                Some(color),
                None,
            )?);
            if let Some(text) = &dot.label {
                reference_labels.push(ChartReferenceLabel {
                    key: format!("dot-{index}"),
                    text: text.clone(),
                    x: x + 6.,
                    y: y - 10.,
                });
            }
        }
        let labels = (first..end)
            .filter(|index| {
                (*index - first).is_multiple_of(count.div_ceil(8).max(1)) || *index + 1 == end
            })
            .map(|category| ChartLabel {
                category,
                x: x(category),
                y: bottom + 8.,
            })
            .collect();
        let mut locations = Vec::new();
        let mut clipped_values = 0;
        let mut positive = vec![0.; data.categories.len()];
        let mut negative = positive.clone();
        let visible = data
            .series
            .iter()
            .enumerate()
            .filter(|(_, series)| !options.hidden.contains(&series.key))
            .collect::<Vec<_>>();
        let bar_count = visible
            .iter()
            .filter(|(_, series)| match options.kind {
                CartesianKind::Bar => true,
                CartesianKind::Composite => series.kind == ChartSeriesKind::Bar,
                _ => false,
            })
            .count()
            .max(1);
        let mut bar_index = 0;
        for (series_index, series) in visible {
            let kind = match options.kind {
                CartesianKind::Line => ChartSeriesKind::Line,
                CartesianKind::Area => ChartSeriesKind::Area,
                CartesianKind::Bar => ChartSeriesKind::Bar,
                CartesianKind::Composite => series.kind,
            };
            // Signed stacks are well-defined for bars. Filled continuous areas
            // can cross zero only on an unstacked baseline in this first model.
            if options.stacked
                && kind == ChartSeriesKind::Area
                && series.values.iter().flatten().any(|v| *v < 0.)
            {
                return Err(ChartError::SignedAreaStack);
            }
            let mut commands = Vec::new();
            let mut batches = Vec::new();
            let mut run = Vec::new();
            let mut spans = Vec::new();
            for category in first..end {
                let Some(value) = series.values[category] else {
                    if !run.is_empty() {
                        spans.push(std::mem::take(&mut run));
                    }
                    continue;
                };
                let baseline = if options.stacked {
                    let slot = if value >= 0. {
                        &mut positive[category]
                    } else {
                        &mut negative[category]
                    };
                    let old = *slot;
                    *slot += value;
                    old
                } else {
                    0.
                };
                let total = baseline + value;
                let py = y(total)?;
                let px = x(category);
                if total < domain.min() || total > domain.max() {
                    clipped_values += 1;
                }
                let location = ChartLocation {
                    series: series_index,
                    category,
                    value,
                    x: px,
                    y: py,
                };
                if kind == ChartSeriesKind::Bar {
                    let width = band * 0.72
                        / if options.stacked {
                            1.
                        } else {
                            bar_count as f64
                        };
                    let bx = if options.stacked {
                        px - width / 2.
                    } else {
                        px - band * 0.36 + bar_index as f64 * width
                    };
                    locations.push(ChartLocation {
                        x: bx + width / 2.,
                        ..location
                    });
                    if commands.len() + 5 > 256 {
                        batches.push(std::mem::take(&mut commands));
                    }
                    commands.extend(rectangle(
                        bx,
                        py.min(y(baseline)?),
                        width,
                        (py - y(baseline)?).abs(),
                    ));
                } else {
                    locations.push(location);
                    run.push((px, py, y(baseline)?));
                }
            }
            if !run.is_empty() {
                spans.push(run);
            }
            for span in spans {
                let mut start = 0;
                loop {
                    let end = if kind == ChartSeriesKind::Area {
                        (start + 127).min(span.len())
                    } else {
                        span.len()
                    };
                    let slice = &span[start..end];
                    let size = if kind == ChartSeriesKind::Area {
                        slice.len() * 2 + 1
                    } else {
                        slice.len()
                    };
                    if commands.len() + size > 256 {
                        batches.push(std::mem::take(&mut commands));
                    }
                    commands.push(Cmd::Move(Point::new(slice[0].0, slice[0].1)));
                    commands.extend(
                        slice
                            .iter()
                            .skip(1)
                            .map(|(x, y, _)| Cmd::Line(Point::new(*x, *y))),
                    );
                    if kind == ChartSeriesKind::Area {
                        commands.extend(
                            slice
                                .iter()
                                .rev()
                                .map(|(x, _, baseline)| Cmd::Line(Point::new(*x, *baseline))),
                        );
                        commands.push(Cmd::Close);
                    }
                    if end == span.len() {
                        break;
                    }
                    start = end - 1;
                }
            }
            if !commands.is_empty() {
                batches.push(commands);
            }
            let mut fill = theme.series[series_index];
            if kind == ChartSeriesKind::Area {
                fill.3 = 96;
            }
            for (batch, commands) in batches.into_iter().enumerate() {
                paths.push(path(
                    frame,
                    format!("{}-{batch}", series.key),
                    Some(series_index),
                    commands,
                    (kind != ChartSeriesKind::Line).then_some(fill),
                    (kind != ChartSeriesKind::Bar)
                        .then_some(stroke(theme.series[series_index], 2.)),
                )?);
            }
            if kind == ChartSeriesKind::Bar {
                bar_index += 1;
            }
        }
        paths.extend(reference_dot_paths);
        Ok(Self {
            frame,
            domain,
            paths,
            locations,
            ticks,
            labels,
            reference_labels,
            clipped_values,
        })
    }
    pub fn nearest(&self, x: f64, y: f64) -> Option<ChartLocation> {
        if !x.is_finite() || !y.is_finite() || x.abs() > 1_000_000. || y.abs() > 1_000_000. {
            return None;
        }
        self.locations
            .iter()
            .min_by(|a, b| {
                let da = (a.x - x).powi(2) + (a.y - y).powi(2);
                let db = (b.x - x).powi(2) + (b.y - y).powi(2);
                da.total_cmp(&db)
            })
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn data(count: usize, series: usize) -> ChartData {
        ChartData {
            categories: (0..count)
                .map(|i| ChartCategory::new(format!("c{i}"), format!("Category {i}")))
                .collect(),
            series: (0..series)
                .map(|s| {
                    ChartSeries::new(
                        format!("s{s}"),
                        format!("Series {s}"),
                        (0..count).map(|i| Some((i as f64 - count as f64 / 2.) * (s + 1) as f64)),
                    )
                })
                .collect(),
        }
    }
    fn build(data: &ChartData, options: CartesianOptions) -> Result<CartesianScene, ChartError> {
        CartesianScene::build(
            data,
            ChartFrame::default(),
            &options,
            &ChartTheme::from_theme(&crate::Theme::default()),
        )
    }
    #[test]
    fn every_cartesian_mode_retains_finite_protocol_budgets_at_maximum_data() {
        let mut d = data(128, 8);
        for kind in [
            CartesianKind::Line,
            CartesianKind::Area,
            CartesianKind::Bar,
            CartesianKind::Composite,
        ] {
            for stacked in [false, true] {
                if stacked && kind == CartesianKind::Area {
                    for s in &mut d.series {
                        for v in s.values.iter_mut().flatten() {
                            *v = v.abs();
                        }
                    }
                }
                let scene = build(
                    &d,
                    CartesianOptions {
                        kind,
                        stacked,
                        ..Default::default()
                    },
                )
                .unwrap();
                assert_eq!(scene.locations.len(), 1024);
                assert!(scene.paths.len() <= 33);
                assert!(scene.labels.len() <= 9);
                for layer in scene.paths {
                    layer.drawing.validate().unwrap();
                    assert!(layer.drawing.commands.len() <= 256);
                }
            }
        }
    }
    #[test]
    fn gaps_split_line_and_filled_area_without_joining_missing_categories() {
        let mut d = data(5, 1);
        d.series[0].values = vec![Some(1.), Some(2.), None, Some(3.), Some(1.)];
        for kind in [CartesianKind::Line, CartesianKind::Area] {
            let scene = build(
                &d,
                CartesianOptions {
                    kind,
                    ..Default::default()
                },
            )
            .unwrap();
            let drawing = &scene.paths[1].drawing;
            assert_eq!(
                drawing
                    .commands
                    .iter()
                    .filter(|c| matches!(c, Cmd::Move(_)))
                    .count(),
                2
            );
            assert_eq!(
                drawing
                    .commands
                    .iter()
                    .filter(|c| matches!(c, Cmd::Close))
                    .count(),
                if kind == CartesianKind::Area { 2 } else { 0 }
            );
            assert_eq!(
                scene
                    .locations
                    .iter()
                    .map(|p| p.category)
                    .collect::<Vec<_>>(),
                [0, 1, 3, 4]
            );
        }
    }
    #[test]
    fn grouped_bar_hit_locations_match_their_actual_distinct_centers() {
        let d = data(1, 2);
        let scene = build(
            &d,
            CartesianOptions {
                kind: CartesianKind::Bar,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(scene.locations[0].x < scene.locations[1].x);
        for p in &scene.locations {
            assert_eq!(scene.nearest(p.x, p.y), Some(*p));
        }
        assert!(scene.nearest(f64::NAN, 0.).is_none());
        assert!(scene.nearest(1e7, 0.).is_none());
    }
    #[test]
    fn signed_bars_stack_separately_and_negative_area_stack_is_explicitly_rejected() {
        let mut d = data(1, 4);
        for (s, v) in d.series.iter_mut().zip([-2., -3., 4., 1.]) {
            s.values = vec![Some(v)];
        }
        let scene = build(
            &d,
            CartesianOptions {
                kind: CartesianKind::Bar,
                stacked: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(scene.domain, ChartDomain::new(-5., 5.).unwrap());
        assert!(scene.locations[1].y > scene.locations[0].y);
        assert!(scene.locations[3].y < scene.locations[2].y);
        assert_eq!(
            build(
                &d,
                CartesianOptions {
                    kind: CartesianKind::Area,
                    stacked: true,
                    ..Default::default()
                }
            ),
            Err(ChartError::SignedAreaStack)
        );
    }
    #[test]
    fn brush_changes_only_current_geometry_hidden_series_and_explicit_domain_are_validated() {
        let d = data(6, 2);
        let scene = build(
            &d,
            CartesianOptions {
                brush: Some(ChartBrush { first: 2, last: 3 }),
                hidden: BTreeSet::from(["s1".into()]),
                domain: Some(ChartDomain::new(0., 0.5).unwrap()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(scene.locations.len(), 2);
        assert_eq!(
            scene
                .locations
                .iter()
                .map(|p| p.category)
                .collect::<Vec<_>>(),
            [2, 3]
        );
        assert_eq!(scene.clipped_values, 1);
        assert!(scene.locations.iter().all(|p| (12. ..=284.).contains(&p.y)));
        assert_eq!(
            build(
                &d,
                CartesianOptions {
                    hidden: BTreeSet::from(["unknown".into()]),
                    domain: Some(ChartDomain::new(0., 1.).unwrap()),
                    ..Default::default()
                }
            ),
            Err(ChartError::Key)
        );
        assert_eq!(
            build(
                &d,
                CartesianOptions {
                    brush: Some(ChartBrush { first: 4, last: 6 }),
                    ..Default::default()
                }
            ),
            Err(ChartError::Brush)
        );
    }
    #[test]
    fn empty_data_and_finite_frame_rejection_never_generate_nan() {
        let empty = build(&ChartData::default(), CartesianOptions::default()).unwrap();
        assert!(empty.locations.is_empty() && empty.labels.is_empty());
        assert_eq!(empty.paths.len(), 1);
        for frame in [
            ChartFrame {
                width: f64::INFINITY,
                height: 320.,
            },
            ChartFrame {
                width: 159.,
                height: 320.,
            },
            ChartFrame {
                width: 640.,
                height: 119.,
            },
        ] {
            assert_eq!(
                CartesianScene::build(
                    &data(1, 1),
                    frame,
                    &CartesianOptions::default(),
                    &ChartTheme::from_theme(&crate::Theme::default())
                ),
                Err(ChartError::Geometry)
            );
        }
    }
    #[test]
    fn finite_reference_layers_validate_and_follow_the_current_brush() {
        let d = data(6, 1);
        let scene = build(
            &d,
            CartesianOptions {
                brush: Some(ChartBrush { first: 1, last: 3 }),
                reference_areas: vec![
                    ChartReferenceArea::new()
                        .categories("c1", "c3")
                        .values(0., 1.)
                        .label("Window"),
                ],
                reference_lines: vec![
                    ChartReferenceLine::category("c0").label("Outside"),
                    ChartReferenceLine::value(0.5).label("Target"),
                ],
                reference_dots: vec![ChartReferenceDot::new("c2", 0.5).label("Peak")],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            scene
                .paths
                .iter()
                .any(|path| path.key == "reference-area-0")
        );
        assert!(
            scene
                .paths
                .iter()
                .any(|path| path.key == "reference-line-1")
        );
        assert!(scene.paths.iter().any(|path| path.key == "reference-dot-0"));
        assert!(
            !scene
                .paths
                .iter()
                .any(|path| path.key == "reference-line-0")
        );
        assert_eq!(
            scene
                .reference_labels
                .iter()
                .map(|label| label.text.as_str())
                .collect::<Vec<_>>(),
            ["Window", "Target", "Peak"]
        );

        assert_eq!(
            build(
                &d,
                CartesianOptions {
                    reference_dots: vec![ChartReferenceDot::new("missing", 1.)],
                    ..Default::default()
                }
            ),
            Err(ChartError::Key)
        );
        assert_eq!(
            build(
                &d,
                CartesianOptions {
                    reference_lines: (0..=MAX_CHART_REFERENCES)
                        .map(|_| ChartReferenceLine::value(1.))
                        .collect(),
                    ..Default::default()
                }
            ),
            Err(ChartError::Budget)
        );
    }
    #[test]
    fn svg_export_validates_mutated_public_geometry_and_contains_no_business_markup() {
        let mut d = data(3, 1);
        d.series[0].label = "<script>untrusted</script>".into();
        let mut scene = build(&d, CartesianOptions::default()).unwrap();
        let svg = scene.to_svg().unwrap();
        assert!(svg.starts_with("<svg xmlns="));
        assert!(!svg.contains("script"));
        assert!(svg.contains("<path d=\"M"));
        scene.paths[0]
            .drawing
            .commands
            .push(Cmd::Line(Point::new(f64::NAN, 0.)));
        assert_eq!(scene.to_svg(), Err(ChartError::Geometry));
    }
}
