use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::field("json-blur")
            .theme(theme.clone())
            .label("Format on blur")
            .control(|field| {
                sapid::json_input("json-blur-editor")
                    .field(field)
                    .rows(4)
                    .default_value("{\"enabled\":true}")
                    .format_on_blur(true)
                    .show_formatter(false)
            }),
        sapid::field("json-explicit")
            .theme(theme.clone())
            .label("Keep whitespace until requested")
            .control(|field| {
                sapid::json_input("json-explicit-editor")
                    .field(field)
                    .rows(4)
                    .default_value("{\"enabled\":true}")
                    .format_on_blur(false)
            }),
        sapid::button().theme(theme).label("Move focus out of JSON"),
    ))
}
