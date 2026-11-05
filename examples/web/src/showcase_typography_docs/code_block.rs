use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::code()
        .theme(theme)
        .class("code-doc-block")
        .content(CODE)
        .style(
            Style::new()
                .display(glory::style::Display::Block)
                .width(Length::Percent(100.))
                .white_space(glory::style::WhiteSpace::Pre)
                .overflow_x(glory::style::Overflow::Auto)
                .padding(Edges::all(12.)),
        )
}
const CODE: &str = r#"let count = scope.cage(0);
count.revise(|value| *value += 1);
let message = format!("The current project count is {} and the workspace remains ready for another update", count.get());"#;
