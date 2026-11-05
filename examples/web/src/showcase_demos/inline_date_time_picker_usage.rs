use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let confirmation = scope.cage("No review has been confirmed yet.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::inline_date_time_picker(
                    "inline-project-review",
                    provider(),
                    DateTimePolicy::Civil,
                )
                .theme(theme.clone())
                .name("Project review date and time")
                .default_time(LocalTime::new(9, 30, 0).unwrap())
                .default_value(Some(
                    sapid::parse_local_date_time("2026-11-04T09:30").unwrap(),
                ))
                .time_dropdown(true)
                .time_steps(1, 15, 1)
                .time_presets(vec![
                    LocalTime::new(9, 0, 0).unwrap(),
                    LocalTime::new(10, 30, 0).unwrap(),
                    LocalTime::new(14, 0, 0).unwrap(),
                ])
                .on_commit(move |outcome| {
                    confirmation.revise(|s| {
                        *s = outcome
                            .map(|value| {
                                format!(
                                    "Last confirmed review: {}",
                                    sapid::format_local_date_time(value.local)
                                )
                            })
                            .unwrap_or_else(|| "No review time is confirmed.".to_owned())
                    })
                }),
                sapid::text()
                    .theme(theme.clone())
                    .variant(TextVariant::Small)
                    .content(confirmation),
            ))
            .show_in(scope)
    })
}
fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap())
        .today(Some(CivilDate::parse_iso("2026-11-04").unwrap()))
}
