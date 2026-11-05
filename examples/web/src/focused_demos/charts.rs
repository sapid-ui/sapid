use glory::prelude::*;
use sapid::charts::*;

fn contribution_range() -> HeatmapRange {
    HeatmapRange::new(
        CivilDate::parse_iso("2024-01-01").unwrap(),
        CivilDate::parse_iso("2024-03-31").unwrap(),
    )
    .unwrap()
}
fn contributions() -> HeatmapData {
    let range = contribution_range();
    HeatmapData::new(
        (0..range.len())
            .filter(|i| i % 4 != 0)
            .map(|i| (range.date(i).unwrap(), (i % 11) as f64)),
    )
    .unwrap()
}

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

fn points() -> PlotData {
    PlotData::Points(vec![
        PointDatum {
            key: "north".into(),
            label: "North".into(),
            x: 1.,
            y: 3.,
            size: 1.,
        },
        PointDatum {
            key: "south".into(),
            label: "South".into(),
            x: 4.,
            y: 2.,
            size: 4.,
        },
        PointDatum {
            key: "west".into(),
            label: "West".into(),
            x: 2.,
            y: 5.,
            size: 9.,
        },
        PointDatum {
            key: "east".into(),
            label: "East".into(),
            x: 5.,
            y: 4.,
            size: 3.,
        },
    ])
}

