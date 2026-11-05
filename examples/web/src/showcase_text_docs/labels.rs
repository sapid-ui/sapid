use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("input-required")
            .theme(theme.clone())
            .label("Work email")
            .description("Use the address associated with your workspace.")
            .required(true)
            .control(|field| {
                sapid::text_field()
                    .field(field)
                    .placeholder("you@company.com")
            }),
        sapid::field("input-error")
            .theme(theme.clone())
            .label("Workspace name")
            .description("A unique public identifier.")
            .error("That name is already in use.")
            .control(|field| sapid::text_field().field(field).default_value("north-star")),
        sapid::field("input-success")
            .theme(theme)
            .label("Available name")
            .success(true)
            .control(|field| {
                sapid::text_field()
                    .field(field)
                    .default_value("north-star-team")
            }),
    ))
}
