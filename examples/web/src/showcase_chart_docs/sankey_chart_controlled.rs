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
    widget(move |scope| {
        let accept = scope.cage(false);
        let selected = scope.cage(None::<String>);
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Accept chart changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sankey_chart("sankey-chart-controlled")
                    .theme(theme.clone())
                    .data(sample_data())
                    .selection(selected)
                    .on_selection_change(move |v| {
                        if accept.get() {
                            selected.revise(|old| *old = v);
                        }
                    }),
            ))
            .show_in(scope)
    })
}
