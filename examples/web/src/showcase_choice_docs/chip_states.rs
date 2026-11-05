use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::chip().theme(theme.clone()).label("A long selectable label that wraps inside a narrow parent without losing its native checkbox")
            .style(Style::new().max_width(Limit::Percent(100.))),
        sapid::chip().theme(theme.clone()).label("Read-only chip").default_checked(true).readonly(true),
        sapid::chip().theme(theme.clone()).label("Disabled chip").default_checked(true).enabled(false),
        sapid::chip_group("chip-unavailable").theme(theme.clone()).label("Available plans")
            .default_value("starter").items([sapid::chip_item("starter", "Starter"),
                sapid::chip_item("team", "Team"), sapid::chip_item("enterprise", "Enterprise").enabled(false)]),
    ))
}
