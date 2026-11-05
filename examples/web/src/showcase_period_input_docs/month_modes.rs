use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::month_picker_input(
            "month-multiple",
            sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()),
        )
        .theme(theme.clone())
        .name("Multiple months")
        .mode(sapid::DatePickerMode::Multiple)
        .clearable(true)
        .display_format(sapid::PeriodInputFormat::Localized)
        .default_value(
            sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                sapid::DatePickerMode::Multiple,
                "2026-11,2026-12",
            )
            .unwrap(),
        ),
        sapid::month_picker_input(
            "month-range",
            sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()),
        )
        .theme(theme.clone())
        .name("Month range")
        .form_name("month_range")
        .mode(sapid::DatePickerMode::Range)
        .clearable(true)
        .display_format(sapid::PeriodInputFormat::Localized)
        .default_value(
            sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                sapid::DatePickerMode::Range,
                "2026-11/2026-12",
            )
            .unwrap(),
        ),
    ))
}
