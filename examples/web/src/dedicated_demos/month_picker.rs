use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(PeriodSelection::Single(None::<YearMonth>));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Start)
            .gap(16.)
            .fill((
                sapid::month_picker("planning-month", provider())
                    .theme(theme.clone())
                    .name("Planning month")
                    .value(value)
                    .on_value_change(move |v| value.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(value.map(|v| match v {
                        PeriodSelection::Single(Some(m)) => format!("Planning month: {m}"),
                        _ => "Choose a month for the next project.".to_owned(),
                    })),
            ))
            .show_in(scope)
    })
}

fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap())
        .today(Some(CivilDate::parse_iso("2026-11-04").unwrap()))
}
