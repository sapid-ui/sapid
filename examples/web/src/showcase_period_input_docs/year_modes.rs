use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::year_picker_input(
            "year-multiple",
            sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()),
        )
        .theme(theme.clone())
        .name("Multiple years")
        .mode(sapid::DatePickerMode::Multiple)
        .clearable(true)
        .display_format(sapid::PeriodInputFormat::Localized)
        .default_value(
            sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                sapid::DatePickerMode::Multiple,
                "2026,2027",
            )
            .unwrap(),
        ),
        sapid::year_picker_input(
            "year-range",
            sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()),
        )
        .theme(theme.clone())
        .name("Year range")
        .form_name("year_range")
        .mode(sapid::DatePickerMode::Range)
        .clearable(true)
        .display_format(sapid::PeriodInputFormat::Localized)
        .default_value(
            sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                sapid::DatePickerMode::Range,
                "2026/2027",
            )
            .unwrap(),
        ),
    ))
}
