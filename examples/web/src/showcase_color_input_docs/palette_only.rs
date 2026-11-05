use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let accept = scope.cage(true);
        let color = scope.cage("#228be6".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept palette visibility changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::field("palette-only-field")
                    .theme(theme.clone())
                    .label("Palette-only color")
                    .control(move |field| {
                        sapid::color_input("controlled-palette-only")
                            .field(field)
                            .show_picker(false)
                            .swatches(["#fa5252", "#12b886", "#228be6"])
                            .open(open)
                            .value(color)
                            .close_on_color_swatch_click(true)
                            .on_open_change(move |v| {
                                if accept.get() {
                                    open.revise(|old| *old = v);
                                }
                            })
                            .on_value_change(move |v| color.revise(|old| *old = v))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(open.map(|v| format!("Palette open: {v}"))),
            ))
            .show_in(scope)
    })
}
