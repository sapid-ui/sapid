use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap()).bounds(
        sapid::DateBounds::new(
            Some(sapid::CivilDate::new(2026, 1, 1).unwrap()),
            Some(sapid::CivilDate::new(2028, 12, 31).unwrap()),
        )
        .unwrap(),
    );
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::year_picker_input("year-bounded", provider)
            .theme(theme.clone())
            .name("Bounded year")
            .exclude_value(sapid::CivilYear::parse_iso("2026").unwrap())
            .placeholder("Choose an available year"),
        sapid::year_picker_input("year-readonly", provider)
            .theme(theme.clone())
            .name("Read only year")
            .default_value(
                sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                    sapid::DatePickerMode::Single,
                    "2026",
                )
                .unwrap(),
            )
            .clearable(true)
            .readonly(true),
        sapid::year_picker_input("year-disabled", provider)
            .theme(theme.clone())
            .name("Disabled year")
            .default_value(
                sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                    sapid::DatePickerMode::Single,
                    "2026",
                )
                .unwrap(),
            )
            .clearable(true)
            .enabled(false),
        sapid::field("year-required")
            .theme(theme.clone())
            .label("Required year")
            .required(true)
            .error("Choose an available year")
            .control(move |field| sapid::year_picker_input("year-required", provider).field(field)),
    ))
}
