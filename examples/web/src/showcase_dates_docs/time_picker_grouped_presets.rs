use glory::prelude::*;
use sapid::{
    DateLocale, HourCycle, LocalTime, TimeFormat, TimePickerPresetGroup, TimePrecision, time_picker,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    time_picker("showcase-time-picker-grouped-presets")
        .theme(theme.clone())
        .name("Grouped meeting time")
        .format(TimeFormat::new(
            HourCycle::H12,
            TimePrecision::Minute,
            DateLocale::En,
        ))
        .presets(vec![
            TimePickerPresetGroup::new("Morning", [at(9, 0, 0), at(10, 30, 0)]),
            TimePickerPresetGroup::new("Afternoon", [at(13, 30, 0), at(15, 0, 0), at(17, 30, 0)]),
        ])
        .close_on_preset_select(true)
        .default_value(Some(at(13, 30, 0)))
}
