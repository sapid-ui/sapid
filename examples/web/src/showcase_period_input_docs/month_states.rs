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
        sapid::month_picker_input("month-bounded", provider)
            .theme(theme.clone())
            .name("Bounded month")
            .exclude_value(sapid::YearMonth::new(2026, 11).unwrap())
            .placeholder("Choose an available month"),
        sapid::month_picker_input("month-readonly", provider)
            .theme(theme.clone())
            .name("Read only month")
            .default_value(
                sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                    sapid::DatePickerMode::Single,
                    "2026-11",
                )
                .unwrap(),
            )
            .clearable(true)
            .readonly(true),
        sapid::month_picker_input("month-disabled", provider)
            .theme(theme.clone())
            .name("Disabled month")
            .default_value(
                sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                    sapid::DatePickerMode::Single,
                    "2026-11",
                )
                .unwrap(),
            )
            .clearable(true)
            .enabled(false),
        sapid::field("month-required")
            .theme(theme.clone())
            .label("Required month")
            .required(true)
            .error("Choose an available month")
            .control(move |field| {
                sapid::month_picker_input("month-required", provider).field(field)
            }),
    ))
}
