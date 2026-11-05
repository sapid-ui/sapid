use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::time_value(None).theme(theme.clone()),
        sapid::time_value(None)
            .theme(theme.clone())
            .empty_label("No appointment time"),
        sapid::time_value(Some(clock("00:00"))).theme(theme.clone()),
    ))
}
fn clock(value: &str) -> LocalTime {
    LocalTime::parse_iso(value).unwrap()
}
