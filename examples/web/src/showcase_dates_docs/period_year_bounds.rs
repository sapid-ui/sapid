use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::year_picker(
        "period_year-bounds",
        provider()
            .bounds(DateBounds::new(Some(day("2024-01-01")), Some(day("2030-12-31"))).unwrap()),
    )
    .theme(theme.clone())
    .name("Available planning years")
    .exclude_value(CivilYear::new(2026).unwrap())
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
