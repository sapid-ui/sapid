use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("input-email")
        .theme(theme)
        .label("Email address")
        .description("We will only use this for important account updates.")
        .required(true)
        .control(|field| {
            sapid::text_field()
                .field(field)
                .placeholder("you@example.com")
        })
}
