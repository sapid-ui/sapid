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
    candlestick_chart("candlestick-chart-data")
        .theme(theme)
        .data(sample_data())
}
