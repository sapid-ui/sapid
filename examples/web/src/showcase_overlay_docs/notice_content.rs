use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            (sapid::ContentPolicy::KeepMounted, "retained"),
            (sapid::ContentPolicy::Unmount, "recreated"),
        ]
        .into_iter()
        .map(|(policy, key)| {
            let body = theme.clone();
            sapid::floating_dialog(format!("notice-{key}"))
                .theme(theme.clone())
                .label(format!("Open {key} notice"))
                .title(format!("{key} note"))
                .content_policy(policy)
                .content(move || {
                    sapid::text_field()
                        .theme(body.clone())
                        .name(format!("{key} notice draft"))
                        .default_value("Initial draft")
                })
        })
        .collect::<Vec<_>>(),
    )
}
