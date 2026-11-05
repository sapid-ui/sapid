use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(true);
        let keep_theme = theme.clone();
        let unmount_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle both editors")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .mounted(mounted)
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .content(move || {
                        sapid::text_field()
                            .theme(keep_theme.clone())
                            .name("Kept editor")
                            .default_value("Kept initial value")
                    }),
                sapid::transition()
                    .mounted(mounted)
                    .content_policy(sapid::ContentPolicy::Unmount)
                    .content(move || {
                        sapid::text_field()
                            .theme(unmount_theme.clone())
                            .name("Unmounted editor")
                            .default_value("Fresh initial value")
                    }),
            ))
            .show_in(scope)
    })
}
