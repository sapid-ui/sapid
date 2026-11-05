use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::select("popover-empty")
            .theme(theme.clone())
            .name("Empty framework list")
            .placeholder("No frameworks configured")
            .empty_message("No frameworks are available"),
        sapid::select("popover-disabled")
            .theme(theme.clone())
            .name("Unavailable framework list")
            .enabled(false)
            .option(sapid::select_option("glory", "Glory")),
        sapid::select("popover-readonly")
            .theme(theme)
            .name("Read-only framework")
            .readonly(true)
            .default_value(Some("glory".into()))
            .option(sapid::select_option("glory", "Glory")),
    ))
}
