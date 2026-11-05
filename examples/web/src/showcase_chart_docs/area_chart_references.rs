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
    area_chart("area-chart-annotations")
        .theme(theme)
        .data(cartesian_data())
        .reference_lines([ChartReferenceLine::value(16.).label("Target, USD thousands")])
        .reference_areas([ChartReferenceArea::new()
            .categories("m1", "m3")
            .label("Campaign")])
        .reference_dots([ChartReferenceDot::new("m3", 20.).label("Peak")])
}
