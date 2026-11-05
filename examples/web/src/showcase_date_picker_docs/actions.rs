use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let initial =
            sapid::DateSelection::parse_iso(sapid::DatePickerMode::Single, "2026-11-04").unwrap();
        let value = scope.cage(initial);
        let accept = scope.cage(true);
        let readonly = scope.cage(false);
        let submitted = scope.cage("Not submitted".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept date changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Read only date")
                    .checked(readonly)
                    .on_checked_change(move |next| readonly.revise(|old| *old = next)),
                sapid::form("date-actions")
                    .theme(theme.clone())
                    .on_submit(move |_| {
                        submitted.revise(|old| {
                            *old = format!("Submitted date: {}", value.get().format_iso())
                        })
                    })
                    .on_reset(move || value.revise(|old| *old = initial))
                    .fill((
                        sapid::field("date-actions-field")
                            .theme(theme.clone())
                            .label("Formatted appointment date")
                            .description("Display: day/month/year; submitted value: ISO")
                            .control(move |field| {
                                sapid::date_picker_input(
                                    "date-actions",
                                    sapid::DatesProvider::new(
                                        sapid::CivilDate::new(2026, 11, 4).unwrap(),
                                    ),
                                )
                                .field(field)
                                .form_name("appointment_date")
                                .value(value)
                                .readonly(readonly)
                                .clearable(true)
                                .display_format(sapid::DateInputFormat::DayMonthYear)
                                .on_value_change(move |next| {
                                    if accept.get() {
                                        value.revise(|old| *old = next);
                                    }
                                })
                            }),
                        sapid::submit_button()
                            .theme(theme.clone())
                            .label("Submit date"),
                        sapid::reset_button()
                            .theme(theme.clone())
                            .label("Reset date"),
                    )),
                glory::ui::text().content(submitted),
            ))
            .show_in(scope)
    })
}
