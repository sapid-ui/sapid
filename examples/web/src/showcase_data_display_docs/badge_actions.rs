use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let reviewed = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).gap(10.).fill((
                    sapid::badge()
                        .theme(theme.clone())
                        .variant(sapid::BadgeVariant::Dot)
                        .content("Unread"),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Review comments")
                        .on_press(move || reviewed.revise(|v| *v = true)),
                )),
                sapid::text()
                    .theme(theme.clone())
                    .content(reviewed.map(|v| {
                        if *v {
                            "Comments marked as reviewed.".to_owned()
                        } else {
                            "Three comments need your review.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
