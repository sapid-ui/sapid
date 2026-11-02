use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("RadialBarChart", theme)
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
        match id {
            "RadialBarChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Radial bar chart",
                "Three independent scores shown as radial bars.",
                radial_bar_chart("radial-demo")
                    .theme(theme)
                    .data(values(&[
                        ("speed", "Speed", 80.),
                        ("quality", "Quality", 65.),
                        ("support", "Support", 72.),
                    ]))
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
