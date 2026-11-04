use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let keys = scope.cage(vec!["shipping".to_owned(), "payments".to_owned()]);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Add account section")
                    .on_press(move || {
                        keys.revise(|items| {
                            if !items.iter().any(|key| key == "account") {
                                items.push("account".into());
                            }
                        })
                    }),
                sapid::accordion("keyed-questions")
                    .theme(theme.clone())
                    .heading_level(4)
                    .regions(true)
                    .items(keys, |key| {
                        let description = format!("Information about {key}.");
                        sapid::accordion_item(key)
                            .label(key.to_owned())
                            .content(move || text().content(description.clone()))
                    }),
            ))
            .show_in(scope)
    })
}
