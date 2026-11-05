use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("relative-review-date")
        .theme(theme.clone())
        .label("Date or tomorrow")
        .description("Try tomorrow or an ISO date such as 2026-11-12.")
        .control(|field| {
            sapid::date_input("relative-review-date", provider())
                .field(field)
                .placeholder("tomorrow")
                .parser(|draft_text| {
                    if draft_text == "tomorrow" {
                        DateParse::Date(day("2026-11-05"))
                    } else {
                        sapid::parse_iso_date_draft(draft_text)
                    }
                })
        })
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
