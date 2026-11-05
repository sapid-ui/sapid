use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let focused = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::anchor()
                    .theme(theme.clone())
                    .label("Skip to project content")
                    .href("#project-focus-content")
                    .class("misc-focus-link")
                    .style(focused.map(|v| {
                        if *v {
                            Style::new().position(glory::style::Position::Static)
                        } else {
                            Style::new()
                                .position(glory::style::Position::Absolute)
                                .width(Length::Px(1.))
                                .height(Length::Px(1.))
                                .overflow_x(glory::style::Overflow::Hidden)
                                .overflow_y(glory::style::Overflow::Hidden)
                        }
                    }))
                    .on(glory::render::EventKind::FocusChanged, move |event| {
                        if let glory::render::Event::FocusChanged(value) = event {
                            focused.revise(|v| *v = value)
                        }
                    }),
                sapid::field("focus-link-note")
                    .theme(theme.clone())
                    .label("Try Tab from this note")
                    .control(|field| sapid::text_field().field(field)),
                glory::ui::view()
                    .semantics(Some(Box::new(glory::render::Semantics {
                        id: Some(glory::render::SemanticId::new("project-focus-content").unwrap()),
                        ..Default::default()
                    })))
                    .fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("Project content is available here."),
                    ),
            ))
            .show_in(scope)
    })
}
