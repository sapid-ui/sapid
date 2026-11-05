use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::calendar("full-review-calendar", provider())
        .theme(theme.clone())
        .name("Full width review calendar")
        .full_width(true)
        .with_week_numbers(true)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(glory::style::Limit::Px(600.)),
        )
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
