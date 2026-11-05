use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<String>);
        let open = scope.cage(false);
        let accept = scope.cage(false);
        let target = scope.cage(None);
        let weak = scope.downgrade();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::select("popover-controlled")
                    .theme(theme.clone())
                    .name("Controlled framework")
                    .node_ref(target)
                    .value(value)
                    .open(open)
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v)
                        }
                    })
                    .options([
                        sapid::select_option("glory", "Glory"),
                        sapid::select_option("dioxus", "Dioxus"),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept framework proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Open framework choices")
                    .on_press(move || {
                        if let Some(scope) = weak.upgrade() {
                            open.revise(|v| *v = true);
                            if let Some(node) = target.get()
                                && scope.capabilities().is_ok_and(|c| c.focus)
                                && let Err(error) = scope.action(glory::render::Action::Focus(node))
                            {
                                scope.report_error(error);
                            }
                        }
                    }),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.as_ref()
                        .map(|v| format!("Selected framework: {v}"))
                        .unwrap_or_else(|| "No framework selected".into())
                })),
            ))
            .show_in(scope)
    })
}
