use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let mounted = scope.cage(false);
        let log = scope.cage(Vec::<&'static str>::new());
        let content_theme = theme.clone();
        let record = move |name| {
            log.revise(|v| {
                if v.len() == 8 {
                    v.remove(0);
                }
                v.push(name);
            })
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle delayed transition")
                    .on_press(move || mounted.revise(|v| *v = !*v)),
                sapid::transition()
                    .mounted(mounted)
                    .duration(300)
                    .exit_duration(600)
                    .enter_delay(150)
                    .exit_delay(100)
                    .on_enter(move || record("enter"))
                    .on_entered(move || record("entered"))
                    .on_exit(move || record("exit"))
                    .on_exited(move || record("exited"))
                    .content(move || {
                        sapid::paper().theme(content_theme.clone()).fill(
                            sapid::text()
                                .theme(content_theme.clone())
                                .content("Delayed content"),
                        )
                    }),
                sapid::text().theme(theme.clone()).content(log.map(|v| {
                    if v.is_empty() {
                        "No lifecycle events".into()
                    } else {
                        v.join(" → ")
                    }
                })),
            ))
            .show_in(scope)
    })
}
