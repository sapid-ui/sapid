use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("json-settings").theme(theme).label("Package configuration")
        .description("Edit JSON, then leave the field to format valid content.")
        .control(|field|sapid::json_input("json-settings-editor").field(field).rows(5)
            .default_value("{\"name\":\"my-project\",\"private\":true,\"scripts\":{\"build\":\"cargo build\"}}")
            .format_on_blur(true).show_formatter(false))
}
