use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let commit_theme = theme.clone();
    widget(move |scope| {
        let committed = scope.cage("Move the slider".to_owned());
        sapid::stack()
            .theme(commit_theme.clone())
            .gap(10.)
            .fill((
                sapid::slider("showcase-slider-commit")
                    .theme(commit_theme.clone())
                    .default_value(30.)
                    .on_value_commit(move |value| {
                        committed.revise(|text| *text = format!("Committed: {value}"))
                    }),
                glory::ui::text().content(scope.bond(move || committed.get())),
            ))
            .show_in(scope)
    })
}
