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
        ("speed", "Speed", 80.0),
        ("quality", "Quality", 65.0),
        ("support", "Support", 72.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    radial_bar_chart("radial-bar-chart-overview")
        .name("Independent tracks and a fixed maximum")
        .theme(theme)
        .data(sample_data())
        .default_options(PlotOptions {
            domain: Some(ChartDomain::new(0., 100.).unwrap()),
            ..Default::default()
        })
}
