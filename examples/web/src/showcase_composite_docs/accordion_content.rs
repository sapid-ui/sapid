use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            (
                "retained",
                sapid::ContentPolicy::KeepMounted,
                "Retained draft",
            ),
            ("retired", sapid::ContentPolicy::Unmount, "Unmounted draft"),
        ]
        .into_iter()
        .map(|(key, policy, label)| {
            let input_theme = theme.clone();
            sapid::accordion(format!("draft-{key}"))
                .theme(theme.clone())
                .default_open(["note"])
                .content_policy(policy)
                .item(sapid::accordion_item("note").label(label).content(move || {
                    sapid::text_field()
                        .theme(input_theme.clone())
                        .name(label)
                        .placeholder("Write a note, close and reopen")
                }))
        })
        .collect::<Vec<_>>(),
    )
}
