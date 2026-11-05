use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(12.)
        .fill((
            sapid::code()
                .theme(theme.clone())
                .content("let answer = 42;"),
            sapid::code()
                .theme(theme.clone())
                .style(Style {
                    background: Some(theme.surface),
                    color: Some(theme.foreground),
                    ..Style::new()
                })
                .content("<button>Save & close</button>"),
            sapid::anchor()
                .theme(theme.clone())
                .label("Explore CodeHighlight")
                .href("?widget=code-highlight"),
        ))
}
