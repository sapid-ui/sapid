use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(sapid::DateSelection::Single(None));
        let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
        sapid::field("appointment-date")
            .theme(theme.clone())
            .label("Appointment date")
            .description("Choose a date from the calendar")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::date_picker_input("appointment-date", provider)
                    .field(field)
                    .form_name("appointment_date")
                    .placeholder("Pick a date")
                    .trigger_label("Choose appointment date")
                    .clearable(true)
                    .value(current)
                    .on_value_change(move |next| current.revise(|old| *old = next))
            })
            .show_in(scope)
    })
}
