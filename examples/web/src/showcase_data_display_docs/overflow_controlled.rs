use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let allow = scope.cage(false);
        let open = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow more tags popup")
                    .checked(allow)
                    .on_checked_change(move |v| allow.revise(|s| *s = v)),
                sapid::overflow_list("controlled-tags")
                    .theme(theme.clone())
                    .visible_count(1usize)
                    .open(open)
                    .on_open_change(move |v| {
                        if !v || allow.get() {
                            open.revise(|s| *s = v)
                        }
                    })
                    .overflow_label("More")
                    .items(
                        ["Design", "Accessibility", "Release", "Research"]
                            .into_iter()
                            .map(|name| {
                                let t = theme.clone();
                                sapid::overflow_item(name)
                                    .content(move || sapid::badge().theme(t.clone()).content(name))
                            }),
                    ),
            ))
            .show_in(scope)
    })
}
