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
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => ChartData::default(),
            2 => {
                let mut d = cartesian_data();
                d.series[0].values[0] = Some(f64::NAN);
                d
            }
            _ => cartesian_data(),
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show empty data")
                        .on_press(move || state.revise(|old| *old = 1)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reject invalid update")
                        .on_press(move || state.revise(|old| *old = 2)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restore chart data")
                        .on_press(move || state.revise(|old| *old = 0)),
                )),
                sapid::text().theme(theme.clone()).content(feedback),
                composite_chart("composite-chart-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
