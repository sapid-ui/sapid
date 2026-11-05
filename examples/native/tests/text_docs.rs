use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/showcase_text_docs/controlled.rs"]
mod controlled;
#[path = "../../web/src/showcase_text_docs/textarea_fixed.rs"]
mod fixed;
#[path = "../../web/src/showcase_text_docs/sections.rs"]
mod sections;
fn themes() -> [sapid::Theme; 4] {
    [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ]
}
fn nodes(app: &App, predicate: impl Fn(glory::render::DescriptorView<'_>) -> bool) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().iter().rev().copied().collect::<Vec<_>>();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        let n = tree.node(id).unwrap();
        if predicate(n.descriptor.view()) {
            result.push(id)
        }
        pending.extend(n.children.iter().rev().copied())
    }
    result
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    app.request(Query::Bounds(id), move |v| {
        *out.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = saved.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
fn edit(app: &App, id: NodeId, text: &str) {
    let revision = match app.tree().node(id).unwrap().descriptor.view().props {
        Props::Input(p) => p.revision,
        _ => panic!("input"),
    };
    app.dispatch(
        id,
        Event::ValueChanged {
            value: text.into(),
            revision,
            composing: false,
        },
    )
    .unwrap();
    app.settle(16).unwrap();
}
fn text(app: &App, id: NodeId) -> String {
    match app.tree().node(id).unwrap().descriptor.view().props {
        Props::Input(p) => p.value.clone(),
        _ => panic!("input"),
    }
}
#[test]
fn fixed_row_examples_use_actual_native_height_and_preserve_editors_for_long_content() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(fixed::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = nodes(app, |v| matches!(v.props, Props::Input(_)));
        assert_eq!(ids.len(), 2);
        let before = bounds(app, ids[0]);
        let larger = bounds(app, ids[1]);
        assert!(larger.height > before.height + 10.);
        assert!(before.width <= 320.);
        let identity = host
            .renderer()
            .engine_id(ids[0], glory::html::Part::Main)
            .unwrap();
        edit(app, ids[0], &"Long content line\n".repeat(20));
        assert_eq!(bounds(app, ids[0]).height, before.height);
        assert_eq!(
            host.renderer()
                .engine_id(ids[0], glory::html::Part::Main)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn native_controlled_text_refusal_and_availability_keep_the_same_input() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(controlled::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let id = nodes(app, |v| matches!(v.props, Props::Input(_)))[0];
        let identity = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        edit(app, id, "Refused");
        assert_eq!(text(app, id), "Ada");
        let accept = nodes(
            app,
            |v| matches!(v.props,Props::Check(p)if p.label=="Accept text changes"),
        )[0];
        app.dispatch(accept, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        edit(app, id, "Accepted");
        assert_eq!(text(app, id), "Accepted");
        let readonly = nodes(
            app,
            |v| matches!(v.props,Props::Check(p)if p.label=="Make text read-only"),
        )[0];
        app.dispatch(readonly, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        edit(app, id, "Rejected");
        assert_eq!(text(app, id), "Accepted");
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
    }
}
#[test]
fn native_inset_sections_reserve_padding_and_clear_the_same_editor() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(sections::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let id = nodes(
            app,
            |v| matches!(v.props,Props::Input(p)if p.value=="design"),
        )[0];
        let identity = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        let clear = nodes(app, |v| v.name.as_deref() == Some("Clear workspace URL"))[0];
        let button = bounds(app, clear);
        let input = bounds(app, id);
        assert!(button.width <= 25. && button.height <= 25., "{button:?}");
        assert!(button.x >= input.x && button.x + button.width <= input.x + input.width + 1.);
        app.dispatch(clear, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(text(app, id), "");
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
    }
}
#[test]
fn native_autosize_remains_an_explicit_rejection() {
    let host = WindowApp::new();
    assert!(
        host.mount(sapid::textarea().autosize(true).min_rows(2).max_rows(5))
            .is_err()
    );
    assert!(host.app().tree().is_empty());
}
