use glory::prelude::*;
use sapid::{DateTimePolicy, DatesProvider, date_time_picker, parse_local_date_time, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = |text: &str| parse_local_date_time(text).unwrap();
    let provider = DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-04").unwrap());
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let current = scope.cage(Some(value("2026-11-04T14:45")));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.)
            .fill((
                date_time_picker(
                    "showcase-date-time-controlled",
                    provider,
                    DateTimePolicy::Civil,
                )
                .theme(controlled_theme.clone())
                .value(current)
                .clearable(true)
                .on_value_change(move |next| current.revise(|old| *old = next)),
                glory::ui::text().content(scope.bond(move || {
                    current
                        .get()
                        .map(sapid::format_local_date_time)
                        .unwrap_or_else(|| "No date and time selected".into())
                })),
            ))
            .show_in(scope)
    })
}
