use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(None::<sapid::LocalDateTime>);
        let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
        sapid::field("appointment-time")
            .theme(theme.clone())
            .label("Appointment")
            .description("Choose a local date and time")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::date_time_picker("appointment-time", provider, sapid::DateTimePolicy::Civil)
                    .field(field)
                    .form_name("appointment_time")
                    .placeholder("Pick date and time")
                    .default_time(sapid::LocalTime::new(9, 0, 0).unwrap())
                    .clearable(true)
                    .value(current)
                    .on_value_change(move |next| current.revise(|old| *old = next))
            })
            .show_in(scope)
    })
}
