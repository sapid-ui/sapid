use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(None::<sapid::LocalTime>);
        let accept = scope.cage(true);
        let time = |hour, minute| sapid::LocalTime::new(hour, minute, 0).unwrap();
        sapid::stack()
            .theme(theme.clone())
            .gap(10.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept time changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::time_grid("bounded-time")
                    .theme(theme.clone())
                    .name("Available appointments")
                    .spec(sapid::TimeGridSpec::new(time(9, 0), time(12, 0), 1800, 4).unwrap())
                    .bounds(sapid::TimeBounds::new(Some(time(9, 30)), Some(time(11, 30))).unwrap())
                    .disable_time(time(10, 30))
                    .value(selected)
                    .on_value_change(move |next| {
                        if accept.get() {
                            selected.revise(|old| *old = next);
                        }
                    }),
                text().content(selected.map(|value| {
                    value.map_or("No accepted appointment".into(), |time| {
                        format!("Accepted: {time}")
                    })
                })),
            ))
            .show_in(scope)
    })
}
