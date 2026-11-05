use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(
            sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                sapid::DatePickerMode::Single,
                "2026",
            )
            .unwrap(),
        );
        let accept = scope.cage(true);
        let proposed = scope.cage("No proposal yet".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept year changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::year_picker_input(
                    "year-controlled",
                    sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()),
                )
                .theme(theme.clone())
                .name("Controlled year")
                .value(value)
                .clearable(true)
                .on_value_change(move |next| {
                    proposed.revise(|old| *old = format!("Proposed: {}", next.format_iso()));
                    if accept.get() {
                        value.revise(|old| *old = next);
                    }
                }),
                glory::ui::text()
                    .content(scope.bond(move || format!("Accepted: {}", value.get().format_iso()))),
                glory::ui::text().content(proposed),
            ))
            .show_in(scope)
    })
}
