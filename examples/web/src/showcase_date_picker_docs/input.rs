use glory::prelude::*;
use sapid::{DatePickerMode, DateSelection, DatesProvider, date_picker_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let input_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage(DateSelection::empty(DatePickerMode::Range));
        date_picker_input(
            "showcase-date-picker-input-columns",
            DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
        )
        .theme(input_theme.clone())
        .name("Travel dates")
        .trigger_label("Choose travel dates")
        .mode(DatePickerMode::Range)
        .value(value)
        .number_of_columns(2)
        .columns_to_scroll(1)
        .on_value_change(move |next| value.revise(|current| *current = next))
        .show_in(scope)
    })
}
