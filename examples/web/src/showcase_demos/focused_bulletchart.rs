use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("BulletChart", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::charts::*;

    fn targets() -> PlotData {
        PlotData::Targets(vec![TargetDatum {
            key: "capacity".into(),
            label: "Capacity".into(),
            min: 0.,
            max: 100.,
            value: 68.,
            target: Some(80.),
            bands: vec![40., 70., 100.],
        }])
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
            "BulletChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Bullet chart",
                "Current capacity is 68, target is 80.",
                bullet_chart("bullet-demo")
                    .theme(theme)
                    .data(targets())
                    .data_table(false),
            )])),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
