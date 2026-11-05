use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/showcase_data_input_docs/number_clamp.rs"]
mod clamp;
#[path = "../../web/src/showcase_data_input_docs/json_blur.rs"]
mod json_blur;
#[path = "../../web/src/showcase_data_input_docs/json_errors.rs"]
mod json_errors;
#[path = "../../web/src/showcase_data_input_docs/number_sizes.rs"]
mod sizes;
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
fn text(app: &App, id: NodeId) -> String {
    match app.tree().node(id).unwrap().descriptor.view().props {
        Props::Input(p) => p.value.clone(),
        _ => panic!("input"),
    }
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
#[test]
fn native_json_blur_formats_the_same_editor_and_keeps_explicit_formatting_separate() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(json_blur::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = nodes(app, |v| matches!(v.props, Props::Input(_)));
        assert_eq!(ids.len(), 2);
        let identities = ids
            .iter()
            .map(|id| {
                host.renderer()
                    .engine_id(*id, glory::html::Part::Main)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        for id in &ids {
            edit(app, *id, "{\"z\":2}");
            app.dispatch(*id, Event::FocusChanged(false)).unwrap();
            app.settle(16).unwrap();
        }
        assert_eq!(text(app, ids[0]), "{\n  \"z\": 2\n}");
        assert_eq!(text(app, ids[1]), "{\"z\":2}");
        for (i, id) in ids.into_iter().enumerate() {
            assert_eq!(
                host.renderer()
                    .engine_id(id, glory::html::Part::Main)
                    .unwrap(),
                identities[i]
            );
        }
        app.clear().unwrap();
    }
}
#[test]
fn native_number_clamp_docs_keep_real_inputs_and_apply_each_policy() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(clamp::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = nodes(app, |v| matches!(v.props, Props::Input(_)));
        assert_eq!(ids.len(), 3);
        for (id, expected) in ids.into_iter().zip(["10", "2", "27"]) {
            let identity = host
                .renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap();
            edit(app, id, "27");
            app.dispatch(id, Event::FocusChanged(false)).unwrap();
            app.settle(16).unwrap();
            assert_eq!(text(app, id), expected);
            assert_eq!(
                host.renderer()
                    .engine_id(id, glory::html::Part::Main)
                    .unwrap(),
                identity
            );
        }
        app.clear().unwrap();
    }
}
#[test]
fn native_number_sizes_keep_two_compact_buttons_in_each_row() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(sizes::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let rows = nodes(app, |v| {
            v.classes.iter().any(|c| c == "sapid-number-input-row")
        });
        let buttons = nodes(app, |v| {
            v.classes.iter().any(|c| c == "sapid-number-input-control")
        });
        assert_eq!(rows.len(), 5);
        assert_eq!(buttons.len(), 10);
        let rects = rows.iter().map(|id| bounds(app, *id)).collect::<Vec<_>>();
        assert!(rects.windows(2).all(|rs| rs[1].height > rs[0].height));
        for row in rects {
            assert!(row.width <= 320.);
            assert!(
                buttons.iter().any(|id| {
                    let b = bounds(app, *id);
                    b.y >= row.y
                        && b.y + b.height <= row.y + row.height + 1.
                        && b.x + b.width >= row.x + row.width - 2.
                }),
                "stepper must finish at the row edge: {row:?}"
            );
            assert_eq!(
                buttons
                    .iter()
                    .filter(|id| {
                        let b = bounds(app, **id);
                        b.x >= row.x
                            && b.y >= row.y
                            && b.x + b.width <= row.x + row.width + 1.
                            && b.y + b.height <= row.y + row.height + 1.
                    })
                    .count(),
                2
            );
        }
        app.clear().unwrap();
    }
}
#[test]
fn native_json_error_sources_keep_syntax_positions_and_valid_output_failure_separate() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(json_errors::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = nodes(app, |v| matches!(v.props, Props::Input(_)));
        assert_eq!(ids.len(), 2);
        assert!(
            app.tree()
                .node(ids[0])
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .invalid
        );
        let messages = nodes(
            app,
            |v| matches!(v.props,Props::Text(p)if p.content.contains("line")&&p.content.contains("column")),
        );
        assert!(!messages.is_empty());
        let button = nodes(app, |v| v.classes.iter().any(|c| c == "sapid-button"));
        let budget_button = *button.last().unwrap();
        app.dispatch(budget_button, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(text(app, ids[1]), "[1,2,3]");
        assert!(
            !app.tree()
                .node(ids[1])
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .invalid
        );
        app.clear().unwrap();
    }
}
