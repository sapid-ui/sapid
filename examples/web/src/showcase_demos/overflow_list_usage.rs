use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("Overview".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::overflow_list("project-navigation")
                    .theme(theme.clone())
                    .overflow_label("More")
                    .style(Style::new().width(Length::Percent(100.)))
                    .items(
                        [
                            "Overview",
                            "Activity",
                            "Members",
                            "Documents",
                            "Settings",
                            "Archive",
                        ]
                        .into_iter()
                        .map(|name| {
                            let t = theme.clone();
                            sapid::overflow_item(name).content(move || {
                                sapid::button()
                                    .theme(t.clone())
                                    .variant(sapid::ButtonVariant::Outline)
                                    .label(name)
                                    .on_press(move || selected.revise(|s| *s = name.to_owned()))
                            })
                        }),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|s| format!("Current project page: {s}"))),
            ))
            .show_in(scope)
    })
}
