use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some(sapid::CivilDate::parse_iso("2024-02-29").unwrap()));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::mini_calendar(
                    "appointment-days",
                    sapid::DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-27").unwrap()),
                )
                .theme(theme.clone())
                .name("Appointment dates")
                .number_of_days(6)
                .value(value)
                .on_value_change(move |next| value.revise(|old| *old = next)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|value| {
                        value.map_or_else(
                            || "Choose an appointment date".into(),
                            |date| format!("Appointment: {date}"),
                        )
                    })),
            ))
            .show_in(scope)
    })
}
