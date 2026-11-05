use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::toc_context(scope, "offset-guide")?;
        let entries = context.entries(scope);
        let reveal = context.clone();
        let article_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
   sapid::table_of_contents(context.clone(),"offset-index").theme(theme.clone()).name("Offset article sections"),
   sapid::toc_document(context,"offset-article").theme(theme.clone()).name("Article with a heading offset").offset(32.).style(Style::new().height(Length::Px(220.))).content(move|ctx|{
    sapid::stack().theme(article_theme.clone()).gap(24.).fill((
     sapid::toc_heading(ctx.clone(),"start",2).theme(article_theme.clone()).content("Start with one project").order(0),
     sapid::text().theme(article_theme.clone()).content("Start with a small project and give it a clear purpose. Choose a name, a description and the people who can edit it. Keep the application state separate from the presentation so updates remain predictable."),
     sapid::toc_heading(ctx.clone(),"review",2).theme(article_theme.clone()).content("Review the configuration").order(1),
     sapid::text().theme(article_theme.clone()).content("Review the configuration before sharing. Keyboard focus, labels, unavailable controls and errors should remain understandable at narrow widths. Each heading in this article registers with its explicit context and retires with its owner."),
     sapid::toc_heading(ctx,"publish",2).theme(article_theme.clone()).content("Publish the guide").order(2),
     sapid::text().theme(article_theme.clone()).content("Share a preview and gather feedback from collaborators. Following the table of contents scrolls this local document. The offset leaves space above the revealed heading and does not change the surrounding page's scroll position."),
    ))
   }),
   sapid::button().theme(theme.clone()).label("Reveal publishing section").on_press(move||{
    if let Some(entry)=entries.get().into_iter().find(|entry|entry.label=="Publish the guide") {reveal.reveal(&entry.id).expect("live guide");}
   }),
  )).show_in(scope)
    })
}
