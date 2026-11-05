use glory::prelude::*;
use sapid::{LocalTime, stack, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage(Some(at(14, 45, 0)));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .clearable(true)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    value.with(|value| match value {
                        Some(value) => {
                            format!("Selected: {:02}:{:02}", value.hour(), value.minute())
                        }
                        None => "Selected: empty".to_owned(),
                    })
                })),
            ))
            .show_in(scope)
    })
}
