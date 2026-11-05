use glory::prelude::*;
use sapid::charts::*;
fn cartesian_data() -> ChartData {
    ChartData {
        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
            .into_iter()
            .enumerate()
            .map(|(index, label)| ChartCategory::new(format!("m{index}"), label))
            .collect(),
        series: vec![
            ChartSeries::new(
                "revenue",
                "Revenue",
                [12., 18., 14., 20., 17., 24.].map(Some),
            ),
            ChartSeries::new("cost", "Cost", [8., 10., 9., 12., 11., 13.].map(Some))
                .kind(ChartSeriesKind::Bar),
        ],
    }
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accepted = scope.cage(false);
        let selected = scope.cage(None::<ChartSelection>);
        let options = scope.cage(CartesianOptions::default());
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Accept chart changes")
                    .checked(accepted)
                    .on_checked_change(move |v| accepted.revise(|old| *old = v)),
                bar_chart("bar-chart-controlled")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .selection(selected)
                    .options(options)
                    .on_selection_change(move |v| {
                        if accepted.get() {
                            selected.revise(|old| *old = v);
                        }
                    })
                    .on_options_change(move |v| {
                        if accepted.get() {
                            options.revise(|old| *old = v);
                        }
                    }),
            ))
            .show_in(scope)
    })
}
