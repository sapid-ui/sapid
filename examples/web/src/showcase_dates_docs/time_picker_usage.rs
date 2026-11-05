use glory::prelude::*;
use sapid::{LocalTime, field, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    field("showcase-time-picker-field")
        .theme(theme.clone())
        .label("Appointment time")
        .description("Enter hours and minutes as independent spinbutton segments")
        .control({
            let theme = theme.clone();
            move |field| {
                time_picker("showcase-time-picker-usage")
                    .theme(theme.clone())
                    .field(field)
                    .default_value(Some(at(10, 30, 0)))
            }
        })
}
