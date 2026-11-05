use glory::prelude::*;
use sapid::charts::*;
fn cartesian_data() -> ChartData {
    ChartData {
        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
            .into_iter()
            .enumerate()
            .map(|(index, label)| ChartCategory::new(format!("m{index}"), label))
            .collect(),
        series: vec![
            ChartSeries::new(
                "revenue",
                "Revenue",
                [12., 18., 14., 20., 17., 24.].map(Some),
            ),
            ChartSeries::new("cost", "Cost", [8., 10., 9., 12., 11., 13.].map(Some))
                .kind(ChartSeriesKind::Bar),
        ],
    }
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let brushed = scope.cage(false);
        let options = scope.bond(move || CartesianOptions {
            brush: brushed.get().then_some(ChartBrush { first: 2, last: 4 }),
            domain: Some(ChartDomain::new(0., 18.).unwrap()),
            ..Default::default()
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Show March through May")
                    .checked(brushed)
                    .on_checked_change(move |v| brushed.revise(|old| *old = v)),
                bar_chart("bar-chart-window")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .options(options),
            ))
            .show_in(scope)
    })
}
