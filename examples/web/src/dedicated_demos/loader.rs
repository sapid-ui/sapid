use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::group().theme(theme.clone()).gap(12.).fill((
                    sapid::loader()
                        .theme(theme.clone())
                        .name("Loading workspace")
                        .animated(animated),
                    sapid::text()
                        .theme(theme.clone())
                        .content("Loading your workspace…"),
                )),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate loading indicator")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
