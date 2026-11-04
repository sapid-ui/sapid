use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("FunnelChart", theme)
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
            "FunnelChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Funnel chart",
                "Visitors 1000, trials 640, activated 280, paid 96.",
                funnel_chart("funnel-demo")
                    .theme(theme)
                    .data(values(&[
                        ("visitors", "Visitors", 1000.),
                        ("trials", "Trials", 640.),
                        ("activated", "Activated", 280.),
                        ("paid", "Paid", 96.),
                    ]))
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
