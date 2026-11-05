use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{NodeId, Props, Query, QueryValue, Rect};
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
fn d(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn query(app: &App, q: Query) -> QueryValue {
    let r = Rc::new(RefCell::new(None));
    let out = r.clone();
    app.request(q, move |v| *out.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    r.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(r) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    r
}
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        result.push(id);
    }
    result
}
fn submitted(app: &App, name: &str) -> String {
    let controls = all(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .and_then(|value| value.form_control.as_ref())
                .is_some_and(|control| control.name == name)
        })
        .collect::<Vec<_>>();
    assert_eq!(controls.len(), 1);
    let tree = app.tree();
    let Props::Input(input) = tree.node(controls[0]).unwrap().descriptor.view().props else {
        panic!("input")
    };
    input.value.clone()
}
fn class(app: &App, class: &str) -> Vec<NodeId> {
    all(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|value| value == class)
        })
        .collect()
}
fn day(app: &App, date: CivilDate) -> NodeId {
    let name = DateLocale::En.date_label(date);
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let v = tree.node(*id).unwrap().descriptor.view();
            matches!(v.props, Props::Button(_)) && v.name.as_deref() == Some(name.as_str())
        })
        .unwrap()
}

fn button(app: &App, label: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let value = tree.node(*id).unwrap().descriptor.view();
            matches!(value.props, Props::Button(props) if props.label == label)
                || matches!(value.props, Props::Button(_)) && value.name.as_deref() == Some(label)
        })
        .unwrap_or_else(|| panic!("missing button {label}"))
}

#[test]
fn native_week_number_column_is_passive_physical_table_content() {
    let host = WindowApp::new();
    host.renderer().set_viewport(720, 580, 1.).unwrap();
    let app = host.app();
    host.mount(
        date_picker("native-week-numbers", DatesProvider::new(d("2024-02-29")))
            .default_month(YearMonth::new(2024, 2).unwrap())
            .with_week_numbers(true),
    )
    .unwrap();
    app.settle(16).unwrap();

    let weeks = class(app, "sapid-calendar-week-number");
    assert_eq!(weeks.len(), 6);
    let week = weeks
        .iter()
        .copied()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Week 9")
        })
        .unwrap();
    let week_bounds = bounds(app, week);
    let day_bounds = bounds(app, day(app, d("2024-02-29")));
    assert!(week_bounds.width > 20. && week_bounds.height > 30.);
    assert!(week_bounds.x + week_bounds.width < day_bounds.x);
    assert!(matches!(
        app.tree().node(week).unwrap().descriptor.view().props,
        Props::View
    ));
    app.clear().unwrap();
}

#[test]
fn native_multi_column_range_has_consecutive_real_grids_and_shared_selection() {
    let host = WindowApp::new();
    host.renderer().set_viewport(980, 620, 1.).unwrap();
    let app = host.app();
    let value = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Range));
    host.mount(
        date_picker("native-range-columns", DatesProvider::new(d("2024-02-29")))
            .mode(DatePickerMode::Range)
            .value(value)
            .default_month(YearMonth::new(2024, 2).unwrap())
            .number_of_columns(2)
            .on_value_change(move |next| value.revise(|current| *current = next)),
    )
    .unwrap();
    app.settle(16).unwrap();

    let february = day(app, d("2024-02-28"));
    let march = day(app, d("2024-03-02"));
    let left = bounds(app, february);
    let right = bounds(app, march);
    assert!(right.x > left.x + left.width);
    assert!(left.width > 30. && right.width > 30.);

    app.dispatch(february, Event::Activate).unwrap();
    app.dispatch(march, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get().format_iso(), "2024-02-28/2024-03-02");

    let selected = class(app, "sapid-calendar-cell")
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .is_some_and(|semantics| semantics.selected == Some(true))
        })
        .count();
    assert_eq!(
        selected, 4,
        "count day cells independently from retained month/year grids"
    );
    app.clear().unwrap();
}

