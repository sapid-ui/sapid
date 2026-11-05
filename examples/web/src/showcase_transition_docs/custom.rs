use glory::prelude::*;
use sapid::{TransitionEffect, TransitionStyles};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(true);
        let child_theme = theme.clone();
        let closed = TransitionStyles {
            opacity: 300,
            scale_x: 800,
            scale_y: 500,
            translate_y: -12,
            rotate: -6,
            origin_y: 0,
            ..Default::default()
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle custom transition")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .mounted(mounted)
                    .effect(TransitionEffect::Custom(closed))
                    .duration(500)
                    .content(move || {
                        sapid::paper().theme(child_theme.clone()).fill(
                            sapid::text()
                                .theme(child_theme.clone())
                                .content("Custom top-origin presence"),
                        )
                    }),
            ))
            .show_in(scope)
    })
}
