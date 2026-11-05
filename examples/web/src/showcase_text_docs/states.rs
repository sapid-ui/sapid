use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("text-long-hint").theme(theme.clone()).label("Project description")
            .description("A hint is not an accessible label.")
            .control(|field|sapid::text_field().field(field)
                .placeholder("Enter a descriptive project name that your whole team will recognize, including its purpose and owner")),
        sapid::field("text-readonly").theme(theme.clone()).label("Read-only account").readonly(true)
            .control(|field|sapid::text_field().field(field).default_value("saved-account")),
        sapid::field("text-disabled").theme(theme).label("Disabled account").enabled(false)
            .control(|field|sapid::text_field().field(field).default_value("retained-account")),
    ))
}
