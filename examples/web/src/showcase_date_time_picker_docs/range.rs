use glory::prelude::*;
use sapid::{
    DateTimePolicy, DateTimeRangePreset, DateTimeRangeValue, DatesProvider, LocalTime,
    date_time_range_picker, parse_local_date_time, stack,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    let conference = DateTimeRangeValue::new(
        Some(value("2026-11-04T09:00")),
        Some(value("2026-11-06T17:30")),
    )
    .unwrap();
    let range_theme = theme.clone();
    widget(move |scope| {
        let current = scope.cage(DateTimeRangeValue::default());
        stack()
            .theme(range_theme.clone())
            .gap(8.)
            .fill((
                date_time_range_picker("showcase-date-time-range", provider, DateTimePolicy::Civil)
                    .theme(range_theme.clone())
                    .name("Conference period")
                    .placeholder("Pick conference dates and times")
                    .value(current)
                    .clearable(true)
                    .default_times(
                        LocalTime::new(9, 0, 0).unwrap(),
                        LocalTime::new(17, 30, 0).unwrap(),
                    )
                    .presets([DateTimeRangePreset::new("Conference", conference)])
                    .on_value_change(move |next| current.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || match current.get() {
                    DateTimeRangeValue {
                        start: Some(start),
                        end: Some(end),
                    } => format!(
                        "{} – {}",
                        sapid::format_local_date_time(start),
                        sapid::format_local_date_time(end)
                    ),
                    DateTimeRangeValue {
                        start: Some(start),
                        end: None,
                    } => format!("Start: {}", sapid::format_local_date_time(start)),
                    _ => "No range selected".into(),
                })),
            ))
            .show_in(scope)
    })
}
