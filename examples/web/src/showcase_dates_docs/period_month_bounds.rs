use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::month_picker(
        "period_month-bounds",
        provider()
            .bounds(DateBounds::new(Some(day("2026-03-01")), Some(day("2027-06-30"))).unwrap()),
    )
    .theme(theme.clone())
    .name("Available planning months")
    .exclude_value(YearMonth::new(2026, 11).unwrap())
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
