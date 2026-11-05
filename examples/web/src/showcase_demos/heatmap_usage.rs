use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let range = HeatmapRange::new(
            CivilDate::parse_iso("2024-01-01").unwrap(),
            CivilDate::parse_iso("2024-03-31").unwrap(),
        )
        .unwrap();
        let data = HeatmapData::new(
            (0..range.len())
                .filter(|index| index % 5 != 0)
                .map(|index| (range.date(index).unwrap(), (index % 11) as f64)),
        )
        .unwrap();
        let selected = scope.cage("Choose a date to inspect its contributions.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                heatmap("project-contributions")
                    .theme(theme.clone())
                    .name("Project contributions")
                    .range(range)
                    .data(data)
                    .with_month_labels(true)
                    .with_weekday_labels(true)
                    .with_legend(true)
                    .with_tooltip(true)
                    .on_day_click(move |cell| {
                        selected.revise(|old| {
                            *old = cell.value.map_or_else(
                                || format!("{}: No data", cell.date),
                                |value| {
                                    format!(
                                        "{}: {value} contribution{}",
                                        cell.date,
                                        if value == 1. { "" } else { "s" }
                                    )
                                },
                            )
                        })
                    }),
                sapid::text().theme(theme.clone()).content(selected),
            ))
            .show_in(scope)
    })
}
