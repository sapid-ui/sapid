use glory::prelude::*;
use sapid::charts::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let start = CivilDate::parse_iso("2024-02-26").unwrap();
        let range = HeatmapRange::new(start, start.checked_add_days(13).unwrap()).unwrap();
        let data = HeatmapData::new((0..14).map(|i| (range.date(i).unwrap(), (i % 3 + 2) as f64)))
            .unwrap();
        let fixed = scope.cage(false);
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::checkbox().theme(theme.clone()).label("Use a fixed zero to ten domain").checked(fixed).on_checked_change(move|next|fixed.revise(|old|*old=next)),
            heatmap("domain-values").theme(theme.clone()).name("Contribution value domain").range(range).data(data).domain(fixed.map(|fixed|fixed.then(||HeatmapDomain::new(0.,10.).unwrap()))).with_legend(true).with_tooltip(true).label_format(|cell|format!("{} — {} tasks",cell.date,cell.value.unwrap())),
            sapid::text().theme(theme.clone()).content("The data stays between 2 and 4. Change the domain to compare the same dates on a shared scale."),
        )).show_in(scope)
    })
}
