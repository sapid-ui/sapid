use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let initial = sapid::CivilDate::parse_iso("2024-02-27").unwrap();
        let date = scope.cage(initial);
        let value = scope.cage(Some(initial));
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept date and range requests")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Reset date range")
                    .on_press(move || date.revise(|old| *old = initial)),
                sapid::mini_calendar("controlled-dates", sapid::DatesProvider::new(initial))
                    .theme(theme.clone())
                    .name("Controlled date strip")
                    .date(date)
                    .value(value)
                    .allow_deselect(true)
                    .on_date_change(move |next| {
                        if accept.get() {
                            date.revise(|old| *old = next)
                        }
                    })
                    .on_value_change(move |next| {
                        if accept.get() {
                            value.revise(|old| *old = next)
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || {
                        format!(
                            "Range starts {0}; selected {1}",
                            date.get(),
                            value
                                .get()
                                .map_or_else(|| "none".into(), |date| date.to_string())
                        )
                    })),
            ))
            .show_in(scope)
    })
}
