use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(3.5);
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::rating("product-rating")
                    .theme(theme.clone())
                    .name("Product rating")
                    .precision(sapid::RatingPrecision::Half)
                    .value(value)
                    .on_value_change(move |v| value.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Rating: {v} of 5"))),
            ))
            .show_in(scope)
    })
}
