use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let destination = scope.cage("Choose a destination".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::anchor()
                    .theme(theme.clone())
                    .label("Open project settings")
                    .href("/projects/settings")
                    .on_navigate(move |request| {
                        destination.revise(|old| *old = format!("Requested {}", request.href))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || destination.get())),
            ))
            .show_in(scope)
    })
}
