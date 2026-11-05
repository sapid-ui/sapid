use glory::render::{Event, EventKind, FocusMode, MenuCommand, NodeId, Props, Role};
use glory::ui::App;
use sapid::*;
use std::{cell::RefCell, rc::Rc};

fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut nodes = Vec::new();
    while let Some(id) = pending.pop() {
        nodes.push(id);
        pending.extend(tree.node(id).unwrap().children.iter().copied());
    }
    nodes
}
fn named(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let node = tree.node(*id).unwrap().descriptor.view();
            node.name.as_deref() == Some(name)
                || matches!(node.props, Props::Button(button) if button.label == name)
                || matches!(node.props, Props::Check(check) if check.label == name)
        })
        .unwrap()
}
fn edit(app: &App, editor: NodeId, text: &str) {
    let Props::Input(input) = app
        .tree()
        .node(editor)
        .unwrap()
        .descriptor
        .view()
        .props
        .clone()
    else {
        panic!()
    };
    app.dispatch(
        editor,
        Event::ValueChanged {
            value: text.into(),
            composing: false,
            revision: input.revision,
        },
    )
    .unwrap();
}
fn command(app: &App, editor: NodeId, command: MenuCommand) {
    app.dispatch(
        editor,
        Event::Menu {
            from: editor,
            command,
        },
    )
    .unwrap();
}
fn active(app: &App, editor: NodeId) -> Option<NodeId> {
    app.tree().resolve_active_descendant(editor).unwrap()
}

#[test]
fn query_filtering_keeps_editors_and_items_and_uses_live_action_check_and_radio_callbacks() {
    let app = App::memory();
    let scope = app.scope();
    let query = scope.cage(String::new());
    let ruler = scope.cage(false);
    let layout = scope.cage("wide".to_owned());
    let layout_changes = scope.cage(0usize);
    let saves = scope.cage(0usize);
    let menu_values = Rc::new(RefCell::new(Vec::new()));
    let observed_menu = menu_values.clone();
    let order = Rc::new(RefCell::new(Vec::new()));
    let typed = order.clone();
    let observed = order.clone();
    app.mount(
        dropdown_menu("searchable")
            .label("Commands")
            .default_open(true)
            .search(
                menu_search()
                    .name("Find command")
                    .class("custom-search")
                    .value(query)
                    .clear_on_close(false)
                    .on(EventKind::Menu, move |_| {
                        observed_menu.borrow_mut().push(ruler.get())
                    })
                    .on_value_change(move |next| {
                        query.revise(|value| *value = next.clone());
                        typed.borrow_mut().push(format!("typed:{next}"));
                    })
                    .on(EventKind::ValueChanged, move |_| {
                        observed
                            .borrow_mut()
                            .push(format!("generic:{}", query.get()))
                    }),
            )
            .item(menu_label("group", "Document"))
            .item(
                menu_item("disabled")
                    .label("Unavailable")
                    .enabled(false)
                    .visible(query.map(|query| query.is_empty())),
            )
            .item(
                menu_item("save")
                    .label("Save")
                    .close_on_select(false)
                    .visible(query.map(|query| "save".contains(query.as_str())))
                    .on_activate(move || saves.revise(|value| *value += 1)),
            )
            .item(
                menu_checkbox("ruler")
                    .label("Ruler")
                    .checked(ruler)
                    .visible(query.map(|query| "ruler".contains(query.as_str())))
                    .on_checked_change(move |next| ruler.revise(|value| *value = next)),
            )
            .radio_group(
                menu_radio_group("layout")
                    .value(layout)
                    .on_value_change(move |next| {
                        layout.revise(|value| *value = next);
                        layout_changes.revise(|count| *count += 1);
                    })
                    .items([
                        menu_radio_item("wide", "Wide")
                            .visible(query.map(|query| "wide".contains(query.as_str()))),
                        menu_radio_item("compact", "Compact")
                            .visible(query.map(|query| "compact".contains(query.as_str()))),
                    ]),
            )
            .submenu(
                menu_submenu("export")
                    .label("Export")
                    .visible(query.map(|query| query.is_empty()))
                    .item(menu_item("pdf").label("PDF")),
            ),
    )
    .unwrap();
    let editor = named(&app, "Find command");
    let save = named(&app, "Save");
    assert!(
        app.tree()
            .node(editor)
            .unwrap()
            .descriptor
            .view()
            .classes
            .iter()
            .any(|name| name == "custom-search")
    );
    let ruler_node = named(&app, "Ruler");
    let compact = named(&app, "Compact");
    assert_eq!(active(&app, editor), None);
    command(&app, editor, MenuCommand::SearchNext);
    assert_eq!(active(&app, editor), Some(save));
    assert_eq!(
        app.tree().node(save).unwrap().descriptor.view().focus,
        FocusMode::Programmatic
    );
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(saves.get(), 1);
    edit(&app, editor, "ru");
    assert_eq!(*order.borrow(), ["typed:ru", "generic:ru"]);
    assert_eq!(active(&app, editor), None);
    assert!(!app.tree().active(save));
    assert!(app.tree().active(ruler_node));
    command(&app, editor, MenuCommand::SearchNext);
    assert_eq!(active(&app, editor), Some(ruler_node));
    command(&app, editor, MenuCommand::SearchCommit);
    assert!(ruler.get());
    assert_eq!(menu_values.borrow().last(), Some(&true));
    command(&app, editor, MenuCommand::SearchCommit);
    assert!(!ruler.get());
    edit(&app, editor, "compact");
    command(&app, editor, MenuCommand::SearchPrevious);
    assert_eq!(active(&app, editor), Some(compact));
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(layout.get(), "compact");
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(
        layout_changes.get(),
        1,
        "the current radio must not emit another value change"
    );
    edit(&app, editor, "no matching action");
    command(&app, editor, MenuCommand::SearchNext);
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(active(&app, editor), None);
    assert_eq!(saves.get(), 1);
    edit(&app, editor, "");
    assert_eq!(named(&app, "Save"), save);
    assert_eq!(named(&app, "Find command"), editor);
    assert_eq!(named(&app, "Ruler"), ruler_node);
    app.clear().unwrap();
}

