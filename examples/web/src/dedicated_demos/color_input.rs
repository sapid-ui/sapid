use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("#228be6b3".to_owned());
        sapid::field("accent-field")
            .theme(theme.clone())
            .label("Accent color")
            .description("Enter a color or open the picker.")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::color_input("accent-color")
                    .field(field)
                    .value(color)
                    .format(sapid::ColorFormat::Hexa)
                    .swatches(["#228be6b3", "#12b886b3", "#fa5252b3"])
                    .on_value_change(move |v| color.revise(|old| *old = v))
            })
            .show_in(scope)
    })
}
