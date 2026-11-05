use glory::prelude::*;
use sapid::charts::*;
fn points() -> PlotData {
    PlotData::Points(vec![
        PointDatum {
            key: "north".into(),
            label: "North".into(),
            x: 1.,
            y: 3.,
            size: 1.,
        },
        PointDatum {
            key: "south".into(),
            label: "South".into(),
            x: 4.,
            y: 2.,
            size: 4.,
        },
        PointDatum {
            key: "west".into(),
            label: "West".into(),
            x: 2.,
            y: 5.,
            size: 9.,
        },
        PointDatum {
            key: "east".into(),
            label: "East".into(),
            x: 5.,
            y: 4.,
            size: 3.,
        },
    ])
}
fn sample_data() -> PlotData {
    points()
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
                bubble_chart("bubble-chart-controlled")
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
