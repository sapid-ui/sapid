use glory::prelude::*;
use sapid::{DatePickerLevel, DatesProvider, date_picker, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let levels_theme = theme.clone();
    stack().theme(levels_theme.clone()).gap(20.).fill((
        date_picker(
            "showcase-date-picker-levels",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(levels_theme.clone())
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .default_level(DatePickerLevel::Decade),
        date_picker(
            "showcase-date-picker-month-level-only",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(levels_theme)
        .name("Month-only date picker")
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .max_level(DatePickerLevel::Month),
    ))
}
