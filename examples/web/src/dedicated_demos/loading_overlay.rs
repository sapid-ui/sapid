use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let draft = scope.cage("Personal workspace".to_owned());
        let saved = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Saving workspace settings")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::loading_overlay(loading)
                    .theme(theme.clone())
                    .name("Workspace settings")
                    .label("Saving workspace")
                    .opacity(0.7)
                    .style(
                        Style::new()
                            .padding(Edges::all(16.))
                            .border_width(Edges::all(1.))
                            .border_style(BorderStyle::Solid)
                            .radius(8.),
                    )
                    .fill(
                        sapid::stack().theme(theme.clone()).gap(12.).fill((
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Workspace settings name")
                                .value(draft)
                                .on_value_change(move |v| draft.revise(|old| *old = v)),
                            sapid::button()
                                .theme(theme.clone())
                                .label("Save workspace settings")
                                .on_press(move || {
                                    saved.revise(|v| *v = true);
                                    loading.revise(|v| *v = true);
                                }),
                        )),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        if loading.get() {
                            "Saving settings… Clear the saving checkbox to finish.".to_owned()
                        } else if saved.get() {
                            format!("Saved workspace: {}", draft.get())
                        } else {
                            "Edit your workspace, then save its settings.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
