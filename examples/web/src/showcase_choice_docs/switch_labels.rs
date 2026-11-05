use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::switch()
            .theme(theme.clone())
            .label("Before the track")
            .label_position(sapid::SwitchLabelPosition::Before)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            ),
        sapid::switch()
            .theme(theme.clone())
            .label("After the track")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            ),
        sapid::field("switch-feedback")
            .theme(theme)
            .label("Account alerts")
            .description("Send important account updates.")
            .error("Confirm your notification preference.")
            .control(|field| sapid::switch().field(field).with_marks(true)),
    ))
}
