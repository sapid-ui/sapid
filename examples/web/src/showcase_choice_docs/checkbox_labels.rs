use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::checkbox()
            .theme(theme.clone())
            .label("Label after the checkbox"),
        sapid::field("checkbox-label-before")
            .theme(theme.clone())
            .label("Label before the checkbox")
            .style(Style {
                align: Some(Align::Center),
                ..Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Row)
            })
            .control(|field| sapid::checkbox().field(field)),
        sapid::field("checkbox-error")
            .theme(theme.clone())
            .label("Accept the privacy policy")
            .description("Your consent is required before continuing.")
            .required(true)
            .error("Please accept the privacy policy.")
            .control(|field| sapid::checkbox().field(field)),
    ))
}
