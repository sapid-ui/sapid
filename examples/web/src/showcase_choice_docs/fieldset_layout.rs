use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::fieldset("outer-address")
            .theme(theme.clone())
            .legend("Address")
            .fill((
                sapid::field("street")
                    .theme(theme.clone())
                    .label("Street")
                    .control(|field| {
                        sapid::text_field()
                            .field(field)
                            .placeholder("Street address")
                    }),
                sapid::fieldset("delivery-instructions")
                    .theme(theme.clone())
                    .legend("Delivery instructions")
                    .style(Style::new().padding(Edges::all(12.)).gap(10.))
                    .fill(
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Leave with a neighbour"),
                    ),
            )),
        sapid::fieldset("borderless-preferences")
            .theme(theme.clone())
            .legend("Borderless preferences")
            .style(
                Style::new()
                    .border_width(Edges::all(0.))
                    .padding(Edges::all(0.)),
            )
            .fill(
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Remember this address"),
            ),
    ))
}
