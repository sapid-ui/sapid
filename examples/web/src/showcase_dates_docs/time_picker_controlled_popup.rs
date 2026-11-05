use glory::prelude::*;
use sapid::{LocalTime, stack, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    let popup_theme = theme.clone();
    widget(move |scope| {
        let open = scope.cage(false);
        stack()
            .theme(popup_theme.clone())
            .gap(8.)
            .fill((
                time_picker("showcase-time-picker-controlled-popup")
                    .theme(popup_theme.clone())
                    .open(open)
                    .presets(vec![at(8, 0, 0), at(12, 0, 0), at(18, 0, 0)])
                    .on_open_change(move |next| open.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    if open.get() {
                        "Preset popup: open".to_owned()
                    } else {
                        "Preset popup: closed".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
