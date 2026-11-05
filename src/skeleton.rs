//! A finite placeholder. Applications own replacement content and loading state.
use crate::appearance::Common;
use crate::composite;
use glory::render::{
    GraphicAnimation, GraphicFit, GraphicPoint, GraphicProps, NodeId, PathCommand, Semantics,
};
use glory::style::{Display, Length, Overflow, Position, Style};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};
pub struct Skeleton {
    common: Common,
    width: Vary<Length>,
    height: Vary<f64>,
    loading: Vary<bool>,
    radius: f64,
    animated: Vary<bool>,
    height_explicit: bool,
    children: Option<Children>,
}
pub fn skeleton() -> Skeleton {
    Skeleton {
        common: Common::default(),
        width: Length::Percent(100.0).into(),
        height: 16.0.into(),
        loading: true.into(),
        radius: 4.0,
        animated: true.into(),
        height_explicit: false,
        children: None,
    }
}
impl Skeleton {
    pub fn width(mut self, value: impl Into<Vary<Length>>) -> Self {
        self.width = value.into();
        self
    }
    pub fn height(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.height = value.into();
        self.height_explicit = true;
        self
    }
    pub fn loading(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.loading = value.into();
        self
    }
    pub fn animated(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.animated = value.into();
        self
    }
    /// Keep real content mounted behind the loading presentation. Its layout
    /// remains intrinsic unless height is explicitly configured.
    pub fn fill(mut self, value: impl IntoChildren) -> Self {
        self.children = Some(value.into_children());
        self
    }
    pub fn radius(mut self, value: f64) -> Self {
        self.radius = value;
        self
    }
    pub fn circle(mut self, diameter: f64) -> Self {
        self.width = Length::Px(diameter).into();
        self.height = diameter.into();
        self.height_explicit = true;
        self.radius = diameter / 2.0;
        self
    }
    /// Names the busy region. Its graphic remains explicitly decorative.
    pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(value.into());
        self
    }
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self
    }
    pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.class = value.into();
        self
    }
    pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
        self.common.style = value.into();
        self
    }
    pub fn node_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
}
fn dimension(value: f64) -> Result {
    if !value.is_finite() || !(0.000001..=1_000_000.0).contains(&value) {
        return Err(glory::render::Error::InvalidProps(
            "skeleton dimensions must be finite and in 0.000001..=1000000",
        )
        .into());
    }
    Ok(())
}
impl Widget for Skeleton {
    fn build(&mut self, scope: &Scope) -> Result {
        if !self.radius.is_finite() || !(0.0..=1_000_000.0).contains(&self.radius) {
            return Err(glory::render::Error::InvalidProps(
                "skeleton radius must be finite and in 0..=1000000",
            )
            .into());
        }
        let width = composite::validated(scope, &self.width, |value| match value {
            Length::Px(value) | Length::Percent(value) => dimension(*value),
            Length::Auto | Length::Ch(_) | Length::ChPx { .. } => {
                Err(glory::render::Error::InvalidProps(
                    "skeleton width must be explicit pixels or percent",
                )
                .into())
            }
        })?;
        let height = composite::validated(scope, &self.height, |value| dimension(*value))?;
        let own = self.common.style.clone();
        let unstyled = self.common.unstyled;
        let radius = self.radius;
        let with_children = self.children.is_some();
        let fixed_height = self.height_explicit || !with_children;
        self.common.style = scope
            .bond(move || {
                let mut base = Style::new()
                    .display(Display::Block)
                    .width(width.get())
                    .radius(radius);
                if fixed_height {
                    base.height = Some(Length::Px(height.get()));
                }
                if with_children {
                    base.position = Some(Position::Relative);
                }
                crate::appearance::merge(base, own.get())
            })
            .into();
        let (classes, styles) = self
            .common
            .appearance(scope, "sapid-skeleton", Style::new());
        let loading = self.loading.clone();
        let mut root = glory::ui::view()
            .class(classes)
            .style(styles)
            .semantics(scope.bond(move || {
                let loading = loading.get();
                Some(Box::new(Semantics {
                    busy: Some(loading),
                    hidden: !loading && !with_children,
                    ..Default::default()
                }))
            }));
        if let Some(name) = self.common.name.take() {
            root = root.name(name)
        }
        if let Some(reference) = self.common.node_ref {
            root = root.node_ref(reference)
        }
        if let Some(children) = self.children.take() {
            let loading = self.loading.clone();
            let available = self.loading.clone();
            root =
                root.fill(
                    glory::ui::view()
                        .class("sapid-skeleton-content")
                        .style(scope.bond(move || {
                            Style::new().opacity(if loading.get() { 0. } else { 1. })
                        }))
                        .semantics(scope.bond(move || {
                            Some(Box::new(Semantics {
                                inert: available.get(),
                                ..Default::default()
                            }))
                        }))
                        .fill(children),
                );
        }
        let mut graphic_style = Style::new()
            .display(Display::Block)
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .radius(radius)
            .overflow_x(Overflow::Hidden)
            .overflow_y(Overflow::Hidden);
        if with_children {
            graphic_style = graphic_style
                .position(Position::Absolute)
                .top(Length::Px(0.))
                .left(Length::Px(0.));
        }
        let style_loading = self.loading.clone();
        let graphic_style = scope.bond(move || {
            let mut style = graphic_style.clone();
            if !style_loading.get() {
                style.display = Some(Display::None);
            }
            style
        });
        let animated = self.animated.clone();
        let loading = self.loading.clone();
        let color = (!unstyled).then_some(self.common.theme.border);
        let drawing = scope.bond(move || GraphicProps {
            width: 100.,
            height: 100.,
            fit: GraphicFit::Stretch,
            commands: vec![
                PathCommand::Move(GraphicPoint::new(0., 0.)),
                PathCommand::Line(GraphicPoint::new(100., 0.)),
                PathCommand::Line(GraphicPoint::new(100., 100.)),
                PathCommand::Line(GraphicPoint::new(0., 100.)),
                PathCommand::Close,
            ],
            fill: color,
            animation: (animated.get() && loading.get()).then_some(GraphicAnimation::Pulse {
                period_millis: 1500,
                phase_millis: 0,
            }),
            ..Default::default()
        });
        root.fill(
            glory::ui::graphic()
                .class("sapid-skeleton-placeholder")
                .drawing(drawing)
                .style(graphic_style),
        )
        .show_in(scope)
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        skeleton()
    }
}
