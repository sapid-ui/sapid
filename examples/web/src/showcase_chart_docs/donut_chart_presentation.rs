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
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        donut_chart("donut-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
            .default_options(PlotOptions {
                donut_inner_ratio: Some(0.45),
                angles: Some(PlotAngles {
                    start: -180.,
                    sweep: 270.,
                }),
                center_label: Some("Completion".into()),
                ..Default::default()
            })
    }
}
