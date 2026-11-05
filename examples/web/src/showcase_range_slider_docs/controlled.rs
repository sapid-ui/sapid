use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage([25.0, 75.0]);
        sapid::stack()
            .theme(controlled_theme.clone())
            .gap(10.)
            .fill((
                sapid::range_slider("showcase-range-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    let [lower, upper] = value.get();
                    format!("Selected: {lower} – {upper}")
                })),
            ))
            .show_in(scope)
    })
}
