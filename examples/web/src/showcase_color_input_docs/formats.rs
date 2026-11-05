use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let formats_theme = theme.clone();
    widget(move |scope| {
        let specifications = [
            ("hex", sapid::ColorFormat::Hex, "#228be6"),
            ("hexa", sapid::ColorFormat::Hexa, "#228be6b3"),
            ("rgb", sapid::ColorFormat::Rgb, "rgb(34, 139, 230)"),
            ("rgba", sapid::ColorFormat::Rgba, "rgba(34, 139, 230, 0.7)"),
            ("hsl", sapid::ColorFormat::Hsl, "hsl(208, 82%, 52%)"),
            ("hsla", sapid::ColorFormat::Hsla, "hsla(208, 82%, 52%, 0.7)"),
        ];
        let mut fields = Vec::new();
        for (key, format, initial) in specifications {
            let value = scope.cage(initial.to_owned());
            fields.push(
                sapid::field(format!("showcase-color-format-{key}"))
                    .theme(formats_theme.clone())
                    .label(key.to_uppercase())
                    .control(move |field| {
                        sapid::color_input(format!("showcase-color-format-{key}"))
                            .field(field)
                            .format(format)
                            .value(value)
                            .on_value_change(move |next| value.revise(|old| *old = next))
                    }),
            );
        }
        sapid::stack()
            .theme(formats_theme.clone())
            .gap(12.)
            .fill(fields)
            .show_in(scope)
    })
}
