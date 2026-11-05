use super::showcase_examples::DetailSection;
mod actions;
mod calendar_rows;
mod columns;
mod formats;
mod input;
mod levels;
mod modes;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "date-picker-actions",
            title: "Clear, controlled refusal and forms",
            note: "Inset actions keep full accessible names. Clear proposes the empty selection and returns focus to the same input. Readonly blocks proposals. Display formatting is separate from the single successful ISO control. Submit handles the accepted typed value locally; reset is explicit application policy.",
            example: Box::new(actions::demo(theme.clone())),
            source: include_str!("actions.rs"),
        },
        DetailSection {
            id: "date-picker-modes",
            title: "Multiple dates and ranges",
            note: "Single, multiple and range modes use the same controlled selection protocol.",
            example: Box::new(modes::demo(theme.clone())),
            source: include_str!("modes.rs"),
        },
        DetailSection {
            id: "date-picker-columns",
            title: "Number of columns",
            note: "Consecutive month columns share one range selection and one month anchor. Outer navigation advances the configured number of columns.",
            example: Box::new(columns::demo(theme.clone())),
            source: include_str!("columns.rs"),
        },
        DetailSection {
            id: "date-picker-input-columns",
            title: "Multiple columns in DatePickerInput",
            note: "The popup input forwards the same bounded multi-column calendar behavior.",
            example: Box::new(input::demo(theme.clone())),
            source: include_str!("input.rs"),
        },
        DetailSection {
            id: "date-picker-value-format",
            title: "Value format",
            note: "Readonly picker inputs can use finite ISO, day-first or month-first presentation without changing their typed DateSelection value.",
            example: Box::new(formats::demo(theme.clone())),
            source: include_str!("formats.rs"),
        },
        DetailSection {
            id: "date-picker-levels",
            title: "Month, year and decade levels",
            note: "The header switches between the real day, month and year grids. default_level chooses the initial grid and max_level limits upward navigation.",
            example: Box::new(levels::demo(theme.clone())),
            source: include_str!("levels.rs"),
        },
        DetailSection {
            id: "date-picker-calendar-rows",
            title: "Week numbers and weekday labels",
            note: "ISO week numbers add a passive table column. Weekday labels can be hidden without removing any of the 42 real day controls.",
            example: Box::new(calendar_rows::demo(theme.clone())),
            source: include_str!("calendar_rows.rs"),
        },
    ]
}
