use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        sapid::breadcrumbs("long-path")
            .theme(theme.clone())
            .max_items(4)
            .expanded(open)
            .on_expanded_change(move |next| open.revise(|old| *old = next))
            .items(
                [
                    ("home", "Home"),
                    ("docs", "Docs"),
                    ("widgets", "Widgets"),
                    ("navigation", "Navigation"),
                    ("breadcrumbs", "Breadcrumbs"),
                ]
                .into_iter()
                .map(|(key, label)| sapid::breadcrumb_item(key, label).href(format!("#{key}"))),
            )
            .show_in(scope)
    })
}
