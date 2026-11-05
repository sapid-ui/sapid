use super::*;
use glory::render::{Event, MenuCommand, Navigation, Props};
use glory::ui::App;
use std::cell::{Cell, RefCell};

fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut queue = tree.roots().to_vec();
    let mut nodes = Vec::new();
    while let Some(id) = queue.pop() {
        nodes.push(id);
        queue.extend(tree.node(id).unwrap().children.iter().copied());
    }
    nodes
}
fn button(app: &App, label: &str) -> NodeId {
    nodes(app).into_iter().find(|id| {
        let tree = app.tree();
        let value = tree.node(*id).unwrap().descriptor.view();
        matches!(value.props, Props::Button(button) if button.label == label || value.name.as_deref() == Some(label))
    }).unwrap()
}
fn panel(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let node = tree.node(*id).unwrap().descriptor.view();
            node.role == Some(Role::Menu) && node.name.as_deref() == Some(name)
        })
        .unwrap()
}
fn hidden(app: &App, id: NodeId) -> bool {
    app.tree()
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap()
        .hidden
}
fn menu_command(app: &App, from: NodeId, command: MenuCommand) {
    app.dispatch(from, Event::Menu { from, command }).unwrap();
}
fn navigate(app: &App, from: NodeId, command: Navigation) {
    app.dispatch(from, Event::Navigate { from, command })
        .unwrap();
}
fn sample() -> Menubar {
    menubar("application").menus([
        menubar_menu("file")
            .label("File")
            .item(menu_item("new").label("New file"))
            .submenu(
                menu_submenu("export")
                    .label("Export")
                    .item(menu_item("pdf").label("PDF")),
            ),
        menubar_menu("disabled")
            .label("Disabled")
            .enabled(false)
            .item(menu_item("unused").label("Unused")),
        menubar_menu("view")
            .label("View")
            .item(menu_item("zoom").label("Zoom")),
    ])
}

#[test]
fn pointer_switches_one_open_root_hover_is_opt_in_and_controlled_refusals_hold() {
    for hover in [false, true] {
        let app = App::memory();
        app.mount(sample().open_on_hover(hover)).unwrap();
        let file = button(&app, "File");
        let view = button(&app, "View");
        let disabled = button(&app, "Disabled");
        let pointer = |from| Event::Pointer {
            from,
            related: None,
            phase: PointerPhase::Move,
        };
        app.dispatch(file, pointer(file)).unwrap();
        app.flush().unwrap();
        assert_eq!(!hidden(&app, panel(&app, "File")), hover);
        if !hover {
            app.dispatch(file, Event::Activate).unwrap();
            app.flush().unwrap();
        }
        app.dispatch(view, pointer(view)).unwrap();
        app.flush().unwrap();
        assert!(hidden(&app, panel(&app, "File")));
        assert!(!hidden(&app, panel(&app, "View")));
        app.dispatch(disabled, pointer(disabled)).unwrap();
        app.flush().unwrap();
        assert!(!hidden(&app, panel(&app, "View")));
        if hover {
            let bar = nodes(&app)
                .into_iter()
                .find(|id| {
                    app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Menubar)
                })
                .unwrap();
            app.dispatch(
                bar,
                Event::PointerWithin {
                    from: view,
                    related: None,
                    phase: PointerPhase::Leave,
                },
            )
            .unwrap();
            app.flush().unwrap();
            assert!(hidden(&app, panel(&app, "View")));
        }
    }
    let app = App::memory();
    app.mount(sample().open_on_hover(true).value(Some("file".to_owned())))
        .unwrap();
    let view = button(&app, "View");
    app.dispatch(
        view,
        Event::Pointer {
            from: view,
            related: None,
            phase: PointerPhase::Move,
        },
    )
    .unwrap();
    app.flush().unwrap();
    assert!(!hidden(&app, panel(&app, "File")));
    assert!(hidden(&app, panel(&app, "View")));
}

#[test]
fn dropdown_position_and_width_override_per_root_and_validate_before_mount() {
    use glory::render::{AnchorAlign, AnchorPlacement, AnchorSide};
    let position = AnchorPlacement::new(AnchorSide::Top, AnchorAlign::End);
    let app = App::memory();
    app.mount(
        menubar("position").position(position).menus([
            menubar_menu("file")
                .label("File")
                .width(260.)
                .item(menu_item("new").label("New")),
            menubar_menu("edit")
                .label("Edit")
                .width(180.)
                .position(AnchorPlacement::default())
                .item(menu_item("undo").label("Undo")),
        ]),
    )
    .unwrap();
    for (name, width, placement) in [
        ("File", 260., position),
        ("Edit", 180., AnchorPlacement::default()),
    ] {
        let tree = app.tree();
        let id = panel(&app, name);
        let descriptor = tree.node(id).unwrap().descriptor.view();
        assert_eq!(descriptor.style.width, Some(Length::Px(width)));
        assert_eq!(
            descriptor
                .semantics
                .as_ref()
                .unwrap()
                .layer
                .as_ref()
                .unwrap()
                .placement,
            LayerPlacement::Anchor(placement)
        );
    }
    for width in [f64::NAN, 0., 1001.] {
        let app = App::memory();
        assert!(
            app.mount(
                menubar("invalid").menu(menubar_menu("file").width(width).item(menu_item("new")))
            )
            .is_err()
        );
        assert!(app.tree().is_empty());
    }
    let app = App::memory();
    assert!(
        app.mount(sample().position(position.gap(f64::NAN)))
            .is_err()
    );
    assert!(app.tree().is_empty());
}