fn candles() -> PlotData {
    PlotData::Candles(vec![
        CandleDatum {
            key: "mon".into(),
            label: "Monday".into(),
            open: 3.,
            high: 8.,
            low: 1.,
            close: 6.,
        },
        CandleDatum {
            key: "tue".into(),
            label: "Tuesday".into(),
            open: 6.,
            high: 9.,
            low: 2.,
            close: 4.,
        },
        CandleDatum {
            key: "wed".into(),
            label: "Wednesday".into(),
            open: 4.,
            high: 7.,
            low: 3.,
            close: 5.,
        },
        CandleDatum {
            key: "thu".into(),
            label: "Thursday".into(),
            open: 5.,
            high: 10.,
            low: 4.,
            close: 9.,
        },
    ])
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

fn area(key: &str) -> Chart {
    area_chart(key)
}
fn bars(key: &str) -> Chart {
    bar_chart(key)
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
    if id != "P11-01" {
        let proportions = || {
            values(&[
                ("a", "Product A", 42.),
                ("b", "Product B", 31.),
                ("c", "Product C", 18.),
                ("other", "Other", 9.),
            ])
        };
        return match id {
            "AreaChart" => Box::new(cartesian_page(
                theme,
                "Area geometry emphasizes the monthly shape and accumulation.",
                area,
                true,
            )),
            "BarChart" => Box::new(cartesian_page(
                theme,
                "Bar geometry compares exact monthly values.",
                bars,
                true,
            )),
            "LineChart" => Box::new(cartesian_page(
                theme,
                "Line geometry shows the monthly revenue trend.",
                line,
                false,
            )),
            "CompositeChart" => Box::new(cartesian_page(
                theme,
                "Composite geometry combines line and bar series.",
                composite,
                false,
            )),
            "DonutChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Donut chart",
                "Completion is 72%; the remaining share is 28%.",
                donut_chart("donut-demo")
                    .theme(theme)
                    .data(values(&[
                        ("done", "Completed", 72.),
                        ("remaining", "Remaining", 28.),
                    ]))
                    .data_table(false),
            )])),
            "PieChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Pie chart",
                "Product share: A 42, B 31, C 18, other 9.",
                pie_chart("pie-demo")
                    .theme(theme)
                    .data(proportions())
                    .data_table(false),
            )])),
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
            "CandlestickChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Candlestick chart",
                "Open, high, low and close for four trading days.",
                candlestick_chart("candlestick-demo")
                    .theme(theme)
                    .data(candles())
                    .data_table(false),
            )])),
            "ScatterChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Scatter chart",
                "Four locations compared by x/y position.",
                scatter_chart("scatter-demo")
                    .theme(theme)
                    .data(points())
                    .data_table(false),
            )])),
            "BubbleChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Bubble chart",
                "Position and bubble size carry independent values.",
                bubble_chart("bubble-demo")
                    .theme(theme)
                    .data(points())
                    .data_table(false),
            )])),
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
            "RadarChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Radar chart",
                "Four dimensions compared on one scale.",
                radar_chart("radar-demo")
                    .theme(theme)
                    .data(values(&[
                        ("speed", "Speed", 80.),
                        ("quality", "Quality", 65.),
                        ("reliability", "Reliability", 90.),
                        ("support", "Support", 72.),
                    ]))
                    .data_table(false),
            )])),
            "Sparkline" => Box::new(grid(vec![card(
                theme.clone(),
                "Sparkline",
                "A compact trend across seven weeks.",
                sparkline("sparkline-demo")
                    .theme(theme)
                    .data(values(&[
                        ("w1", "W1", 4.),
                        ("w2", "W2", 7.),
                        ("w3", "W3", 5.),
                        ("w4", "W4", 9.),
                        ("w5", "W5", 8.),
                        ("w6", "W6", 12.),
                        ("w7", "W7", 15.),
                    ]))
                    .data_table(false),
            )])),
            "Heatmap" => Box::new(grid(vec![card(
                theme.clone(),
                "Heatmap",
                "Dated contributions arranged by week and weekday.",
                heatmap("heatmap-demo")
                    .theme(theme)
                    .range(contribution_range())
                    .data(contributions())
                    .with_month_labels(true)
                    .with_weekday_labels(true)
                    .with_legend(true),
            )])),
            "MatrixChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Matrix chart",
                "A direct row-and-column comparison.",
                matrix_chart("matrix-demo")
                    .theme(theme)
                    .data(matrix())
                    .data_table(false),
            )])),
            "WaffleChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Waffle chart",
                "Proportions rendered as a compact matrix.",
                waffle_chart("waffle-demo")
                    .theme(theme)
                    .data(proportions())
                    .data_table(false),
            )])),
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
            "BulletChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Bullet chart",
                "Current capacity is 68, target is 80.",
                bullet_chart("bullet-demo")
                    .theme(theme)
                    .data(targets())
                    .data_table(false),
            )])),
            "GaugeChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Gauge chart",
                "Capacity and target in a bounded gauge.",
                gauge_chart("gauge-demo")
                    .theme(theme)
                    .data(targets())
                    .data_table(false),
            )])),
            "Treemap" => Box::new(grid(vec![card(
                theme.clone(),
                "Treemap",
                "North, south and west nested under Regions.",
                treemap("treemap-demo")
                    .theme(theme)
                    .data(hierarchy())
                    .data_table(false),
            )])),
            "SunburstChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Sunburst chart",
                "The hierarchy displayed as concentric regions.",
                sunburst_chart("sunburst-demo")
                    .theme(theme)
                    .data(hierarchy())
                    .data_table(false),
            )])),
            "SankeyChart" => Box::new(grid(vec![card(
                theme.clone(),
                "Sankey chart",
                "Source flows into Review, then Published.",
                sankey_chart("sankey-demo")
                    .theme(theme)
                    .data(flows())
                    .data_table(false),
            )])),
            _ => unreachable!("unknown chart guide: {id}"),
        };
    }
    match id {
        "P11-01" => Box::new(widget(move |scope| {
            let data = scope.cage(cartesian_data());
            let selection = scope.cage(None::<ChartSelection>);
            let details = scope.bond(move || {
                selection.with(|selected| {
                    selected.as_ref().and_then(|selected| {
                        data.with(|data| {
                            let series = data
                                .series
                                .iter()
                                .find(|series| series.key == selected.series)?;
                            let category = data
                                .categories
                                .iter()
                                .position(|category| category.key == selected.category)?;
                            Some(ChartPointDetails {
                                selection: selected.clone(),
                                series_label: series.label.clone(),
                                category_label: data.categories[category].label.clone(),
                                value: series.values[category]?,
                            })
                        })
                    })
                })
            });
            view()
                .class("site-chart-grid")
                .fill((
                    card(
                        theme.clone(),
                        "Monthly revenue",
                        "Revenue and cost from January to June. Select a point to inspect its exact value.",
                        line_chart("monthly-revenue")
                            .theme(theme.clone())
                            .data(data)
                            .selection(selection)
                            .pointer_selection(true)
                            .data_table(false),
                    ),
                    tooltip("chart-point-details")
                        .theme(theme.clone())
                        .label("Selected chart point")
                        .point(details),
                ))
                .show_in(scope)
        })),
        "P11-02" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Line chart",
                "Revenue trend; cost is retained as a separate series.",
                line_chart("line-demo")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Area chart",
                "Emphasizes the shape and accumulation of monthly values.",
                area_chart("area-demo")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Bar chart",
                "Compares exact monthly values without implying a trend.",
                bar_chart("bar-demo")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Composite chart",
                "Combines line and bar geometry in one cartesian view.",
                composite_chart("composite-demo")
                    .theme(theme.clone())
                    .data(cartesian_data())
                    .data_table(false),
            ),
        ])),
        "P11-03" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Pie chart",
                "Product share: A 42, B 31, C 18, other 9.",
                pie_chart("pie-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("a", "Product A", 42.),
                        ("b", "Product B", 31.),
                        ("c", "Product C", 18.),
                        ("other", "Other", 9.),
                    ]))
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Donut chart",
                "Completion is 72%; the remaining share is 28%.",
                donut_chart("donut-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("done", "Completed", 72.),
                        ("remaining", "Remaining", 28.),
                    ]))
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Radial bar chart",
                "Three independent scores shown as radial bars.",
                radial_bar_chart("radial-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("speed", "Speed", 80.),
                        ("quality", "Quality", 65.),
                        ("support", "Support", 72.),
                    ]))
                    .data_table(false),
            ),
        ])),
        "P11-04" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Scatter chart",
                "Four locations compared by x/y position.",
                scatter_chart("scatter-demo")
                    .theme(theme.clone())
                    .data(points())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Bubble chart",
                "Position and bubble size carry independent values.",
                bubble_chart("bubble-demo")
                    .theme(theme.clone())
                    .data(points())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Candlestick chart",
                "Open, high, low and close for four trading days.",
                candlestick_chart("candlestick-demo")
                    .theme(theme.clone())
                    .data(candles())
                    .data_table(false),
            ),
        ])),
        "P11-05" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Radar chart",
                "Speed, quality, reliability and support compared on one scale.",
                radar_chart("radar-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("speed", "Speed", 80.),
                        ("quality", "Quality", 65.),
                        ("reliability", "Reliability", 90.),
                        ("support", "Support", 72.),
                    ]))
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Funnel chart",
                "Visitors 1000, trials 640, activated 280, paid 96.",
                funnel_chart("funnel-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("visitors", "Visitors", 1000.),
                        ("trials", "Trials", 640.),
                        ("activated", "Activated", 280.),
                        ("paid", "Paid", 96.),
                    ]))
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Sparkline",
                "A compact trend: 4, 7, 5, 9, 8, 12, 15.",
                sparkline("sparkline-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("w1", "W1", 4.),
                        ("w2", "W2", 7.),
                        ("w3", "W3", 5.),
                        ("w4", "W4", 9.),
                        ("w5", "W5", 8.),
                        ("w6", "W6", 12.),
                        ("w7", "W7", 15.),
                    ]))
                    .data_table(false),
            ),
        ])),
        "P11-06" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Heatmap",
                "Dated contributions arranged by week and weekday.",
                heatmap("heatmap-demo")
                    .theme(theme.clone())
                    .range(contribution_range())
                    .data(contributions())
                    .with_month_labels(true)
                    .with_weekday_labels(true)
                    .with_legend(true),
            ),
            card(
                theme.clone(),
                "Matrix chart",
                "A direct row-and-column comparison of the same values.",
                matrix_chart("matrix-demo")
                    .theme(theme.clone())
                    .data(matrix())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Waffle chart",
                "Proportions rendered as a compact 100-cell matrix.",
                waffle_chart("waffle-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("a", "Product A", 42.),
                        ("b", "Product B", 31.),
                        ("c", "Product C", 18.),
                        ("other", "Other", 9.),
                    ]))
                    .data_table(false),
            ),
        ])),
        "P11-07" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Bars list",
                "Ranked values for product A, B and C.",
                bars_list("bars-list-demo")
                    .theme(theme.clone())
                    .data(values(&[
                        ("a", "Product A", 42.),
                        ("b", "Product B", 31.),
                        ("c", "Product C", 18.),
                    ]))
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Bullet chart",
                "Current capacity is 68, target is 80, with three reference bands.",
                bullet_chart("bullet-demo")
                    .theme(theme.clone())
                    .data(targets())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Gauge chart",
                "A gauge of the same capacity and target range.",
                gauge_chart("gauge-demo")
                    .theme(theme.clone())
                    .data(targets())
                    .data_table(false),
            ),
        ])),
        "P11-08" => Box::new(grid(vec![
            card(
                theme.clone(),
                "Treemap",
                "North, south and west nested under Regions.",
                treemap("treemap-demo")
                    .theme(theme.clone())
                    .data(hierarchy())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Sunburst chart",
                "The same hierarchy displayed as concentric regions.",
                sunburst_chart("sunburst-demo")
                    .theme(theme.clone())
                    .data(hierarchy())
                    .data_table(false),
            ),
            card(
                theme.clone(),
                "Sankey chart",
                "Source flows into Review, then Published.",
                sankey_chart("sankey-demo")
                    .theme(theme.clone())
                    .data(flows())
                    .data_table(false),
            ),
        ])),
        _ => unreachable!("unknown chart guide"),
    }
}
