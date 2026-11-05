use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).fill((
        sapid::code_highlight()
            .theme(theme.clone())
            .name("Themed source")
            .source(
                "pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n",
            ),
        sapid::code_highlight()
            .theme(theme)
            .name("Unstyled source")
            .source(
                "pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n",
            )
            .unstyled(),
    ))
}
