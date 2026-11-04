//! Read-only numeric progress, with shared finite line and Bezier views.
use crate::appearance::{Common, configure, merge};
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, NodeId, PathCommand as Path, ProgressValue,
    Role, Semantics, StrokeCap,
};
use glory::style::{Align, Color, Display, Justify, Length, Overflow, Position, Style};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
use std::f64::consts::{PI, TAU};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum View {
    Line,
    Ring,
    SemiCircle,
}
/// Direction of a linear progress indicator. Circular views have no axis and
/// reject vertical orientation instead of silently ignoring it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressOrientation {
    #[default]
    Horizontal,
    Vertical,
}
/// Which half of the circle a semicircle indicator occupies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SemiCircleOrientation {
    #[default]
    Up,
    Down,
}
/// Direction in which a semicircle value advances.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SemiCircleFillDirection {
    #[default]
    LeftToRight,
    RightToLeft,
}
/// Placement of the optional visible semicircle label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SemiCircleLabelPosition {
    #[default]
    Bottom,
    Center,
}
/// One finite colored portion of a compound line or ring progress indicator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressSection {
    pub value: f64,
    pub color: Color,
}
impl ProgressSection {
    pub const fn new(value: f64, color: Color) -> Self {
        Self { value, color }
    }
}
pub struct Progress {
    common: Common,
    view: View,
    value: Vary<f64>,
    indeterminate: Vary<bool>,
    value_text: Option<Vary<String>>,
    min: f64,
    max: f64,
    size: f64,
    thickness: f64,
    width: Option<Length>,
    height: Option<Length>,
    length: Option<Length>,
    orientation: ProgressOrientation,
    semi_orientation: SemiCircleOrientation,
    semi_fill_direction: SemiCircleFillDirection,
    semi_label_position: SemiCircleLabelPosition,
    label: Option<Vary<String>>,
    filled_segment_color: Option<Color>,
    empty_segment_color: Option<Color>,
    ring_start_angle: f64,
    ring_section_gap: f64,
    ring_round_caps: bool,
    ring_root_color: Option<Color>,
    sections: Vec<ProgressSection>,
}
fn create(view: View, value: impl Into<Vary<f64>>) -> Progress {
    let common = Common {
        name: Some("Progress".into()),
        ..Common::default()
    };
    Progress {
        common,
        view,
        value: value.into(),
        indeterminate: false.into(),
        value_text: None,
        min: 0.,
        max: 100.,
        size: 100.,
        thickness: 8.,
        width: None,
        height: None,
        length: None,
        orientation: ProgressOrientation::Horizontal,
        semi_orientation: SemiCircleOrientation::Up,
        semi_fill_direction: SemiCircleFillDirection::LeftToRight,
        semi_label_position: SemiCircleLabelPosition::Bottom,
        label: None,
        filled_segment_color: None,
        empty_segment_color: None,
        ring_start_angle: 270.,
        ring_section_gap: 0.,
        ring_round_caps: false,
        ring_root_color: None,
        sections: Vec::new(),
    }
}
pub fn progress(value: impl Into<Vary<f64>>) -> Progress {
    create(View::Line, value)
}
pub fn ring_progress(value: impl Into<Vary<f64>>) -> Progress {
    create(View::Ring, value)
}
pub fn semi_circle_progress(value: impl Into<Vary<f64>>) -> Progress {
    create(View::SemiCircle, value)
}
impl Progress {
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.common.theme = theme;
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn class(mut self, class: impl Into<Vary<String>>) -> Self {
        self.common.class = class.into();
        self
    }
    pub fn style(mut self, style: impl Into<Vary<Style>>) -> Self {
        self.common.style = style.into();
        self
    }
    pub fn node_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(reference);
        self
    }
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn indeterminate(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.indeterminate = value.into();
        self
    }
    pub fn value_text(mut self, text: impl Into<Vary<String>>) -> Self {
        self.value_text = Some(text.into());
        self
    }
    /// Circular diameter in pixels; for a line this explicitly sets its width.
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self.width = Some(Length::Px(size));
        self
    }
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }
    /// Sets the main-axis length of a linear indicator in either orientation.
    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }
    pub fn orientation(mut self, orientation: ProgressOrientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.orientation = if vertical {
            ProgressOrientation::Vertical
        } else {
            ProgressOrientation::Horizontal
        };
        self
    }
    pub fn semi_orientation(mut self, orientation: SemiCircleOrientation) -> Self {
        self.semi_orientation = orientation;
        self
    }
    pub fn fill_direction(mut self, direction: SemiCircleFillDirection) -> Self {
        self.semi_fill_direction = direction;
        self
    }
    pub fn label_position(mut self, position: SemiCircleLabelPosition) -> Self {
        self.semi_label_position = position;
        self
    }
    pub fn label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn filled_segment_color(mut self, color: Color) -> Self {
        self.filled_segment_color = Some(color);
        self
    }
    pub fn empty_segment_color(mut self, color: Color) -> Self {
        self.empty_segment_color = Some(color);
        self
    }
    pub fn start_angle(mut self, degrees: f64) -> Self {
        self.ring_start_angle = degrees;
        self
    }
    pub fn section_gap(mut self, degrees: f64) -> Self {
        self.ring_section_gap = degrees;
        self
    }
    pub fn round_caps(mut self, round: bool) -> Self {
        self.ring_round_caps = round;
        self
    }
    pub fn root_color(mut self, color: Color) -> Self {
        self.ring_root_color = Some(color);
        self
    }
    pub fn thickness(mut self, thickness: f64) -> Self {
        self.thickness = thickness;
        self
    }
    /// Replaces the single visual fill with up to 16 ordered colored sections.
    /// Section values are amounts in the configured range and must add up to
    /// the accepted progress value. Compound sections are supported by line
    /// and ring views; the semicircle remains a single-value indicator.
    pub fn sections(mut self, sections: impl IntoIterator<Item = ProgressSection>) -> Self {
        self.sections = sections.into_iter().collect();
        self
    }
}
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn validate_name(name: &str) -> Result {
    if name.trim().is_empty() || name.len() > 256 {
        return Err(invalid(
            "progress name must be nonempty and at most 256 UTF-8 bytes",
        ));
    }
    Ok(())
}
fn validate_label(label: &str) -> Result {
    if label.trim().is_empty() || label.len() > 1024 {
        return Err(invalid(
            "progress label must be nonempty and at most 1024 UTF-8 bytes",
        ));
    }
    Ok(())
}
fn selection(
    value: &Vary<f64>,
    indeterminate: &Vary<bool>,
    text: Option<&Vary<String>>,
    min: f64,
    max: f64,
) -> Result<ProgressValue> {
    let value = value.try_get()?;
    glory::render::RangeValue { min, max, value }.validate()?;
    let current = if indeterminate.try_get()? {
        None
    } else {
        Some(value)
    };
    let value_text = text
        .map(|text| {
            text.try_with(|text| {
                if text.trim().is_empty() || text.len() > 1024 {
                    return Err(invalid(
                        "progress text must be nonempty and at most 1024 UTF-8 bytes",
                    ));
                }
                Ok(text.clone())
            })
        })
        .transpose()?
        .transpose()?;
    let result = ProgressValue {
        min,
        max,
        current,
        value_text,
    };
    result.validate()?;
    Ok(result)
}
pub(crate) fn arc(
    size: f64,
    thickness: f64,
    semi: bool,
    fraction: f64,
    color: Color,
) -> GraphicProps {
    arc_span(size, thickness, ArcGeometry::new(semi), 0., fraction, color)
}
#[derive(Clone, Copy)]
struct ArcGeometry {
    semi: bool,
    orientation: SemiCircleOrientation,
    direction: SemiCircleFillDirection,
    start: f64,
    cap: StrokeCap,
}
impl ArcGeometry {
    const fn new(semi: bool) -> Self {
        Self {
            semi,
            orientation: SemiCircleOrientation::Up,
            direction: SemiCircleFillDirection::LeftToRight,
            start: -PI / 2.,
            cap: StrokeCap::Round,
        }
    }
    const fn semicircle(
        orientation: SemiCircleOrientation,
        direction: SemiCircleFillDirection,
    ) -> Self {
        Self {
            semi: true,
            orientation,
            direction,
            start: -PI / 2.,
            cap: StrokeCap::Butt,
        }
    }
    fn ring(start_angle: f64, round_caps: bool) -> Self {
        Self {
            semi: false,
            orientation: SemiCircleOrientation::Up,
            direction: SemiCircleFillDirection::LeftToRight,
            start: start_angle.to_radians(),
            cap: if round_caps {
                StrokeCap::Round
            } else {
                StrokeCap::Butt
            },
        }
    }
}
fn arc_span(
    size: f64,
    thickness: f64,
    geometry: ArcGeometry,
    offset: f64,
    fraction: f64,
    color: Color,
) -> GraphicProps {
    let radius = (size - thickness) / 2.0;
    let center_x = size / 2.;
    let center_y = if geometry.semi && geometry.orientation == SemiCircleOrientation::Down {
        thickness / 2.
    } else {
        size / 2.
    };
    let (start, sweep) = if !geometry.semi {
        (geometry.start + offset * TAU, TAU)
    } else {
        match (geometry.orientation, geometry.direction) {
            (SemiCircleOrientation::Up, SemiCircleFillDirection::LeftToRight) => {
                (PI + offset * PI, PI)
            }
            (SemiCircleOrientation::Up, SemiCircleFillDirection::RightToLeft) => {
                (TAU - offset * PI, -PI)
            }
            (SemiCircleOrientation::Down, SemiCircleFillDirection::LeftToRight) => {
                (PI - offset * PI, -PI)
            }
            (SemiCircleOrientation::Down, SemiCircleFillDirection::RightToLeft) => {
                (offset * PI, PI)
            }
        }
    };
    let mut commands = Vec::with_capacity(6);
    if fraction > 0. {
        let segments = (fraction * sweep.abs() / (PI / 2.)).ceil() as usize;
        let step = fraction * sweep / segments as f64;
        let point = |angle: f64| {
            Point::new(
                center_x + radius * angle.cos(),
                center_y + radius * angle.sin(),
            )
        };
        commands.push(Path::Move(point(start)));
        for index in 0..segments {
            let a = start + step * index as f64;
            let b = a + step;
            let factor = 4. / 3. * (step / 4.).tan();
            let p = point(a);
            let q = point(b);
            commands.push(Path::Cubic {
                first: Point::new(
                    p.x - factor * radius * a.sin(),
                    p.y + factor * radius * a.cos(),
                ),
                second: Point::new(
                    q.x + factor * radius * b.sin(),
                    q.y - factor * radius * b.cos(),
                ),
                end: q,
            });
        }
        if fraction == 1. && !geometry.semi {
            commands.push(Path::Close);
        }
    }
    GraphicProps {
        width: size,
        height: if geometry.semi {
            size / 2. + thickness / 2.
        } else {
            size
        },
        commands,
        stroke: Some(GraphicStroke {
            color,
            width: thickness,
            cap: geometry.cap,
        }),
        ..Default::default()
    }
}
impl Widget for Progress {
    fn build(&mut self, scope: &Scope) -> Result {
        if !self.size.is_finite()
            || !(16.0..=512.0).contains(&self.size)
            || !self.thickness.is_finite()
            || !(1.0..=128.0).contains(&self.thickness)
            || self.view != View::Line && self.thickness > self.size / 2.
        {
            return Err(invalid(
                "progress size must be 16..=512 and thickness 1..=128, at most half a circular diameter",
            ));
        }
        if self.view != View::Line && self.orientation != ProgressOrientation::Horizontal {
            return Err(invalid(
                "progress vertical orientation is supported only by the linear view",
            ));
        }
        if self.view != View::SemiCircle
            && (self.semi_orientation != SemiCircleOrientation::Up
                || self.semi_fill_direction != SemiCircleFillDirection::LeftToRight
                || self.semi_label_position != SemiCircleLabelPosition::Bottom
                || self.filled_segment_color.is_some()
                || self.empty_segment_color.is_some())
        {
            return Err(invalid(
                "semicircle orientation, direction, label and segment colors require the semicircle view",
            ));
        }
        if self.view == View::Line && self.label.is_some() {
            return Err(invalid("progress labels require a circular view"));
        }
        if !self.ring_start_angle.is_finite()
            || self.ring_start_angle.abs() > 360_000.
            || !self.ring_section_gap.is_finite()
            || !(0.0..=360.0).contains(&self.ring_section_gap)
        {
            return Err(invalid(
                "ring start angle must be finite within +/-360000 degrees and section gap must be 0..=360 degrees",
            ));
        }
        if self.view != View::Ring
            && (self.ring_start_angle != 270.
                || self.ring_section_gap != 0.
                || self.ring_round_caps
                || self.ring_root_color.is_some())
        {
            return Err(invalid(
                "ring start angle, section gap, caps and root color require the ring view",
            ));
        }
        let min = self.min;
        let max = self.max;
        let initial = selection(
            &self.value,
            &self.indeterminate,
            self.value_text.as_ref(),
            min,
            max,
        )?;
        let section_total = self
            .sections
            .iter()
            .map(|section| section.value)
            .sum::<f64>();
        if self.sections.len() > 16
            || self
                .sections
                .iter()
                .any(|section| !section.value.is_finite() || section.value <= 0.)
            || (!self.sections.is_empty()
                && (self.view == View::SemiCircle
                    || self.value.is_reactive()
                    || self.indeterminate.is_reactive()
                    || initial.current.is_none()
                    || section_total > max - min
                    || initial.current != Some(min + section_total)))
        {
            return Err(invalid(
                "progress sections require 1..=16 positive finite static portions whose sum equals the line or ring value",
            ));
        }
        let source = self.value.clone();
        let indeterminate = self.indeterminate.clone();
        let text = self.value_text.clone();
        let read = move || selection(&source, &indeterminate, text.as_ref(), min, max);
        let name = self
            .common
            .name
            .take()
            .expect("constructor installs a progress name");
        name.try_with(|name| validate_name(name))??;
        let name = crate::composite::validated(scope, &name, |name| validate_name(name))?;
        let value = scope.cage(initial);
        let weak = scope.downgrade();
        scope.structural(move || {
            let Some(scope) = weak.upgrade() else { return };
            let result = (|| -> Result {
                let next = read()?;
                if value.try_with_untracked(|old| *old != next)? {
                    value.try_revise(|old| *old = next)?;
                }
                Ok(())
            })();
            if let Err(error) = result {
                scope.report_error(error);
            }
        });
        self.common.name = Some(name.into());
        let label = self
            .label
            .take()
            .map(|label| {
                label.try_with(|label| validate_label(label))??;
                crate::composite::validated(scope, &label, |label| validate_label(label))
            })
            .transpose()?;
        let vertical = self.orientation == ProgressOrientation::Vertical;
        let height = match self.view {
            View::Line => self.thickness,
            View::Ring => self.size,
            View::SemiCircle => self.size / 2. + self.thickness / 2.,
        };
        let (width, height) = if self.view == View::Line && vertical {
            (
                Length::Px(self.thickness),
                self.length.or(self.height).unwrap_or(Length::Px(self.size)),
            )
        } else {
            (
                self.length
                    .or(self.width)
                    .unwrap_or(if self.view == View::Line {
                        Length::Percent(100.)
                    } else {
                        Length::Px(self.size)
                    }),
                Length::Px(height),
            )
        };
        let required = Style::new()
            .position(Position::Relative)
            .display(Display::Block)
            .width(width)
            .height(height);
        let custom = self.common.style.clone();
        self.common.style = if custom.is_reactive() {
            scope
                .bond(move || merge(required.clone(), custom.get()))
                .into()
        } else {
            merge(required, custom.get()).into()
        };
        let mut base = Style::new();
        if self.view == View::Line {
            base.background = Some(self.common.theme.border);
            base.radius = Some(self.thickness / 2.);
            base.overflow_x = Some(Overflow::Hidden);
            base.overflow_y = Some(Overflow::Hidden);
        }
        let color = self
            .filled_segment_color
            .unwrap_or(if self.common.unstyled {
                self.common.theme.foreground
            } else {
                self.common.theme.accent
            });
        let track_color = self
            .ring_root_color
            .or(self.empty_segment_color)
            .unwrap_or(self.common.theme.border);
        let class = match (self.view, vertical) {
            (View::Line, false) => "sapid-progress sapid-progress-line",
            (View::Line, true) => "sapid-progress sapid-progress-line sapid-progress-vertical",
            (View::Ring, _) => "sapid-progress sapid-progress-ring",
            (View::SemiCircle, _) => "sapid-progress sapid-progress-semicircle",
        };
        let metadata = value.map(|value| {
            Some(Box::new(Semantics {
                progress: Some(value.clone()),
                ..Default::default()
            }))
        });
        let mut root = configure!(self.common, glory::ui::view(), scope, class, base)
            .role(Role::ProgressBar)
            .semantics(metadata);
        if self.sections.is_empty() {
            if self.view == View::Line {
                root = root.fill(
                    glory::ui::view()
                        .class("sapid-progress-fill")
                        .style(value.map(move |value| {
                            let fraction = value.fraction().expect("accepted progress is finite");
                            let mut style = Style::new().position(Position::Absolute);
                            if vertical {
                                style.bottom = Some(Length::Percent(if fraction.is_some() {
                                    0.
                                } else {
                                    35.
                                }));
                                style.left = Some(Length::Px(0.));
                                style.width = Some(Length::Percent(100.));
                                style.height =
                                    Some(Length::Percent(fraction.unwrap_or(0.3) * 100.));
                            } else {
                                style.top = Some(Length::Px(0.));
                                style.left = Some(Length::Percent(if fraction.is_some() {
                                    0.
                                } else {
                                    35.
                                }));
                                style.width = Some(Length::Percent(fraction.unwrap_or(0.3) * 100.));
                                style.height = Some(Length::Percent(100.));
                            }
                            style.background = Some(color);
                            style
                        })),
                );
            } else {
                let semi = self.view == View::SemiCircle;
                let size = self.size;
                let thickness = self.thickness;
                let geometry = if semi {
                    ArcGeometry::semicircle(self.semi_orientation, self.semi_fill_direction)
                } else {
                    ArcGeometry::ring(self.ring_start_angle, self.ring_round_caps)
                };
                let drawing_style = Style::new()
                    .position(Position::Absolute)
                    .top(Length::Px(0.))
                    .left(Length::Px(0.))
                    .width(Length::Percent(100.))
                    .height(Length::Percent(100.))
                    .display(Display::Block);
                if !self.common.unstyled
                    || self.empty_segment_color.is_some()
                    || self.ring_root_color.is_some()
                {
                    root = root.fill(
                        glory::ui::graphic()
                            .class("sapid-progress-track")
                            .drawing(arc_span(size, thickness, geometry, 0., 1., track_color))
                            .style(drawing_style.clone()),
                    );
                }
                root = root.fill(
                    glory::ui::graphic()
                        .class("sapid-progress-arc")
                        .drawing(value.map(move |value| {
                            arc_span(
                                size,
                                thickness,
                                geometry,
                                0.,
                                value
                                    .fraction()
                                    .expect("accepted progress is finite")
                                    .unwrap_or(0.25),
                                color,
                            )
                        }))
                        .style(drawing_style),
                );
                if let Some(label) = label {
                    let center = self.view == View::Ring
                        || self.semi_label_position == SemiCircleLabelPosition::Center;
                    let down = self.view == View::SemiCircle
                        && self.semi_orientation == SemiCircleOrientation::Down;
                    let mut label_style = Style::new()
                        .position(Position::Absolute)
                        .left(Length::Px(0.))
                        .width(Length::Percent(100.))
                        .display(Display::Flex);
                    label_style.align = Some(Align::Center);
                    label_style.justify = Some(Justify::Center);
                    if center {
                        label_style.top = Some(Length::Px(0.));
                        label_style.height = Some(Length::Percent(100.));
                    } else {
                        label_style.height = Some(Length::Px(thickness * 2.));
                        if down {
                            label_style.top = Some(Length::Px(0.));
                        } else {
                            label_style.bottom = Some(Length::Px(0.));
                        }
                    }
                    root = root.fill(
                        glory::ui::view()
                            .class("sapid-progress-label")
                            .style(label_style)
                            .fill(glory::ui::text().content(label)),
                    );
                }
            }
        } else if self.view == View::Line {
            let span = max - min;
            let mut offset = 0.;
            for section in &self.sections {
                let start = offset / span * 100.;
                let extent = section.value / span * 100.;
                offset += section.value;
                root = root.fill(glory::ui::view().class("sapid-progress-section").style({
                    let mut style = Style::new().position(Position::Absolute);
                    if vertical {
                        style.bottom = Some(Length::Percent(start));
                        style.left = Some(Length::Px(0.));
                        style.width = Some(Length::Percent(100.));
                        style.height = Some(Length::Percent(extent));
                    } else {
                        style.top = Some(Length::Px(0.));
                        style.left = Some(Length::Percent(start));
                        style.width = Some(Length::Percent(extent));
                        style.height = Some(Length::Percent(100.));
                    }
                    style.background = Some(section.color);
                    style
                }));
            }
        } else {
            let drawing_style = Style::new()
                .position(Position::Absolute)
                .top(Length::Px(0.))
                .left(Length::Px(0.))
                .width(Length::Percent(100.))
                .height(Length::Percent(100.))
                .display(Display::Block);
            if !self.common.unstyled || self.ring_root_color.is_some() {
                root = root.fill(
                    glory::ui::graphic()
                        .class("sapid-progress-track")
                        .drawing(arc_span(
                            self.size,
                            self.thickness,
                            ArcGeometry::ring(self.ring_start_angle, false),
                            0.,
                            1.,
                            track_color,
                        ))
                        .style(drawing_style.clone()),
                );
            }
            let span = max - min;
            let mut offset = 0.;
            let section_count = self.sections.len();
            let gap = self.ring_section_gap / 360.;
            for (index, section) in self.sections.iter().enumerate() {
                let fraction = section.value / span;
                let visible_fraction = (fraction - gap).max(0.);
                let round = self.ring_round_caps
                    && (section_count == 1 || index == 0 || index + 1 == section_count);
                root = root.fill(
                    glory::ui::graphic()
                        .class("sapid-progress-section")
                        .drawing(arc_span(
                            self.size,
                            self.thickness,
                            ArcGeometry::ring(self.ring_start_angle, round),
                            offset,
                            visible_fraction,
                            section.color,
                        ))
                        .style(drawing_style.clone()),
                );
                offset += fraction;
            }
            if let Some(label) = label {
                let mut label_style = Style::new()
                    .position(Position::Absolute)
                    .top(Length::Px(0.))
                    .left(Length::Px(0.))
                    .width(Length::Percent(100.))
                    .height(Length::Percent(100.))
                    .display(Display::Flex);
                label_style.align = Some(Align::Center);
                label_style.justify = Some(Justify::Center);
                root = root.fill(
                    glory::ui::view()
                        .class("sapid-progress-label")
                        .style(label_style)
                        .fill(glory::ui::text().content(label)),
                );
            }
        }
        root.show_in(scope)
    }
}
