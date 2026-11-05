use glory::prelude::*;
use sapid::{DatesProvider, date_picker, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let calendar_rows_theme = theme.clone();
    stack().theme(calendar_rows_theme.clone()).gap(20.).fill((
        date_picker(
            "showcase-date-picker-week-numbers",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(calendar_rows_theme.clone())
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .with_week_numbers(true),
        date_picker(
            "showcase-date-picker-hidden-weekdays",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(calendar_rows_theme)
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .hide_weekdays(true),
    ))
}
