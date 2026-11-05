use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(false);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(12.).fill((
                    sapid::color_swatch(Color(35, 115, 210, 255))
                        .theme(theme.clone())
                        .name("Controlled lake blue")
                        .pressed(selected)
                        .on_pressed_change(move |v| {
                            if accept.get() {
                                selected.revise(|s| *s = v)
                            }
                        }),
                    sapid::color_swatch(Color(170, 170, 170, 255))
                        .theme(theme.clone())
                        .name("Unavailable gray")
                        .enabled(false)
                        .interactive(true),
                )),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept swatch selection")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|s| format!("Lake blue selected: {s}"))),
            ))
            .show_in(scope)
    })
}