#[test]
fn controlled_query_and_check_refusals_do_not_advance_or_replace_the_current_controls() {
    let app = App::memory();
    let scope = app.scope();
    let query = scope.cage("ru".to_owned());
    let checked = scope.cage(false);
    let proposals = Rc::new(RefCell::new(Vec::new()));
    let output = proposals.clone();
    app.mount(
        dropdown_menu("refusal")
            .label("Refused commands")
            .default_open(true)
            .search(menu_search().name("Refused query").value(query))
            .item(
                menu_checkbox("ruler")
                    .label("Ruler")
                    .checked(checked)
                    .on_checked_change(move |value| output.borrow_mut().push(value)),
            ),
    )
    .unwrap();
    let editor = named(&app, "Refused query");
    edit(&app, editor, "rejected query");
    assert_eq!(query.get(), "ru");
    assert!(
        matches!(app.tree().node(editor).unwrap().descriptor.view().props, Props::Input(input) if input.value == "ru")
    );
    command(&app, editor, MenuCommand::SearchNext);
    command(&app, editor, MenuCommand::SearchCommit);
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(*proposals.borrow(), [true, true]);
    assert!(!checked.get());
    assert_eq!(named(&app, "Refused query"), editor);
    app.clear().unwrap();
}

#[test]
fn close_clear_is_a_single_proposal_and_preserve_policy_keeps_the_accepted_query() {
    for clear in [false, true] {
        let app = App::memory();
        let scope = app.scope();
        let query = scope.cage("save".to_owned());
        let calls = Rc::new(RefCell::new(Vec::new()));
        let output = calls.clone();
        app.mount(
            dropdown_menu("close-search")
                .label("Search commands")
                .default_open(true)
                .search(
                    menu_search()
                        .name("Search query")
                        .value(query)
                        .clear_on_close(clear)
                        .on_value_change(move |value| {
                            output.borrow_mut().push(value.clone());
                            query.revise(|old| *old = value);
                        }),
                )
                .item(menu_item("save").label("Save")),
        )
        .unwrap();
        let editor = named(&app, "Search query");
        command(&app, editor, MenuCommand::SearchNext);
        command(&app, editor, MenuCommand::SearchCommit);
        assert_eq!(query.get(), if clear { "" } else { "save" });
        assert_eq!(calls.borrow().len(), usize::from(clear));
        let panel = nodes(&app)
            .into_iter()
            .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Menu))
            .unwrap();
        assert!(!app.tree().active(panel));
        app.dispatch(named(&app, "Search commands"), Event::Activate)
            .unwrap();
        assert_eq!(named(&app, "Search query"), editor);
        assert_eq!(active(&app, editor), None);
        app.clear().unwrap();
    }
}

