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
        ("done", "Completed", 72.0),
        ("remaining", "Remaining", 28.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    donut_chart("donut-chart-data")
        .theme(theme)
        .data(sample_data())
}
