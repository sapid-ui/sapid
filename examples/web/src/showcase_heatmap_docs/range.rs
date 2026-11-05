use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let start = CivilDate::parse_iso("2024-01-01").unwrap();
        let year = HeatmapRange::new(start, CivilDate::parse_iso("2024-12-31").unwrap()).unwrap();
        let small = HeatmapRange::new(start, start.checked_add_days(13).unwrap()).unwrap();
        let range = scope.cage(small);
        let enabled = scope.cage(true);
        let data = HeatmapData::new(
            (0..year.len())
                .filter(|i| i % 3 != 0)
                .map(|i| (year.date(i).unwrap(), (i % 11) as f64)),
        )
        .unwrap();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show full year")
                        .on_press(move || range.revise(|old| *old = year)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show two weeks")
                        .on_press(move || range.revise(|old| *old = small)),
                )),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable calendar interaction")
                    .checked(enabled)
                    .on_checked_change(move |next| enabled.revise(|old| *old = next)),
                heatmap("range-calendar")
                    .theme(theme.clone())
                    .name("Contribution range")
                    .range(range)
                    .data(data)
                    .enabled(enabled)
                    .with_month_labels(true)
                    .with_weekday_labels(true)
                    .with_legend(true),
                sapid::text()
                    .theme(theme.clone())
                    .content(range.map(|range| format!("Showing {} dates", range.len()))),
            ))
            .show_in(scope)
    })
}
