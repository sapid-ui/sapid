use glory::prelude::*;
use sapid::{
    DateTimePolicy, DateTimePreset, DatesProvider, LocalTime, inline_date_time_picker,
    parse_local_date_time,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    inline_date_time_picker(
        "showcase-inline-date-time-detail",
        provider,
        DateTimePolicy::Civil,
    )
    .theme(theme.clone())
    .name("Inline date and time")
    .default_time(LocalTime::new(8, 0, 0).unwrap())
    .presets([
        DateTimePreset::new("Morning", value("2026-11-04T08:00")),
        DateTimePreset::new("Evening", value("2026-11-04T18:00")),
    ])
    .default_value(Some(value("2026-11-04T08:00")))
}
