use glory::prelude::*;
use sapid::charts::*;
fn candles() -> PlotData {
    PlotData::Candles(vec![
        CandleDatum {
            key: "mon".into(),
            label: "Monday".into(),
            open: 3.,
            high: 8.,
            low: 1.,
            close: 6.,
        },
        CandleDatum {
            key: "tue".into(),
            label: "Tuesday".into(),
            open: 6.,
            high: 9.,
            low: 2.,
            close: 4.,
        },
        CandleDatum {
            key: "wed".into(),
            label: "Wednesday".into(),
            open: 4.,
            high: 7.,
            low: 3.,
            close: 5.,
        },
        CandleDatum {
            key: "thu".into(),
            label: "Thursday".into(),
            open: 5.,
            high: 10.,
            low: 4.,
            close: 9.,
        },
    ])
}
fn sample_data() -> PlotData {
    candles()
}

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let state = scope.cage(0u8);
        let feedback = scope.cage("No chart validation errors.".to_owned());
        let data = scope.bond(move || match state.get() {
            1 => PlotData::Candles(Vec::new()),
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
                candlestick_chart("candlestick-chart-recovery")
                    .theme(theme.clone())
                    .data(data)
                    .on_error(move |error| {
                        feedback.revise(|v| *v = format!("Rejected update: {error}"))
                    }),
            ))
            .show_in(scope)
    })
}
