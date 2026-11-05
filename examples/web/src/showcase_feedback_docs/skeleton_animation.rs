use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::skeleton()
                    .theme(theme.clone())
                    .name("Animated placeholder")
                    .height(28.)
                    .animated(animated),
                sapid::skeleton()
                    .theme(theme.clone())
                    .name("Static placeholder")
                    .height(28.)
                    .animated(false),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate placeholders")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
