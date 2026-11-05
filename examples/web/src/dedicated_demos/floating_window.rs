use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let notes = scope.cage("Plan the next release together.".to_owned());
        let content_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::button().theme(theme.clone()).label("Open notes")
                .on_press(move || open.revise(|old| *old = true)),
            sapid::text().theme(theme.clone()).content("Open a floating notes panel. Drag its title bar or resize with the corner handle."),
            sapid::floating_window("notes-preview").theme(theme.clone()).title("Release notes")
                .open(open).on_open_change(move |next| open.revise(|old| *old = next))
                .default_bounds(sapid::WindowBounds { x: 32., y: 96., width: 300., height: 240. })
                .limits(sapid::WindowLimits { min_width: 240., min_height: 180., max_width: 720., max_height: 600. })
                .content(move || sapid::field("notes-editor").theme(content_theme.clone()).label("Notes")
                    .control(move |field| sapid::textarea().field(field).value(notes).on_value_change(move |next| notes.revise(|old| *old = next)))),
        )).show_in(scope)
    })
}
