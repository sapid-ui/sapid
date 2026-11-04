use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            (
                "retained",
                sapid::ContentPolicy::KeepMounted,
                "Retained tab draft",
            ),
            (
                "retired",
                sapid::ContentPolicy::Unmount,
                "Unmounted tab draft",
            ),
        ]
        .into_iter()
        .map(|(key, policy, label)| {
            let input_theme = theme.clone();
            sapid::tabs(format!("draft-tabs-{key}"))
                .theme(theme.clone())
                .name(label)
                .content_policy(policy)
                .tab(sapid::tab("draft").label("Draft").content(move || {
                    sapid::text_field()
                        .theme(input_theme.clone())
                        .name(label)
                        .placeholder("Write a draft, switch tabs and return")
                }))
                .tab(
                    sapid::tab("preview")
                        .label("Preview")
                        .content(|| text().content("The application owns any preview generation.")),
                )
        })
        .collect::<Vec<_>>(),
    )
}
