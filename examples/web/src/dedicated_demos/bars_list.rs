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
        ("north", "North", 42.0),
        ("south", "South", 31.0),
        ("west", "West", 18.0),
        ("returns", "Returns", -9.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    bars_list("bars-list-overview")
        .name("Ranked labels and signed values")
        .theme(theme)
        .data(sample_data())
}