#[test]
fn native_range_preset_is_a_real_action_and_closes_only_after_controlled_acceptance() {
    let host = WindowApp::new();
    host.renderer().set_viewport(760, 600, 1.).unwrap();
    let app = host.app();
    let input = app.scope().cage(None);
    let value = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Range));
    let preset = DateSelection::Range(DateRangeSelection::Complete(
        CivilRange::new(d("2024-02-26"), d("2024-02-29")).unwrap(),
    ));
    host.mount(
        date_picker_input("native-range-preset", DatesProvider::new(d("2024-02-29")))
            .form_name("preset")
            .mode(DatePickerMode::Range)
            .default_open(true)
            .node_ref(input)
            .value(value)
            .presets([DatePickerPreset::new("Work week", preset)])
            .on_value_change(move |next| value.revise(|current| *current = next)),
    )
    .unwrap();
    app.settle(16).unwrap();
    let action = button(app, "Work week");
    let action_bounds = bounds(app, action);
    assert!(action_bounds.width > 20. && action_bounds.height > 20.);
    app.dispatch(action, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get(), preset);
    assert_eq!(
        host.renderer()
            .inspect(|document| document.top_layers().len()),
        0
    );
    let tree = app.tree();
    let Props::Input(props) = tree
        .node(input.get().unwrap())
        .unwrap()
        .descriptor
        .view()
        .props
    else {
        panic!()
    };
    assert_eq!(props.value, "2024-02-26 – 2024-02-29");
    drop(tree);
    assert_eq!(submitted(app, "preset"), "2024-02-26/2024-02-29");
}

#[test]
fn native_default_picker_dialog_four_themes_has_real_popup_hit_testing_focus_return_and_pixels() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(820, 650, 1.).unwrap();
        let app = host.app();
        let input = app.scope().cage(None);
        let trigger = app.scope().cage(None);
        let panel = app.scope().cage(None);
        let grid = app.scope().cage(None);
        let value = app
            .scope()
            .cage(DateSelection::Single(Some(d("2024-02-28"))));
        host.mount(
            date_picker_input("native-date", DatesProvider::new(d("2024-02-29")))
                .theme(theme.clone())
                .name("Appointment")
                .value(value)
                .node_ref(input)
                .trigger_ref(trigger)
                .panel_ref(panel)
                .calendar_ref(grid)
                .on_value_change(move |v| value.revise(|old| *old = v)),
        )
        .unwrap();
        app.settle(16).unwrap();
        let input = input.get().unwrap();
        let trigger = trigger.get().unwrap();
        let panel = panel.get().unwrap();
        let grid = grid.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(input, glory::html::Part::Main)
            .unwrap();
        app.scope()
            .action(glory::render::Action::Focus(trigger))
            .unwrap();
        app.settle(16).unwrap();
        app.dispatch(trigger, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
        let rect = bounds(app, panel);
        assert!(rect.width >= 300. && rect.height >= 250.);
        assert!(rect.x >= 0. && rect.y >= 0.);
        let selected = day(app, d("2024-02-28"));
        let next = day(app, d("2024-02-29"));
        let cell = bounds(app, selected);
        assert!((cell.width - 36.).abs() < 0.2 && (cell.height - 36.).abs() < 0.2);
        assert!(cell.x >= rect.x && cell.y >= rect.y && cell.x + cell.width <= rect.x + rect.width);
        let hit = host.renderer().inspect(|dom| {
            dom.hit((cell.x + 18.) as f32, (cell.y + 18.) as f32)
                .unwrap()
                .node_id
        });
        assert_eq!(
            hit,
            host.renderer()
                .engine_id(selected, glory::html::Part::Main)
                .unwrap()
        );
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let frame = raster.frame().unwrap();
        let at = ((cell.y as usize + 5) * frame.width as usize + cell.x as usize + 5) * 4;
        for (a, b) in raster.pixels()[at..at + 4].iter().zip([
            theme.accent.0,
            theme.accent.1,
            theme.accent.2,
            theme.accent.3,
        ]) {
            assert!(a.abs_diff(b) <= 2);
        }
        app.scope()
            .action(glory::render::Action::Focus(selected))
            .unwrap();
        app.settle(16).unwrap();
        app.dispatch(
            grid,
            Event::GridNavigate {
                from: selected,
                command: GridCommand::Right,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(query(app, Query::Focus(next)), QueryValue::Focus(true));
        app.dispatch(next, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(value.get(), DateSelection::Single(Some(d("2024-02-29"))));
        assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 0);
        assert_eq!(query(app, Query::Focus(trigger)), QueryValue::Focus(true));
        assert_eq!(
            host.renderer()
                .engine_id(input, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        app.dispatch(trigger, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
        app.dispatch(panel, Event::Dismiss(glory::render::DismissReason::Escape))
            .unwrap();
        app.settle(16).unwrap();
        assert_eq!(query(app, Query::Focus(trigger)), QueryValue::Focus(true));
        app.clear().unwrap();
        app.pump().unwrap();
        assert_eq!(app.stats().queries, 0);
        assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 0);
    }
}
#[test]
fn native_range_projection_readonly_and_disabled_keep_finite_real_selection_and_accepted_state() {
    let host = WindowApp::new();
    host.renderer().set_viewport(720, 540, 1.).unwrap();
    let app = host.app();
    let readonly = app.scope().cage(false);
    let enabled = app.scope().cage(true);
    let value = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Range));
    host.mount(
        date_picker("native-range", DatesProvider::new(d("2024-02-29")))
            .mode(DatePickerMode::Range)
            .value(value)
            .readonly(readonly)
            .enabled(enabled)
            .exclude_date(d("2024-02-28"))
            .on_value_change(move |v| value.revise(|old| *old = v)),
    )
    .unwrap();
    app.settle(16).unwrap();
    app.dispatch(day(app, d("2024-02-26")), Event::Activate)
        .unwrap();
    app.dispatch(day(app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get().format_iso(), "2024-02-26/2024-02-29");
    let cells = class(app, "sapid-calendar-cell")
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .is_some_and(|s| s.selected == Some(true))
        })
        .count();
    assert_eq!(cells, 4);
    readonly.revise(|v| *v = true);
    app.settle(16).unwrap();
    let old = value.get();
    app.dispatch(day(app, d("2024-02-27")), Event::Activate)
        .unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get(), old);
    enabled.revise(|v| *v = false);
    app.settle(16).unwrap();
    assert!(
        app.scope()
            .action(glory::render::Action::Focus(day(app, d("2024-02-27"))))
            .is_err()
    );
    app.clear().unwrap();
}

