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
                gauge_chart("gauge-chart-controlled")
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
