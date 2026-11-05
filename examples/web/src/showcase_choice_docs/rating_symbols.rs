use glory::prelude::*;
use glory::render::{GraphicPoint as Point, GraphicProps, GraphicStroke, PathCommand as Path};
fn heart(fill: Color, stroke: Color) -> GraphicProps {
    GraphicProps {
        width: 24.,
        height: 24.,
        fill: Some(fill),
        stroke: Some(GraphicStroke {
            color: stroke,
            width: 1.,
            cap: Default::default(),
        }),
        commands: vec![
            Path::Move(Point::new(12., 21.)),
            Path::Cubic {
                first: Point::new(0., 13.),
                second: Point::new(0., 1.),
                end: Point::new(8., 3.),
            },
            Path::Cubic {
                first: Point::new(10., 3.),
                second: Point::new(12., 5.),
                end: Point::new(12., 6.),
            },
            Path::Cubic {
                first: Point::new(12., 5.),
                second: Point::new(14., 3.),
                end: Point::new(16., 3.),
            },
            Path::Cubic {
                first: Point::new(24., 1.),
                second: Point::new(24., 13.),
                end: Point::new(12., 21.),
            },
            Path::Close,
        ],
        ..Default::default()
    }
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rating("heart-score")
        .theme(theme.clone())
        .name("Heart rating")
        .count(3)
        .precision(sapid::RatingPrecision::Half)
        .default_value(1.5)
        .empty_symbol(heart(theme.surface, theme.border))
        .filled_symbol(heart(theme.accent, theme.accent))
}
