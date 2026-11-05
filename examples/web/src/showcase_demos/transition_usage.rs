use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(true);
        let status = scope.cage("Ready".to_owned());
        let content_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle transition")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .theme(theme.clone())
                    .mounted(mounted)
                    .effect(sapid::TransitionEffect::FadeUp)
                    .duration(250)
                    .exit_duration(300)
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .on_enter(move || status.revise(|s| *s = "Entering".into()))
                    .on_entered(move || status.revise(|s| *s = "Entered".into()))
                    .on_exit(move || status.revise(|s| *s = "Exiting".into()))
                    .on_exited(move || status.revise(|s| *s = "Exited".into()))
                    .content(move || {
                        sapid::paper().theme(content_theme.clone()).fill((
                            sapid::text()
                                .theme(content_theme.clone())
                                .content("Presence with a retained editor"),
                            sapid::text_field()
                                .theme(content_theme.clone())
                                .name("Transition draft")
                                .default_value("Keep my draft"),
                        ))
                    }),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
