use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("CompositeChart", theme)
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

    fn area(key: &str) -> Chart {
        area_chart(key)
    }
    fn line(key: &str) -> Chart {
        line_chart(key)
    }
    fn composite(key: &str) -> Chart {
        composite_chart(key)
    }

    fn cartesian_page(
        theme: sapid::Theme,
        title: &'static str,
        make: fn(&str) -> Chart,
        stacked: bool,
    ) -> glory::ui::View {
        let mut cards = vec![card(
            theme.clone(),
            "Usage",
            title,
            make("cartesian-usage")
                .theme(theme.clone())
                .data(cartesian_data())
                .data_table(false),
        )];
        if stacked {
            cards.push(card(
                theme.clone(),
                "Stacked series",
                "The finite model stacks admitted series without changing their data identities.",
                make("cartesian-stacked")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .default_options(CartesianOptions {
                        stacked: true,
                        ..Default::default()
                    })
                    .data_table(false),
            ));
        } else {
            cards.push(card(
                theme.clone(),
                "Legend",
                "Legend controls use stable series keys and can hide or restore current geometry.",
                make("cartesian-legend")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .data_table(false),
            ));
        }
        cards.extend([
        card(
            theme.clone(),
            "Reference annotations",
            "A bounded area, target line and exact point share the portable chart geometry.",
            make("cartesian-references")
                .theme(theme.clone())
                .data(cartesian_data())
                .reference_areas([
                    ChartReferenceArea::new()
                        .categories("m1", "m3")
                        .label("Campaign"),
                ])
                .reference_lines([ChartReferenceLine::value(16.).label("Target")])
                .reference_dots([ChartReferenceDot::new("m3", 20.).label("Peak")])
                .data_table(false),
        ),
        card(
            theme.clone(),
            "Brush range",
            "The inclusive brush displays March through May while the admitted data remains complete.",
            make("cartesian-brush")
                .theme(theme)
                .data(cartesian_data())
                .default_options(CartesianOptions {
                    brush: Some(ChartBrush { first: 2, last: 4 }),
                    ..Default::default()
                })
                .data_table(false),
        ),
    ]);
        grid(cards)
    }

    pub fn demo_for(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
        match id {
            "CompositeChart" => Box::new(cartesian_page(
                theme,
                "Composite geometry combines line and bar series.",
                composite,
                false,
            )),
            _ => unreachable!("unsupported named demo: {id}"),
        }
    }
}
