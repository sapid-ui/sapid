use super::showcase_examples::DetailSection;
mod month_controlled;
mod month_format;
mod month_modes;
mod month_states;
mod year_controlled;
mod year_format;
mod year_modes;
mod year_states;
pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match id {
        "month-picker-input" => vec![
            DetailSection {
                id: "month-input-format",
                title: "Display format and locale",
                note: "Localized labels follow the reactive DatesProvider. ISO is the default format. A form_name always submits the accepted ISO month or year, independently of display spelling; month values never invent a representative day.",
                example: Box::new(month_format::demo(theme.clone())),
                source: include_str!("month_format.rs"),
            },
            DetailSection {
                id: "month-input-controlled",
                title: "Controlled and clearable",
                note: "Selection and the inset clear action propose typed values. Turn acceptance off to inspect refused proposals. Clear returns focus to the same display input. Escape closes the host-owned dialog and restores its trigger.",
                example: Box::new(month_controlled::demo(theme.clone())),
                source: include_str!("month_controlled.rs"),
            },
            DetailSection {
                id: "month-input-modes",
                title: "Multiple values and ranges",
                note: "Multiple and range modes use the same typed period controller. A partial range remains explicit; a complete range uses an en dash for display and a slash for ISO submission. Clear proposes the empty value for the current mode.",
                example: Box::new(month_modes::demo(theme.clone())),
                source: include_str!("month_modes.rs"),
            },
            DetailSection {
                id: "month-input-states",
                title: "Bounds, exclusions and field states",
                note: "DateBounds project to the month or year domain. Excluded periods are unavailable. readonly blocks selection and clear proposals; enabled(false) disables the input and trigger. Field connects the actual input to its label, required state and error.",
                example: Box::new(month_states::demo(theme.clone())),
                source: include_str!("month_states.rs"),
            },
        ],
        "year-picker-input" => vec![
            DetailSection {
                id: "year-input-format",
                title: "Display format and locale",
                note: "Localized labels follow the reactive DatesProvider. ISO is the default format. A form_name always submits the accepted ISO month or year, independently of display spelling; month values never invent a representative day.",
                example: Box::new(year_format::demo(theme.clone())),
                source: include_str!("year_format.rs"),
            },
            DetailSection {
                id: "year-input-controlled",
                title: "Controlled and clearable",
                note: "Selection and the inset clear action propose typed values. Turn acceptance off to inspect refused proposals. Clear returns focus to the same display input. Escape closes the host-owned dialog and restores its trigger.",
                example: Box::new(year_controlled::demo(theme.clone())),
                source: include_str!("year_controlled.rs"),
            },
            DetailSection {
                id: "year-input-modes",
                title: "Multiple values and ranges",
                note: "Multiple and range modes use the same typed period controller. A partial range remains explicit; a complete range uses an en dash for display and a slash for ISO submission. Clear proposes the empty value for the current mode.",
                example: Box::new(year_modes::demo(theme.clone())),
                source: include_str!("year_modes.rs"),
            },
            DetailSection {
                id: "year-input-states",
                title: "Bounds, exclusions and field states",
                note: "DateBounds project to the month or year domain. Excluded periods are unavailable. readonly blocks selection and clear proposals; enabled(false) disables the input and trigger. Field connects the actual input to its label, required state and error.",
                example: Box::new(year_states::demo(theme.clone())),
                source: include_str!("year_states.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "month-picker-input" => vec![
            ("Display format and locale", "#month-input-format"),
            ("Controlled and clearable", "#month-input-controlled"),
            ("Multiple values and ranges", "#month-input-modes"),
            ("Bounds, exclusions and field states", "#month-input-states"),
        ],
        "year-picker-input" => vec![
            ("Display format and locale", "#year-input-format"),
            ("Controlled and clearable", "#year-input-controlled"),
            ("Multiple values and ranges", "#year-input-modes"),
            ("Bounds, exclusions and field states", "#year-input-states"),
        ],
        _ => Vec::new(),
    }
}
