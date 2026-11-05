use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect, SemanticId},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/showcase_data_input_docs/native_groups.rs"]
mod groups;
#[path = "../../web/src/showcase_data_input_docs/mask_values.rs"]
mod masked;
#[path = "../../web/src/showcase_data_input_docs/native_modes.rs"]
mod modes;
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
fn selection(app: &App, id: NodeId, keys: &[&str]) -> glory::ui::Result<bool> {
    let revision = match app.tree().node(id).unwrap().descriptor.view().props {
        Props::Select(p) => p.selection.revision,
        _ => panic!("select"),
    };
    app.dispatch(
        id,
        Event::SelectChanged {
            keys: keys.iter().map(|k| SemanticId::new(*k).unwrap()).collect(),
            revision,
        },
    )
}
#[test]
fn native_grouped_selection_updates_the_real_caption_and_refuses_disabled_keys() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(groups::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let id = nodes(app, |v| matches!(v.props, Props::Select(_)))[0];
        let engine = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        for (key, label) in [("fr", "France"), ("jp", "Japan")] {
            selection(app, id, &[key]).unwrap();
            app.settle(16).unwrap();
            host.renderer().inspect(|doc| {
                let caption = doc.get_node(engine).unwrap().children[0];
                let text = doc.get_node(caption).unwrap().children[0];
                assert_eq!(
                    doc.get_node(text).unwrap().text_data().unwrap().content,
                    label
                );
            });
        }
        assert!(selection(app, id, &["de"]).is_err());
        assert!(selection(app, id, &["xx"]).is_err());
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            engine
        );
        app.clear().unwrap();
    }
}
#[test]
fn native_multiple_rows_update_selection_without_replacing_the_native_control() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(modes::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = nodes(app, |v| matches!(v.props, Props::Select(_)));
        assert_eq!(ids.len(), 2);
        assert!(bounds(app, ids[1]).height > bounds(app, ids[0]).height + 20.);
        let engine = host
            .renderer()
            .engine_id(ids[1], glory::html::Part::Main)
            .unwrap();
        let writes = host.renderer().stats().property_writes;
        selection(app, ids[1], &["js", "py"]).unwrap();
        app.settle(16).unwrap();
        assert!(host.renderer().stats().property_writes > writes);
        assert!(
            matches!(app.tree().node(ids[1]).unwrap().descriptor.view().props,Props::Select(p)if p.selection.keys.iter().map(SemanticId::as_str).collect::<Vec<_>>()==["js","py"])
        );
        assert_eq!(
            host.renderer()
                .engine_id(ids[1], glory::html::Part::Main)
                .unwrap(),
            engine
        );
        app.clear().unwrap();
    }
}
#[test]
fn native_mask_raw_source_updates_keep_real_editor_and_placeholder_layout() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(masked::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let id = nodes(app, |v| matches!(v.props, Props::Input(_)))[0];
        let engine = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        for (label, expected) in [
            ("Set reference from application", "98-76"),
            ("Clear reference", ""),
        ] {
            let button = nodes(app, |v| matches!(v.props,Props::Button(p)if p.label==label))[0];
            app.dispatch(button, Event::Activate).unwrap();
            app.settle(16).unwrap();
            host.renderer().inspect(|doc| {
                let input = doc
                    .get_node(engine)
                    .unwrap()
                    .element_data()
                    .unwrap()
                    .text_input_data()
                    .unwrap();
                assert_eq!(input.editor.raw_text(), expected);
                if expected.is_empty() {
                    assert!(input.placeholder_shown());
                    assert!(input.placeholder_layout().is_some());
                }
            });
        }
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            engine
        );
        app.clear().unwrap();
    }
}
