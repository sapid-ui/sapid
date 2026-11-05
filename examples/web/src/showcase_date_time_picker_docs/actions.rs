use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let initial = Some(sapid::parse_local_date_time("2026-11-04T10:30").unwrap());
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
                    .label("Accept date time changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Read only date time")
                    .checked(readonly)
                    .on_checked_change(move |next| readonly.revise(|old| *old = next)),
                sapid::form("date-time-actions")
                    .theme(theme.clone())
                    .on_submit(move |_| {
                        submitted.revise(|old| {
                            *old = format!(
                                "Submitted date time: {}",
                                value
                                    .get()
                                    .map(sapid::format_local_date_time)
                                    .unwrap_or_default()
                            )
                        })
                    })
                    .on_reset(move || value.revise(|old| *old = initial))
                    .fill((
                        sapid::field("date-time-actions-field")
                            .theme(theme.clone())
                            .label("Formatted appointment time")
                            .description("Display format does not change the submitted ISO value")
                            .control(move |field| {
                                sapid::date_time_picker(
                                    "date-time-actions",
                                    sapid::DatesProvider::new(
                                        sapid::CivilDate::new(2026, 11, 4).unwrap(),
                                    ),
                                    sapid::DateTimePolicy::Civil,
                                )
                                .field(field)
                                .form_name("appointment_time")
                                .value(value)
                                .readonly(readonly)
                                .clearable(true)
                                .display_format(sapid::DateTimeDisplayFormat::DayMonthYear)
                                .on_value_change(move |next| {
                                    if accept.get() {
                                        value.revise(|old| *old = next);
                                    }
                                })
                            }),
                        sapid::submit_button()
                            .theme(theme.clone())
                            .label("Submit date time"),
                        sapid::reset_button()
                            .theme(theme.clone())
                            .label("Reset date time"),
                    )),
                glory::ui::text().content(submitted),
            ))
            .show_in(scope)
    })
}
