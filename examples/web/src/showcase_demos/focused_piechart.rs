use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("PieChart", theme)
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

    pub fn demo_for(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
        let proportions = || {
            values(&[
                ("a", "Product A", 42.),
                ("b", "Product B", 31.),
                ("c", "Product C", 18.),
                ("other", "Other", 9.),
            ])
        };
        match id {
            "PieChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Pie chart",
                "Product share: A 42, B 31, C 18, other 9.",
                pie_chart("pie-demo")
                    .theme(theme)
                    .data(proportions())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
