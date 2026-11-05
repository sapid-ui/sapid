use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        let draft = scope.cage("Personal workspace".to_owned());
        let field_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Loading workspace card")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::skeleton()
                    .theme(theme.clone())
                    .name("Workspace card")
                    .loading(loading)
                    .fill(
                        sapid::field("skeleton-workspace-field")
                            .theme(theme.clone())
                            .label("Workspace name")
                            .description("Your draft stays mounted while the placeholder is shown.")
                            .control(move |field| {
                                sapid::text_field()
                                    .theme(field_theme.clone())
                                    .field(field)
                                    .value(draft)
                                    .on_value_change(move |v| draft.revise(|old| *old = v))
                            }),
                    ),
            ))
            .show_in(scope)
    })
}
