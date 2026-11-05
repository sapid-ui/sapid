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
        ("w1", "Week 1", 4.0),
        ("w2", "Week 2", 7.0),
        ("w3", "Week 3", 5.0),
        ("w4", "Week 4", 9.0),
        ("w5", "Week 5", 8.0),
        ("w6", "Week 6", 12.0),
        ("w7", "Week 7", 15.0),
    ])
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
                sparkline("sparkline-controlled")
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
