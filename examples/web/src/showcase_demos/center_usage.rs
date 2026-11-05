use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::center()
        .theme(theme.clone())
        .height(Length::Px(160.))
        .style(Style {
            background: Some(theme.accent_soft),
            ..Style::new().width(Length::Percent(100.))
        })
        .fill(
            sapid::text()
                .theme(theme)
                .content("Centered in both directions"),
        )
}
