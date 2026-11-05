use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                glory::ui::view()
                    .role(glory::render::Role::Status)
                    .name("Document search")
                    .semantics(scope.bond(move || {
                        Some(Box::new(glory::render::Semantics {
                            busy: Some(loading.get()),
                            ..Default::default()
                        }))
                    }))
                    .style(Style {
                        align: Some(glory::style::Align::Center),
                        ..Style::new().display(Display::Flex).gap(8.)
                    })
                    .fill((
                        sapid::loader()
                            .theme(theme.clone())
                            .diameter(16.)
                            .decorative()
                            .visible(loading),
                        sapid::text().theme(theme.clone()).content(loading.map(|v| {
                            if *v {
                                "Searching documents…".to_owned()
                            } else {
                                "Search complete.".to_owned()
                            }
                        })),
                    )),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Search is running")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
