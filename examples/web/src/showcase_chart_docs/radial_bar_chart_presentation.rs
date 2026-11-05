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
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        radial_bar_chart("radial-bar-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
            .default_options(PlotOptions {
                domain: Some(ChartDomain::new(0., 100.).unwrap()),
                angles: Some(PlotAngles {
                    start: -180.,
                    sweep: 180.,
                }),
                ..Default::default()
            })
    }
}