#[test]
fn native_initially_open_range_input_retains_partial_then_closes_only_the_complete_range() {
    let host = WindowApp::new();
    host.renderer().set_viewport(760, 600, 1.).unwrap();
    let app = host.app();
    let input = app.scope().cage(None);
    let trigger = app.scope().cage(None);
    host.mount(
        date_picker_input("initial-range", DatesProvider::new(d("2024-02-29")))
            .form_name("range")
            .mode(DatePickerMode::Range)
            .default_open(true)
            .node_ref(input)
            .trigger_ref(trigger),
    )
    .unwrap();
    app.settle(16).unwrap();
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
    app.dispatch(day(app, d("2024-02-26")), Event::Activate)
        .unwrap();
    app.settle(16).unwrap();
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
    let t = app.tree();
    let Props::Input(p) = t
        .node(input.get().unwrap())
        .unwrap()
        .descriptor
        .view()
        .props
    else {
        panic!()
    };
    assert_eq!(p.value, "2024-02-26 –");
    drop(t);
    assert_eq!(submitted(app, "range"), "2024-02-26/");
    app.dispatch(day(app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.settle(16).unwrap();
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 0);
    assert_eq!(
        query(app, Query::Focus(trigger.get().unwrap())),
        QueryValue::Focus(true)
    );
    let t = app.tree();
    let Props::Input(p) = t
        .node(input.get().unwrap())
        .unwrap()
        .descriptor
        .view()
        .props
    else {
        panic!()
    };
    assert_eq!(p.value, "2024-02-26 – 2024-02-29");
    drop(t);
    assert_eq!(submitted(app, "range"), "2024-02-26/2024-02-29");
    app.clear().unwrap();
    assert_eq!(app.stats().queries, 0);
}
