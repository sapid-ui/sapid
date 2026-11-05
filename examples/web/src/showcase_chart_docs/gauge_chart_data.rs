use glory::prelude::*;
use sapid::charts::*;
fn targets() -> PlotData {
    PlotData::Targets(vec![TargetDatum {
        key: "capacity".into(),
        label: "Capacity".into(),
        min: 0.,
        max: 100.,
        value: 68.,
        target: Some(80.),
        bands: vec![40., 70., 100.],
    }])
}
fn sample_data() -> PlotData {
    targets()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    gauge_chart("gauge-chart-data")
        .theme(theme)
        .data(sample_data())
}
