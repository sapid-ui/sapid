use glory::prelude::*;
use sapid::charts::*;
fn hierarchy() -> PlotData {
    PlotData::Hierarchy(vec![
        HierarchyDatum {
            key: "regions".into(),
            label: "Regions".into(),
            parent: None,
            weight: None,
        },
        HierarchyDatum {
            key: "north".into(),
            label: "North".into(),
            parent: Some("regions".into()),
            weight: Some(3.),
        },
        HierarchyDatum {
            key: "south".into(),
            label: "South".into(),
            parent: Some("regions".into()),
            weight: Some(7.),
        },
        HierarchyDatum {
            key: "west".into(),
            label: "West".into(),
            parent: Some("regions".into()),
            weight: Some(5.),
        },
    ])
}
fn sample_data() -> PlotData {
    hierarchy()
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
                treemap("treemap-controlled")
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
