use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(
        [
            (Color(0, 0, 0, 255), 0.2),
            (theme.accent, 0.35),
            (Color(0, 0, 0, 128), 0.5),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (color, opacity))| {
            sapid::overlay()
                .theme(theme.clone())
                .color(color)
                .opacity(opacity)
                .fill(
                    sapid::card().theme(theme.clone()).fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content(format!("Shade sample {} keeps its content opacity", i + 1)),
                    ),
                )
        })
        .collect::<Vec<_>>(),
    )
}
