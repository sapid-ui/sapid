use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill(
        [
            ("ISO date", DateInputFormat::Iso, "YYYY-MM-DD"),
            (
                "Day-first date",
                DateInputFormat::DayMonthYear,
                "DD/MM/YYYY",
            ),
            (
                "Month-first date",
                DateInputFormat::MonthDayYear,
                "MM/DD/YYYY",
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (label, format, hint))| {
            sapid::field(format!("date-format-{i}"))
                .theme(theme.clone())
                .label(label)
                .control(move |field| {
                    sapid::date_input(format!("date-format-{i}"), provider())
                        .field(field)
                        .format(format)
                        .placeholder(hint)
                        .default_value(Some(day("2026-11-04")))
                })
        })
        .collect::<Vec<_>>(),
    )
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
