use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva::default());
        sapid::box_()
            .theme(theme.clone())
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(420.)),
            )
            .fill(
                sapid::color_picker("docs-picker-palette")
                    .theme(theme.clone())
                    .value(color)
                    .show_picker(false)
                    .full_width(true)
                    .swatches([
                        "#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf", "#12b886", "#fab005",
                    ])
                    .swatches_per_row(7)
                    .on_value_change(move |v| color.revise(|old| *old = v)),
            )
            .show_in(scope)
    })
}
