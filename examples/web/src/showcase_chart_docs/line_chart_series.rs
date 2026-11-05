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
    line_chart("line-chart-series").theme(theme).data({
        let mut data = cartesian_data();
        data.series[0].values[2] = None;
        data.series[1].values[4] = Some(0.);
        data
    })
}
