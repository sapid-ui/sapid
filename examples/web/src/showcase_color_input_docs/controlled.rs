use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        sapid::stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                sapid::color_input("showcase-color-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-color-controlled-value")
                    .content(scope.bond(move || format!("Value: {}", value.get()))),
            ))
            .show_in(scope)
    })
}
