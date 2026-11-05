use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::month_picker(
                "period_month-localized",
                provider().locale(DateLocale::ZhCn),
            )
            .theme(theme.clone())
            .name("Localized planning month"),
            sapid::month_picker("period_month-rtl", provider())
                .theme(theme.clone())
                .direction(glory::render::Direction::Rtl)
                .name("RTL planning month"),
        ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
