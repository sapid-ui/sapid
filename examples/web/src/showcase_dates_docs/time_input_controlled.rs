use glory::prelude::*;
use sapid::{LocalTime, stack, time_input};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let value = scope.cage(Some(at(10, 30, 0)));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                time_input("showcase-time-controlled")
                    .theme(controlled_theme.clone())
                    .value(value)
                    .on_value_change(move |next| value.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    value.with(|value| match value {
                        Some(value) => format!(
                            "Accepted: {:02}:{:02}:{:02}",
                            value.hour(),
                            value.minute(),
                            value.second()
                        ),
                        None => "Accepted: empty".to_owned(),
                    })
                })),
            ))
            .show_in(scope)
    })
}
