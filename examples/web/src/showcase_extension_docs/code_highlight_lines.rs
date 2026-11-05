use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::code_highlight()
        .theme(theme)
        .name("Numbered source")
        .source("pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n")
        .line_numbers(true)
        .start_line(20)
        .highlighted_lines([2])
}
