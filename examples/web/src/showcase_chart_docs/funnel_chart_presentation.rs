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
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        funnel_chart("funnel-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
    }
}
