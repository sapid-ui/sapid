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
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => PlotData::Hierarchy(Vec::new()),
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
                treemap("treemap-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
