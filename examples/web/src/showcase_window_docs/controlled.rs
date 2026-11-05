use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let accept = scope.cage(true);
        let bounds = scope.cage(sapid::WindowBounds {
            x: 32.,
            y: 96.,
            width: 300.,
            height: 260.,
        });
        let content_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::checkbox().theme(theme.clone()).label("Accept window geometry").checked(accept).on_checked_change(move |next| accept.revise(|old| *old = next)),
            sapid::button().theme(theme.clone()).label("Open controlled window").on_press(move || open.revise(|old| *old = true)),
            sapid::text().theme(theme.clone()).content(bounds.map(|value| format!("Accepted bounds: {:.0}, {:.0}; {:.0} × {:.0}", value.x, value.y, value.width, value.height))),
            sapid::floating_window("controlled-notes").theme(theme.clone()).title("Controlled geometry")
                .open(open).on_open_change(move |next| open.revise(|old| *old = next))
                .bounds(bounds).on_bounds_change(move |next| { if accept.get() { bounds.revise(|old| *old = next); } })
                .limits(sapid::WindowLimits { min_width: 240., min_height: 180., max_width: 640., max_height: 560. })
                .keyboard_step(20.).default_actions_open(true)
                .content(move || sapid::text().theme(content_theme.clone()).content("Drag the title, resize the corner, or use the movement and size buttons. Refused proposals keep the accepted box.")),
        )).show_in(scope)
    })
}
