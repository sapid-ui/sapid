use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(DateSelection::Single(None));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(16.)
            .fill((
                sapid::date_picker("review-appointment", provider())
                    .theme(theme.clone())
                    .name("Review appointment date")
                    .value(value)
                    .on_value_change(move |v| value.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| match v {
                        DateSelection::Single(Some(d)) => format!("Your review date: {d}"),
                        _ => "Choose a date for your project review.".to_owned(),
                    })),
            ))
            .show_in(scope)
    })
}

fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap())
        .today(Some(CivilDate::parse_iso("2026-11-04").unwrap()))
}
