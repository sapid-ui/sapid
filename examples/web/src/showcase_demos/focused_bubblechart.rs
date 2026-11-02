use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("BubbleChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
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
            "BubbleChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Bubble chart",
                "Position and bubble size carry independent values.",
                bubble_chart("bubble-demo")
                    .theme(theme)
                    .data(points())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
