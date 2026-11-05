#[path = "../../web/src/dedicated_demos/area_chart.rs"]
mod area_chart;
#[path = "../../web/src/dedicated_demos/bar_chart.rs"]
mod bar_chart;
#[path = "../../web/src/dedicated_demos/bars_list.rs"]
mod bars_list;
#[path = "../../web/src/dedicated_demos/bubble_chart.rs"]
mod bubble_chart;
#[path = "../../web/src/dedicated_demos/bullet_chart.rs"]
mod bullet_chart;
#[path = "../../web/src/dedicated_demos/candlestick_chart.rs"]
mod candlestick_chart;
#[path = "../../web/src/dedicated_demos/composite_chart.rs"]
mod composite_chart;
#[path = "../../web/src/dedicated_demos/donut_chart.rs"]
mod donut_chart;
#[path = "../../web/src/showcase_chart_docs/donut_chart_presentation.rs"]
mod donut_chart_presentation;
#[path = "../../web/src/dedicated_demos/funnel_chart.rs"]
mod funnel_chart;
#[path = "../../web/src/dedicated_demos/gauge_chart.rs"]
mod gauge_chart;
#[path = "../../web/src/dedicated_demos/line_chart.rs"]
mod line_chart;
#[path = "../../web/src/dedicated_demos/matrix_chart.rs"]
mod matrix_chart;
#[path = "../../web/src/dedicated_demos/pie_chart.rs"]
mod pie_chart;
#[path = "../../web/src/dedicated_demos/radar_chart.rs"]
mod radar_chart;
#[path = "../../web/src/dedicated_demos/radial_bar_chart.rs"]
mod radial_bar_chart;
#[path = "../../web/src/showcase_chart_docs/line_chart_recovery.rs"]
mod recovery;
#[path = "../../web/src/dedicated_demos/sankey_chart.rs"]
mod sankey_chart;
#[path = "../../web/src/dedicated_demos/scatter_chart.rs"]
mod scatter_chart;
#[path = "../../web/src/dedicated_demos/sparkline.rs"]
mod sparkline;
#[path = "../../web/src/dedicated_demos/sunburst_chart.rs"]
mod sunburst_chart;
#[path = "../../web/src/dedicated_demos/treemap.rs"]
mod treemap;
#[path = "../../web/src/dedicated_demos/waffle_chart.rs"]
mod waffle_chart;
#[path = "../../web/src/showcase_chart_docs/waffle_chart_presentation.rs"]
mod waffle_chart_presentation;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
fn observe(host: &WindowApp) {
    for _ in 0..6 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = vec![];
    while let Some(id) = todo.pop() {
        todo.extend(tree.node(id).unwrap().children.iter().rev());
        out.push(id);
    }
    out
}
fn classes(app: &App, class: &str) -> Vec<NodeId> {
    nodes(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|v| v == class)
        })
        .collect()
}
fn named(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let v = tree.node(*id).unwrap().descriptor.view();
            v.name.as_deref() == Some(name) || matches!(v.props,Props::Button(p) if p.label==name)
        })
        .unwrap_or_else(|| panic!("missing {name}"))
}
fn bounds(app: &App, node: NodeId) -> Rect {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(Query::Bounds(node), move |v| {
        *saved.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(r) = out.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    r
}
fn physical(host: &WindowApp, node: NodeId) -> apace_dom::NodeId {
    host.renderer()
        .engine_id(node, glory::html::Part::Main)
        .unwrap()
}
fn content(app: &App) -> String {
    nodes(app)
        .into_iter()
        .filter_map(
            |id| match app.tree().node(id).unwrap().descriptor.view().props {
                Props::Text(p) => Some(p.content.clone()),
                _ => None,
            },
        )
        .collect::<Vec<_>>()
        .join("|")
}
fn actual_example(slug: &str, theme: sapid::Theme) -> Box<dyn Widget> {
    match slug {
        "area-chart" => Box::new(area_chart::demo(theme)),
        "bar-chart" => Box::new(bar_chart::demo(theme)),
        "line-chart" => Box::new(line_chart::demo(theme)),
        "composite-chart" => Box::new(composite_chart::demo(theme)),
        "candlestick-chart" => Box::new(candlestick_chart::demo(theme)),
        "donut-chart" => Box::new(donut_chart::demo(theme)),
        "pie-chart" => Box::new(pie_chart::demo(theme)),
        "funnel-chart" => Box::new(funnel_chart::demo(theme)),
        "radar-chart" => Box::new(radar_chart::demo(theme)),
        "scatter-chart" => Box::new(scatter_chart::demo(theme)),
        "bubble-chart" => Box::new(bubble_chart::demo(theme)),
        "radial-bar-chart" => Box::new(radial_bar_chart::demo(theme)),
        "sparkline" => Box::new(sparkline::demo(theme)),
        "bars-list" => Box::new(bars_list::demo(theme)),
        "bullet-chart" => Box::new(bullet_chart::demo(theme)),
        "gauge-chart" => Box::new(gauge_chart::demo(theme)),
        "waffle-chart" => Box::new(waffle_chart::demo(theme)),
        "treemap" => Box::new(treemap::demo(theme)),
        "sunburst-chart" => Box::new(sunburst_chart::demo(theme)),
        "sankey-chart" => Box::new(sankey_chart::demo(theme)),
        "matrix-chart" => Box::new(matrix_chart::demo(theme)),
        _ => unreachable!(),
    }
}
#[test]
fn every_actual_primary_chart_has_full_width_narrow_geometry_compact_wrapping_and_live_selection() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for slug in [
            "area-chart",
            "bar-chart",
            "line-chart",
            "composite-chart",
            "candlestick-chart",
            "donut-chart",
            "pie-chart",
            "funnel-chart",
            "radar-chart",
            "scatter-chart",
            "bubble-chart",
            "radial-bar-chart",
            "sparkline",
            "bars-list",
            "bullet-chart",
            "gauge-chart",
            "waffle-chart",
            "treemap",
            "sunburst-chart",
            "sankey-chart",
            "matrix-chart",
        ] {
            let host = WindowApp::new();
            host.renderer().set_viewport(320, 1000, 1.).unwrap();
            host.mount(
                glory::ui::view()
                    .style(Style::new().width(Length::Px(300.)))
                    .fill(actual_example(slug, theme.clone())),
            )
            .unwrap_or_else(|e| panic!("{slug}: {e}"));
            observe(&host);
            let app = host.app();
            let cartesian = matches!(
                slug,
                "area-chart" | "bar-chart" | "line-chart" | "composite-chart"
            );
            let frame = classes(
                app,
                if cartesian {
                    "sapid-chart-plot"
                } else {
                    "sapid-plot-drawing"
                },
            )[0];
            let r = bounds(app, frame);
            assert!((r.width - 300.).abs() < 0.5, "{slug}: {r:?}");
            assert!(r.height >= 70. && r.height <= 160., "{slug}: {r:?}");
            let first = physical(&host, frame);
            let legend = classes(
                app,
                if cartesian {
                    "sapid-chart-legend-item"
                } else {
                    "sapid-plot-legend-item"
                },
            );
            if slug == "sparkline" {
                assert!(legend.is_empty());
                assert!(classes(app, "sapid-plot-controls").is_empty());
                assert!(content(app).contains("Weekly values: 4, 7, 5, 9, 8, 12, 15"));
            } else {
                for id in &legend {
                    let b = bounds(app, *id);
                    assert!(b.height <= 30.5 && b.x + b.width <= 300.5, "{slug}: {b:?}");
                }
                if let Some(id) = legend.first().copied() {
                    let retained = physical(&host, id);
                    app.dispatch(id, Event::Activate).unwrap();
                    observe(&host);
                    assert_eq!(physical(&host, id), retained);
                    app.dispatch(id, Event::Activate).unwrap();
                    observe(&host);
                }
                let next = named(app, "Next data point");
                let b = bounds(app, next);
                assert!(b.height <= 30.5 && b.x + b.width <= 300.5, "{slug}: {b:?}");
                app.dispatch(next, Event::Activate).unwrap();
                observe(&host);
                assert!(!content(app).contains("No data point selected"), "{slug}");
            }
            host.renderer().set_viewport(700, 1000, 1.).unwrap();
            observe(&host);
            assert_eq!(physical(&host, frame), first);
            app.clear().unwrap();
            assert_eq!(app.stats().handlers, 0);
        }
    }
}
#[test]
fn actual_configured_donut_and_waffle_sources_render_center_text_and_exact_twenty_cells() {
    for waffle in [false, true] {
        let host = WindowApp::new();
        host.renderer().set_viewport(420, 800, 1.).unwrap();
        let example: Box<dyn Widget> = if waffle {
            Box::new(waffle_chart_presentation::demo(sapid::Theme::blue_light()))
        } else {
            Box::new(donut_chart_presentation::demo(sapid::Theme::blue_light()))
        };
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(400.)))
                .fill(example),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let frame = bounds(app, classes(app, "sapid-plot-drawing")[0]);
        if waffle {
            let cells: usize = classes(app, "sapid-plot-layer")
                .into_iter()
                .map(
                    |id| match app.tree().node(id).unwrap().descriptor.view().props {
                        Props::Graphic(p) => p
                            .commands
                            .iter()
                            .filter(|c| matches!(c, glory::render::PathCommand::Move(_)))
                            .count(),
                        _ => 0,
                    },
                )
                .sum();
            assert_eq!(cells, 20);
        } else {
            let label=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Text(p) if p.content=="Completion")).unwrap();
            let b = bounds(app, label);
            assert!((b.x + b.width / 2. - frame.x - frame.width / 2.).abs() < 1.);
            assert!((b.y + b.height / 2. - frame.y - frame.height / 2.).abs() < 2.);
            assert!(b.width < frame.width / 2.);
        }
        let mut raster = glory::apace::Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert!(!raster.pixels().is_empty());
        app.clear().unwrap();
    }
}
#[test]
fn actual_rejection_feedback_retains_scene_and_recovers_without_native_host_failure() {
    let host = WindowApp::new();
    host.mount(recovery::demo(sapid::Theme::green_dark()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let frame = classes(app, "sapid-chart-plot")[0];
    let original = physical(&host, frame);
    let points = classes(app, "sapid-chart-point");
    app.dispatch(named(app, "Reject invalid update"), Event::Activate)
        .unwrap();
    observe(&host);
    assert_eq!(classes(app, "sapid-chart-point"), points);
    assert!(content(app).contains("Rejected update:"));
    app.dispatch(named(app, "Show empty data"), Event::Activate)
        .unwrap();
    observe(&host);
    assert!(classes(app, "sapid-chart-point").is_empty());
    app.dispatch(named(app, "Restore chart data"), Event::Activate)
        .unwrap();
    observe(&host);
    assert_eq!(classes(app, "sapid-chart-point").len(), points.len());
    assert_eq!(physical(&host, frame), original);
    app.clear().unwrap();
}

#[test]
fn actual_matrix_axis_names_have_narrow_geometry_and_values_paint_an_opaque_readable_backdrop() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 900, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(matrix_chart::demo(theme.clone())),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let labels = classes(app, "sapid-plot-label");
        assert_eq!(labels.len(), 15);
        let find_label = |value: &str| {
            *labels.iter().find(|id|matches!(app.tree().node(**id).unwrap().descriptor.view().props,Props::Text(p) if p.content==value)).unwrap()
        };
        let row = bounds(app, find_label("Product A"));
        let column = bounds(app, find_label("Q1"));
        assert!(row.x + row.width < column.x && row.y > column.y);
        assert!(row.width >= 50. && row.height >= 12.);
        let value = bounds(app, find_label("10"));
        assert!(value.width >= 9. && value.width <= 40. && value.height >= 12.);
        let mut raster = glory::apace::Raster::new();
        raster.draw(host.renderer()).unwrap();
        let width = raster.frame().unwrap().width as usize;
        let x = (value.x + 1.).round() as usize;
        let y = (value.y + 1.).round() as usize;
        let offset = (y * width + x) * 4;
        let actual = &raster.pixels()[offset..offset + 4];
        let expected = [
            theme.surface.0,
            theme.surface.1,
            theme.surface.2,
            theme.surface.3,
        ];
        for (a, b) in actual.iter().zip(expected) {
            assert!(a.abs_diff(b) <= 2, "{actual:?} vs {expected:?}");
        }
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
