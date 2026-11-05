use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{
    Action, Direction, DismissReason, InputProps, NodeId, Props, Query, QueryValue, Rect,
};
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
fn t(v: &str) -> LocalTime {
    LocalTime::parse_iso(v).unwrap()
}
fn query(app: &App, q: Query) -> QueryValue {
    let reply = Rc::new(RefCell::new(None));
    let out = reply.clone();
    app.request(q, move |v| *out.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    reply.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    rect
}

#[test]
fn time_grid_fits_narrow_containers_and_keeps_dense_columns_in_a_local_viewport() {
    for (cycle, columns, direction) in [
        (HourCycle::H24, 4, Direction::Ltr),
        (HourCycle::H12, 4, Direction::Ltr),
        (HourCycle::H12, 12, Direction::Ltr),
        (HourCycle::H24, 4, Direction::Rtl),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(300, 400, 1.).unwrap();
        let app = host.app();
        let root = app.scope().cage(None);
        host.mount(
            time_grid("narrow")
                .node_ref(root)
                .direction(direction)
                .spec(TimeGridSpec::new(t("09:00"), t("15:30"), 1800, columns).unwrap())
                .format(TimeFormat::new(
                    cycle,
                    TimePrecision::Minute,
                    DateLocale::En,
                )),
        )
        .unwrap();
        app.settle(16).unwrap();
        let root = root.get().unwrap();
        let rect = bounds(app, root);
        assert!(rect.width <= 300.1, "viewport widened: {rect:?}");
        let cells = nodes(app, "sapid-time-cell");
        assert!(!cells.is_empty());
        for cell in cells {
            assert!(bounds(app, cell).width >= 51.9);
        }
        let row = app.tree().node(root).unwrap().children[0];
        let cells = app.tree().node(row).unwrap().children.clone();
        let first = bounds(app, cells[0]);
        let last = bounds(app, *cells.last().unwrap());
        assert!(
            (first.y - last.y).abs() < 0.2,
            "columns must share their physical row"
        );
        assert_eq!(first.x > last.x, direction == Direction::Rtl);
        let QueryValue::ScrollMetrics(metrics) = query(app, Query::ScrollMetrics(root)) else {
            panic!("metrics")
        };
        if columns == 12 {
            assert!(metrics.content_width > metrics.viewport_width + 300.);
            let from = button(app, "09:00 AM");
            app.scope().action(Action::Focus(from)).unwrap();
            app.settle(16).unwrap();
            app.dispatch(
                root,
                Event::GridNavigate {
                    from,
                    command: GridCommand::LastInRow,
                },
            )
            .unwrap();
            app.settle(16).unwrap();
            let QueryValue::ScrollMetrics(revealed) = query(app, Query::ScrollMetrics(root)) else {
                panic!("metrics")
            };
            assert!(
                revealed.x > 300.,
                "keyboard target must be revealed: {revealed:?}"
            );
        } else {
            assert!(metrics.content_width <= metrics.viewport_width + 1.);
        }
    }
}
fn props(app: &App, id: NodeId) -> InputProps {
    let tree = app.tree();
    let Props::Input(p) = tree.node(id).unwrap().descriptor.view().props else {
        panic!()
    };
    p.clone()
}
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut result = vec![];
    while let Some(id) = todo.pop() {
        todo.extend(tree.node(id).unwrap().children.iter().copied());
        result.push(id)
    }
    result
}
fn button(app: &App, name: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let view = tree.node(*id).unwrap().descriptor.view();
            matches!(view.props,Props::Button(p)if p.label==name)
                || view.name.as_deref() == Some(name)
        })
        .unwrap()
}
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    all(app)
        .into_iter()
        .filter(|id| {
            app.tree().node(*id).is_ok_and(|node| {
                node.descriptor
                    .view()
                    .classes
                    .iter()
                    .any(|item| item == class)
            })
        })
        .collect()
}
#[test]
fn native_time_segments_four_themes_have_real_spin_editors_geometry_focus_and_shared_grid_value() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(800, 650, 1.).unwrap();
        let app = host.app();
        let h = app.scope().cage(None);
        let m = app.scope().cage(None);
        let s = app.scope().cage(None);
        let value = app.scope().cage(Some(t("09:30:00")));
        host.mount(
            time_picker("native-segments")
                .theme(theme.clone())
                .format(TimeFormat::new(
                    HourCycle::H24,
                    TimePrecision::Second,
                    DateLocale::En,
                ))
                .hour_ref(h)
                .minute_ref(m)
                .second_ref(s)
                .value(value)
                .clearable(true)
                .grid(TimeGridSpec::new(t("09:00"), t("10:30"), 1800, 2).unwrap())
                .on_value_change(move |v| value.revise(|old| *old = v)),
        )
        .unwrap();
        app.settle(16).unwrap();
        let h = h.get().unwrap();
        let m = m.get().unwrap();
        let s = s.get().unwrap();
        let hr = bounds(app, h);
        let mr = bounds(app, m);
        let sr = bounds(app, s);
        assert!(hr.width >= 71. && hr.height >= 35.);
        assert!(mr.x >= hr.x + hr.width && sr.x >= mr.x + mr.width);
        let physical = host
            .renderer()
            .engine_id(h, glory::html::Part::Main)
            .unwrap();
        app.scope().action(Action::Focus(h)).unwrap();
        app.settle(16).unwrap();
        assert_eq!(query(app, Query::Focus(h)), QueryValue::Focus(true));
        app.dispatch(
            h,
            Event::Spin {
                command: glory::render::RangeCommand::Increase,
                from: h,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(value.get(), Some(t("10:30:00")));
        assert_eq!(props(app, h).value, "10");
        let slot = button(app, "09:00:00");
        let rect = bounds(app, slot);
        assert!(rect.width >= 89. && rect.height >= 35.);
        let hit = host.renderer().inspect(|dom| {
            dom.hit(
                (rect.x + rect.width / 2.) as f32,
                (rect.y + rect.height / 2.) as f32,
            )
            .unwrap()
            .node_id
        });
        assert_eq!(
            hit,
            host.renderer()
                .engine_id(slot, glory::html::Part::Main)
                .unwrap()
        );
        app.dispatch(slot, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(value.get(), Some(t("09:00:00")));
        assert_eq!(props(app, m).value, "00");
        assert_eq!(props(app, s).value, "00");
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let frame = raster.frame().unwrap();
        let at = ((rect.y as usize + 5) * frame.width as usize + rect.x as usize + 5) * 4;
        for (a, b) in raster.pixels()[at..at + 4].iter().zip([
            theme.accent.0,
            theme.accent.1,
            theme.accent.2,
            theme.accent.3,
        ]) {
            assert!(a.abs_diff(b) <= 2);
        }
        let clear = button(app, "Clear time");
        let clear_rect = bounds(app, clear);
        assert!(clear_rect.width >= 20. && clear_rect.height >= 20.);
        let clear_hit = host.renderer().inspect(|dom| {
            dom.hit(
                (clear_rect.x + clear_rect.width / 2.) as f32,
                (clear_rect.y + clear_rect.height / 2.) as f32,
            )
            .unwrap()
            .node_id
        });
        assert_eq!(
            clear_hit,
            host.renderer()
                .engine_id(clear, glory::html::Part::Main)
                .unwrap()
        );
        app.dispatch(clear, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(value.get(), None);
        assert!(!all(app).contains(&clear));
        assert_eq!(props(app, h).value, "");
        host.renderer().inspect(|dom| {
            let input = dom
                .get_node(physical)
                .unwrap()
                .element_data()
                .unwrap()
                .text_input_data()
                .unwrap();
            assert!(input.placeholder_shown());
            assert!(input.placeholder_layout().is_some());
            assert!(input.editor.raw_text().is_empty());
        });
        assert_eq!(
            host.renderer()
                .engine_id(h, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        app.clear().unwrap();
        assert_eq!(app.stats().queries, 0);
    }
}
#[test]
fn native_time_grid_queued_navigation_skips_disabled_and_readonly_keeps_focus_without_value_change()
{
    let host = WindowApp::new();
    host.renderer().set_viewport(800, 500, 1.).unwrap();
    let app = host.app();
    let root = app.scope().cage(None);
    let ro = app.scope().cage(false);
    let value = app.scope().cage(Some(t("09:00")));
    host.mount(
        time_grid("native-grid")
            .spec(TimeGridSpec::new(t("09:00"), t("11:30"), 1800, 3).unwrap())
            .node_ref(root)
            .value(value)
            .readonly(ro)
            .disable_time(t("09:30"))
            .on_value_change(move |v| value.revise(|old| *old = v)),
    )
    .unwrap();
    app.settle(16).unwrap();
    let from = button(app, "09:00");
    let target = button(app, "10:00");
    app.scope().action(Action::Focus(from)).unwrap();
    app.settle(16).unwrap();
    app.dispatch(
        root.get().unwrap(),
        Event::GridNavigate {
            from,
            command: GridCommand::Right,
        },
    )
    .unwrap();
    app.settle(16).unwrap();
    assert_eq!(query(app, Query::Focus(target)), QueryValue::Focus(true));
    assert_eq!(value.get(), Some(t("09:00")));
    ro.revise(|v| *v = true);
    app.settle(16).unwrap();
    app.dispatch(target, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get(), Some(t("09:00")));
    ro.revise(|v| *v = false);
    app.settle(16).unwrap();
    app.dispatch(target, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get(), Some(t("10:00")));
    app.clear().unwrap();
    assert!(root.get().is_none());
    assert_eq!(app.stats().queries, 0);
}

#[test]
fn native_time_picker_presets_use_a_real_anchored_layer_and_keep_editor_identity() {
    let host = WindowApp::new();
    host.renderer().set_viewport(800, 600, 1.).unwrap();
    let app = host.app();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(t("09:00")));
    let open = app.scope().cage(false);
    host.mount(
        time_picker("native-presets")
            .hour_ref(hour)
            .value(value)
            .open(open)
            .presets(vec![t("09:00"), t("12:30"), t("17:45")])
            .close_on_preset_select(true)
            .on_value_change(move |next| value.revise(|old| *old = next))
            .on_open_change(move |next| open.revise(|old| *old = next)),
    )
    .unwrap();
    app.settle(16).unwrap();
    let hour = hour.get().unwrap();
    let physical = host
        .renderer()
        .engine_id(hour, glory::html::Part::Main)
        .unwrap();
    app.dispatch(hour, Event::FocusChanged(true)).unwrap();
    app.settle(16).unwrap();
    assert!(open.get());
    let panel = nodes(app, "sapid-popover")[0];
    let panel_rect = bounds(app, panel);
    let trigger_rect = bounds(app, nodes(app, "sapid-time-picker-preset-trigger")[0]);
    assert!(panel_rect.width > 100. && panel_rect.height > 35.);
    assert!(panel_rect.y >= trigger_rect.y + trigger_rect.height - 1.);
    let preset = button(app, "12:30");
    let preset_rect = bounds(app, preset);
    let hit = host.renderer().inspect(|dom| {
        dom.hit(
            (preset_rect.x + preset_rect.width / 2.) as f32,
            (preset_rect.y + preset_rect.height / 2.) as f32,
        )
        .unwrap()
        .node_id
    });
    assert_eq!(
        hit,
        host.renderer()
            .engine_id(preset, glory::html::Part::Main)
            .unwrap()
    );
    app.dispatch(preset, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(value.get(), Some(t("12:30")));
    assert!(!open.get());
    assert_eq!(
        host.renderer()
            .engine_id(hour, glory::html::Part::Main)
            .unwrap(),
        physical
    );
    app.dispatch(hour, Event::FocusChanged(true)).unwrap();
    app.settle(16).unwrap();
    app.dispatch(panel, Event::Dismiss(DismissReason::Escape))
        .unwrap();
    app.settle(16).unwrap();
    assert!(!open.get());
}

#[test]
fn native_duration_picker_expands_hours_beyond_a_day_and_keeps_real_editors() {
    let host = WindowApp::new();
    host.renderer().set_viewport(800, 420, 1.).unwrap();
    let app = host.app();
    let hour = app.scope().cage(None);
    let minute = app.scope().cage(None);
    let value = app
        .scope()
        .cage(Some(TimeDuration::new(123, 45, 0).unwrap()));
    host.mount(
        time_picker("native-duration")
            .duration()
            .min_hours_digits(4)
            .hour_ref(hour)
            .minute_ref(minute)
            .value(value)
            .on_value_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    app.settle(16).unwrap();
    let hour = hour.get().unwrap();
    let minute = minute.get().unwrap();
    let initial_rect = bounds(app, hour);
    assert!(initial_rect.width >= 67. && bounds(app, minute).x > initial_rect.x);
    let physical = host
        .renderer()
        .engine_id(hour, glory::html::Part::Main)
        .unwrap();
    app.dispatch(
        hour,
        Event::ValueChanged {
            value: "123456".into(),
            revision: props(app, hour).revision,
            composing: false,
        },
    )
    .unwrap();
    app.settle(16).unwrap();
    assert_eq!(
        value.get(),
        Some(TimeDuration::new(123_456, 45, 0).unwrap())
    );
    assert!(bounds(app, hour).width > initial_rect.width);
    assert_eq!(
        host.renderer()
            .engine_id(hour, glory::html::Part::Main)
            .unwrap(),
        physical
    );
}
