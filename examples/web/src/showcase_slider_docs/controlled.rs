use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage(40.0);
        sapid::stack()
            .theme(controlled_theme.clone())
            .gap(10.)
            .fill((
                sapid::slider("showcase-slider-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || format!("Value: {}", value.get()))),
            ))
            .show_in(scope)
    })
}
