use super::showcase_examples::DetailSection;
mod calendar_constraints;
mod calendar_custom_day;
mod calendar_full_width;
mod calendar_locale;
mod calendar_static;
mod date_edit_controlled;
mod date_edit_drafts;
mod date_edit_formats;
mod date_edit_parser;
mod date_edit_states;
mod dates_constraints;
mod dates_controlled;
mod dates_keyboard;
mod dates_locale;
mod inline_time_bounds;
mod inline_time_controlled;
mod inline_time_format;
mod inline_time_presets;
mod inline_time_range;
mod inline_time_states;
mod period_month_bounds;
mod period_month_controlled;
mod period_month_locale;
mod period_month_modes;
mod period_year_bounds;
mod period_year_controlled;
mod period_year_locale;
mod period_year_modes;
mod time_display_empty;
mod time_display_formats;
mod time_display_reactive;
mod time_input_controlled;
mod time_input_placeholder;
mod time_input_precision;
mod time_input_states;
mod time_input_usage;
mod time_picker_controlled;
mod time_picker_controlled_popup;
mod time_picker_dropdown;
mod time_picker_duration;
mod time_picker_duration_digits;
mod time_picker_format;
mod time_picker_grid;
mod time_picker_grouped_presets;
mod time_picker_labels_focus;
mod time_picker_paste;
mod time_picker_popup_layout;
mod time_picker_presets;
mod time_picker_states;
mod time_picker_steps;
mod time_picker_usage;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "time-input" => vec![
            DetailSection {
                id: "time-input-usage",
                title: "Usage and field metadata",
                note: "TimeInput is one real editor connected to the shared Field label and description contract.",
                example: Box::new(time_input_usage::demo(theme.clone())),
                source: include_str!("time_input_usage.rs"),
            },
            DetailSection {
                id: "time-input-controlled",
                title: "Controlled local time",
                note: "Valid edits propose an exact LocalTime; incomplete or invalid drafts remain in the editor without fabricating a business value.",
                example: Box::new(time_input_controlled::demo(theme.clone())),
                source: include_str!("time_input_controlled.rs"),
            },
            DetailSection {
                id: "time-input-precision",
                title: "Hour cycle and seconds",
                note: "12/24-hour cycles, minute/second precision and locale markers are explicit finite formatting rather than ambient browser locale behavior.",
                example: Box::new(time_input_precision::demo(theme.clone())),
                source: include_str!("time_input_precision.rs"),
            },
            DetailSection {
                id: "time-input-placeholder",
                title: "Placeholder and initial value",
                note: "The placeholder uses the shared host hint layout and disappears when an accepted or draft value is present without replacing the editor.",
                example: Box::new(time_input_placeholder::demo(theme.clone())),
                source: include_str!("time_input_placeholder.rs"),
            },
            DetailSection {
                id: "time-input-states",
                title: "Bounds, readonly and disabled",
                note: "Bounds reject rather than clamp drafts. Readonly and disabled are independent native editor states.",
                example: Box::new(time_input_states::demo(theme.clone())),
                source: include_str!("time_input_states.rs"),
            },
        ],
        "time-picker" => vec![
            DetailSection {
                id: "time-picker-usage",
                title: "Usage and field metadata",
                note: "Each segment is a real spinbutton. The group shares one field label, description and typed LocalTime value.",
                example: Box::new(time_picker_usage::demo(theme.clone())),
                source: include_str!("time_picker_usage.rs"),
            },
            DetailSection {
                id: "time-picker-controlled",
                title: "Controlled and clearable",
                note: "The clear action proposes None through the same controlled protocol and disappears only after the source accepts it.",
                example: Box::new(time_picker_controlled::demo(theme.clone())),
                source: include_str!("time_picker_controlled.rs"),
            },
            DetailSection {
                id: "time-picker-format",
                title: "Seconds and 12-hour format",
                note: "Second precision, the 12-hour afternoon control and bounded per-segment placeholders are explicit finite format choices.",
                example: Box::new(time_picker_format::demo(theme.clone())),
                source: include_str!("time_picker_format.rs"),
            },
            DetailSection {
                id: "time-picker-duration",
                title: "Duration type",
                note: "The typed duration variant permits hours beyond 24 without pretending the value is a wall-clock LocalTime. Its hour editor grows with the finite value.",
                example: Box::new(time_picker_duration::demo(theme.clone())),
                source: include_str!("time_picker_duration.rs"),
            },
            DetailSection {
                id: "time-picker-duration-digits",
                title: "Minimum hour digits",
                note: "Duration placeholders and committed values honor one to six minimum hour digits while retaining the same editor identity.",
                example: Box::new(time_picker_duration_digits::demo(theme.clone())),
                source: include_str!("time_picker_duration_digits.rs"),
            },
            DetailSection {
                id: "time-picker-paste",
                title: "Paste complete times",
                note: "Paste a complete value into any segment. The default bounded parser accepts ISO time text such as 17:33:43; applications can provide a finite custom parser such as the word noon.",
                example: Box::new(time_picker_paste::demo(theme.clone())),
                source: include_str!("time_picker_paste.rs"),
            },
            DetailSection {
                id: "time-picker-dropdown",
                title: "Time controls dropdown",
                note: "The anchored popup can render independent hour, minute, second and day-period columns. Segment steps, reversed lists and a bounded scroll height apply to the real controls.",
                example: Box::new(time_picker_dropdown::demo(theme.clone())),
                source: include_str!("time_picker_dropdown.rs"),
            },
            DetailSection {
                id: "time-picker-steps",
                title: "Bounds and segment steps",
                note: "Hours, minutes and seconds have independent spin steps. Bounds reject invalid proposals instead of silently clamping them.",
                example: Box::new(time_picker_steps::demo(theme.clone())),
                source: include_str!("time_picker_steps.rs"),
            },
            DetailSection {
                id: "time-picker-grid",
                title: "Time grid and disabled slots",
                note: "The optional finite grid and typed segments share one value. Disabled slots are unavailable from either input path.",
                example: Box::new(time_picker_grid::demo(theme.clone())),
                source: include_str!("time_picker_grid.rs"),
            },
            DetailSection {
                id: "time-picker-presets",
                title: "Preset popup",
                note: "Focusing a segment or activating the adjacent trigger opens a real anchored popup. Presets use the same controlled value path as typed edits.",
                example: Box::new(time_picker_presets::demo(theme.clone())),
                source: include_str!("time_picker_presets.rs"),
            },
            DetailSection {
                id: "time-picker-grouped-presets",
                title: "Grouped presets",
                note: "Finite labeled groups preserve the active value, disable unavailable times, and can close the popup after an accepted selection.",
                example: Box::new(time_picker_grouped_presets::demo(theme.clone())),
                source: include_str!("time_picker_grouped_presets.rs"),
            },
            DetailSection {
                id: "time-picker-controlled-popup",
                title: "Controlled preset popup",
                note: "Open state is controllable and Escape or outside dismissal proposes false through the same callback without bypassing the source.",
                example: Box::new(time_picker_controlled_popup::demo(theme.clone())),
                source: include_str!("time_picker_controlled_popup.rs"),
            },
            DetailSection {
                id: "time-picker-labels-focus",
                title: "Labels and focus boundary",
                note: "Each segment and the period toggle has an independent accessible label. The first focus entry and final exit are observable at the composite boundary.",
                example: Box::new(time_picker_labels_focus::demo(theme.clone())),
                source: include_str!("time_picker_labels_focus.rs"),
            },
            DetailSection {
                id: "time-picker-popup-layout",
                title: "Popup placement and width",
                note: "The presets popup uses the shared typed anchor placement model and a bounded explicit width instead of arbitrary browser positioning props.",
                example: Box::new(time_picker_popup_layout::demo(theme.clone())),
                source: include_str!("time_picker_popup_layout.rs"),
            },
            DetailSection {
                id: "time-picker-states",
                title: "Readonly and disabled",
                note: "Readonly and disabled state applies to every segment, period toggle, grid control and clear action.",
                example: Box::new(time_picker_states::demo(theme.clone())),
                source: include_str!("time_picker_states.rs"),
            },
        ],
        "date-input" => vec![
            DetailSection {
                id: "date-edit-formats",
                title: "Explicit date spellings",
                note: "ISO, day-first and month-first spellings parse exact finite civil dates. Choose a format explicitly; an arbitrary date-library pattern or ambient locale is not inferred.",
                example: Box::new(date_edit_formats::demo(theme.clone())),
                source: include_str!("date_edit_formats.rs"),
            },
            DetailSection {
                id: "date-edit-drafts",
                title: "Drafts, parsing and commit",
                note: "Incomplete and invalid drafts do not fabricate a civil value. The accepted date stays separate; A valid commit confirms the date. Invalid or incomplete commit retains the draft for correction and leaves the accepted civil value unchanged.",
                example: Box::new(date_edit_drafts::demo(theme.clone())),
                source: include_str!("date_edit_drafts.rs"),
            },
            DetailSection {
                id: "date-edit-parser",
                title: "Application-owned parser",
                note: "This parser recognizes tomorrow relative to the explicit provider reference, then falls back to exact ISO parsing. It does not read a system clock or accept an unbounded arbitrary parser result.",
                example: Box::new(date_edit_parser::demo(theme.clone())),
                source: include_str!("date_edit_parser.rs"),
            },
            DetailSection {
                id: "date-edit-controlled",
                title: "Controlled refusal and recovery",
                note: "A valid proposal only becomes business state when the application accepts it. Rejected proposals restore the accepted spelling without replacing the actual editor. Clearing is an empty edit.",
                example: Box::new(date_edit_controlled::demo(theme.clone())),
                source: include_str!("date_edit_controlled.rs"),
            },
            DetailSection {
                id: "date-edit-states",
                title: "Bounds, readonly, disabled and no picker",
                note: "Bounds reject invalid dates rather than silently clamping them. Readonly and disabled retain their native editor semantics; with_picker(false) removes the calendar action and its reserved space.",
                example: Box::new(date_edit_states::demo(theme.clone())),
                source: include_str!("date_edit_states.rs"),
            },
        ],
        "month-picker" => vec![
            DetailSection {
                id: "period-month-modes",
                title: "Single, multiple and range selection",
                note: "Each mode owns the correct PeriodSelection domain. A range first stores its start and completes on the next accepted endpoint. There is no representative day or implicit timestamp.",
                example: Box::new(period_month_modes::demo(theme.clone())),
                source: include_str!("period_month_modes.rs"),
            },
            DetailSection {
                id: "period-month-bounds",
                title: "Navigation and unavailable periods",
                note: "Navigation follows actual year or decade bounds. Disabled periods cannot be selected; the provider today marker is separate from an accepted selected period.",
                example: Box::new(period_month_bounds::demo(theme.clone())),
                source: include_str!("period_month_bounds.rs"),
            },
            DetailSection {
                id: "period-month-locale",
                title: "Explicit locale and RTL direction",
                note: "The provider selects finite localized labels. Reading direction changes physical navigation and visual order without changing the civil period values.",
                example: Box::new(period_month_locale::demo(theme.clone())),
                source: include_str!("period_month_locale.rs"),
            },
            DetailSection {
                id: "period-month-controlled",
                title: "Controlled acceptance and readonly",
                note: "A refused proposal keeps the accepted selection. Enabling acceptance resumes normal selection on the retained grid; readonly is an independent selection policy.",
                example: Box::new(period_month_controlled::demo(theme.clone())),
                source: include_str!("period_month_controlled.rs"),
            },
        ],
        "year-picker" => vec![
            DetailSection {
                id: "period-year-modes",
                title: "Single, multiple and range selection",
                note: "Each mode owns the correct PeriodSelection domain. A range first stores its start and completes on the next accepted endpoint. There is no representative day or implicit timestamp.",
                example: Box::new(period_year_modes::demo(theme.clone())),
                source: include_str!("period_year_modes.rs"),
            },
            DetailSection {
                id: "period-year-bounds",
                title: "Navigation and unavailable periods",
                note: "Navigation follows actual year or decade bounds. Disabled periods cannot be selected; the provider today marker is separate from an accepted selected period.",
                example: Box::new(period_year_bounds::demo(theme.clone())),
                source: include_str!("period_year_bounds.rs"),
            },
            DetailSection {
                id: "period-year-locale",
                title: "Explicit locale and RTL direction",
                note: "The provider selects finite localized labels. Reading direction changes physical navigation and visual order without changing the civil period values.",
                example: Box::new(period_year_locale::demo(theme.clone())),
                source: include_str!("period_year_locale.rs"),
            },
            DetailSection {
                id: "period-year-controlled",
                title: "Controlled acceptance and readonly",
                note: "A refused proposal keeps the accepted selection. Enabling acceptance resumes normal selection on the retained grid; readonly is an independent selection policy.",
                example: Box::new(period_year_controlled::demo(theme.clone())),
                source: include_str!("period_year_controlled.rs"),
            },
        ],
        "time-value" => vec![
            DetailSection {
                id: "time-display-formats",
                title: "Hour cycle, seconds and finite locale",
                note: "TimeValue is display text, not an editor. Precision is explicit: minute values have zero seconds; second values retain seconds rather than silently losing precision.",
                example: Box::new(time_display_formats::demo(theme.clone())),
                source: include_str!("time_display_formats.rs"),
            },
            DetailSection {
                id: "time-display-empty",
                title: "Empty values and custom empty text",
                note: "None displays the explicit empty label. Empty presentation is separate from a midnight value and does not install a clock or a placeholder editor.",
                example: Box::new(time_display_empty::demo(theme.clone())),
                source: include_str!("time_display_empty.rs"),
            },
            DetailSection {
                id: "time-display-reactive",
                title: "Reactive display keeps its text identity",
                note: "A reactive accepted value updates one display text. The button changes application state; TimeValue has no editing or selection controller.",
                example: Box::new(time_display_reactive::demo(theme.clone())),
                source: include_str!("time_display_reactive.rs"),
            },
        ],
        "calendar" => vec![
            DetailSection {
                id: "calendar-custom-day",
                title: "Custom passive day content",
                note: "Calendar supplies the grid and date proposals; the application supplies accepted selection and passive day content. The event marker keeps the full accessible date name on the real day button.",
                example: Box::new(calendar_custom_day::demo(theme.clone())),
                source: include_str!("calendar_custom_day.rs"),
            },
            DetailSection {
                id: "calendar-full-width",
                title: "Full width and ISO week numbers",
                note: "Full-width cells divide the actual container width. Week numbers add a passive column and do not become selectable dates. Resizing retains the current grid and day controls.",
                example: Box::new(calendar_full_width::demo(theme.clone())),
                source: include_str!("calendar_full_width.rs"),
            },
            DetailSection {
                id: "calendar-locale",
                title: "Locale and week start",
                note: "The explicit provider controls localized labels and first weekday. A month reference and today are supplied by the application, keeping rendering deterministic across hosts.",
                example: Box::new(calendar_locale::demo(theme.clone())),
                source: include_str!("calendar_locale.rs"),
            },
            DetailSection {
                id: "calendar-static",
                title: "Static calendar and hidden rows",
                note: "Static mode emits passive day cells and omits navigation controls. Hiding weekday headings or outside dates changes presentation while keeping a fixed bounded month grid.",
                example: Box::new(calendar_static::demo(theme.clone())),
                source: include_str!("calendar_static.rs"),
            },
            DetailSection {
                id: "calendar-constraints",
                title: "Bounds and disabled dates",
                note: "Weekends and explicitly excluded dates cannot propose selection. Provider bounds apply to both navigation and dates; readonly and disabled are separate widget states.",
                example: Box::new(calendar_constraints::demo(theme.clone())),
                source: include_str!("calendar_constraints.rs"),
            },
        ],
        "date-picker" => vec![
            DetailSection {
                id: "dates-locale",
                title: "Localized labels and week start",
                note: "DatePicker forwards the explicit provider locale and first weekday to the actual calendar. Localized labels do not change the underlying civil date values.",
                example: Box::new(dates_locale::demo(theme.clone())),
                source: include_str!("dates_locale.rs"),
            },
            DetailSection {
                id: "dates-constraints",
                title: "Bounds, excluded dates and weekends",
                note: "Out-of-bounds, excluded and weekend dates cannot be selected. This example keeps today separate from selection and does not infer availability from color alone.",
                example: Box::new(dates_constraints::demo(theme.clone())),
                source: include_str!("dates_constraints.rs"),
            },
            DetailSection {
                id: "dates-controlled",
                title: "Controlled date proposals",
                note: "The checkbox determines whether the application accepts a date proposal. A refused proposal retains the same accepted selection; later acceptance and explicit deselection use the same date domain.",
                example: Box::new(dates_controlled::demo(theme.clone())),
                source: include_str!("dates_controlled.rs"),
            },
            DetailSection {
                id: "dates-keyboard",
                title: "Keyboard and explicit disabled states",
                note: "Use arrows for adjacent days, Home/End for the week and PageUp/PageDown for month navigation. Disabled and readonly calendars retain explicit semantic state and cannot accept date changes.",
                example: Box::new(dates_keyboard::demo(theme.clone())),
                source: include_str!("dates_keyboard.rs"),
            },
        ],
        "inline-date-time-picker" => vec![
            DetailSection {
                id: "inline-time-format",
                title: "Seconds and 12-hour time",
                note: "The inline picker keeps date and time controls on the page. Explicit second precision and hour cycle preserve a complete local civil value without implying a timezone.",
                example: Box::new(inline_time_format::demo(theme.clone())),
                source: include_str!("inline_time_format.rs"),
            },
            DetailSection {
                id: "inline-time-presets",
                title: "Date-time and time-only presets",
                note: "Date-time presets choose a complete value; time-only presets preserve the accepted date. The real time dropdown uses bounded quarter-hour steps and closes according to its current selection policy.",
                example: Box::new(inline_time_presets::demo(theme.clone())),
                source: include_str!("inline_time_presets.rs"),
            },
            DetailSection {
                id: "inline-time-bounds",
                title: "Complete date-time bounds",
                note: "The complete local date-time is validated together. A calendar day can be available while a proposed time is outside the bound; the application receives a rejection rather than an invented clamped time.",
                example: Box::new(inline_time_bounds::demo(theme.clone())),
                source: include_str!("inline_time_bounds.rs"),
            },
            DetailSection {
                id: "inline-time-controlled",
                title: "Controlled state and explicit draft reset",
                note: "Date and time proposals share one controlled value. An explicit reset revision restores the accepted value after unfinished editing; refusal and later recovery retain the existing editors.",
                example: Box::new(inline_time_controlled::demo(theme.clone())),
                source: include_str!("inline_time_controlled.rs"),
            },
            DetailSection {
                id: "inline-time-range",
                title: "Inline date and time range",
                note: "A range has separately retained start/end editors and a validated ordered civil interval. Inline controls remain on the page; use DateTimePicker for a popup input instead.",
                example: Box::new(inline_time_range::demo(theme.clone())),
                source: include_str!("inline_time_range.rs"),
            },
            DetailSection {
                id: "inline-time-states",
                title: "Readonly and disabled",
                note: "Readonly preserves the chosen value and prevents editing proposals; disabled also removes interaction. Both retain the complete date-time display without substituting unrelated placeholder content.",
                example: Box::new(inline_time_states::demo(theme.clone())),
                source: include_str!("inline_time_states.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "time-input" => vec![
            ("Usage and field metadata", "#time-input-usage"),
            ("Controlled local time", "#time-input-controlled"),
            ("Hour cycle and seconds", "#time-input-precision"),
            ("Placeholder and initial value", "#time-input-placeholder"),
            ("Bounds, readonly and disabled", "#time-input-states"),
        ],
        "time-picker" => vec![
            ("Usage and field metadata", "#time-picker-usage"),
            ("Controlled and clearable", "#time-picker-controlled"),
            ("Seconds and 12-hour format", "#time-picker-format"),
            ("Duration type", "#time-picker-duration"),
            ("Minimum hour digits", "#time-picker-duration-digits"),
            ("Paste complete times", "#time-picker-paste"),
            ("Time controls dropdown", "#time-picker-dropdown"),
            ("Bounds and segment steps", "#time-picker-steps"),
            ("Time grid and disabled slots", "#time-picker-grid"),
            ("Preset popup", "#time-picker-presets"),
            ("Grouped presets", "#time-picker-grouped-presets"),
            ("Controlled preset popup", "#time-picker-controlled-popup"),
            ("Labels and focus boundary", "#time-picker-labels-focus"),
            ("Popup placement and width", "#time-picker-popup-layout"),
            ("Readonly and disabled", "#time-picker-states"),
        ],
        "date-input" => vec![
            ("Explicit date spellings", "#date-edit-formats"),
            ("Drafts, parsing and commit", "#date-edit-drafts"),
            ("Application-owned parser", "#date-edit-parser"),
            ("Controlled refusal and recovery", "#date-edit-controlled"),
            (
                "Bounds, readonly, disabled and no picker",
                "#date-edit-states",
            ),
        ],
        "month-picker" => vec![
            (
                "Single, multiple and range selection",
                "#period-month-modes",
            ),
            ("Navigation and unavailable periods", "#period-month-bounds"),
            ("Explicit locale and RTL direction", "#period-month-locale"),
            (
                "Controlled acceptance and readonly",
                "#period-month-controlled",
            ),
        ],
        "year-picker" => vec![
            ("Single, multiple and range selection", "#period-year-modes"),
            ("Navigation and unavailable periods", "#period-year-bounds"),
            ("Explicit locale and RTL direction", "#period-year-locale"),
            (
                "Controlled acceptance and readonly",
                "#period-year-controlled",
            ),
        ],
        "time-value" => vec![
            (
                "Hour cycle, seconds and finite locale",
                "#time-display-formats",
            ),
            ("Empty values and custom empty text", "#time-display-empty"),
            (
                "Reactive display keeps its text identity",
                "#time-display-reactive",
            ),
        ],
        "calendar" => vec![
            ("Custom passive day content", "#calendar-custom-day"),
            ("Full width and ISO week numbers", "#calendar-full-width"),
            ("Locale and week start", "#calendar-locale"),
            ("Static calendar and hidden rows", "#calendar-static"),
            ("Bounds and disabled dates", "#calendar-constraints"),
        ],
        "date-picker" => vec![
            ("Localized labels and week start", "#dates-locale"),
            ("Bounds, excluded dates and weekends", "#dates-constraints"),
            ("Controlled date proposals", "#dates-controlled"),
            ("Keyboard and explicit disabled states", "#dates-keyboard"),
        ],
        "inline-date-time-picker" => vec![
            ("Seconds and 12-hour time", "#inline-time-format"),
            ("Date-time and time-only presets", "#inline-time-presets"),
            ("Complete date-time bounds", "#inline-time-bounds"),
            (
                "Controlled state and explicit draft reset",
                "#inline-time-controlled",
            ),
            ("Inline date and time range", "#inline-time-range"),
            ("Readonly and disabled", "#inline-time-states"),
        ],
        _ => Vec::new(),
    }
}
