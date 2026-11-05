use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva {
            hue: 208.,
            saturation: 0.82,
            value: 0.9,
            alpha: 0.7,
        });
        let hex = color.map(|v| {
            format!(
                "HEX: {}",
                sapid::format_color(
                    v.to_rgba().expect("bounded color"),
                    sapid::ColorFormat::Hexa
                )
            )
        });
        let rgb = color.map(|v| {
            format!(
                "RGB: {}",
                sapid::format_color(
                    v.to_rgba().expect("bounded color"),
                    sapid::ColorFormat::Rgba
                )
            )
        });
        let hsl = color.map(|v| {
            format!(
                "HSL: {}",
                sapid::format_color(
                    v.to_rgba().expect("bounded color"),
                    sapid::ColorFormat::Hsla
                )
            )
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::color_picker("docs-picker-form")
                    .theme(theme.clone())
                    .value(color)
                    .form_name("accent")
                    .form_format(sapid::ColorFormat::Rgba)
                    .on_value_change(move |v| color.revise(|old| *old = v)),
                sapid::text().theme(theme.clone()).content(hex),
                sapid::text().theme(theme.clone()).content(rgb),
                sapid::text().theme(theme.clone()).content(hsl),
            ))
            .show_in(scope)
    })
}
