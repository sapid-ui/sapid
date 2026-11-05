#[path = "../../web/src/showcase_misc_docs/divider_labels.rs"]
mod divider_labels;
#[path = "../../web/src/showcase_misc_docs/focus_initial.rs"]
mod focus_initial;
#[path = "../../web/src/showcase_misc_docs/hidden_description.rs"]
mod hidden_description;
#[path = "../../web/src/showcase_misc_docs/portal_destinations.rs"]
mod portal_destinations;
#[path = "../../web/src/dedicated_demos/toolbar.rs"]
mod public_toolbar;
#[path = "../../web/src/showcase_misc_docs/scroll_position.rs"]
mod scroll_position;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Action, Event, NodeId, Props, Query, QueryValue, Rect},
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
fn button(app: &App, label: &str) -> NodeId {
    nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label==label)).unwrap()
}
fn query(app: &App, q: Query) -> QueryValue {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(q, move |v| *saved.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    out.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(r) = query(app, Query::Bounds(id)) else {
        panic!("bounds")
    };
    r
}
fn physical(host: &WindowApp, id: NodeId) -> apace_dom::NodeId {
    host.renderer()
        .engine_id(id, glory::html::Part::Main)
        .unwrap()
}
#[test]
fn public_labelled_dividers_keep_labels_between_real_lines_in_four_narrow_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 640, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(divider_labels::demo(theme)),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let labels = classes(app, "sapid-separator-label");
        assert_eq!(labels.len(), 3);
        let lines = classes(app, "sapid-separator-line");
        assert_eq!(lines.len(), 4);
        for id in labels {
            let r = bounds(app, id);
            assert!(
                r.width > 30. && r.height > 10. && r.x >= 0. && r.x + r.width <= 300.5,
                "{r:?}"
            );
        }
        for id in lines {
            let r = bounds(app, id);
            assert!(r.width >= 12. && r.height > 0., "{r:?}");
        }
        let mut raster = glory::apace::Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert!(raster.pixels().as_chunks::<4>().0.iter().any(|p| p[3] != 0));
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn actual_portal_example_moves_one_physical_editor_between_public_destinations() {
    let host = WindowApp::new();
    host.renderer().set_viewport(650, 800, 1.).unwrap();
    host.mount(portal_destinations::demo(sapid::Theme::blue_light()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let editor = nodes(app)
        .into_iter()
        .find(|id| {
            matches!(
                app.tree().node(*id).unwrap().descriptor.view().props,
                Props::Input(_)
            )
        })
        .unwrap();
    let initial = physical(&host, editor);
    let first = bounds(app, editor);
    app.dispatch(button(app, "Move project note"), Event::Activate)
        .unwrap();
    observe(&host);
    let second = bounds(app, editor);
    assert!(
        second.x > first.x + 100. || second.y > first.y + 30.,
        "{first:?} -> {second:?}"
    );
    assert_eq!(physical(&host, editor), initial);
    app.dispatch(button(app, "Move project note"), Event::Activate)
        .unwrap();
    observe(&host);
    assert_eq!(physical(&host, editor), initial);
    assert!((bounds(app, editor).x - first.x).abs() < 1.);
    app.clear().unwrap();
    assert_eq!(app.stats().handlers, 0);
}
#[test]
fn shipping_form_focuses_requested_address_and_returns_to_its_live_trigger() {
    let host = WindowApp::new();
    host.renderer().set_viewport(450, 800, 1.).unwrap();
    host.mount(focus_initial::demo(sapid::Theme::green_dark()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let entry = button(app, "Edit shipping details");
    app.scope().action(Action::Focus(entry)).unwrap();
    app.flush().unwrap();
    app.dispatch(entry, Event::Activate).unwrap();
    observe(&host);
    let inputs: Vec<_> = nodes(app)
        .into_iter()
        .filter(|id| {
            matches!(
                app.tree().node(*id).unwrap().descriptor.view().props,
                Props::Input(_)
            )
        })
        .collect();
    assert_eq!(inputs.len(), 2);
    assert_eq!(
        host.renderer().inspect(|doc| doc.get_focussed_node_id()),
        Some(physical(&host, inputs[1]))
    );
    app.dispatch(button(app, "Done editing shipping"), Event::Activate)
        .unwrap();
    observe(&host);
    assert_eq!(
        host.renderer().inspect(|doc| doc.get_focussed_node_id()),
        Some(physical(&host, entry))
    );
    app.clear().unwrap();
}
#[test]
fn public_position_buttons_scroll_the_real_viewport_without_replacing_its_children() {
    let host = WindowApp::new();
    host.renderer().set_viewport(340, 800, 1.).unwrap();
    host.mount(scroll_position::demo(sapid::Theme::blue_light()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let viewport = classes(app, "sapid-scroll-area")[0];
    let text = classes(app, "sapid-text")[0];
    let before = physical(&host, text);
    app.dispatch(button(app, "Jump to latest notes"), Event::Activate)
        .unwrap();
    observe(&host);
    let QueryValue::ScrollMetrics(bottom) = query(app, Query::ScrollMetrics(viewport)) else {
        panic!()
    };
    assert!(bottom.y > 200., "{bottom:?}");
    app.dispatch(button(app, "Read notes from the start"), Event::Activate)
        .unwrap();
    observe(&host);
    let QueryValue::ScrollMetrics(top) = query(app, Query::ScrollMetrics(viewport)) else {
        panic!()
    };
    assert!(top.y < 1.);
    assert_eq!(physical(&host, text), before);
    app.clear().unwrap();
    assert_eq!(app.runtime_stats().tasks, 0);
}
#[test]
fn hidden_supplement_is_an_explicit_description_with_zero_native_layout_extent() {
    let host = WindowApp::new();
    host.mount(hidden_description::demo(sapid::Theme::blue_dark()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let hidden = classes(app, "sapid-visually-hidden")[0];
    let b = button(app, "Share project");
    let tree = app.tree();
    let h = tree.node(hidden).unwrap().descriptor.view();
    let d = tree.node(b).unwrap().descriptor.view();
    assert_eq!(
        d.semantics.as_ref().unwrap().described_by,
        h.semantics.as_ref().unwrap().id
    );
    drop(tree);
    let r = bounds(app, hidden);
    assert_eq!((r.width, r.height), (0., 0.));
    app.dispatch(b, Event::Activate).unwrap();
    observe(&host);
    app.clear().unwrap();
    assert!(app.tree().is_empty());
}

#[test]
fn public_toolbar_icon_actions_have_names_and_square_native_hit_boxes() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 500, 1.).unwrap();
    host.mount(public_toolbar::demo(sapid::Theme::green_light()))
        .unwrap();
    observe(&host);
    let app = host.app();
    for name in ["Undo", "Redo"] {
        let id = nodes(app)
            .into_iter()
            .find(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some(name)
            })
            .unwrap();
        let r = bounds(app, id);
        assert!(
            (r.width - 30.).abs() < 0.5 && (r.height - 30.).abs() < 0.5,
            "{r:?}"
        );
        assert!(
            matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Button(p) if p.label.is_empty())
        );
        let before = physical(&host, id);
        app.dispatch(id, Event::Activate).unwrap();
        observe(&host);
        assert_eq!(physical(&host, id), before);
    }
    app.clear().unwrap();
    assert_eq!(app.stats().handlers, 0);
}
