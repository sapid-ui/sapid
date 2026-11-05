#![cfg(feature = "charts")]
use glory::{
    prelude::*,
    render::{Event, EventKind, GridCommand, NodeId, Props, Role},
};
use sapid::charts::*;
use std::{cell::RefCell, rc::Rc};
fn d(s: &str) -> CivilDate {
    CivilDate::parse_iso(s).unwrap()
}
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        out.push(id);
    }
    out
}
fn day(app: &App, date: CivilDate) -> NodeId {
    let label = DateLocale::En.date_label(date);
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let node = tree.node(*id).unwrap().descriptor.view();
            matches!(node.props, Props::Button(_))
                && node
                    .name
                    .as_ref()
                    .is_some_and(|name| name.starts_with(&label))
        })
        .unwrap()
}
fn grid(app: &App) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Grid))
        .unwrap()
}
#[test]
fn dated_cells_distinguish_missing_zero_and_current_values_and_retain_identity() {
    let app = App::memory();
    let range = HeatmapRange::new(d("2024-02-26"), d("2024-03-10")).unwrap();
    let data = app
        .scope()
        .cage(HeatmapData::new([(d("2024-02-29"), 0.), (d("2024-03-01"), 10.)]).unwrap());
    app.mount(
        heatmap("contributions")
            .range(range)
            .data(data)
            .domain(Some(HeatmapDomain::new(0., 10.).unwrap()))
            .with_month_labels(true)
            .with_weekday_labels(true)
            .with_legend(true),
    )
    .unwrap();
    let missing = day(&app, d("2024-02-28"));
    let zero = day(&app, d("2024-02-29"));
    {
        let tree = app.tree();
        let a = tree.node(missing).unwrap().descriptor.view();
        let b = tree.node(zero).unwrap().descriptor.view();
        assert!(a.name.as_ref().unwrap().ends_with("No data"));
        assert!(b.name.as_ref().unwrap().ends_with("0 contributions"));
        assert_ne!(a.style.background, b.style.background);
    }
    let mut before = nodes(&app);
    before.sort();
    data.revise(|old| old.set(d("2024-02-29"), Some(5.)).unwrap());
    app.settle(16).unwrap();
    assert_eq!(day(&app, d("2024-02-29")), zero);
    let mut after = nodes(&app);
    after.sort();
    assert_eq!(before, after);
    assert!(
        app.tree()
            .node(zero)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_ref()
            .unwrap()
            .ends_with("5 contributions")
    );
    let html = glory::ssr::render(
        heatmap("ssr")
            .range(range)
            .data(data.get())
            .with_tooltip(true),
    )
    .unwrap();
    assert!(html.contains("role=\"grid\""));
    assert!(html.contains("role=\"tooltip\""));
    assert!(html.contains("2024"));
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn keyboard_cursor_and_callbacks_read_current_sources_and_retire_removed_dates() {
    let app = App::memory();
    let start = d("2024-02-26");
    let range = app
        .scope()
        .cage(HeatmapRange::new(start, d("2024-03-10")).unwrap());
    let data = app.scope().cage(HeatmapData::default());
    let enabled = app.scope().cage(true);
    let calls = Rc::new(RefCell::new(Vec::new()));
    let seen = calls.clone();
    let root = app.scope().cage(None);
    app.mount(
        heatmap("live")
            .range(range)
            .data(data)
            .enabled(enabled)
            .node_ref(root)
            .week_start(Weekday::Monday)
            .on_day_click(move |cell| seen.borrow_mut().push(cell)),
    )
    .unwrap();
    let from = day(&app, start);
    let boundary = grid(&app);
    app.scope()
        .batch(|| {
            for command in [GridCommand::Right, GridCommand::Down] {
                app.dispatch(boundary, Event::GridNavigate { from, command })
                    .unwrap();
            }
            data.revise(|old| old.set(d("2024-03-05"), Some(8.)).unwrap());
            app.dispatch(from, Event::Activate).unwrap();
        })
        .unwrap();
    assert_eq!(
        *calls.borrow(),
        vec![HeatmapCell {
            date: d("2024-03-05"),
            value: Some(8.)
        }]
    );
    app.settle(16).unwrap();
    enabled.revise(|old| *old = false);
    app.dispatch(from, Event::Activate).unwrap();
    assert_eq!(calls.borrow().len(), 1);
    enabled.revise(|old| *old = true);
    let removed = day(&app, d("2024-03-05"));
    range.revise(|old| *old = HeatmapRange::new(start, d("2024-03-03")).unwrap());
    app.settle(16).unwrap();
    assert!(app.tree().node(removed).is_err());
    assert_eq!(day(&app, start), from);
    assert_eq!(
        nodes(&app)
            .into_iter()
            .filter(
                |id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::GridCell)
            )
            .count(),
        7
    );
    app.clear().unwrap();
    assert!(root.get().is_none());
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn invalid_geometry_missing_range_and_text_budgets_publish_no_nodes() {
    let app = App::memory();
    let range = HeatmapRange::new(
        CivilDate::MIN,
        CivilDate::MIN.checked_add_days(365).unwrap(),
    )
    .unwrap();
    assert!(app.mount(heatmap("missing-range")).is_err());
    assert!(
        app.mount(heatmap("invalid-size").range(range).cell_size(f64::NAN))
            .is_err()
    );
    assert!(
        app.mount(heatmap("invalid-key".repeat(5)).range(range))
            .is_err()
    );
    assert!(
        app.mount(
            heatmap("invalid-label")
                .range(range)
                .label_format(|_| String::new())
        )
        .is_err()
    );
    assert!(
        app.mount(
            heatmap("label-budget")
                .range(range)
                .label_format(|_| "x".repeat(256))
        )
        .is_err()
    );
    assert!(
        app.mount(
            heatmap("colors-budget")
                .range(range)
                .colors([glory::style::Color(0, 0, 0, 255); 9])
        )
        .is_err()
    );
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn pointer_and_keyboard_share_one_current_tooltip_and_close_on_disable() {
    let app = App::memory();
    let first = d("2024-02-28");
    let second = d("2024-02-29");
    let enabled = app.scope().cage(true);
    app.mount(
        heatmap("single-hint")
            .range(HeatmapRange::new(first, second).unwrap())
            .enabled(enabled)
            .with_tooltip(true),
    )
    .unwrap();
    let count = || {
        nodes(&app)
            .into_iter()
            .filter(|id| {
                let tree = app.tree();
                let node = tree.node(*id).unwrap().descriptor.view();
                node.role == Some(Role::Tooltip)
                    && !node
                        .semantics
                        .as_ref()
                        .is_some_and(|semantics| semantics.hidden)
            })
            .count()
    };
    let hovered = day(&app, first);
    app.dispatch(
        hovered,
        Event::Pointer {
            from: hovered,
            related: None,
            phase: glory::render::PointerPhase::Enter,
        },
    )
    .unwrap();
    assert_eq!(count(), 1);
    app.dispatch(day(&app, second), Event::FocusChanged(true))
        .unwrap();
    assert_eq!(
        count(),
        1,
        "a still-hovered date cannot keep an earlier tooltip open"
    );
    enabled.revise(|value| *value = false);
    app.settle(16).unwrap();
    assert_eq!(count(), 0);
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn typed_callbacks_precede_generic_observers_and_retirement_skips_later_work() {
    let app = App::memory();
    let date = d("2024-02-29");
    let range = HeatmapRange::new(date, date).unwrap();
    let trace = Rc::new(RefCell::new(Vec::new()));
    let typed = trace.clone();
    let generic = trace.clone();
    app.mount(
        heatmap("callbacks")
            .range(range)
            .on_day_click(move |_| typed.borrow_mut().push("typed"))
            .on(EventKind::Activate, move |_| {
                generic.borrow_mut().push("generic")
            }),
    )
    .unwrap();
    app.dispatch(day(&app, date), Event::Activate).unwrap();
    assert_eq!(*trace.borrow(), vec!["typed", "generic"]);
    app.clear().unwrap();
    trace.borrow_mut().clear();
    let owner = app.scope().child().unwrap();
    let retire = owner.downgrade();
    let generic = trace.clone();
    heatmap("retirement")
        .range(range)
        .on_day_click(move |_| {
            if let Some(owner) = retire.upgrade() {
                owner.dispose();
            }
        })
        .on(EventKind::Activate, move |_| {
            generic.borrow_mut().push("late")
        })
        .show_in(&owner)
        .unwrap();
    app.flush().unwrap();
    app.dispatch(day(&app, date), Event::Activate).unwrap();
    assert!(trace.borrow().is_empty());
    assert_eq!(app.stats().nodes, 0);
}
