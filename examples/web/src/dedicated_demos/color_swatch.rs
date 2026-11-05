use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(Color(34, 139, 90, 255));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(10.).fill(
                    [
                        (Color(34, 139, 90, 255), "Forest green"),
                        (Color(35, 115, 210, 255), "Lake blue"),
                        (Color(234, 165, 35, 180), "Warm amber"),
                    ]
                    .into_iter()
                    .map(|(color, name)| {
                        sapid::color_swatch(color)
                            .theme(theme.clone())
                            .name(name)
                            .round(true)
                            .side(36.)
                            .pressed(selected.map(move |s| *s == color))
                            .on_pressed_change(move |pressed| {
                                if pressed {
                                    selected.revise(|s| *s = color)
                                }
                            })
                    })
                    .collect::<Vec<_>>(),
                ),
                sapid::text().theme(theme.clone()).content(
                    selected.map(|c| {
                        format!("Selected color: rgba({}, {}, {}, {})", c.0, c.1, c.2, c.3)
                    }),
                ),
            ))
            .show_in(scope)
    })
}
