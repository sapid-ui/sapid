use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("CandlestickChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
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

    fn card(
        theme: sapid::Theme,
        title: &'static str,
        note: &'static str,
        chart: impl Widget,
    ) -> glory::ui::View {
        col().class("site-chart-card").fill((
            sapid::title(3).theme(theme.clone()).content(title),
            sapid::text()
                .theme(theme)
                .class("site-chart-note")
                .content(note),
            view().class("site-chart-canvas").fill(chart),
        ))
    }

    fn grid(cards: Vec<glory::ui::View>) -> glory::ui::View {
        view().class("site-chart-grid").fill(cards)
    }

    pub fn demo_for(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
        match id {
            "CandlestickChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Candlestick chart",
                "Open, high, low and close for four trading days.",
                candlestick_chart("candlestick-demo")
                    .theme(theme)
                    .data(candles())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
