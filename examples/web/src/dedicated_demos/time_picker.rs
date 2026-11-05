use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("review-time-parts")
        .theme(theme.clone())
        .label("Project review time")
        .description("Edit hours and minutes or choose an available time.")
        .control(move |field| {
            sapid::time_picker("review-time-parts")
                .field(field)
                .default_value(Some(LocalTime::new(9, 30, 0).unwrap()))
                .with_dropdown(true)
                .minute_step(15)
                .presets(vec![
                    LocalTime::new(9, 0, 0).unwrap(),
                    LocalTime::new(10, 30, 0).unwrap(),
                    LocalTime::new(14, 0, 0).unwrap(),
                ])
        })
}
