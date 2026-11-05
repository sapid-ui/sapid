use glory::prelude::*;
use sapid::{field, time_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    field("showcase-time-usage")
        .theme(theme.clone())
        .label("Appointment time")
        .description("Enter the local wall-clock time")
        .control({
            let theme = theme.clone();
            move |field| {
                time_input("showcase-time-usage")
                    .theme(theme.clone())
                    .field(field)
                    .placeholder("Select a time")
            }
        })
}
