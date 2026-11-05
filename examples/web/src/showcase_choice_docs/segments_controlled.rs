use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("list".to_owned());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept view changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::segmented_control("controlled-view")
                    .theme(theme.clone())
                    .label("Controlled view")
                    .value(value)
                    .segments([
                        sapid::segment("list").label("List"),
                        sapid::segment("grid").label("Grid"),
                    ])
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v);
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("View: {v}"))),
                sapid::segmented_control("readonly-view")
                    .theme(theme.clone())
                    .label("Read-only view")
                    .readonly(true)
                    .default_value("list")
                    .segments([
                        sapid::segment("list").label("List"),
                        sapid::segment("grid").label("Grid"),
                    ]),
            ))
            .show_in(scope)
    })
}
