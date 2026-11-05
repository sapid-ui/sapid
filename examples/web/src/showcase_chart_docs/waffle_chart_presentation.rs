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
        ("a", "Product A", 42.0),
        ("b", "Product B", 31.0),
        ("c", "Product C", 18.0),
        ("other", "Other", 9.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let mut palette = ChartTheme::from_theme(&theme);
        palette.series.rotate_left(2);
        waffle_chart("waffle-chart-presentation")
            .theme(theme)
            .data(sample_data())
            .frame(ChartFrame {
                width: 480.,
                height: 300.,
            })
            .chart_theme(palette)
            .default_options(PlotOptions {
                waffle_grid: Some(WaffleGrid {
                    rows: 4,
                    columns: 5,
                    gap: 3.,
                }),
                ..Default::default()
            })
    }
}
