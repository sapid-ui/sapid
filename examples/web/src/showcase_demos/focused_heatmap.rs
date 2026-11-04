use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Heatmap", theme)
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

    fn matrix() -> PlotData {
        PlotData::Matrix(MatrixData {
            rows: vec![
                ChartCategory::new("product-a", "Product A"),
                ChartCategory::new("product-b", "Product B"),
                ChartCategory::new("product-c", "Product C"),
            ],
            columns: vec![
                ChartCategory::new("q1", "Q1"),
                ChartCategory::new("q2", "Q2"),
                ChartCategory::new("q3", "Q3"),
            ],
            cells: [
                ("a1", "product-a", "q1", 4.),
                ("a2", "product-a", "q2", 7.),
                ("a3", "product-a", "q3", 10.),
                ("b1", "product-b", "q1", 2.),
                ("b2", "product-b", "q2", -3.),
                ("b3", "product-b", "q3", 6.),
                ("c1", "product-c", "q1", 8.),
                ("c2", "product-c", "q2", 5.),
                ("c3", "product-c", "q3", 1.),
            ]
            .into_iter()
            .map(|(key, row, column, value)| MatrixCell {
                key: key.into(),
                row: row.into(),
                column: column.into(),
                value,
            })
            .collect(),
        })
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
            "Heatmap" => Box::new(grid(vec![card(
                theme.clone(),
                "Heatmap",
                "Product intensity across three quarters, including negative values.",
                heatmap("heatmap-demo")
                    .theme(theme)
                    .data(matrix())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
