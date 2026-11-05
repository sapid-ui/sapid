use glory::prelude::*;
use sapid::{
    DateTimePolicy, DateTimePreset, DatesProvider, LocalTime, date_time_picker,
    parse_local_date_time,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    date_time_picker(
        "showcase-date-time-presets",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Schedule from a preset")
    .default_time(LocalTime::new(9, 30, 0).unwrap())
    .presets([
        DateTimePreset::new("Yesterday", value("2026-11-03T09:30")),
        DateTimePreset::new("Today", value("2026-11-04T09:30")),
        DateTimePreset::new("Tomorrow", value("2026-11-05T09:30")),
    ])
}
