use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::field("tags-blur-field")
            .theme(theme.clone())
            .label("Accept keyword on blur")
            .description("Type a draft, then use the button below.")
            .control(|field| {
                sapid::tags_input("tags-blur")
                    .field(field)
                    .accept_value_on_blur(true)
            }),
        sapid::button()
            .theme(theme)
            .label("Move focus out of keywords"),
    ))
}
