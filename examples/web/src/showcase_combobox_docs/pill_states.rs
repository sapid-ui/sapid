use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill((
        sapid::pill().theme(theme.clone()).content("Passive topic"),
        sapid::pill()
            .theme(theme.clone())
            .content("Locked topic")
            .enabled(false)
            .removable("Remove locked topic"),
        sapid::pill()
            .theme(theme)
            .unstyled()
            .content("Custom topic")
            .style(
                Style::new()
                    .padding(glory::style::Edges::all(8.))
                    .radius(4.),
            ),
    ))
}
