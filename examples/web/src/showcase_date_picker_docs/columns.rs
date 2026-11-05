use glory::prelude::*;
use sapid::{DatePickerMode, DateSelection, DatesProvider, date_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let columns_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage(DateSelection::empty(DatePickerMode::Range));
        date_picker(
            "showcase-date-picker-columns",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(columns_theme.clone())
        .mode(DatePickerMode::Range)
        .value(value)
        .default_month(sapid::YearMonth::new(2024, 2).unwrap())
        .number_of_columns(2)
        .columns_to_scroll(2)
        .on_value_change(move |next| value.revise(|current| *current = next))
        .show_in(scope)
    })
}
