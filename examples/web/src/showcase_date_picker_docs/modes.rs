use glory::prelude::*;
use sapid::{DatePickerMode, DateSelection, DatesProvider, date_picker, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let modes_theme = theme.clone();
    widget(move |scope| {
        let multiple = scope.cage(DateSelection::empty(DatePickerMode::Multiple));
        let range = scope.cage(DateSelection::empty(DatePickerMode::Range));
        stack()
            .theme(modes_theme.clone())
            .gap(20.)
            .fill((
                date_picker(
                    "showcase-date-picker-multiple",
                    DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
                )
                .theme(modes_theme.clone())
                .mode(DatePickerMode::Multiple)
                .value(multiple)
                .on_value_change(move |next| multiple.revise(|current| *current = next)),
                date_picker(
                    "showcase-date-picker-range",
                    DatesProvider::new(sapid::CivilDate::parse_iso("2024-02-29").unwrap()),
                )
                .theme(modes_theme.clone())
                .mode(DatePickerMode::Range)
                .value(range)
                .on_value_change(move |next| range.revise(|current| *current = next)),
            ))
            .show_in(scope)
    })
}
