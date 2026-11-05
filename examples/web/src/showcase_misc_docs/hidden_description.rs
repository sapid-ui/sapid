use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let description = glory::render::SemanticId::new("project-share-help").unwrap();
        let shared = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::visually_hidden()
                    .semantic_id(description.clone())
                    .content("Only members of your workspace can open this project."),
                glory::ui::button()
                    .label("Share project")
                    .semantics(Some(Box::new(Semantics {
                        described_by: Some(description),
                        ..Default::default()
                    })))
                    .style(Style {
                        background: Some(theme.surface),
                        color: Some(theme.foreground),
                        border_color: Some(theme.border),
                        ..Style::new()
                            .padding(Edges::all(10.))
                            .border_width(Edges::all(1.))
                            .border_style(glory::style::BorderStyle::Solid)
                            .radius(6.)
                    })
                    .on(glory::render::EventKind::Activate, move |_| {
                        shared.revise(|v| *v = true)
                    }),
                sapid::text().theme(theme.clone()).content(shared.map(|v| {
                    if *v {
                        "Project link is ready for your workspace.".to_owned()
                    } else {
                        "The visible button name and its extra description are separate.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
