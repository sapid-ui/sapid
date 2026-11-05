use glory::prelude::*;
use sapid::{DateTimePolicy, DatesProvider, date_time_picker, group, parse_local_date_time};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    group().theme(theme.clone()).gap(20.).fill((
        date_time_picker(
            "showcase-date-time-readonly",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Readonly appointment")
        .readonly(true)
        .clearable(true)
        .default_value(Some(value("2026-11-04T11:30"))),
        date_time_picker(
            "showcase-date-time-disabled",
            provider,
            DateTimePolicy::Civil,
        )
        .theme(theme.clone())
        .name("Disabled appointment")
        .enabled(false)
        .default_value(Some(value("2026-11-04T13:00"))),
    ))
}
