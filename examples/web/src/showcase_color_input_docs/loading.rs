use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let loading_theme = theme.clone();
    widget(move |scope| {
        let pending = scope.cage(true);
        sapid::stack()
            .theme(loading_theme.clone())
            .gap(8.)
            .fill((
                sapid::color_input("showcase-color-loading-only")
                    .theme(loading_theme.clone())
                    .loading(pending),
                sapid::button()
                    .theme(loading_theme.clone())
                    .label(scope.bond(move || {
                        if pending.get() {
                            "Stop loading".to_owned()
                        } else {
                            "Start loading".to_owned()
                        }
                    }))
                    .on_press(move || pending.revise(|value| *value = !*value)),
            ))
            .show_in(scope)
    })
}
