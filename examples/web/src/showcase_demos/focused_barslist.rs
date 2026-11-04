use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("BarsList", theme)
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

    fn bars(key: &str) -> Chart {
        bar_chart(key)
    }
    pub fn demo_for(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
        match id {
            "BarsList" => Box::new(grid(vec![card(
                theme.clone(),
                "Bars list",
                "Ranked values for product A, B and C.",
                bars_list("bars-list-demo")
                    .theme(theme)
                    .data(values(&[
                        ("a", "Product A", 42.),
                        ("b", "Product B", 31.),
                        ("c", "Product C", 18.),
                    ]))
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