#[test]
fn invalid_search_is_rejected_before_menu_nodes_and_query_callbacks_retire_with_the_owner() {
    let app = App::memory();
    assert!(
        app.mount(
            dropdown_menu("too-long")
                .search(menu_search().default_value("x".repeat(MAX_MENU_SEARCH_BYTES + 1)))
                .item(menu_item("one").label("One"))
        )
        .is_err()
    );
    assert!(nodes(&app).is_empty());
    let guard = Rc::new(());
    let captured = guard.clone();
    let reference = app.scope().cage(None);
    app.mount(
        dropdown_menu("lifetime")
            .search(menu_search().node_ref(reference).on_value_change(move |_| {
                let _ = &captured;
            }))
            .item(menu_item("one").label("One")),
    )
    .unwrap();
    assert!(reference.get().is_some());
    app.clear().unwrap();
    assert!(reference.get().is_none());
    assert_eq!(Rc::strong_count(&guard), 1);
}

#[test]
fn disabling_hides_a_controlled_open_menu_without_writing_its_source_or_accepting_late_commands() {
    let app = App::memory();
    let open = app.scope().cage(true);
    let enabled = app.scope().cage(true);
    let requests = Rc::new(RefCell::new(Vec::new()));
    let output = requests.clone();
    let selected = app.scope().cage(0usize);
    app.mount(
        dropdown_menu("availability")
            .label("Available actions")
            .open(open)
            .enabled(enabled)
            .on_open_change(move |next| output.borrow_mut().push(next))
            .search(menu_search().name("Available search"))
            .item(
                menu_item("save")
                    .label("Save")
                    .close_on_select(false)
                    .on_activate(move || selected.revise(|old| *old += 1)),
            ),
    )
    .unwrap();
    let editor = named(&app, "Available search");
    enabled.revise(|value| *value = false);
    app.settle(16).unwrap();
    assert!(open.get());
    assert!(!app.tree().active(editor));
    assert!(requests.borrow().is_empty());
    command(&app, editor, MenuCommand::SearchNext);
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(selected.get(), 0);
    enabled.revise(|value| *value = true);
    app.settle(16).unwrap();
    assert!(app.tree().active(editor));
    assert_eq!(named(&app, "Available search"), editor);
    command(&app, editor, MenuCommand::SearchNext);
    command(&app, editor, MenuCommand::SearchCommit);
    assert_eq!(selected.get(), 1);
    app.clear().unwrap();
}

#[test]
fn menu_and_submenu_geometry_validate_before_creating_roots() {
    let app = App::memory();
    for width in [f64::NAN, f64::INFINITY, 79., 1001.] {
        assert!(
            app.mount(
                dropdown_menu("root-width")
                    .width(width)
                    .item(menu_item("save").label("Save"))
            )
            .is_err()
        );
        assert!(
            app.mount(
                dropdown_menu("child-width").submenu(
                    menu_submenu("export")
                        .label("Export")
                        .width(width)
                        .item(menu_item("text").label("Text"))
                )
            )
            .is_err()
        );
        assert!(nodes(&app).is_empty());
    }
}
