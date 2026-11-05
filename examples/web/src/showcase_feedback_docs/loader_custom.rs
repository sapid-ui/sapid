use glory::prelude::*;
use glory::render::{
    GraphicAnimation, GraphicPoint, GraphicProps, GraphicStroke, PathCommand, Role, Semantics,
    StrokeCap,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view()
        .role(Role::Status)
        .name("Loading a custom report")
        .semantics(Some(Box::new(Semantics {
            busy: Some(true),
            ..Default::default()
        })))
        .style(Style::new().width(Length::Px(36.)).height(Length::Px(36.)))
        .fill(
            glory::ui::graphic()
                .drawing(GraphicProps {
                    commands: vec![
                        PathCommand::Move(GraphicPoint::new(12., 3.)),
                        PathCommand::Line(GraphicPoint::new(21., 12.)),
                        PathCommand::Line(GraphicPoint::new(12., 21.)),
                    ],
                    stroke: Some(GraphicStroke {
                        color: theme.accent,
                        width: 3.,
                        cap: StrokeCap::Round,
                    }),
                    animation: Some(GraphicAnimation::Rotate {
                        period_millis: 1200,
                    }),
                    ..Default::default()
                })
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .height(Length::Percent(100.)),
                ),
        )
}
