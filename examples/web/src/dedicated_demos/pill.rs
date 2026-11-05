use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        sapid::group()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::pill()
                    .theme(theme.clone())
                    .content("Rust")
                    .visible(visible)
                    .removable("Remove Rust tag")
                    .on_remove(move || visible.revise(|v| *v = false)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore Rust tag")
                    .enabled(visible.map(|v| !*v))
                    .on_press(move || visible.revise(|v| *v = true)),
            ))
            .show_in(scope)
    })
}
