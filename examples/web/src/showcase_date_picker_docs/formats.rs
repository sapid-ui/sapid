use glory::prelude::*;
use sapid::{
    DateInputFormat, DatePickerMode, DateSelection, DatesProvider, date_picker_input, stack,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let formats_theme = theme.clone();
    stack().theme(formats_theme.clone()).gap(12.).fill((
        date_picker_input(
            "showcase-date-picker-format-single",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme.clone())
        .name("Day first appointment")
        .trigger_label("Choose day first appointment")
        .display_format(DateInputFormat::DayMonthYear)
        .default_value(DateSelection::parse_iso(DatePickerMode::Single, "2026-11-04").unwrap()),
        date_picker_input(
            "showcase-date-picker-format-multiple",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme.clone())
        .name("Month first dates")
        .trigger_label("Choose month first dates")
        .mode(DatePickerMode::Multiple)
        .display_format(DateInputFormat::MonthDayYear)
        .default_value(
            DateSelection::parse_iso(DatePickerMode::Multiple, "2026-11-04,2026-11-06").unwrap(),
        ),
        date_picker_input(
            "showcase-date-picker-format-range",
            DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap()),
        )
        .theme(formats_theme)
        .name("Day first range")
        .trigger_label("Choose day first range")
        .mode(DatePickerMode::Range)
        .display_format(DateInputFormat::DayMonthYear)
        .default_value(
            DateSelection::parse_iso(DatePickerMode::Range, "2026-11-04/2026-11-06").unwrap(),
        ),
    ))
}
