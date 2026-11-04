use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(4.).fill((
        sapid::nav_link("workspace-overview")
            .theme(theme.clone())
            .label("Overview")
            .href("#overview")
            .current(true)
            .left_section(text().content("⌂"))
            .description("Recent activity across your workspace"),
        sapid::nav_link("workspace-projects")
            .theme(theme.clone())
            .label("Projects")
            .href("#projects")
            .left_section(text().content("▤"))
            .right_section(sapid::badge().theme(theme.clone()).content("12")),
        sapid::nav_link("workspace-settings")
            .theme(theme.clone())
            .label("Settings")
            .href("#settings")
            .left_section(text().content("⚙")),
    ))
}
