use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("text-name")
        .theme(theme)
        .label("Your name")
        .description("How should we address you?")
        .control(|field| sapid::text_field().field(field).placeholder("Ada Lovelace"))
}
