use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::field("password-required")
            .theme(theme.clone())
            .label("Required password")
            .description("The description remains associated with the editor")
            .required(true)
            .control(|field| {
                sapid::password_input()
                    .field(field)
                    .placeholder("Enter a password")
            }),
        sapid::field("password-invalid")
            .theme(theme.clone())
            .label("Invalid password")
            .error("Use a longer password")
            .control(|field| sapid::password_input().field(field).default_value("short")),
        sapid::field("password-readonly")
            .theme(theme.clone())
            .label("Read-only password")
            .readonly(true)
            .control(|field| {
                sapid::password_input()
                    .field(field)
                    .default_value("Read only sample")
            }),
        sapid::field("password-disabled")
            .theme(theme)
            .label("Disabled password")
            .enabled(false)
            .control(|field| {
                sapid::password_input()
                    .field(field)
                    .default_value("Disabled sample")
            }),
    ))
}
