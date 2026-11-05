//! Passive themed icons backed by Glory's finite cross-host vector paths.
use crate::appearance::{Common, configure};
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, NodeId, PathCommand as Path, StrokeCap,
};
use glory::style::{Align, BorderStyle, Color, Display, Edges, Justify, Length, Style};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Icon {
    #[default]
    Check,
    Close,
    Info,
    ArrowRight,
    ChevronDown,
    ChevronRight,
    ChevronLeft,
    ChevronUp,
    More,
    Circle,
    Calendar,
}
impl Icon {
    /// A passive finite 24×24 drawing for intrinsic button, link and menu slots.
    pub fn drawing(self, color: Color) -> GraphicProps {
        GraphicProps {
            width: 24.,
            height: 24.,
            commands: path(self),
            stroke: Some(GraphicStroke {
                color,
                width: 2.,
                cap: StrokeCap::Round,
            }),
            ..Default::default()
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeIconVariant {
    #[default]
    Soft,
    Solid,
    Outline,
}
pub struct ThemeIcon {
    common: Common,
    icon: Vary<Icon>,
    variant: ThemeIconVariant,
    color: Option<Color>,
    icon_size: Option<Vary<f64>>,
}
pub fn theme_icon(icon: impl Into<Vary<Icon>>) -> ThemeIcon {
    ThemeIcon {
        common: Common::default(),
        icon: icon.into(),
        variant: ThemeIconVariant::default(),
        color: None,
        icon_size: None,
    }
}
impl ThemeIcon {
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.common.theme = theme;
        self
    }
    pub fn size(mut self, size: crate::Size) -> Self {
        self.common.size = size;
        self
    }
    /// Independent glyph extent, in finite 1..=128 pixels. Invalid updates
    /// preserve the last accepted extent and the actual Graphic identity.
    pub fn icon_size(mut self, size: impl Into<Vary<f64>>) -> Self {
        self.icon_size = Some(size.into());
        self
    }
    pub fn variant(mut self, variant: ThemeIconVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
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
    /// Supply an accessible image alternative. Omit for a decorative icon.
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
}
pub(crate) fn path(icon: Icon) -> Vec<Path> {
    let p = Point::new;
    match icon {
        Icon::Calendar => vec![
            Path::Move(p(4., 6.)),
            Path::Line(p(20., 6.)),
            Path::Line(p(20., 21.)),
            Path::Line(p(4., 21.)),
            Path::Close,
            Path::Move(p(4., 10.)),
            Path::Line(p(20., 10.)),
            Path::Move(p(8., 3.)),
            Path::Line(p(8., 7.)),
            Path::Move(p(16., 3.)),
            Path::Line(p(16., 7.)),
        ],
        Icon::Check => vec![
            Path::Move(p(5., 12.)),
            Path::Line(p(10., 17.)),
            Path::Line(p(19., 7.)),
        ],
        Icon::Close => vec![
            Path::Move(p(6., 6.)),
            Path::Line(p(18., 18.)),
            Path::Move(p(18., 6.)),
            Path::Line(p(6., 18.)),
        ],
        Icon::ArrowRight => vec![
            Path::Move(p(4., 12.)),
            Path::Line(p(20., 12.)),
            Path::Move(p(13., 5.)),
            Path::Line(p(20., 12.)),
            Path::Line(p(13., 19.)),
        ],
        Icon::ChevronDown => vec![
            Path::Move(p(6., 9.)),
            Path::Line(p(12., 15.)),
            Path::Line(p(18., 9.)),
        ],
        Icon::ChevronRight => vec![
            Path::Move(p(9., 6.)),
            Path::Line(p(15., 12.)),
            Path::Line(p(9., 18.)),
        ],
        Icon::ChevronLeft => vec![
            Path::Move(p(15., 6.)),
            Path::Line(p(9., 12.)),
            Path::Line(p(15., 18.)),
        ],
        Icon::ChevronUp => vec![
            Path::Move(p(6., 15.)),
            Path::Line(p(12., 9.)),
            Path::Line(p(18., 15.)),
        ],
        Icon::More => vec![
            Path::Move(p(5., 12.)),
            Path::Line(p(5.1, 12.)),
            Path::Move(p(12., 12.)),
            Path::Line(p(12.1, 12.)),
            Path::Move(p(19., 12.)),
            Path::Line(p(19.1, 12.)),
        ],
        Icon::Circle => circle_path(),
        Icon::Info => vec![
            Path::Move(p(21., 12.)),
            Path::Cubic {
                first: p(21., 16.9705627485),
                second: p(16.9705627485, 21.),
                end: p(12., 21.),
            },
            Path::Cubic {
                first: p(7.0294372515, 21.),
                second: p(3., 16.9705627485),
                end: p(3., 12.),
            },
            Path::Cubic {
                first: p(3., 7.0294372515),
                second: p(7.0294372515, 3.),
                end: p(12., 3.),
            },
            Path::Cubic {
                first: p(16.9705627485, 3.),
                second: p(21., 7.0294372515),
                end: p(21., 12.),
            },
            Path::Close,
            Path::Move(p(12., 11.)),
            Path::Line(p(12., 17.)),
            Path::Move(p(12., 7.)),
            Path::Line(p(12., 7.1)),
        ],
    }
}
pub(crate) fn circle_path() -> Vec<Path> {
    let p = Point::new;
    vec![
        Path::Move(p(21., 12.)),
        Path::Cubic {
            first: p(21., 16.9705627485),
            second: p(16.9705627485, 21.),
            end: p(12., 21.),
        },
        Path::Cubic {
            first: p(7.0294372515, 21.),
            second: p(3., 16.9705627485),
            end: p(3., 12.),
        },
        Path::Cubic {
            first: p(3., 7.0294372515),
            second: p(7.0294372515, 3.),
            end: p(12., 3.),
        },
        Path::Cubic {
            first: p(16.9705627485, 3.),
            second: p(21., 7.0294372515),
            end: p(21., 12.),
        },
        Path::Close,
    ]
}
fn validate_name(name: &str) -> Result {
    if name.trim().is_empty() || name.len() > 256 {
        return Err(glory::render::Error::InvalidProps(
            "theme icon name must be nonempty and at most 256 UTF-8 bytes",
        )
        .into());
    }
    Ok(())
}
impl Widget for ThemeIcon {
    fn build(&mut self, scope: &Scope) -> Result {
        if let Some(name) = &self.common.name {
            name.try_with(|name| validate_name(name))??;
        }
        let name = self.common.name.take();
        let accepted = if let Some(name) = name {
            let initial = name.try_get()?;
            if name.is_reactive() {
                let accepted = scope.cage(initial);
                let weak = scope.downgrade();
                scope.effect(move || {
                    let Some(scope) = weak.upgrade() else { return };
                    let result = (|| -> Result {
                        let next = name.try_get()?;
                        validate_name(&next)?;
                        if accepted.try_with_untracked(|old| *old != next)? {
                            accepted.try_revise(|old| *old = next)?;
                        }
                        Ok(())
                    })();
                    if let Err(error) = result {
                        scope.report_error(error);
                    }
                });
                Some(Vary::from(accepted))
            } else {
                Some(initial.into())
            }
        } else {
            None
        };
        let side = match self.common.size {
            crate::Size::ExtraSmall => 24.,
            crate::Size::Small => 30.,
            crate::Size::Medium => 36.,
            crate::Size::Large => 42.,
            crate::Size::ExtraLarge => 50.,
        };
        let icon_size: Vary<f64> = if let Some(source) = self.icon_size.take() {
            let validate = |size: &f64| -> Result {
                if !size.is_finite() || !(1. ..=128.).contains(size) {
                    Err(glory::render::Error::InvalidProps(
                        "theme icon glyph size must be finite and in 1..=128 pixels",
                    )
                    .into())
                } else {
                    Ok(())
                }
            };
            let initial = source.try_get()?;
            validate(&initial)?;
            if source.is_reactive() {
                crate::composite::validated(scope, &source, validate)?.into()
            } else {
                initial.into()
            }
        } else {
            (side * 0.6).into()
        };
        let mut base = Style::new()
            .display(Display::InlineFlex)
            .width(Length::Px(side))
            .height(Length::Px(side))
            .radius(self.common.theme.radius);
        base.align = Some(Align::Center);
        base.justify = Some(Justify::Center);
        base.shrink = Some(0.0);
        let color = if self.common.unstyled {
            self.common.theme.foreground
        } else {
            match self.variant {
                ThemeIconVariant::Soft => {
                    base.background = Some(self.common.theme.accent_soft);
                    self.common.theme.accent_text
                }
                ThemeIconVariant::Solid => {
                    base.background = Some(self.common.theme.accent);
                    self.common.theme.on_accent
                }
                ThemeIconVariant::Outline => {
                    base.border_width = Some(Edges::all(1.0));
                    base.border_style = Some(BorderStyle::Solid);
                    base.border_color = Some(self.common.theme.accent);
                    self.common.theme.accent_text
                }
            }
        };
        let color = self.color.unwrap_or(color);
        let icon = self.icon.clone();
        let reactive = icon.is_reactive() || accepted.as_ref().is_some_and(Vary::is_reactive);
        let drawing = move || GraphicProps {
            width: 24.,
            height: 24.,
            commands: path(icon.get()),
            stroke: Some(GraphicStroke {
                color,
                width: 2.,
                cap: StrokeCap::Round,
            }),
            alternative: accepted.as_ref().map(Vary::get),
            ..Default::default()
        };
        let drawing: Vary<GraphicProps> = if reactive {
            scope.bond(drawing).into()
        } else {
            drawing().into()
        };
        // Geometry is content, so an unstyled icon still has a visible path.
        let reactive = icon_size.is_reactive();
        let drawing_style = move || {
            Style::new()
                .display(Display::Block)
                .width(Length::Px(icon_size.get()))
                .height(Length::Px(icon_size.get()))
        };
        let drawing_style: Vary<Style> = if reactive {
            scope.bond(drawing_style).into()
        } else {
            drawing_style().into()
        };
        configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-theme-icon",
            base
        )
        .fill(
            glory::ui::graphic()
                .class("sapid-theme-icon-graphic")
                .drawing(drawing)
                .style(drawing_style),
        )
        .show_in(scope)
    }
}
