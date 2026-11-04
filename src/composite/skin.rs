//! Shared finite presentation for static and keyed composite controls.
use crate::{Size, Theme};
use glory::render::Orientation;
use glory::style::{
    Align, BorderStyle, Color, Display, Edges, FlexDirection, FlexWrap, Justify, Length, Style,
    WhiteSpace,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionVariant {
    #[default]
    Default,
    Contained,
    Filled,
    Separated,
}

pub(crate) fn accordion_trigger(theme: &Theme, size: Size) -> Style {
    let mut style = theme.control(size);
    style.display = Some(Display::Flex);
    style.align = Some(Align::Center);
    style.justify = Some(Justify::Between);
    style.gap = Some(12.);
    style.background = Some(Color(0, 0, 0, 0));
    style.border_width = Some(Edges::all(0.));
    style.radius = Some(0.);
    style.width = Some(Length::Percent(100.));
    style.white_space = Some(WhiteSpace::Normal);
    style
}

pub(crate) fn accordion_item(theme: &Theme, variant: AccordionVariant, open: bool) -> Style {
    let mut style = Style::new()
        .border_style(BorderStyle::Solid)
        .border_color(theme.border);
    style.border_width = Some(match variant {
        AccordionVariant::Default => Edges {
            top: 0.,
            right: 0.,
            bottom: 1.,
            left: 0.,
        },
        AccordionVariant::Filled => Edges::all(0.),
        _ => Edges::all(1.),
    });
    if variant != AccordionVariant::Default {
        style.radius = Some(theme.radius);
    }
    if variant == AccordionVariant::Filled || variant == AccordionVariant::Separated && open {
        style.background = Some(theme.surface);
    }
    style
}

pub(crate) fn accordion_root(variant: AccordionVariant) -> Style {
    crate::composite::column(if variant == AccordionVariant::Separated {
        12.
    } else {
        0.
    })
}

pub(crate) fn chevron(
    scope: &glory::Scope,
    open: glory::Vary<bool>,
    color: Color,
) -> glory::ui::Graphic {
    use glory::render::{GraphicPoint as P, PathCommand as C};
    glory::ui::graphic()
        .class("sapid-disclosure-chevron")
        .drawing(scope.bond(move || {
            let points = if open.get() {
                [(5., 8.), (12., 15.), (19., 8.)]
            } else {
                [(8., 5.), (15., 12.), (8., 19.)]
            };
            glory::render::GraphicProps {
                commands: vec![
                    C::Move(P::new(points[0].0, points[0].1)),
                    C::Line(P::new(points[1].0, points[1].1)),
                    C::Line(P::new(points[2].0, points[2].1)),
                ],
                stroke: Some(glory::render::GraphicStroke {
                    color,
                    width: 1.7,
                    cap: glory::render::StrokeCap::Round,
                }),
                ..Default::default()
            }
        }))
        .style(Style {
            shrink: Some(0.),
            ..Style::new().width(Length::Px(16.)).height(Length::Px(16.))
        })
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    #[default]
    Default,
    Outline,
    Pills,
}

pub(crate) fn tab(
    theme: &Theme,
    size: Size,
    variant: TabsVariant,
    axis: Orientation,
    selected: bool,
    grow: bool,
) -> Style {
    let mut style = theme.control(size);
    style.shrink = Some(0.);
    style.grow = Some(if grow { 1. } else { 0. });
    style.min_width = Some(Length::Px(0.));
    style.white_space = Some(WhiteSpace::NoWrap);
    style.color = Some(if selected {
        theme.accent_text
    } else {
        theme.foreground
    });
    style.background = Some(Color(0, 0, 0, 0));
    style.border_color = Some(if selected {
        theme.accent
    } else {
        Color(0, 0, 0, 0)
    });
    style.radius = Some(if variant == TabsVariant::Pills {
        theme.radius
    } else {
        0.
    });
    style.border_width = Some(match variant {
        TabsVariant::Pills => Edges::all(0.),
        TabsVariant::Outline => Edges::all(1.),
        TabsVariant::Default if axis == Orientation::Vertical => Edges {
            left: 2.,
            top: 0.,
            right: 0.,
            bottom: 0.,
        },
        TabsVariant::Default => Edges {
            left: 0.,
            top: 0.,
            right: 0.,
            bottom: 2.,
        },
    });
    if selected {
        style.font_weight = Some(600.);
        if variant == TabsVariant::Pills {
            style.background = Some(theme.accent);
            style.color = Some(theme.on_accent);
        } else if variant == TabsVariant::Outline {
            style.background = Some(theme.surface);
            style.border_color = Some(theme.border);
        }
    }
    style
}

pub(crate) fn tab_list(
    theme: &Theme,
    variant: TabsVariant,
    axis: Orientation,
    unstyled: bool,
) -> Style {
    let mut style = Style::new()
        .display(Display::Flex)
        .flex_direction(if axis == Orientation::Horizontal {
            FlexDirection::Row
        } else {
            FlexDirection::Column
        })
        .gap(if variant == TabsVariant::Pills {
            6.
        } else {
            0.
        });
    style.shrink = Some(0.);
    if axis == Orientation::Horizontal {
        style.flex_wrap = Some(FlexWrap::Wrap);
    }
    if !unstyled && variant != TabsVariant::Pills {
        style.border_style = Some(BorderStyle::Solid);
        style.border_color = Some(theme.border);
        style.border_width = Some(if axis == Orientation::Horizontal {
            Edges {
                bottom: 1.,
                top: 0.,
                left: 0.,
                right: 0.,
            }
        } else {
            Edges {
                right: 1.,
                top: 0.,
                left: 0.,
                bottom: 0.,
            }
        });
    }
    style
}

pub(crate) fn tabs_root(axis: Orientation) -> Style {
    let mut style = crate::composite::column(0.);
    if axis == Orientation::Vertical {
        style.flex_direction = Some(FlexDirection::Row);
    }
    style.min_width = Some(Length::Px(0.));
    style
}
