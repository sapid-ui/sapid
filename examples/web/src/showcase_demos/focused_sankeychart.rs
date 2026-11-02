use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("SankeyChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::charts::*;

    fn flows() -> PlotData {
        PlotData::Flows(FlowData {
            nodes: vec![
                ChartCategory::new("source", "Source"),
                ChartCategory::new("review", "Review"),
                ChartCategory::new("published", "Published"),
            ],
            links: vec![
                FlowLink {
                    key: "draft".into(),
                    source: "source".into(),
                    target: "review".into(),
                    value: 8.,
                },
                FlowLink {
                    key: "accepted".into(),
                    source: "review".into(),
                    target: "published".into(),
                    value: 5.,
                },
            ],
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
            "SankeyChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Sankey chart",
                "Source flows into Review, then Published.",
                sankey_chart("sankey-demo")
                    .theme(theme)
                    .data(flows())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
