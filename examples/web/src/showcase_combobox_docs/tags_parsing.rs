use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill(
        [
            (sapid::TagDuplicatePolicy::Reject, "Reject duplicates"),
            (sapid::TagDuplicatePolicy::Ignore, "Ignore duplicates"),
            (
                sapid::TagDuplicatePolicy::Allow,
                "Allow duplicate occurrences",
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (policy, label))| {
            let theme = theme.clone();
            widget(move |scope| {
                let feedback =
                    scope.cage("Try Rust twice, separated by comma or semicolon".to_owned());
                sapid::stack()
                    .theme(theme.clone())
                    .gap(8.)
                    .fill((
                        sapid::field(format!("tags-policy-field-{i}"))
                            .theme(theme.clone())
                            .label(label)
                            .control(move |field| {
                                sapid::tags_input(format!("tags-policy-{i}"))
                                    .field(field)
                                    .default_value(vec!["Rust".into()])
                                    .duplicate_policy(policy)
                                    .separators(glory::render::TagSeparators::EnterCommaSemicolon)
                                    .max_tags(3)
                                    .on_reject(move |_| {
                                        feedback.revise(|v| {
                                            *v = "The complete keyword proposal was refused".into()
                                        })
                                    })
                            }),
                        sapid::text().theme(theme.clone()).content(feedback),
                    ))
                    .show_in(scope)
            })
        })
        .collect::<Vec<_>>(),
    )
}
