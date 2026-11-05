use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("standard".to_owned());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept delivery changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::radio_group("controlled-delivery")
                    .theme(theme.clone())
                    .label("Controlled delivery")
                    .value(value)
                    .items([
                        sapid::radio("standard").label("Standard"),
                        sapid::radio("unavailable")
                            .label("Unavailable")
                            .enabled(false),
                        sapid::radio("express").label("Express"),
                    ])
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Delivery: {v}"))),
            ))
            .show_in(scope)
    })
}
