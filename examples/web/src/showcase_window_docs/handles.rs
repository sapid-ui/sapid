use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let mounted = scope.cage(true);
        let enabled = scope.cage(true);
        let handle = scope.cage(None::<sapid::FloatingWindowHandle>);
        let result = scope.cage("No handle command requested".to_owned());
        let window_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Mount notes window")
                    .checked(mounted)
                    .on_checked_change(move |next| mounted.revise(|old| *old = next)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable notes window")
                    .checked(enabled)
                    .on_checked_change(move |next| enabled.revise(|old| *old = next)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Open handled notes")
                    .enabled(mounted)
                    .on_press(move || open.revise(|old| *old = true)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Raise current window")
                    .on_press(move || {
                        let message = match handle.get().map(|handle| handle.raise()) {
                            Some(Ok(())) => "Raised the current window",
                            _ => "No available window: open or remount it first",
                        };
                        result.revise(|old| *old = message.to_owned());
                    }),
                sapid::text().theme(theme.clone()).content(result),
                glory::ui::when(mounted, move || {
                    let inner = window_theme.clone();
                    sapid::floating_window("handled-notes")
                        .theme(window_theme.clone())
                        .title("Current notes window")
                        .open(open)
                        .on_open_change(move |next| open.revise(|old| *old = next))
                        .enabled(enabled)
                        .default_bounds(sapid::WindowBounds {
                            x: 32.,
                            y: 96.,
                            width: 300.,
                            height: 240.,
                        })
                        .on_ready(move |next| handle.revise(|old| *old = Some(next)))
                        .content(move || {
                            sapid::text_field()
                                .theme(inner.clone())
                                .name("Handled draft")
                                .default_value("This editor follows the window's enabled state")
                        })
                }),
            ))
            .show_in(scope)
    })
}
