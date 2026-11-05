use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let style = Style {
        color: Some(theme.foreground),
        background: Some(theme.input),
        border_color: Some(theme.border),
        ..Style::new()
            .padding(Edges::all(8.))
            .border_width(Edges::all(1.))
            .border_style(BorderStyle::Solid)
            .radius(theme.radius)
    };
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::tooltip("project-search-help")
            .theme(theme.clone())
            .text("Search by project title or team member.")
            .target(move |target| {
                target.input(
                    glory::ui::input()
                        .name("Project search with help")
                        .style(style.clone()),
                )
            }),
        sapid::text()
            .theme(theme)
            .content("Search by project title or team member."),
    ))
}
