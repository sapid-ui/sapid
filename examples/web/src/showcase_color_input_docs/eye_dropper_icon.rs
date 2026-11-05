use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let eye_icon_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage("#228be6".to_owned());
        sapid::color_input("showcase-color-eye-dropper-icon")
            .theme(eye_icon_theme.clone())
            .value(value)
            .eye_dropper(sapid::ColorEyeDropper::new(|| async {
                Ok(Some(glory::style::Color(250, 82, 82, 255)))
            }))
            .eye_dropper_label("Use custom sampler")
            .eye_dropper_icon(glory::ui::text().content("◎"))
            .on_value_change(move |next| value.revise(|old| *old = next))
            .show_in(scope)
    })
}
