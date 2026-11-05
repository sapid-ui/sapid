use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("outer-modal")
        .theme(theme)
        .label("Open project review")
        .title("Project review")
        .content(move || {
            let inner = body.clone();
            sapid::dialog("inner-modal")
                .theme(body.clone())
                .label("Open review details")
                .title("Review details")
                .initial_focus(glory::render::SemanticId::new("review-safe-action").unwrap())
                .content(move || {
                    glory::ui::button()
                        .label("Safe review action")
                        .semantics(Some(Box::new(glory::render::Semantics {
                            id: Some(glory::render::SemanticId::new("review-safe-action").unwrap()),
                            ..Default::default()
                        })))
                        .style(Style {
                            color: Some(inner.foreground),
                            background: Some(inner.surface),
                            ..Style::new().padding(Edges::all(8.))
                        })
                })
        })
}
