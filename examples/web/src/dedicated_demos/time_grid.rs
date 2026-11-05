use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(None::<sapid::LocalTime>);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::time_grid("appointment-time")
                    .theme(theme.clone())
                    .name("Appointment time")
                    .spec(
                        sapid::TimeGridSpec::new(
                            sapid::LocalTime::new(9, 0, 0).unwrap(),
                            sapid::LocalTime::new(12, 0, 0).unwrap(),
                            1800,
                            4,
                        )
                        .unwrap(),
                    )
                    .value(selected)
                    .on_value_change(move |next| selected.revise(|old| *old = next)),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|value| {
                        value.map_or("Choose an appointment time".into(), |time| {
                            format!("Selected: {time}")
                        })
                    })),
            ))
            .show_in(scope)
    })
}
