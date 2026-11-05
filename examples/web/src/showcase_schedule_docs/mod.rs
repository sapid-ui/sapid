use super::showcase_examples::DetailSection;
mod agenda_view_controlled;
mod agenda_view_empty;
mod agenda_view_locale;
mod agenda_view_presentation;
mod agenda_view_window;
mod day_view_controlled;
mod day_view_locale;
mod day_view_presentation;
mod day_view_scroll;
mod day_view_window;
mod events_data_intervals;
mod events_data_validation;
mod events_data_zones;
mod mobile_month_view_controlled;
mod mobile_month_view_disclosure;
mod mobile_month_view_locale;
mod mobile_month_view_presentation;
mod mobile_month_view_window;
mod month_view_controlled;
mod month_view_disclosure;
mod month_view_locale;
mod month_view_presentation;
mod month_view_window;
mod recurring_events_exceptions;
mod recurring_events_missing;
mod recurring_events_replacement;
mod recurring_events_rules;
mod resources_day_view_axis;
mod resources_day_view_controlled;
mod resources_day_view_editing;
mod resources_day_view_locale;
mod resources_day_view_window;
mod resources_month_view_axis;
mod resources_month_view_controlled;
mod resources_month_view_editing;
mod resources_month_view_locale;
mod resources_month_view_window;
mod resources_schedule_axis;
mod resources_schedule_controlled;
mod resources_schedule_editing;
mod resources_schedule_locale;
mod resources_schedule_window;
mod resources_week_view_axis;
mod resources_week_view_controlled;
mod resources_week_view_editing;
mod resources_week_view_locale;
mod resources_week_view_window;
mod schedule_controlled;
mod schedule_locale;
mod schedule_presentation;
mod schedule_scroll;
mod schedule_window;
mod week_view_controlled;
mod week_view_locale;
mod week_view_presentation;
mod week_view_scroll;
mod week_view_window;
mod year_view_controlled;
mod year_view_disclosure;
mod year_view_locale;
mod year_view_presentation;
mod year_view_window;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "schedule" => vec![
            DetailSection {
                id: "schedule-schedule-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(schedule_controlled::demo(theme.clone())),
                source: include_str!("schedule_controlled.rs"),
            },
            DetailSection {
                id: "schedule-schedule-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(schedule_window::demo(theme.clone())),
                source: include_str!("schedule_window.rs"),
            },
            DetailSection {
                id: "schedule-schedule-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(schedule_presentation::demo(theme.clone())),
                source: include_str!("schedule_presentation.rs"),
            },
            DetailSection {
                id: "schedule-schedule-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(schedule_locale::demo(theme.clone())),
                source: include_str!("schedule_locale.rs"),
            },
            DetailSection {
                id: "schedule-schedule-scroll",
                title: "Explicit time and local scrolling",
                note: "The application chooses 10:00 and scrolls four half-hour slots inside the current viewport. The component reads no system clock. Arrow/Home/End grid navigation also reveals the current slot.",
                example: Box::new(schedule_scroll::demo(theme.clone())),
                source: include_str!("schedule_scroll.rs"),
            },
        ],
        "day-view" => vec![
            DetailSection {
                id: "schedule-day-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(day_view_controlled::demo(theme.clone())),
                source: include_str!("day_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-day-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(day_view_window::demo(theme.clone())),
                source: include_str!("day_view_window.rs"),
            },
            DetailSection {
                id: "schedule-day-view-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(day_view_presentation::demo(theme.clone())),
                source: include_str!("day_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-day-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(day_view_locale::demo(theme.clone())),
                source: include_str!("day_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-day-view-scroll",
                title: "Explicit time and local scrolling",
                note: "The application chooses 10:00 and scrolls four half-hour slots inside the current viewport. The component reads no system clock. Arrow/Home/End grid navigation also reveals the current slot.",
                example: Box::new(day_view_scroll::demo(theme.clone())),
                source: include_str!("day_view_scroll.rs"),
            },
        ],
        "week-view" => vec![
            DetailSection {
                id: "schedule-week-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(week_view_controlled::demo(theme.clone())),
                source: include_str!("week_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-week-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(week_view_window::demo(theme.clone())),
                source: include_str!("week_view_window.rs"),
            },
            DetailSection {
                id: "schedule-week-view-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(week_view_presentation::demo(theme.clone())),
                source: include_str!("week_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-week-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(week_view_locale::demo(theme.clone())),
                source: include_str!("week_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-week-view-scroll",
                title: "Explicit time and local scrolling",
                note: "The application chooses 10:00 and scrolls four half-hour slots inside the current viewport. The component reads no system clock. Arrow/Home/End grid navigation also reveals the current slot.",
                example: Box::new(week_view_scroll::demo(theme.clone())),
                source: include_str!("week_view_scroll.rs"),
            },
        ],
        "month-view" => vec![
            DetailSection {
                id: "schedule-month-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(month_view_controlled::demo(theme.clone())),
                source: include_str!("month_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-month-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(month_view_window::demo(theme.clone())),
                source: include_str!("month_view_window.rs"),
            },
            DetailSection {
                id: "schedule-month-view-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(month_view_presentation::demo(theme.clone())),
                source: include_str!("month_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-month-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(month_view_locale::demo(theme.clone())),
                source: include_str!("month_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-month-view-disclosure",
                title: "Event summaries and disclosure",
                note: "Only one event is initially visible per period. The disclosure button expands the actual current events; each event remains separately selectable and the period slot has its own civil range.",
                example: Box::new(month_view_disclosure::demo(theme.clone())),
                source: include_str!("month_view_disclosure.rs"),
            },
        ],
        "year-view" => vec![
            DetailSection {
                id: "schedule-year-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(year_view_controlled::demo(theme.clone())),
                source: include_str!("year_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-year-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(year_view_window::demo(theme.clone())),
                source: include_str!("year_view_window.rs"),
            },
            DetailSection {
                id: "schedule-year-view-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(year_view_presentation::demo(theme.clone())),
                source: include_str!("year_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-year-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(year_view_locale::demo(theme.clone())),
                source: include_str!("year_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-year-view-disclosure",
                title: "Event summaries and disclosure",
                note: "Only one event is initially visible per period. The disclosure button expands the actual current events; each event remains separately selectable and the period slot has its own civil range.",
                example: Box::new(year_view_disclosure::demo(theme.clone())),
                source: include_str!("year_view_disclosure.rs"),
            },
        ],
        "mobile-month-view" => vec![
            DetailSection {
                id: "schedule-mobile-month-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(mobile_month_view_controlled::demo(theme.clone())),
                source: include_str!("mobile_month_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-mobile-month-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(mobile_month_view_window::demo(theme.clone())),
                source: include_str!("mobile_month_view_window.rs"),
            },
            DetailSection {
                id: "schedule-mobile-month-view-presentation",
                title: "Presentation and finite visible density",
                note: "This readonly calendar hides the window toolbar, weekday labels and outside days. Keyboard inspection still changes the full day agenda; readonly prevents selection proposals.",
                example: Box::new(mobile_month_view_presentation::demo(theme.clone())),
                source: include_str!("mobile_month_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-mobile-month-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(mobile_month_view_locale::demo(theme.clone())),
                source: include_str!("mobile_month_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-mobile-month-view-disclosure",
                title: "Month grid and full selected-day agenda",
                note: "MobileMonth displays counts in its date grid. Inspect November 4 to reveal every current event underneath; this list is distinct from the truncated Month/Year preview rows.",
                example: Box::new(mobile_month_view_disclosure::demo(theme.clone())),
                source: include_str!("mobile_month_view_disclosure.rs"),
            },
        ],
        "agenda-view" => vec![
            DetailSection {
                id: "schedule-agenda-view-controlled",
                title: "Controlled selection, readonly and refusal",
                note: "Selection and window changes are typed proposals. Refusing keeps the accepted view and reading unchanged; readonly still allows date navigation.",
                example: Box::new(agenda_view_controlled::demo(theme.clone())),
                source: include_str!("agenda_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-agenda-view-window",
                title: "Application-owned date windows",
                note: "Previous and Next request an exact bounded replacement window. This example supplies only the current data and prints the accepted range; no clock, HTTP loader or visited-window cache is installed.",
                example: Box::new(agenda_view_window::demo(theme.clone())),
                source: include_str!("agenda_view_window.rs"),
            },
            DetailSection {
                id: "schedule-agenda-view-presentation",
                title: "Presentation and finite visible density",
                note: "A separate readonly example hides the window toolbar. Timeline views use an explicit 09:00–12:00 hour range; calendar views demonstrate bounded disclosure and presentation flags.",
                example: Box::new(agenda_view_presentation::demo(theme.clone())),
                source: include_str!("agenda_view_presentation.rs"),
            },
            DetailSection {
                id: "schedule-agenda-view-locale",
                title: "Locale, week start and RTL navigation",
                note: "Chinese date headings, Sunday week start and explicit RTL affect the actual view, rather than a list of option names. Events retain their current keys and civil times.",
                example: Box::new(agenda_view_locale::demo(theme.clone())),
                source: include_str!("agenda_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-agenda-view-empty",
                title: "Empty current windows and grouped dates",
                note: "Only dates with current overlapping events are grouped in Agenda. Empty data has a real empty state; an all-day event keeps its exclusive model boundary and readable inclusive date.",
                example: Box::new(agenda_view_empty::demo(theme.clone())),
                source: include_str!("agenda_view_empty.rs"),
            },
        ],
        "resources-day-view" => vec![
            DetailSection {
                id: "schedule-resources-day-view-controlled",
                title: "Controlled bookings, readonly and refusal",
                note: "The application accepts or refuses selection and date/resource requests. Editing has its own section; readonly still permits window navigation.",
                example: Box::new(resources_day_view_controlled::demo(theme.clone())),
                source: include_str!("resources_day_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-resources-day-view-window",
                title: "Finite date and resource windows",
                note: "Six declared rooms are paged two at a time. Date and resource navigation request exact bounded replacement data; two visible rooms are not a capability limit.",
                example: Box::new(resources_day_view_window::demo(theme.clone())),
                source: include_str!("resources_day_view_window.rs"),
            },
            DetailSection {
                id: "schedule-resources-day-view-axis",
                title: "Resource/date headers and row or column axes",
                note: "Compare the actual row and column layouts. Each view has its own current finite window, complete headers and local scrolling; these are not frozen/sticky-header claims.",
                example: Box::new(resources_day_view_axis::demo(theme.clone())),
                source: include_str!("resources_day_view_axis.rs"),
            },
            DetailSection {
                id: "schedule-resources-day-view-locale",
                title: "Localized headings and RTL order",
                note: "The real resource/date headers use Chinese date labels and RTL navigation. Application-owned event and room titles remain unchanged.",
                example: Box::new(resources_day_view_locale::demo(theme.clone())),
                source: include_str!("resources_day_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-resources-day-view-editing",
                title: "Move, resize and conflict policy",
                note: "Use a booking’s Edit action to enable its accessible move/resize controls. Matching current proposals are validated before publication. The application can refuse overlapping room bookings and publishes the proposed interval only after acceptance.",
                example: Box::new(resources_day_view_editing::demo(theme.clone())),
                source: include_str!("resources_day_view_editing.rs"),
            },
        ],
        "resources-week-view" => vec![
            DetailSection {
                id: "schedule-resources-week-view-controlled",
                title: "Controlled bookings, readonly and refusal",
                note: "The application accepts or refuses selection and date/resource requests. Editing has its own section; readonly still permits window navigation.",
                example: Box::new(resources_week_view_controlled::demo(theme.clone())),
                source: include_str!("resources_week_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-resources-week-view-window",
                title: "Finite date and resource windows",
                note: "Six declared rooms are paged two at a time. Date and resource navigation request exact bounded replacement data; two visible rooms are not a capability limit.",
                example: Box::new(resources_week_view_window::demo(theme.clone())),
                source: include_str!("resources_week_view_window.rs"),
            },
            DetailSection {
                id: "schedule-resources-week-view-axis",
                title: "Resource/date headers and row or column axes",
                note: "Compare the actual row and column layouts. Each view has its own current finite window, complete headers and local scrolling; these are not frozen/sticky-header claims.",
                example: Box::new(resources_week_view_axis::demo(theme.clone())),
                source: include_str!("resources_week_view_axis.rs"),
            },
            DetailSection {
                id: "schedule-resources-week-view-locale",
                title: "Localized headings and RTL order",
                note: "The real resource/date headers use Chinese date labels and RTL navigation. Application-owned event and room titles remain unchanged.",
                example: Box::new(resources_week_view_locale::demo(theme.clone())),
                source: include_str!("resources_week_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-resources-week-view-editing",
                title: "Move, resize and conflict policy",
                note: "Use a booking’s Edit action to enable its accessible move/resize controls. Matching current proposals are validated before publication. The application can refuse overlapping room bookings and publishes the proposed interval only after acceptance.",
                example: Box::new(resources_week_view_editing::demo(theme.clone())),
                source: include_str!("resources_week_view_editing.rs"),
            },
        ],
        "resources-month-view" => vec![
            DetailSection {
                id: "schedule-resources-month-view-controlled",
                title: "Controlled bookings, readonly and refusal",
                note: "The application accepts or refuses selection and date/resource requests. Editing has its own section; readonly still permits window navigation.",
                example: Box::new(resources_month_view_controlled::demo(theme.clone())),
                source: include_str!("resources_month_view_controlled.rs"),
            },
            DetailSection {
                id: "schedule-resources-month-view-window",
                title: "Finite date and resource windows",
                note: "Six declared rooms are paged two at a time. Date and resource navigation request exact bounded replacement data; two visible rooms are not a capability limit.",
                example: Box::new(resources_month_view_window::demo(theme.clone())),
                source: include_str!("resources_month_view_window.rs"),
            },
            DetailSection {
                id: "schedule-resources-month-view-axis",
                title: "Resource/date headers and row or column axes",
                note: "Compare the actual row and column layouts. Each view has its own current finite window, complete headers and local scrolling; these are not frozen/sticky-header claims.",
                example: Box::new(resources_month_view_axis::demo(theme.clone())),
                source: include_str!("resources_month_view_axis.rs"),
            },
            DetailSection {
                id: "schedule-resources-month-view-locale",
                title: "Localized headings and RTL order",
                note: "The real resource/date headers use Chinese date labels and RTL navigation. Application-owned event and room titles remain unchanged.",
                example: Box::new(resources_month_view_locale::demo(theme.clone())),
                source: include_str!("resources_month_view_locale.rs"),
            },
            DetailSection {
                id: "schedule-resources-month-view-editing",
                title: "Move, resize and conflict policy",
                note: "Use a booking’s Edit action to enable its accessible move/resize controls. Matching current proposals are validated before publication. The application can refuse overlapping room bookings and publishes the proposed interval only after acceptance.",
                example: Box::new(resources_month_view_editing::demo(theme.clone())),
                source: include_str!("resources_month_view_editing.rs"),
            },
        ],
        "resources-schedule" => vec![
            DetailSection {
                id: "schedule-resources-schedule-controlled",
                title: "Controlled bookings, readonly and refusal",
                note: "The application accepts or refuses selection and date/resource requests. Editing has its own section; readonly still permits window navigation.",
                example: Box::new(resources_schedule_controlled::demo(theme.clone())),
                source: include_str!("resources_schedule_controlled.rs"),
            },
            DetailSection {
                id: "schedule-resources-schedule-window",
                title: "Finite date and resource windows",
                note: "Six declared rooms are paged two at a time. Date and resource navigation request exact bounded replacement data; two visible rooms are not a capability limit.",
                example: Box::new(resources_schedule_window::demo(theme.clone())),
                source: include_str!("resources_schedule_window.rs"),
            },
            DetailSection {
                id: "schedule-resources-schedule-axis",
                title: "Resource/date headers and row or column axes",
                note: "Compare the actual row and column layouts. Each view has its own current finite window, complete headers and local scrolling; these are not frozen/sticky-header claims.",
                example: Box::new(resources_schedule_axis::demo(theme.clone())),
                source: include_str!("resources_schedule_axis.rs"),
            },
            DetailSection {
                id: "schedule-resources-schedule-locale",
                title: "Localized headings and RTL order",
                note: "The real resource/date headers use Chinese date labels and RTL navigation. Application-owned event and room titles remain unchanged.",
                example: Box::new(resources_schedule_locale::demo(theme.clone())),
                source: include_str!("resources_schedule_locale.rs"),
            },
            DetailSection {
                id: "schedule-resources-schedule-editing",
                title: "Move, resize and conflict policy",
                note: "Use a booking’s Edit action to enable its accessible move/resize controls. Matching current proposals are validated before publication. The application can refuse overlapping room bookings and publishes the proposed interval only after acceptance.",
                example: Box::new(resources_schedule_editing::demo(theme.clone())),
                source: include_str!("resources_schedule_editing.rs"),
            },
        ],
        "recurring-events" => vec![
            DetailSection {
                id: "schedule-recurring-events-exceptions",
                title: "Exception dates do not extend COUNT",
                note: "The rule has seven candidates beginning November 2. Removing November 5 produces six occurrences; restoring it produces seven. The actual calendar changes, not just a rule summary.",
                example: Box::new(recurring_events_exceptions::demo(theme.clone())),
                source: include_str!("recurring_events_exceptions.rs"),
            },
            DetailSection {
                id: "schedule-recurring-events-rules",
                title: "Intervals, weekdays, COUNT and UNTIL",
                note: "Every-other-day and Monday/Wednesday/Friday rules expand only inside the current month. COUNT counts rule candidates before exceptions; UNTIL is an inclusive civil date. Neither rule installs a clock or a historical occurrence store.",
                example: Box::new(recurring_events_rules::demo(theme.clone())),
                source: include_str!("recurring_events_rules.rs"),
            },
            DetailSection {
                id: "schedule-recurring-events-missing",
                title: "Missing month ends and leap days",
                note: "Compare real Skip and Clamp expansions. January 31 is expanded over six months; February 29, 2024 is expanded into 2025. The application supplies both the rule and the finite window.",
                example: Box::new(recurring_events_missing::demo(theme.clone())),
                source: include_str!("recurring_events_missing.rs"),
            },
            DetailSection {
                id: "schedule-recurring-events-replacement",
                title: "Application-owned occurrence replacement",
                note: "Rescheduling one occurrence is explicit: suppress its original civil date and add a distinct one-off event. The series rule remains unchanged. This does not silently rewrite a whole recurring series.",
                example: Box::new(recurring_events_replacement::demo(theme.clone())),
                source: include_str!("recurring_events_replacement.rs"),
            },
        ],
        "events-data" => vec![
            DetailSection {
                id: "schedule-events-data-intervals",
                title: "Half-open intervals, adjacency and all-day dates",
                note: "09:00–10:00 and 10:00–11:00 are adjacent, not overlapping. An all-day event ends at the next midnight. The model keeps the exclusive endpoint while the view presents a readable inclusive date.",
                example: Box::new(events_data_intervals::demo(theme.clone())),
                source: include_str!("events_data_intervals.rs"),
            },
            DetailSection {
                id: "schedule-events-data-zones",
                title: "Civil, fixed-offset and application zone identities",
                note: "Civil values have no instant. FixedOffset uses an explicit offset; an Application key names a supplied resolver, not an installed IANA database. Resolution happens outside reactive borrows.",
                example: Box::new(events_data_zones::demo(theme.clone())),
                source: include_str!("events_data_zones.rs"),
            },
            DetailSection {
                id: "schedule-events-data-validation",
                title: "Keys, invalid declarations and resource references",
                note: "Validate a proposed declaration before publishing a replacement source. These actions expose real typed failures while the current valid agenda remains visible. Unknown resource keys never become display names or paths.",
                example: Box::new(events_data_validation::demo(theme.clone())),
                source: include_str!("events_data_validation.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "schedule" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-schedule-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-schedule-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-schedule-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-schedule-locale",
            ),
            (
                "Explicit time and local scrolling",
                "#schedule-schedule-scroll",
            ),
        ],
        "day-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-day-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-day-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-day-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-day-view-locale",
            ),
            (
                "Explicit time and local scrolling",
                "#schedule-day-view-scroll",
            ),
        ],
        "week-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-week-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-week-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-week-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-week-view-locale",
            ),
            (
                "Explicit time and local scrolling",
                "#schedule-week-view-scroll",
            ),
        ],
        "month-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-month-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-month-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-month-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-month-view-locale",
            ),
            (
                "Event summaries and disclosure",
                "#schedule-month-view-disclosure",
            ),
        ],
        "year-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-year-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-year-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-year-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-year-view-locale",
            ),
            (
                "Event summaries and disclosure",
                "#schedule-year-view-disclosure",
            ),
        ],
        "mobile-month-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-mobile-month-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-mobile-month-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-mobile-month-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-mobile-month-view-locale",
            ),
            (
                "Month grid and full selected-day agenda",
                "#schedule-mobile-month-view-disclosure",
            ),
        ],
        "agenda-view" => vec![
            (
                "Controlled selection, readonly and refusal",
                "#schedule-agenda-view-controlled",
            ),
            (
                "Application-owned date windows",
                "#schedule-agenda-view-window",
            ),
            (
                "Presentation and finite visible density",
                "#schedule-agenda-view-presentation",
            ),
            (
                "Locale, week start and RTL navigation",
                "#schedule-agenda-view-locale",
            ),
            (
                "Empty current windows and grouped dates",
                "#schedule-agenda-view-empty",
            ),
        ],
        "resources-day-view" => vec![
            (
                "Controlled bookings, readonly and refusal",
                "#schedule-resources-day-view-controlled",
            ),
            (
                "Finite date and resource windows",
                "#schedule-resources-day-view-window",
            ),
            (
                "Resource/date headers and row or column axes",
                "#schedule-resources-day-view-axis",
            ),
            (
                "Localized headings and RTL order",
                "#schedule-resources-day-view-locale",
            ),
            (
                "Move, resize and conflict policy",
                "#schedule-resources-day-view-editing",
            ),
        ],
        "resources-week-view" => vec![
            (
                "Controlled bookings, readonly and refusal",
                "#schedule-resources-week-view-controlled",
            ),
            (
                "Finite date and resource windows",
                "#schedule-resources-week-view-window",
            ),
            (
                "Resource/date headers and row or column axes",
                "#schedule-resources-week-view-axis",
            ),
            (
                "Localized headings and RTL order",
                "#schedule-resources-week-view-locale",
            ),
            (
                "Move, resize and conflict policy",
                "#schedule-resources-week-view-editing",
            ),
        ],
        "resources-month-view" => vec![
            (
                "Controlled bookings, readonly and refusal",
                "#schedule-resources-month-view-controlled",
            ),
            (
                "Finite date and resource windows",
                "#schedule-resources-month-view-window",
            ),
            (
                "Resource/date headers and row or column axes",
                "#schedule-resources-month-view-axis",
            ),
            (
                "Localized headings and RTL order",
                "#schedule-resources-month-view-locale",
            ),
            (
                "Move, resize and conflict policy",
                "#schedule-resources-month-view-editing",
            ),
        ],
        "resources-schedule" => vec![
            (
                "Controlled bookings, readonly and refusal",
                "#schedule-resources-schedule-controlled",
            ),
            (
                "Finite date and resource windows",
                "#schedule-resources-schedule-window",
            ),
            (
                "Resource/date headers and row or column axes",
                "#schedule-resources-schedule-axis",
            ),
            (
                "Localized headings and RTL order",
                "#schedule-resources-schedule-locale",
            ),
            (
                "Move, resize and conflict policy",
                "#schedule-resources-schedule-editing",
            ),
        ],
        "recurring-events" => vec![
            (
                "Exception dates do not extend COUNT",
                "#schedule-recurring-events-exceptions",
            ),
            (
                "Intervals, weekdays, COUNT and UNTIL",
                "#schedule-recurring-events-rules",
            ),
            (
                "Missing month ends and leap days",
                "#schedule-recurring-events-missing",
            ),
            (
                "Application-owned occurrence replacement",
                "#schedule-recurring-events-replacement",
            ),
        ],
        "events-data" => vec![
            (
                "Half-open intervals, adjacency and all-day dates",
                "#schedule-events-data-intervals",
            ),
            (
                "Civil, fixed-offset and application zone identities",
                "#schedule-events-data-zones",
            ),
            (
                "Keys, invalid declarations and resource references",
                "#schedule-events-data-validation",
            ),
        ],
        _ => Vec::new(),
    }
}
