use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let count = scope.cage(0usize);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Block project editing")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::loading_overlay(loading)
                    .theme(theme.clone())
                    .name("Project editor")
                    .fill(
                        sapid::stack().theme(theme.clone()).gap(12.).fill((
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Loading region retained draft")
                                .default_value("Keep this real editor"),
                            sapid::button()
                                .theme(theme.clone())
                                .label("Update project draft")
                                .on_press(move || count.revise(|v| *v += 1)),
                        )),
                    ),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|v| format!("Project updates: {v}"))),
            ))
            .show_in(scope)
    })
}
