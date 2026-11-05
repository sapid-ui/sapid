use glory::prelude::*;
use sapid::{DateTimePolicy, DatesProvider, date_time_picker, field, parse_local_date_time};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    field("showcase-date-time-field")
        .theme(theme.clone())
        .label("Appointment")
        .description("Choose a civil date and time")
        .control({
            let theme = theme.clone();
            move |field| {
                date_time_picker("showcase-date-time-usage", provider, DateTimePolicy::Civil)
                    .theme(theme.clone())
                    .field(field)
                    .default_value(Some(value("2026-11-04T10:30")))
            }
        })
}
