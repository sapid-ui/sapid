use super::showcase_examples::DetailSection;
mod actions;
mod calendar_controls;
mod controlled;
mod inline;
mod inline_range;
mod presets;
mod range;
mod seconds;
mod states;
mod time_options;
mod usage;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "date-time-picker-actions",
            title: "Inset actions and ISO form submission",
            note: "Clear, readonly, controlled refusal and reset retain one accepted civil value and the same input identity. Friendly display formatting never changes the ISO successful control. The form handles its submit request locally; applications own validation and I/O.",
            example: Box::new(actions::demo(theme.clone())),
            source: include_str!("actions.rs"),
        },
        DetailSection {
            id: "date-time-picker-usage",
            title: "Usage and input popup",
            note: "The compact field opens a real dialog containing the shared calendar and segmented TimePicker while retaining one typed LocalDateTime value.",
            example: Box::new(usage::demo(theme.clone())),
            source: include_str!("usage.rs"),
        },
        DetailSection {
            id: "date-time-picker-seconds",
            title: "Seconds and 12-hour time",
            note: "The time half uses the same explicit precision, hour cycle and locale contract as TimePicker.",
            example: Box::new(seconds::demo(theme.clone())),
            source: include_str!("seconds.rs"),
        },
        DetailSection {
            id: "date-time-picker-presets",
            title: "Presets and default time",
            note: "Finite labeled presets select a complete civil value. Selecting a date without a time applies the explicit default time instead of fabricating midnight.",
            example: Box::new(presets::demo(theme.clone())),
            source: include_str!("presets.rs"),
        },
        DetailSection {
            id: "date-time-picker-time-options",
            title: "TimePicker configuration",
            note: "DateTimePicker forwards finite time controls to its real TimePicker: seconds, segment steps, the controls dropdown, reversed lists and bounded popup height.",
            example: Box::new(time_options::demo(theme.clone())),
            source: include_str!("time_options.rs"),
        },
        DetailSection {
            id: "date-time-picker-controlled",
            title: "Controlled and clearable",
            note: "The outer clear action proposes None through the same controlled value protocol and respects readonly or disabled state.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "date-time-picker-inline",
            title: "Inline picker",
            note: "InlineDateTimePicker exposes the same calendar, preset and segmented-time behavior without an input popup.",
            example: Box::new(inline::demo(theme.clone())),
            source: include_str!("inline.rs"),
        },
        DetailSection {
            id: "date-time-picker-calendar-controls",
            title: "Calendar levels and columns",
            note: "Popup, inline and atomic range variants forward the real DatePicker month/year/decade levels, controlled month anchor, bounded consecutive columns, scroll step and week rows.",
            example: Box::new(calendar_controls::demo(theme.clone())),
            source: include_str!("calendar_controls.rs"),
        },
        DetailSection {
            id: "date-time-picker-range",
            title: "Date and time range",
            note: "Range mode keeps one typed value, one range calendar and two endpoint time editors. Partial starts remain explicit and Done only commits an empty or complete ordered interval.",
            example: Box::new(range::demo(theme.clone())),
            source: include_str!("range.rs"),
        },
        DetailSection {
            id: "date-time-picker-inline-range",
            title: "Inline date and time range",
            note: "The inline form exposes the same atomic range controller directly, including independent start/end times and reversal validation.",
            example: Box::new(inline_range::demo(theme.clone())),
            source: include_str!("inline_range.rs"),
        },
        DetailSection {
            id: "date-time-picker-states",
            title: "Readonly and disabled",
            note: "Readonly preserves inspection and focus while blocking edits; disabled makes the trigger and all popup controls unavailable.",
            example: Box::new(states::demo(theme.clone())),
            source: include_str!("states.rs"),
        },
    ]
}
