use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(24.)
        .fill((
            sapid::date_picker("keyboard-review-picker", provider())
                .theme(theme.clone())
                .name("Keyboard review calendar")
                .default_value(DateSelection::Single(Some(day("2026-11-11")))),
            sapid::date_picker("readonly-review-picker", provider())
                .theme(theme.clone())
                .name("Readonly review calendar")
                .readonly(true)
                .default_value(DateSelection::Single(Some(day("2026-11-11")))),
            sapid::date_picker("disabled-review-picker", provider())
                .theme(theme.clone())
                .name("Disabled review calendar")
                .enabled(false),
        ))
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
