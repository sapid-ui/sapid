use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("ana@example.com".to_owned());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::autocomplete("autocomplete-controlled")
                    .theme(theme.clone())
                    .label("Controlled contact")
                    .value(value)
                    .on_value_change(move |v| {
                        if accept.get() {
                            value.revise(|old| *old = v)
                        }
                    })
                    .options([
                        sapid::autocomplete_option("ana", "ana@example.com"),
                        sapid::autocomplete_option("sam", "sam@example.com"),
                        sapid::autocomplete_option("lee", "lee@example.com"),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept contact proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| format!("Accepted address: {v}"))),
            ))
            .show_in(scope)
    })
}
