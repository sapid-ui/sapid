use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("SunburstChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::charts::*;

    fn hierarchy() -> PlotData {
        PlotData::Hierarchy(vec![
            HierarchyDatum {
                key: "regions".into(),
                label: "Regions".into(),
                parent: None,
                weight: None,
            },
            HierarchyDatum {
                key: "north".into(),
                label: "North".into(),
                parent: Some("regions".into()),
                weight: Some(3.),
            },
            HierarchyDatum {
                key: "south".into(),
                label: "South".into(),
                parent: Some("regions".into()),
                weight: Some(7.),
            },
            HierarchyDatum {
                key: "west".into(),
                label: "West".into(),
                parent: Some("regions".into()),
                weight: Some(5.),
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
            "SunburstChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Sunburst chart",
                "The hierarchy displayed as concentric regions.",
                sunburst_chart("sunburst-demo")
                    .theme(theme)
                    .data(hierarchy())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
