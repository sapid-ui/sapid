use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva {
            hue: 208.,
            saturation: 0.82,
            value: 0.9,
            alpha: 1.,
        });
        sapid::color_picker("docs-picker-swatches")
            .theme(theme.clone())
            .value(color)
            .swatches([
                "#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf", "#12b886", "#fab005",
            ])
            .swatches_per_row(7)
            .on_value_change(move |v| color.revise(|old| *old = v))
            .show_in(scope)
    })
}
