use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let multiple = scope.cage(PeriodSelection::<CivilYear>::empty(
            DatePickerMode::Multiple,
        ));
        let range = scope.cage(PeriodSelection::<CivilYear>::empty(DatePickerMode::Range));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(24.)
            .fill((
                sapid::year_picker("period_year-multiple", provider())
                    .theme(theme.clone())
                    .name("Multiple planning years")
                    .mode(DatePickerMode::Multiple)
                    .value(multiple)
                    .on_value_change(move |v| multiple.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(multiple.map(|v| {
                        let s = v.format_iso();
                        if s.is_empty() {
                            "Choose several planning years.".to_owned()
                        } else {
                            format!("Selected years: {s}")
                        }
                    })),
                sapid::year_picker("period_year-range", provider())
                    .theme(theme.clone())
                    .name("Planning year range")
                    .mode(DatePickerMode::Range)
                    .value(range)
                    .on_value_change(move |v| range.revise(|s| *s = v)),
                sapid::text().theme(theme.clone()).content(range.map(|v| {
                    let s = v.format_iso();
                    if s.is_empty() {
                        "Choose the start and end year.".to_owned()
                    } else {
                        format!("Planning range: {s}")
                    }
                })),
            ))
            .show_in(scope)
    })
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
