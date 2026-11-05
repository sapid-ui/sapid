use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("bounded-date-editor")
            .theme(theme.clone())
            .label("November appointment")
            .description("Choose November 5–25, 2026.")
            .control(|field| {
                sapid::date_input(
                    "bounded-date-editor",
                    provider().bounds(
                        DateBounds::new(Some(day("2026-11-05")), Some(day("2026-11-25"))).unwrap(),
                    ),
                )
                .field(field)
                .default_value(Some(day("2026-11-12")))
            }),
        sapid::field("readonly-date-editor")
            .theme(theme.clone())
            .label("Readonly appointment")
            .control(|field| {
                sapid::date_input("readonly-date-editor", provider())
                    .field(field)
                    .readonly(true)
                    .default_value(Some(day("2026-11-12")))
            }),
        sapid::field("disabled-date-editor")
            .theme(theme.clone())
            .label("Disabled appointment")
            .control(|field| {
                sapid::date_input("disabled-date-editor", provider())
                    .field(field)
                    .enabled(false)
                    .default_value(Some(day("2026-11-12")))
            }),
        sapid::field("plain-date-editor")
            .theme(theme.clone())
            .label("Date without a picker")
            .control(|field| {
                sapid::date_input("plain-date-editor", provider())
                    .field(field)
                    .with_picker(false)
                    .placeholder("YYYY-MM-DD")
            }),
    ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
