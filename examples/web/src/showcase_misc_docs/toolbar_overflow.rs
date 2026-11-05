use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::toolbar("compact-review-tools")
            .theme(theme.clone())
            .name("Compact review actions")
            .group(sapid::toolbar_group("review").buttons([
                sapid::button().label("Approve"),
                sapid::button().label("Comment"),
            ])),
        sapid::overflow_list("extra-review-actions")
            .theme(theme.clone())
            .visible_count(1usize)
            .overflow_label("More")
            .items(["Share", "Export", "Archive"].into_iter().map(|name| {
                let theme = theme.clone();
                sapid::overflow_item(name).content(move || {
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Outline)
                        .label(name)
                })
            })),
    ))
}
