use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("AreaChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
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

    pub fn demo_for(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
        match id {
            "AreaChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Area chart",
                "Emphasizes the shape and accumulation of monthly values.",
                area_chart("area-demo")
                    .theme(theme)
                    .data(cartesian_data())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
