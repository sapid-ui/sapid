use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let feedback = scope.cage("Enter at most three keywords".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("tags-limit-field")
                    .theme(theme.clone())
                    .label("Up to three keywords")
                    .control(move |field| {
                        sapid::tags_input("tags-limit")
                            .field(field)
                            .max_tags(3)
                            .default_value(vec!["Rust".into()])
                            .clearable(true)
                            .clear_label("Clear all keywords")
                            .on_reject(move |reason| {
                                feedback.revise(|v| {
                                    *v = match reason {
                                        sapid::TagRejection::Duplicate => {
                                            "This keyword already exists"
                                        }
                                        sapid::TagRejection::TooMany => {
                                            "At most three keywords are allowed"
                                        }
                                        sapid::TagRejection::TagTooLong => "Use a shorter keyword",
                                        _ => "These keywords could not be accepted",
                                    }
                                    .to_owned()
                                })
                            })
                    }),
                sapid::text().theme(theme.clone()).content(feedback),
            ))
            .show_in(scope)
    })
}
