use glory::prelude::*;
use sapid::{DateLocale, HourCycle, LocalTime, TimeFormat, TimePrecision, stack, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let labels_theme = theme.clone();
    widget(move |scope| {
        let status = scope.cage("Focus a time segment".to_owned());
        stack()
            .theme(labels_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-labels")
                    .theme(labels_theme.clone())
                    .format(TimeFormat::new(
                        HourCycle::H12,
                        TimePrecision::Second,
                        DateLocale::En,
                    ))
                    .hour_label("Hours")
                    .minute_label("Minutes")
                    .second_label("Seconds")
                    .period_label("Day period")
                    .period_labels("पूर्वाह्न", "अपराह्न")
                    .default_value(Some(at(18, 45, 30)))
                    .on_focus(move || status.revise(|text| *text = "Time picker focused".into()))
                    .on_blur(move || status.revise(|text| *text = "Time picker blurred".into())),
                glory::ui::text().content(status),
            ))
            .show_in(scope)
    })
}
