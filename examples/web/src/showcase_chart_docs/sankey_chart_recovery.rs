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
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => PlotData::Flows(FlowData::default()),
            2 => PlotData::Values(vec![ValueDatum::new("invalid", "Rejected value", f64::NAN)]),
            _ => sample_data(),
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show empty data")
                        .on_press(move || state.revise(|old| *old = 1)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Reject invalid update")
                        .on_press(move || state.revise(|old| *old = 2)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("Restore chart data")
                        .on_press(move || state.revise(|old| *old = 0)),
                )),
                sapid::text().theme(theme.clone()).content(feedback),
                sankey_chart("sankey-chart-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
