use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva {
            hue: 210.,
            saturation: 0.8,
            value: 0.8,
            alpha: 0.6,
        });
        sapid::color_picker("docs-picker-precision")
            .theme(theme.clone())
            .value(color)
            .show_precision_sliders(true)
            .saturation_label("Color intensity")
            .brightness_label("Light output")
            .hue_label("Color angle")
            .alpha_label("Transparency")
            .on_value_change(move |v| color.revise(|old| *old = v))
            .show_in(scope)
    })
}
