use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::toc_context(scope, "nested-guide")?;
        let entries = context.entries(scope);
        let current = scope.cage(None::<glory::render::SemanticId>);
        let article_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
   sapid::table_of_contents(context.clone(),"nested-index").theme(theme.clone()).size(sapid::Size::Small).name("Nested article sections").current(current),
   sapid::toc_document(context,"nested-article").theme(theme.clone()).name("Nested heading article").style(Style::new().height(Length::Px(240.))).content(move|ctx|{
    sapid::stack().theme(article_theme.clone()).gap(20.).fill((
     sapid::toc_heading(ctx.clone(),"workspace",2).theme(article_theme.clone()).content("Workspace").order(0),
     sapid::text().theme(article_theme.clone()).content("The article begins at heading level two. Relative indentation follows its registered hierarchy, rather than the surrounding documentation page."),
     sapid::toc_heading(ctx.clone(),"project",3).theme(article_theme.clone()).content("Project").order(1),
     sapid::text().theme(article_theme.clone()).content("A project belongs to the workspace. Labels and hierarchy are explicit; unrelated headings elsewhere on the page never enter this index."),
     sapid::toc_heading(ctx.clone(),"settings",4).theme(article_theme.clone()).content("Project settings").order(2),
     sapid::text().theme(article_theme.clone()).content("Settings form a deeper section. This example supplies the active marker from application state; clicking the control below changes the marker without scrolling the article."),
     sapid::toc_heading(ctx,"sharing",2).theme(article_theme.clone()).content("Sharing").order(3),
    ))
   }),
   sapid::button().theme(theme.clone()).label("Mark project settings as current").on_press(move||{
    if let Some(entry)=entries.get().into_iter().find(|entry|entry.label=="Project settings") {current.revise(|value|*value=Some(entry.id));}
   }),
  )).show_in(scope)
    })
}
