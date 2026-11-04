use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let keys = scope.cage(vec![
            "overview".to_owned(),
            "activity".to_owned(),
            "notes".to_owned(),
        ]);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Reverse tab order")
                    .on_press(move || keys.revise(|items| items.reverse())),
                sapid::tabs("keyed-workspace")
                    .theme(theme.clone())
                    .name("Keyed workspace")
                    .tabs(keys, |key| {
                        let label = key.to_owned();
                        sapid::tab(key)
                            .label(label.clone())
                            .content(move || sapid::text_field().name(format!("{label} draft")))
                    }),
            ))
            .show_in(scope)
    })
}
