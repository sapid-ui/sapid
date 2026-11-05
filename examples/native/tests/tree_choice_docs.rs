use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Query, QueryValue},
};
use std::{cell::RefCell, rc::Rc};
fn find(app: &App, predicate: impl Fn(glory::render::DescriptorView<'_>) -> bool) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let n = tree.node(id).unwrap();
        if predicate(n.descriptor.view()) {
            return id;
        }
        pending.extend(n.children.iter().copied());
    }
    panic!("node")
}
#[test]
fn search_reveals_the_real_child_row_and_restores_the_controlled_folded_state() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 360, 1.).unwrap();
        let app = host.app();
        let reference = app.scope().cage(None);
        let expanded = app
            .scope()
            .cage(std::collections::BTreeSet::<String>::new());
        host.mount(
            sapid::tree_select("native-search-path")
                .theme(theme)
                .name("Find a folder")
                .node_ref(reference)
                .searchable(true)
                .expanded(expanded)
                .nodes(vec![
                    sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("design", "Design").parent("work"),
                ]),
        )
        .unwrap();
        app.settle(16).unwrap();
        let input = reference.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(input, glory::html::Part::Main)
            .unwrap();
        for query in ["Design", ""] {
            let revision = match &app.tree().node(input).unwrap().descriptor.view().props {
                glory::render::Props::Input(value) => value.revision,
                _ => unreachable!(),
            };
            app.dispatch(
                input,
                glory::render::Event::ValueChanged {
                    value: query.into(),
                    composing: false,
                    revision,
                },
            )
            .unwrap();
            app.settle(16).unwrap();
            let root = find(app, |d| d.name.as_deref() == Some("Work"));
            assert_eq!(
                app.tree()
                    .node(root)
                    .unwrap()
                    .descriptor
                    .view()
                    .semantics
                    .as_ref()
                    .unwrap()
                    .expanded,
                Some(!query.is_empty())
            );
            if !query.is_empty() {
                let child = find(app, |d| d.name.as_deref() == Some("Design"));
                let result = Rc::new(RefCell::new(None));
                let out = result.clone();
                app.request(Query::Bounds(child), move |value| {
                    *out.borrow_mut() = Some(value.unwrap())
                })
                .unwrap();
                app.pump().unwrap();
                let Some(QueryValue::Bounds(rect)) = *result.borrow() else {
                    panic!("visible search result")
                };
                assert!(rect.width > 0. && rect.height > 0.);
            }
            assert!(expanded.get().is_empty());
            assert_eq!(reference.get(), Some(input));
            assert_eq!(
                host.renderer()
                    .engine_id(input, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
        }
    }
}
#[test]
fn hierarchical_clear_restores_the_same_trigger_after_its_action_becomes_hidden() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in 0..3 {
            let host = WindowApp::new();
            host.renderer().set_viewport(360, 260, 1.).unwrap();
            let app = host.app();
            let reference = app.scope().cage(None);
            let nodes = vec![
                sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
                sapid::TreeNode::new("design", "Design").parent("work"),
            ];
            let chosen =
                sapid::TreeChoices::new([
                    sapid::TreePath::new(["work".into(), "design".into()]).unwrap()
                ])
                .unwrap();
            let widget: Box<dyn Widget> = if kind == 2 {
                Box::new(
                    sapid::cascader("native-clear")
                        .theme(theme.clone())
                        .name("Folder")
                        .node_ref(reference)
                        .nodes(nodes)
                        .default_value(chosen)
                        .clearable(true),
                )
            } else {
                Box::new(
                    sapid::tree_select("native-clear")
                        .theme(theme.clone())
                        .name("Folder")
                        .node_ref(reference)
                        .nodes(nodes)
                        .default_value(chosen)
                        .clearable(true)
                        .searchable(kind == 1),
                )
            };
            host.mount(widget).unwrap();
            app.settle(16).unwrap();
            let trigger = reference.get().unwrap();
            let physical = host
                .renderer()
                .engine_id(trigger, glory::html::Part::Main)
                .unwrap();
            let clear = find(app, |d| {
                d.classes.iter().any(|c| c == "sapid-tree-select-clear")
            });
            app.scope()
                .action(glory::render::Action::Focus(clear))
                .unwrap();
            app.settle(16).unwrap();
            app.dispatch(clear, glory::render::Event::Activate).unwrap();
            app.settle(16).unwrap();
            let saved = Rc::new(RefCell::new(None));
            let out = saved.clone();
            app.request(Query::Focus(trigger), move |v| {
                *out.borrow_mut() = Some(v.unwrap())
            })
            .unwrap();
            app.pump().unwrap();
            assert_eq!(
                saved.borrow_mut().take(),
                Some(QueryValue::Focus(true)),
                "kind {kind}"
            );
            assert_eq!(
                host.renderer()
                    .engine_id(trigger, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}

#[test]
fn parent_path_selection_and_compact_layout_use_the_real_tree_and_finish_on_a_leaf() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for columns in [false, true] {
            let host = WindowApp::new();
            host.renderer().set_viewport(360, 300, 1.).unwrap();
            let app = host.app();
            let reference = app.scope().cage(None);
            let value = app.scope().cage(sapid::TreeChoices::default());
            let expanded = app
                .scope()
                .cage(std::collections::BTreeSet::from(["work".to_owned()]));
            host.mount(
                sapid::cascader("native-parent")
                    .theme(theme.clone())
                    .name("Category")
                    .node_ref(reference)
                    .nodes(vec![
                        sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
                        sapid::TreeNode::new("design", "Design").parent("work"),
                    ])
                    .with_columns(columns)
                    .change_on_select(true)
                    .value(value)
                    .on_value_change(move |v| value.revise(|old| *old = v))
                    .expanded(expanded)
                    .on_expanded_change(move |v| expanded.revise(|old| *old = v))
                    .default_open(true),
            )
            .unwrap();
            app.settle(16).unwrap();
            let trigger = reference.get().unwrap();
            let physical = host
                .renderer()
                .engine_id(trigger, glory::html::Part::Main)
                .unwrap();
            let branch = find(app, |d| d.name.as_deref() == Some("Work"));
            app.dispatch(branch, glory::render::Event::Activate)
                .unwrap();
            app.settle(16).unwrap();
            assert_eq!(value.with(|v| v.paths()[0].leaf().to_owned()), "work");
            assert_eq!(
                app.tree()
                    .node(trigger)
                    .unwrap()
                    .descriptor
                    .view()
                    .semantics
                    .as_ref()
                    .unwrap()
                    .expanded,
                Some(true)
            );
            expanded.revise(|v| *v = std::collections::BTreeSet::from(["work".to_owned()]));
            app.settle(16).unwrap();
            let leaf = find(app, |d| d.name.as_deref() == Some("Design"));
            app.dispatch(leaf, glory::render::Event::Activate).unwrap();
            app.settle(16).unwrap();
            assert_eq!(
                value.with(|v| v.paths()[0].keys().to_vec()),
                vec!["work".to_owned(), "design".to_owned()]
            );
            assert_eq!(
                app.tree()
                    .node(trigger)
                    .unwrap()
                    .descriptor
                    .view()
                    .semantics
                    .as_ref()
                    .unwrap()
                    .expanded,
                Some(false)
            );
            assert_eq!(
                host.renderer()
                    .engine_id(trigger, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}
