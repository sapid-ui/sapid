use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{Event, GridCommand, NodeId, Props, Query, QueryValue, Rect, Role};
use sapid::Theme;
use sapid::charts::*;
use std::{cell::RefCell, rc::Rc};
fn d(s: &str) -> CivilDate {
    CivilDate::parse_iso(s).unwrap()
}
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        out.push(id)
    }
    out
}
fn day(app: &App, date: CivilDate) -> NodeId {
    let name = DateLocale::En.date_label(date);
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let node = tree.node(*id).unwrap().descriptor.view();
            matches!(node.props, Props::Button(_))
                && node.name.as_ref().is_some_and(|n| n.starts_with(&name))
        })
        .unwrap()
}
fn query(app: &App, q: Query) -> QueryValue {
    let out = Rc::new(RefCell::new(None));
    let result = out.clone();
    app.request(q, move |value| *result.borrow_mut() = Some(value.unwrap()))
        .unwrap();
    app.pump().unwrap();
    out.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    rect
}
#[test]
fn four_theme_date_columns_have_actual_geometry_pixels_focus_and_stable_cells() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(720, 320, 1.).unwrap();
        let app = host.app();
        let start = d("2024-02-26");
        let data = app
            .scope()
            .cage(HeatmapData::new([(start, 0.), (d("2024-02-29"), 10.)]).unwrap());
        let root = app.scope().cage(None);
        host.mount(
            heatmap("native-calendar")
                .theme(theme.clone())
                .range(HeatmapRange::new(start, d("2024-03-10")).unwrap())
                .data(data)
                .domain(Some(HeatmapDomain::new(0., 10.).unwrap()))
                .week_start(Weekday::Monday)
                .cell_size(16.)
                .gap(4.)
                .with_month_labels(true)
                .with_weekday_labels(true)
                .with_legend(true)
                .node_ref(root),
        )
        .unwrap();
        app.settle(16).unwrap();
        let first = day(app, start);
        let down = day(app, d("2024-02-27"));
        let right = day(app, d("2024-03-04"));
        let a = bounds(app, first);
        let b = bounds(app, down);
        let c = bounds(app, right);
        assert!((a.width - 16.).abs() < 0.2 && (a.height - 16.).abs() < 0.2);
        assert!((b.y - a.y - 20.).abs() < 0.2);
        assert!((c.x - a.x - 20.).abs() < 0.2);
        assert!((c.y - a.y).abs() < 0.2);
        let physical = host
            .renderer()
            .engine_id(first, glory::html::Part::Main)
            .unwrap();
        data.revise(|old| old.set(start, Some(5.)).unwrap());
        app.settle(16).unwrap();
        assert_eq!(day(app, start), first);
        assert_eq!(
            host.renderer()
                .engine_id(first, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let offset = (((a.y + 5.).round() as usize) * raster.frame().unwrap().width as usize
            + (a.x + 5.).round() as usize)
            * 4;
        let color = theme.shades[6];
        for (actual, expected) in raster.pixels()[offset..offset + 4]
            .iter()
            .zip([color.0, color.1, color.2, color.3])
        {
            assert!(actual.abs_diff(expected) <= 2);
        }
        app.scope()
            .action(glory::render::Action::Focus(first))
            .unwrap();
        app.flush().unwrap();
        let boundary = all(app)
            .into_iter()
            .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Grid))
            .unwrap();
        app.dispatch(
            boundary,
            Event::GridNavigate {
                from: first,
                command: GridCommand::Right,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(query(app, Query::Focus(right)), QueryValue::Focus(true));
        app.clear().unwrap();
        assert!(root.get().is_none());
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn tooltip_support_is_an_explicit_native_boundary() {
    let host = WindowApp::new();
    let date = CivilDate::MIN;
    assert!(
        host.mount(
            heatmap("tooltip")
                .range(HeatmapRange::new(date, date).unwrap())
                .with_tooltip(true)
        )
        .is_err()
    );
}
