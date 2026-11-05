use super::showcase_examples::DetailSection;
mod controlled;
mod localized;
mod ranges;
mod states;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident,$id:literal,$title:literal,$note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    vec![
        section!(
            ranges,
            "mini-calendar-ranges",
            "Number of days and range navigation",
            "MiniCalendar is a date strip, with seven days by default and a finite 1–31 day window. Previous/Next move by that window; adjacent civil dates remain correct across leap days, months and years. An explicit provider reference date gives identical SSR and browser output without sampling a clock. Long strips scroll locally."
        ),
        section!(
            controlled,
            "mini-calendar-controlled",
            "Controlled dates, range and deselection",
            "value and date are independent read-only Cage/Bond/Vary sources. on_value_change and on_date_change propose updates; refusing a proposal preserves the accepted selection and range. default_value/default_date configure local state. allow_deselect permits clearing an accepted selection. Range navigation never selects a date."
        ),
        section!(
            localized,
            "mini-calendar-localized",
            "Bounds, localization and day styles",
            "DatesProvider owns min/max bounds, optional today and English/Chinese labels. exclude_date and disable_weekday prevent selection and are skipped by keyboard movement. day_style customizes individual day declarations. Locale and range updates retain the same physical day buttons. A selected date must be inside bounds; invalid external updates retain the last valid presentation."
        ),
        section!(
            states,
            "mini-calendar-states",
            "Keyboard, RTL and unavailable states",
            "The strip is a one-row grid with one day Tab stop. Arrows move without selecting; Home/End reach the available window edges and PageUp/PageDown move by a window. Enter/Space activate the focused real button. RTL reverses physical date order. Read only permits browsing but prevents selection; disabled prevents both. At the finite civil endpoints the window shifts inward instead of dropping days. unstyled retains structural layout."
        ),
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Number of days and range navigation",
            "#mini-calendar-ranges",
        ),
        (
            "Controlled dates, range and deselection",
            "#mini-calendar-controlled",
        ),
        (
            "Bounds, localization and day styles",
            "#mini-calendar-localized",
        ),
        (
            "Keyboard, RTL and unavailable states",
            "#mini-calendar-states",
        ),
    ]
}