#[test]
fn menubar_intrinsic_roots_share_one_tab_stop_disabled_skip_and_horizontal_typeahead() {
    let app = App::memory();
    app.mount(sample().name("Application menu")).unwrap();
    let bar = nodes(&app)
        .into_iter()
        .find(|id| app.tree().node(*id).unwrap().descriptor.view().role == Some(Role::Menubar))
        .unwrap();
    assert_eq!(
        app.tree()
            .node(bar)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref(),
        Some("Application menu")
    );
    let owner = app
        .tree()
        .node(bar)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap()
        .id
        .clone()
        .unwrap();
    let file = button(&app, "File");
    let view = button(&app, "View");
    for label in ["File", "View", "Disabled"] {
        let tree = app.tree();
        let root = tree.node(button(&app, label)).unwrap().descriptor.view();
        let metadata = root.semantics.as_ref().unwrap();
        assert!(matches!(root.props, Props::Button(_)));
        assert_eq!(metadata.popup, Some(PopupKind::Menu));
        assert_eq!(metadata.menu_item.as_ref().unwrap().owner, owner);
    }
    assert_eq!(
        app.tree().node(file).unwrap().descriptor.view().focus,
        FocusMode::TabStop
    );
    assert_eq!(
        app.tree().node(view).unwrap().descriptor.view().focus,
        FocusMode::Programmatic
    );
    navigate(&app, file, Navigation::Next);
    assert_eq!(
        app.tree().node(view).unwrap().descriptor.view().focus,
        FocusMode::TabStop
    );
    assert!(hidden(&app, panel(&app, "File")) && hidden(&app, panel(&app, "View")));
    app.dispatch(
        view,
        Event::Typeahead {
            from: view,
            text: "f".into(),
            at_millis: 1,
        },
    )
    .unwrap();
    assert_eq!(
        app.tree().node(file).unwrap().descriptor.view().focus,
        FocusMode::TabStop
    );
}

#[test]
fn menubar_open_roots_are_atomic_and_keyboard_hierarchy_switches_only_at_the_root() {
    let app = App::memory();
    app.mount(sample()).unwrap();
    let file = button(&app, "File");
    menu_command(&app, file, MenuCommand::First);
    assert!(!hidden(&app, panel(&app, "File")) && hidden(&app, panel(&app, "View")));
    menu_command(&app, button(&app, "New file"), MenuCommand::Open);
    assert!(hidden(&app, panel(&app, "File")) && !hidden(&app, panel(&app, "View")));
    menu_command(&app, button(&app, "Zoom"), MenuCommand::Close);
    assert!(!hidden(&app, panel(&app, "File")) && hidden(&app, panel(&app, "View")));
    menu_command(&app, button(&app, "Export"), MenuCommand::Open);
    assert!(!hidden(&app, panel(&app, "Export")));
    menu_command(&app, button(&app, "PDF"), MenuCommand::Close);
    assert!(hidden(&app, panel(&app, "Export")) && !hidden(&app, panel(&app, "File")));
    app.dispatch(
        panel(&app, "File"),
        Event::Dismiss(glory::render::DismissReason::Escape),
    )
    .unwrap();
    assert!(hidden(&app, panel(&app, "File")));
}

#[test]
fn controlled_menubar_refusal_keeps_focus_and_open_root_together_and_invalid_sources_do_not_clone()
{
    let app = App::memory();
    let value = app.scope().cage(None::<String>);
    let proposals = Rc::new(RefCell::new(Vec::new()));
    let output = proposals.clone();
    app.mount(
        sample()
            .value(value)
            .on_value_change(move |value| output.borrow_mut().push(value)),
    )
    .unwrap();
    let file = button(&app, "File");
    let view = button(&app, "View");
    menu_command(&app, file, MenuCommand::First);
    assert!(hidden(&app, panel(&app, "File")));
    assert_eq!(*proposals.borrow(), [Some("file".to_owned())]);
    value.revise(|old| *old = Some("file".to_owned()));
    app.flush().unwrap();
    navigate(&app, file, Navigation::Next);
    assert!(!hidden(&app, panel(&app, "File")) && hidden(&app, panel(&app, "View")));
    assert_eq!(
        app.tree().node(view).unwrap().descriptor.view().focus,
        FocusMode::Programmatic
    );
    value.revise(|old| *old = Some("x".repeat(1_000_000)));
    assert!(app.flush().is_err());
    assert!(!hidden(&app, panel(&app, "File")));
    value.revise(|old| *old = None);
    app.flush().unwrap();
    assert!(hidden(&app, panel(&app, "File")));
}

#[test]
fn menubar_configuration_is_bounded_before_nodes_and_owner_retirement_releases_callbacks() {
    let app = App::memory();
    let consumed = Rc::new(Cell::new(0));
    let input = consumed.clone();
    let oversized = menubar("huge").menus((0..1_000_000).map(move |index| {
        input.set(input.get() + 1);
        menubar_menu(format!("root-{index}"))
            .label("Root")
            .item(menu_item("one").label("One"))
    }));
    assert!(app.mount(oversized).is_err());
    assert_eq!(consumed.get(), 33);
    assert_eq!(app.stats().nodes, 0);
    let app = App::memory();
    let owner = app.scope().child().unwrap();
    let capture = Rc::new(());
    let inside = capture.clone();
    sample()
        .on_value_change(move |_| {
            let _ = &inside;
        })
        .show_in(&owner)
        .unwrap();
    app.flush().unwrap();
    owner.dispose();
    assert_eq!(Rc::strong_count(&capture), 1);
    assert_eq!(app.stats().nodes, 0);
    assert_eq!(app.stats().handlers, 0);
}
