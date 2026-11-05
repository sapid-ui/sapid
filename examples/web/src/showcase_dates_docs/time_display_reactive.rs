use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(12.)
            .fill((
                sapid::time_value(selected.map(|v| if *v { Some(clock("18:45")) } else { None }))
                    .theme(theme.clone())
                    .class("time-display-retained")
                    .empty_label("No time selected"),
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle displayed time")
                    .on_press(move || selected.revise(|v| *v = !*v)),
            ))
            .show_in(scope)
    })
}
fn clock(value: &str) -> LocalTime {
    LocalTime::parse_iso(value).unwrap()
}
