use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("password-usage")
        .theme(theme)
        .label("Password")
        .description("Use at least 12 characters")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(360.)),
        )
        .control(|field| {
            sapid::password_input()
                .field(field)
                .placeholder("Your password")
        })
}
