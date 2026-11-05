use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let eye_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        sapid::stack()
            .theme(eye_theme.clone())
            .gap(8.)
            .fill((
                sapid::color_input("showcase-color-eye-dropper")
                    .theme(eye_theme.clone())
                    .value(value)
                    .eye_dropper(sapid::ColorEyeDropper::new(|| async {
                        Ok(Some(glory::style::Color(18, 184, 134, 255)))
                    }))
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-color-eye-dropper-value")
                    .content(scope.bond(move || format!("Selected: {}", value.get()))),
            ))
            .show_in(scope)
    })
}
