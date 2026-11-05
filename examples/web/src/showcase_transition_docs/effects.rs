use glory::prelude::*;
use sapid::{TransitionEffect as Effect, TransitionTiming as Timing};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(true);
        let effect = scope.cage(Effect::Fade);
        let content_theme = theme.clone();
        let choices = [
            ("Fade", Effect::Fade),
            ("Scale", Effect::Scale),
            ("Slide left", Effect::SlideLeft),
        ];
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill(
                    choices
                        .into_iter()
                        .map(|(label, value)| {
                            sapid::button()
                                .theme(theme.clone())
                                .label(label)
                                .on_press(move || effect.revise(|old| *old = value))
                        })
                        .collect::<Vec<_>>(),
                ),
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle effect example")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .mounted(mounted)
                    .effect(effect)
                    .duration(450)
                    .timing(Timing::EaseOut)
                    .distance(24)
                    .content(move || {
                        sapid::paper().theme(content_theme.clone()).fill(
                            sapid::text()
                                .theme(content_theme.clone())
                                .content("Fade, scale and slide content"),
                        )
                    }),
            ))
            .show_in(scope)
    })
}
