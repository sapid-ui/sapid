use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(false);
        let animated = scope.cage(false);
        let child_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable requested animation")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle reduced motion example")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .mounted(mounted)
                    .animated(animated)
                    .duration(350)
                    .appear(true)
                    .content(move || {
                        sapid::text()
                            .theme(child_theme.clone())
                            .content("Motion policy content")
                    }),
            ))
            .show_in(scope)
    })
}
