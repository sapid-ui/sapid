use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let date = |s| sapid::CivilDate::parse_iso(s).unwrap();
        let provider = scope.cage(
            sapid::DatesProvider::new(date("2024-04-13"))
                .bounds(
                    sapid::DateBounds::new(Some(date("2024-04-14")), Some(date("2024-04-24")))
                        .unwrap(),
                )
                .today(Some(date("2024-04-15"))),
        );
        let chinese = scope.cage(false);
        let danger = theme.danger;
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Chinese date labels")
                    .checked(chinese)
                    .on_checked_change(move |next| {
                        chinese.revise(|old| *old = next);
                        provider.revise(|old| {
                            *old = old.locale(if next {
                                sapid::DateLocale::ZhCn
                            } else {
                                sapid::DateLocale::En
                            })
                        });
                    }),
                sapid::mini_calendar("localized-dates", provider)
                    .theme(theme.clone())
                    .name("Localized bounded dates")
                    .number_of_days(6)
                    .exclude_date(date("2024-04-16"))
                    .day_style(move |date| {
                        if matches!(
                            date.weekday(),
                            sapid::Weekday::Saturday | sapid::Weekday::Sunday
                        ) {
                            Style {
                                border_color: Some(danger),
                                ..Style::new()
                            }
                        } else {
                            Style::new()
                        }
                    }),
                sapid::text().theme(theme.clone()).content(
                    "Choose April 14–24; April 16 is unavailable. Weekends use a custom border.",
                ),
            ))
            .show_in(scope)
    })
}
