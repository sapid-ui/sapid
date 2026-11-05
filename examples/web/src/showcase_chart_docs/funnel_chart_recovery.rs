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
        ("visitors", "Visitors", 1000.0),
        ("trials", "Trials", 640.0),
        ("active", "Activated", 280.0),
        ("paid", "Paid", 96.0),
    ])
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => PlotData::Values(Vec::new()),
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
                funnel_chart("funnel-chart-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
