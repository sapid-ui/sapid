use glory::prelude::*;
use sapid::{
    DatePickerLevel, DateTimePolicy, DatesProvider, YearMonth, date_time_picker,
    inline_date_time_picker, inline_date_time_range_picker, stack,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    stack().theme(theme.clone()).gap(20.).fill((
        inline_date_time_picker(
            "showcase-inline-date-time-levels",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time level navigation")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .default_level(DatePickerLevel::Decade),
        date_time_picker(
            "showcase-date-time-columns",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time with two months")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .number_of_columns(2)
        .columns_to_scroll(1)
        .with_week_numbers(true),
        inline_date_time_range_picker(
            "showcase-inline-date-time-range-levels",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Date and time range level navigation")
        .default_month(YearMonth::new(2026, 11).unwrap())
        .default_level(DatePickerLevel::Year),
    ))
}
