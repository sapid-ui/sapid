use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(2.5);
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept rating changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::rating("half-controlled")
                    .theme(theme.clone())
                    .name("Half-star rating")
                    .precision(sapid::RatingPrecision::Half)
                    .clearable(true)
                    .value(value)
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Score: {v} of 5"))),
            ))
            .show_in(scope)
    })
}
