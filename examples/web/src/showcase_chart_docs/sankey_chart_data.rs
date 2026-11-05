use glory::prelude::*;
use sapid::charts::*;
fn flows() -> PlotData {
    PlotData::Flows(FlowData {
        nodes: vec![
            ChartCategory::new("source", "Source"),
            ChartCategory::new("review", "Review"),
            ChartCategory::new("published", "Published"),
        ],
        links: vec![
            FlowLink {
                key: "draft".into(),
                source: "source".into(),
                target: "review".into(),
                value: 8.,
            },
            FlowLink {
                key: "accepted".into(),
                source: "review".into(),
                target: "published".into(),
                value: 5.,
            },
        ],
    })
}
fn sample_data() -> PlotData {
    flows()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sankey_chart("sankey-chart-data")
        .theme(theme)
        .data(sample_data())
}
