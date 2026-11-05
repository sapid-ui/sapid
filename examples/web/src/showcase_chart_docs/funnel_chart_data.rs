use glory::prelude::*;
use sapid::charts::*;
fn values(values: &[(&str, &str, f64)]) -> PlotData {
    PlotData::Values(
        values
            .iter()
            .map(|(key, label, value)| ValueDatum::new(*key, *label, *value))
            .collect(),
    )
}
fn sample_data() -> PlotData {
    values(&[
        ("visitors", "Visitors", 1000.0),
        ("trials", "Trials", 640.0),
        ("active", "Activated", 280.0),
        ("paid", "Paid", 96.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    funnel_chart("funnel-chart-data")
        .theme(theme)
        .data(sample_data())
}
