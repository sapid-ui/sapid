use glory::{
    prelude::*,
    render::{ComboboxKind, Event, NodeId, PopupKind, Props, Role, TreeCheckState},
};
use sapid::{
    TreeChoiceModel, TreeChoicePolicy, TreeChoices, TreeKind, TreeModel, TreeNode, tree_select,
};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
fn nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::new("root", "Root").kind(TreeKind::Branch),
        TreeNode::new("a", "Alpha").parent("root"),
        TreeNode::new("b", "Beta").parent("root"),
        TreeNode::new("off", "Disabled")
            .parent("root")
            .disabled(true),
        TreeNode::new("lazy", "Remote").kind(TreeKind::Lazy),
    ]
}
#[test]
fn search_reveals_matching_ancestors_without_writing_controlled_expansion() {
    let app = App::memory();
    let expanded = app.scope().cage(BTreeSet::<String>::new());
    let calls = Rc::new(RefCell::new(Vec::new()));
    let output = calls.clone();
    app.mount(
        tree_select("search-expansion")
            .nodes(nodes())
            .name("Find a file")
            .searchable(true)
            .expanded(expanded)
            .on_expanded_change(move |proposal| output.borrow_mut().push(proposal)),
    )
    .unwrap();
    let input = named(&app, "Find a file");
    for (query, open) in [("Beta", true), ("", false), ("Alpha", true), ("", false)] {
        let revision = match &app.tree().node(input).unwrap().descriptor.view().props {
            Props::Input(value) => value.revision,
            _ => unreachable!(),
        };
        app.dispatch(
            input,
            Event::ValueChanged {
                value: query.into(),
                composing: false,
                revision,
            },
        )
        .unwrap();
        app.flush().unwrap();
        let root = named(&app, "Root");
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
            Some(open)
        );
        assert!(expanded.get().is_empty());
        assert!(calls.borrow().is_empty());
        assert_eq!(named(&app, "Find a file"), input);
    }
}
#[test]
fn focus_loss_then_gain_between_items_in_one_tick_does_not_close_the_popup() {
    let app = App::memory();
    app.mount(
        tree_select("focus-tick")
            .nodes(nodes())
            .name("Choose files")
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()])),
    )
    .unwrap();
    let a = named(&app, "Alpha");
    let b = named(&app, "Beta");
    let trigger = named(&app, "Choose files");
    app.dispatch(a, Event::FocusChanged(true)).unwrap();
    app.scope()
        .batch(|| {
            app.dispatch(a, Event::FocusChanged(false)).unwrap();
            app.dispatch(b, Event::FocusChanged(true)).unwrap();
        })
        .unwrap();
    app.flush().unwrap();
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
    app.dispatch(b, Event::FocusChanged(false)).unwrap();
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
}
fn all(app: &App) -> Vec<NodeId> {
    let t = app.tree();
    let mut pending = t.roots().to_vec();
    let mut out = vec![];
    while let Some(id) = pending.pop() {
        pending.extend(&t.node(id).unwrap().children);
        out.push(id)
    }
    out
}
fn named(app: &App, name: &str) -> NodeId {
    all(app)
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
        .unwrap_or_else(|| panic!("missing {name}"))
}
fn has_name(app: &App, name: &str) -> bool {
    all(app).into_iter().any(|id| {
        app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref()
            == Some(name)
    })
}
fn model() -> TreeChoiceModel {
    TreeChoiceModel::new(
        TreeModel::new(nodes()).unwrap(),
        TreeChoicePolicy::Leaves,
        false,
    )
    .unwrap()
}
#[test]
fn four_theme_ssr_is_an_intrinsic_combobox_with_tree_popup_and_full_path_successful_values() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let value = TreeChoices::new([model().path("a").unwrap()]).unwrap();
        let html = glory::ssr::snapshot(
            tree_select("ssr-tree-choice")
                .name("Choose files")
                .theme(theme)
                .nodes(nodes())
                .default_value(value)
                .default_expanded(BTreeSet::from(["root".into()]))
                .form_name("files"),
        )
        .unwrap()
        .html;
        assert!(
            html.contains("role=\"combobox\"")
                && html.contains("aria-haspopup=\"tree\"")
                && html.contains("role=\"tree\"")
        );
        assert!(
            html.contains("role=\"treeitem\"")
                && !html.contains("role=\"option\"")
                && !html.contains("aria-activedescendant")
        );
        assert!(html.contains("name=\"files\"") && html.contains("value=\"root/a\""));
    }
}
#[test]
fn real_item_activation_proposes_full_paths_and_same_tick_multiple_intents_derive_parent_half_checked()
 {
    let app = App::memory();
    let values = Rc::new(RefCell::new(Vec::new()));
    let output = values.clone();
    app.mount(
        tree_select("checks")
            .name("Choose files")
            .nodes(nodes())
            .multiple(true)
            .policy(TreeChoicePolicy::DescendantLeaves)
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .on_value_change(move |v| output.borrow_mut().push(v)),
    )
    .unwrap();
    app.scope()
        .batch(|| {
            app.dispatch(named(&app, "Alpha"), Event::Activate).unwrap();
            app.dispatch(named(&app, "Beta"), Event::Activate).unwrap();
        })
        .unwrap();
    app.flush().unwrap();
    assert_eq!(values.borrow().len(), 2);
    assert_eq!(values.borrow()[1].paths().len(), 2);
    let root = named(&app, "Root");
    assert_eq!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .tree_checked,
        Some(TreeCheckState::Checked)
    );
    assert!(all(&app).into_iter().any(|id| {
        let tree = app.tree();
        let view = tree.node(id).unwrap().descriptor.view();
        view.classes
            .contains(&"sapid-tree-check-indicator".to_owned())
            && view.style.width == Some(glory::style::Length::Px(18.))
            && view.style.height == Some(glory::style::Length::Px(18.))
            && view.style.background == Some(sapid::Theme::blue_light().accent)
            && view.style.pointer_events == Some(glory::style::PointerEvents::None)
    }));
    app.dispatch(named(&app, "Beta"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .tree_checked,
        Some(TreeCheckState::Mixed)
    );
    app.dispatch(root, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(values.borrow().last().unwrap().paths().len(), 2);
}
#[test]
fn controlled_refusal_keeps_accepted_selection_and_open_while_emitting_proposals() {
    let app = App::memory();
    let out = Rc::new(RefCell::new(Vec::new()));
    let result = out.clone();
    app.mount(widget(move |scope| {
        let value = scope.cage(TreeChoices::default());
        let open = scope.cage(true);
        tree_select("refusal")
            .name("Choose files")
            .nodes(nodes())
            .value(value)
            .open(open)
            .default_expanded(BTreeSet::from(["root".into()]))
            .on_value_change({
                let result = result.clone();
                move |v| result.borrow_mut().push(v)
            })
            .show_in(scope)
    }))
    .unwrap();
    app.dispatch(named(&app, "Alpha"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(out.borrow()[0].contains("a"));
    let d = app.tree();
    let d = d.node(named(&app, "Alpha")).unwrap().descriptor.view();
    assert_eq!(d.semantics.as_ref().unwrap().selected, Some(false));
    assert_eq!(
        app.tree()
            .node(named(&app, "Choose files"))
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .expanded,
        Some(true)
    );
}
#[test]
fn data_and_paths_are_admitted_together_and_invalid_external_reparent_recovers() {
    let app = App::memory();
    let source = Rc::new(RefCell::new(None));
    let holder = source.clone();
    app.mount(widget(move |scope| {
        let nodes = scope.cage(nodes());
        let value = scope.cage(TreeChoices::new([model().path("a").unwrap()]).unwrap());
        holder.replace(Some((nodes, value)));
        tree_select("admission")
            .nodes(nodes)
            .value(value)
            .name("Choose files")
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .show_in(scope)
    }))
    .unwrap();
    let (data, value) = source.borrow().unwrap();
    let before = named(&app, "Alpha");
    data.revise(|nodes| nodes[1].parent = None);
    assert!(app.flush().is_err());
    assert_eq!(named(&app, "Alpha"), before);
    assert_eq!(
        app.tree()
            .node(before)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .tree_item
            .unwrap()
            .level,
        2
    );
    value.revise(|old| {
        *old = TreeChoices::new([sapid::TreePath::new(["a".into()]).unwrap()]).unwrap()
    });
    app.flush().unwrap();
    assert_eq!(named(&app, "Alpha"), before);
    assert_eq!(
        app.tree()
            .node(before)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .tree_item
            .unwrap()
            .level,
        1
    );
}
#[test]
fn clear_action_is_only_visible_when_it_can_clear() {
    let app = App::memory();
    app.mount(
        tree_select("clear-visibility")
            .nodes(nodes())
            .name("Choose files")
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .clearable(true),
    )
    .unwrap();
    let clear = named(&app, "Clear tree selection");
    let style = |display| {
        let tree = app.tree();
        let view = tree.node(clear).unwrap().descriptor.view();
        assert_eq!(view.style.display, Some(display));
        assert_eq!(view.style.position, Some(glory::style::Position::Absolute));
    };
    style(glory::style::Display::None);
    app.dispatch(named(&app, "Alpha"), Event::Activate).unwrap();
    app.flush().unwrap();
    style(glory::style::Display::Flex);
    app.dispatch(clear, Event::Activate).unwrap();
    app.flush().unwrap();
    style(glory::style::Display::None);
    let trigger = named(&app, "Choose files");
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
}
#[test]
fn readonly_disabled_and_clear_are_independent_from_external_controlled_updates() {
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let output = calls.clone();
    app.mount(
        tree_select("readonly")
            .nodes(nodes())
            .name("Choose files")
            .default_open(true)
            .readonly(true)
            .default_value(TreeChoices::new([model().path("a").unwrap()]).unwrap())
            .clearable(true)
            .on_value_change(move |v| output.borrow_mut().push(v)),
    )
    .unwrap();
    let id = named(&app, "Choose files");
    app.dispatch(id, Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(calls.borrow().is_empty());
    let t = app.tree();
    let s = t
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap();
    assert!(s.readonly);
    assert_eq!(s.expanded, Some(false));
    assert_eq!(s.combobox, Some(ComboboxKind::Select));
    assert_eq!(s.popup, Some(PopupKind::Tree));
    assert_eq!(t.node(id).unwrap().descriptor.view().style.grow, Some(1.));
    let clear = named(&app, "Clear tree selection");
    assert!(
        app.tree()
            .node(clear)
            .unwrap()
            .descriptor
            .view()
            .classes
            .contains(&"sapid-close-button".to_owned())
    );
}
#[test]
fn branch_leaf_policy_and_owner_retirement_do_not_publish_hidden_choices() {
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let output = calls.clone();
    app.mount(
        tree_select("owners")
            .nodes(nodes())
            .name("Choose files")
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .on_value_change(move |v| output.borrow_mut().push(v)),
    )
    .unwrap();
    app.dispatch(named(&app, "Root"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(calls.borrow().is_empty());
    app.dispatch(named(&app, "Disabled"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert!(calls.borrow().is_empty());
    let id = named(&app, "Alpha");
    app.clear().unwrap();
    app.flush().unwrap();
    assert!(!app.dispatch(id, Event::Activate).unwrap());
    assert!(app.tree().roots().is_empty());
    assert!(
        all(&app)
            .iter()
            .all(|id| app.tree().node(*id).unwrap().descriptor.view().role != Some(Role::TreeItem))
    );
}

#[test]
fn searchable_hide_picked_limit_rendering_and_popup_configuration_are_component_features() {
    let html = glory::ssr::snapshot(
        tree_select("rendered-values")
            .nodes(nodes())
            .name("Rendered files")
            .default_value(TreeChoices::new([model().path("a").unwrap()]).unwrap())
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .render_node(|node| {
                glory::ui::text().content(node.with(|node| format!("Node: {}", node.label)))
            })
            .render_value(|node| {
                glory::ui::text().content(node.with(|node| format!("Picked: {}", node.label)))
            }),
    )
    .unwrap()
    .html;
    assert!(html.contains("Node: Alpha") && html.contains("Picked: Alpha"));
    let app = App::memory();
    let changes = Rc::new(RefCell::new(Vec::new()));
    let output = changes.clone();
    app.mount(
        tree_select("searchable")
            .nodes(nodes())
            .name("Search files")
            .searchable(true)
            .hide_picked(true)
            .multiple(true)
            .max_values(1)
            .nothing_found_message("No files")
            .with_lines(false)
            .max_dropdown_height(180.)
            .popup_width(240.)
            .default_open(true)
            .default_expanded(BTreeSet::from(["root".into()]))
            .render_node(|node| {
                glory::ui::text().content(node.with(|node| format!("Node: {}", node.label)))
            })
            .render_value(|node| {
                glory::ui::text().content(node.with(|node| format!("Picked: {}", node.label)))
            })
            .on_value_change(move |value| output.borrow_mut().push(value)),
    )
    .unwrap();
    let trigger = named(&app, "Search files");
    let revision = match &app.tree().node(trigger).unwrap().descriptor.view().props {
        Props::Input(value) => value.revision,
        _ => panic!("searchable TreeSelect must use an editable input"),
    };
    app.dispatch(
        trigger,
        Event::ValueChanged {
            value: "Beta".into(),
            composing: false,
            revision,
        },
    )
    .unwrap();
    app.flush().unwrap();
    assert!(has_name(&app, "Root") && has_name(&app, "Beta"));
    assert!(!has_name(&app, "Alpha"));
    app.dispatch(named(&app, "Beta"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(changes.borrow().last().unwrap().paths().len(), 1);
    assert!(!has_name(&app, "Beta"));
    let revision = match &app.tree().node(trigger).unwrap().descriptor.view().props {
        Props::Input(value) => value.revision,
        _ => unreachable!(),
    };
    app.dispatch(
        trigger,
        Event::ValueChanged {
            value: String::new(),
            composing: false,
            revision,
        },
    )
    .unwrap();
    app.flush().unwrap();
    app.dispatch(named(&app, "Alpha"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(changes.borrow().len(), 1);
    let tree = all(&app)
        .into_iter()
        .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Tree))
        .unwrap();
    let current = app.tree();
    let style = &current.node(tree).unwrap().descriptor.view().style;
    assert_eq!(style.width, Some(glory::style::Length::Px(240.)));
    assert_eq!(style.height, Some(glory::style::Length::Px(180.)));
    assert!(all(&app).into_iter().all(|id| {
        !app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .classes
            .contains(&"sapid-tree-guide".to_owned())
    }));
}

#[test]
fn cascader_can_use_compact_layout_and_select_parent_paths_without_closing_browsing() {
    for columns in [false, true] {
        let app = App::memory();
        let reference = app.scope().cage(None);
        let expanded = app.scope().cage(BTreeSet::from(["root".to_owned()]));
        let values = Rc::new(RefCell::new(Vec::new()));
        let output = values.clone();
        app.mount(
            sapid::cascader("parent-choice")
                .name("Category")
                .node_ref(reference)
                .nodes(nodes())
                .with_columns(columns)
                .change_on_select(true)
                .expanded(expanded)
                .on_expanded_change(move |v| expanded.revise(|old| *old = v))
                .default_open(true)
                .on_value_change(move |v| output.borrow_mut().push(v)),
        )
        .unwrap();
        app.dispatch(named(&app, "Root"), Event::Activate).unwrap();
        app.flush().unwrap();
        assert_eq!(
            values.borrow().last().unwrap().paths()[0].keys(),
            &["root".to_owned()]
        );
        assert_eq!(
            app.tree()
                .node(reference.get().unwrap())
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .expanded,
            Some(true)
        );
        expanded.revise(|v| *v = BTreeSet::from(["root".to_owned()]));
        app.flush().unwrap();
        app.dispatch(named(&app, "Alpha"), Event::Activate).unwrap();
        app.flush().unwrap();
        assert_eq!(
            values.borrow().last().unwrap().paths()[0].keys(),
            &["root".to_owned(), "a".to_owned()]
        );
        assert_eq!(
            app.tree()
                .node(reference.get().unwrap())
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .expanded,
            Some(false)
        );
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn hierarchical_clear_action_names_are_bounded_mount_configuration() {
    let app = App::memory();
    assert!(
        app.mount(tree_select("clear-label").nodes(nodes()).clear_label(" "))
            .is_err()
    );
    assert_eq!(app.stats().nodes, 0);
    assert!(
        app.mount(
            sapid::cascader("clear-label")
                .nodes(nodes())
                .clear_label("x".repeat(257))
        )
        .is_err()
    );
    assert_eq!(app.stats().nodes, 0);
}
