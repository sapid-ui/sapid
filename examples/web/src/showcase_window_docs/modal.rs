use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let modal = scope.cage(false);
        let content_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::button().theme(theme.clone()).label("Open review notes").on_press(move || open.revise(|old| *old = true)),
            sapid::text().theme(theme.clone()).content("The notes panel is nonmodal. Its confirmation uses the shared modal Dialog."),
            sapid::floating_window("review-notes").theme(theme.clone()).title("Review notes")
                .open(open).on_open_change(move |next| { open.revise(|old| *old = next); if !next { modal.revise(|old| *old = false); } })
                .default_bounds(sapid::WindowBounds { x: 32., y: 96., width: 300., height: 240. })
                .content(move || {
                    let dialog_theme = content_theme.clone();
                    sapid::stack().theme(content_theme.clone()).gap(12.).fill((
                        sapid::text_field().theme(content_theme.clone()).name("Review draft").default_value("Ready for review"),
                        sapid::dialog("review-confirmation").theme(content_theme.clone()).label("Request confirmation")
                            .title("Confirm review").open(modal).on_open_change(move |next| modal.revise(|old| *old = next))
                            .content(move || sapid::text().theme(dialog_theme.clone()).content("Close this confirmation or press Escape to return to the notes panel.")),
                    ))
                }),
        )).show_in(scope)
    })
}
