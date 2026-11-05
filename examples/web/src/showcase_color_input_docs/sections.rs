use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sections_theme = theme.clone();
    widget(move |scope| {
        let loading = scope.cage(true);
        let first = scope.cage("#228be6".to_owned());
        let second = scope.cage("#2f9e44".to_owned());
        sapid::stack()
            .theme(sections_theme.clone())
            .gap(12.)
            .fill((
                sapid::color_input("showcase-color-loading")
                    .theme(sections_theme.clone())
                    .value(first)
                    .loading(loading)
                    .loading_position(sapid::ColorInputSectionPosition::Right)
                    .on_value_change(move |next| first.revise(|old| *old = next)),
                sapid::button()
                    .theme(sections_theme.clone())
                    .label(scope.bond(move || {
                        if loading.get() {
                            "Stop loading".to_owned()
                        } else {
                            "Start loading".to_owned()
                        }
                    }))
                    .on_press(move || loading.revise(|value| *value = !*value)),
                sapid::color_input("showcase-color-sections")
                    .theme(sections_theme.clone())
                    .value(second)
                    .left_section(glory::ui::text().content("L"))
                    .right_section(glory::ui::text().content("R"))
                    .on_value_change(move |next| second.revise(|old| *old = next)),
            ))
            .show_in(scope)
    })
}
