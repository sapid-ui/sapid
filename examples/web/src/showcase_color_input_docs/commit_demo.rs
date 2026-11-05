use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let commit_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        let committed = scope.cage("No committed change yet".to_owned());
        sapid::stack()
            .theme(commit_theme.clone())
            .gap(8.)
            .fill((
                sapid::color_input("showcase-color-commit")
                    .theme(commit_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next))
                    .on_commit(move |next| {
                        committed.revise(|text| *text = format!("Committed: {next}"))
                    }),
                glory::ui::text().content(scope.bond(move || committed.get())),
            ))
            .show_in(scope)
    })
}
