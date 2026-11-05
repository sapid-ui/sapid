use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let draft = scope.cage("Keep this note".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Hide note behind placeholder")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::skeleton()
                    .theme(theme.clone())
                    .loading(loading)
                    .fill(
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Retained workspace note")
                            .value(draft)
                            .on_value_change(move |v| draft.revise(|old| *old = v)),
                    ),
            ))
            .show_in(scope)
    })
}
