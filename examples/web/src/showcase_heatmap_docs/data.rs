use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let leap = CivilDate::parse_iso("2024-02-29").unwrap();
        let range = HeatmapRange::new(
            CivilDate::parse_iso("2024-02-26").unwrap(),
            CivilDate::parse_iso("2024-03-10").unwrap(),
        )
        .unwrap();
        let data = scope.cage(HeatmapData::default());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Record zero")
                        .on_press(move || data.revise(|old| old.set(leap, Some(0.)).unwrap())),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Record eight")
                        .on_press(move || data.revise(|old| old.set(leap, Some(8.)).unwrap())),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Remove the record")
                        .on_press(move || data.revise(|old| old.set(leap, None).unwrap())),
                )),
                heatmap("sparse-values")
                    .theme(theme.clone())
                    .name("Sparse dated values")
                    .range(range)
                    .data(data)
                    .week_start(Weekday::Monday)
                    .domain(Some(HeatmapDomain::new(0., 10.).unwrap()))
                    .with_weekday_labels(true)
                    .with_legend(true)
                    .with_tooltip(true)
                    .cell_size(18.),
                sapid::text()
                    .theme(theme.clone())
                    .content(data.map(move |data| {
                        data.value(leap).map_or_else(
                            || "February 29 has no record".into(),
                            |value| format!("February 29 has {value} contributions"),
                        )
                    })),
            ))
            .show_in(scope)
    })
}
