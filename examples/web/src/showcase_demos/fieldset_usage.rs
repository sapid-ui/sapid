use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::fieldset("profile")
        .theme(theme.clone())
        .legend("Personal information")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(420.)),
        )
        .fill((
            sapid::field("profile-name")
                .theme(theme.clone())
                .label("Your name")
                .control(|field| sapid::text_field().field(field).placeholder("Your name")),
            sapid::field("profile-email")
                .theme(theme.clone())
                .label("Email address")
                .control(|field| {
                    sapid::text_field()
                        .field(field)
                        .placeholder("you@example.com")
                }),
        ))
}
