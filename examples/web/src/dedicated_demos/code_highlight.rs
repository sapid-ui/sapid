use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::code_highlight()
        .theme(theme)
        .name("Welcome function")
        .language(CodeLanguage::Rust)
        .source("pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n")
        .line_numbers(true)
}
