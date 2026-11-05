use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let narrow = scope.cage(false);
        let current = scope.cage("Overview".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use narrow navigation")
                    .checked(narrow)
                    .on_checked_change(move |v| narrow.revise(|s| *s = v)),
                sapid::overflow_list("width-navigation")
                    .theme(theme.clone())
                    .overflow_label("More")
                    .style(narrow.map(|v| {
                        Style::new().width(if *v {
                            Length::Px(180.)
                        } else {
                            Length::Percent(100.)
                        })
                    }))
                    .items(
                        ["Overview", "Design", "Research", "Resources", "Settings"]
                            .into_iter()
                            .map(|name| {
                                let t = theme.clone();
                                sapid::overflow_item(name).content(move || {
                                    sapid::button()
                                        .theme(t.clone())
                                        .variant(sapid::ButtonVariant::Outline)
                                        .label(name)
                                        .on_press(move || current.revise(|s| *s = name.to_owned()))
                                })
                            }),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(current.map(|s| format!("Selected workspace section: {s}"))),
            ))
            .show_in(scope)
    })
}
