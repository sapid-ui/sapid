use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::toc_context(scope, "getting-started")?;
        let article_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(20.).fill((
            sapid::table_of_contents(context.clone(), "guide-index").theme(theme.clone()).name("Getting started sections"),
            sapid::toc_document(context, "guide-article").theme(theme.clone())
                .name("Getting started article").offset(12.)
                .style(Style::new().height(Length::Px(300.)))
                .content(move |context| {
                    sapid::stack().theme(article_theme.clone()).gap(24.).fill(vec![
                        section(&context, &article_theme, "overview", "Build your first workspace", 2, 0,
                            ["A workspace keeps your project settings and shared resources together. Start with one small project and add more as your team grows.",
                            "Choose a clear name and a short description so collaborators can find the right place to work. Your application owns saving and validation."]),
                        section(&context, &article_theme, "setup", "Set up your project", 2, 1,
                            ["Create the project structure before adding integrations. Keep reusable widgets separate from application data and host services.",
                            "Use explicit keys for data that can move, and keep accepted values in application-owned state. This makes programmatic updates and refused proposals predictable."]),
                        section(&context, &article_theme, "native", "Choose a host", 3, 2,
                            ["The same widgets can be hosted in a browser or a supported native document engine. Pick a host according to the features your application needs.",
                            "Enable optional services explicitly. Resource loading, clipboard access and file selection belong to the application's host configuration."]),
                        section(&context, &article_theme, "share", "Share with your team", 2, 3,
                            ["Review the project with a collaborator before inviting the whole team. Make labels, unavailable states and navigation understandable at narrow widths.",
                            "This index follows registered headings inside this scrolling article. Following a link scrolls the article and preserves the surrounding page position."]),
                    ])
                }),
        )).show_in(scope)
    })
}
fn section(
    context: &sapid::TocContext,
    theme: &sapid::Theme,
    key: &str,
    title: &str,
    level: u8,
    order: u32,
    paragraphs: [&str; 2],
) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::toc_heading(context.clone(), key, level)
            .theme(theme.clone())
            .order(order)
            .content(title.to_owned())
            .style(Style::new().font_weight(400.)),
        sapid::text()
            .theme(theme.clone())
            .content(paragraphs[0].to_owned()),
        sapid::text()
            .theme(theme.clone())
            .content(paragraphs[1].to_owned()),
    ))
}
