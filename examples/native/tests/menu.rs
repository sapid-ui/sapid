use apace_dom::Document;
use apace_traits::events::{
    ApaceKeyEvent, ApacePointerEvent, ApacePointerId, KeyState, MouseEventButton,
    MouseEventButtons, Point, PointerCoords, UiEvent,
};
use glory::apace::shell::WindowApp;
use glory::prelude::*;
use glory::render::{Action, NodeId, Props, Query, QueryValue, Rect};
use sapid::*;
use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Waker},
};

fn observe(host: &WindowApp) {
    for _ in 0..8 {
        host.observe(&Context::from_waker(Waker::noop())).unwrap();
    }
}
fn named(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        let value = node.descriptor.view();
        if value.name.as_deref() == Some(name)
            || matches!(value.props, Props::Button(props) if props.label == name)
            || matches!(value.props, Props::Check(check) if check.label == name)
        {
            return id;
        }
        pending.extend(node.children.iter().copied());
    }
    panic!("missing {name}")
}
fn query(app: &App, query: Query) -> QueryValue {
    let out = Rc::new(RefCell::new(None));
    let result = out.clone();
    app.request(query, move |reply| {
        *result.borrow_mut() = Some(reply.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    out.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    rect
}
fn key(host: &mut WindowApp, key: &str) {
    host.handle_ui_event(UiEvent::KeyDown(ApaceKeyEvent {
        key: key.parse().unwrap(),
        code: Default::default(),
        modifiers: Default::default(),
        location: Default::default(),
        is_auto_repeating: false,
        is_composing: false,
        state: KeyState::Pressed,
        text: None,
    }));
    observe(host);
}
fn click(host: &mut WindowApp, id: NodeId) {
    let rect = bounds(host.app(), id);
    let x = (rect.x + rect.width / 2.) as f32;
    let y = (rect.y + rect.height / 2.) as f32;
    let pointer = |pressed| ApacePointerEvent {
        id: ApacePointerId::Mouse,
        is_primary: true,
        coords: PointerCoords {
            page_x: x,
            page_y: y,
            client_x: x,
            client_y: y,
            screen_x: x,
            screen_y: y,
        },
        button: MouseEventButton::Main,
        buttons: if pressed {
            MouseEventButtons::Primary
        } else {
            MouseEventButtons::empty()
        },
        mods: Default::default(),
        details: Default::default(),
        element: Point::default(),
        active_pointers: Default::default(),
    };
    host.handle_ui_event(UiEvent::PointerMove(pointer(false)));
    host.handle_ui_event(UiEvent::PointerDown(pointer(true)));
    host.handle_ui_event(UiEvent::PointerUp(pointer(false)));
    observe(host);
}

#[test]
fn four_theme_menu_and_menubar_search_keep_real_focus_and_control_identity_through_filtering() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for bar in [false, true] {
            let mut host = WindowApp::new();
            host.renderer().set_viewport(390, 600, 1.).unwrap();
            let query_value = host.app().scope().cage(String::new());
            let checked = host.app().scope().cage(false);
            let selected = host.app().scope().cage(0usize);
            let density = host.app().scope().cage("wide".to_owned());
            let density_changes = host.app().scope().cage(0usize);
            macro_rules! entries {
                ($menu:expr) => {
                    $menu
                        .search(
                            menu_search()
                                .name("Find native action")
                                .value(query_value)
                                .on_value_change(move |next| query_value.revise(|old| *old = next)),
                        )
                        .item(menu_label("group", "Document commands"))
                        .item(
                            menu_item("save")
                                .label("Save document")
                                .close_on_select(false)
                                .left_section(glory::ui::text().content("S"))
                                .right_section(glory::ui::text().content("Ctrl+S"))
                                .visible(
                                    query_value
                                        .map(|value| "save document".contains(value.as_str())),
                                )
                                .on_activate(move || selected.revise(|old| *old += 1)),
                        )
                        .item(
                            menu_checkbox("ruler")
                                .label("Ruler")
                                .checked(checked)
                                .visible(query_value.map(|value| "ruler".contains(value.as_str())))
                                .on_checked_change(move |next| checked.revise(|old| *old = next)),
                        )
                        .radio_group(
                            menu_radio_group("density")
                                .value(density)
                                .on_value_change(move |next| {
                                    density.revise(|old| *old = next);
                                    density_changes.revise(|old| *old += 1);
                                })
                                .items([
                                    menu_radio_item("wide", "Wide").visible(
                                        query_value.map(|query| "wide".contains(query.as_str())),
                                    ),
                                    menu_radio_item("compact", "Compact").visible(
                                        query_value.map(|query| "compact".contains(query.as_str())),
                                    ),
                                ]),
                        )
                };
            }
            let widget: std::boxed::Box<dyn Widget> = if bar {
                std::boxed::Box::new(
                    menubar("native-bar")
                        .theme(theme.clone())
                        .menu(entries!(menubar_menu("file").label("File"))),
                )
            } else {
                std::boxed::Box::new(entries!(
                    dropdown_menu("native-menu")
                        .theme(theme.clone())
                        .label("File")
                ))
            };
            host.mount(
                glory::ui::view()
                    .style(Style::new().padding(Edges::all(20.)))
                    .fill(widget),
            )
            .unwrap();
            observe(&host);
            let trigger = named(host.app(), "File");
            host.app().scope().action(Action::Focus(trigger)).unwrap();
            host.app().flush().unwrap();
            observe(&host);
            key(&mut host, "ArrowDown");
            let editor = named(host.app(), "Find native action");
            let physical = host
                .renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap();
            assert_eq!(
                query(host.app(), Query::Focus(editor)),
                QueryValue::Focus(true)
            );
            let save = named(host.app(), "Save document");
            let area = bounds(host.app(), save);
            let field = bounds(host.app(), editor);
            assert!(field.width > 150. && field.x >= 0. && field.x + field.width <= 390.1);
            assert!(area.y >= field.y + field.height && area.width > 150.);
            key(&mut host, "Enter");
            assert_eq!(selected.get(), 1);
            assert_eq!(
                query(host.app(), Query::Focus(editor)),
                QueryValue::Focus(true)
            );
            query_value.revise(|old| *old = "ru".into());
            observe(&host);
            assert!(!host.app().tree().active(save));
            key(&mut host, "ArrowDown");
            key(&mut host, "Enter");
            assert!(checked.get());
            key(&mut host, "Enter");
            assert!(!checked.get());
            let ruler = named(host.app(), "Ruler");
            click(&mut host, ruler);
            assert!(checked.get());
            assert_eq!(
                query(host.app(), Query::Focus(editor)),
                QueryValue::Focus(true)
            );
            assert_eq!(
                host.renderer()
                    .engine_id(editor, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
            query_value.revise(|old| *old = "co".into());
            observe(&host);
            key(&mut host, "ArrowDown");
            for _ in 0..2 {
                host.handle_ui_event(UiEvent::KeyDown(ApaceKeyEvent {
                    key: "Enter".parse().unwrap(),
                    code: Default::default(),
                    modifiers: Default::default(),
                    location: Default::default(),
                    is_auto_repeating: false,
                    is_composing: false,
                    state: KeyState::Pressed,
                    text: None,
                }));
            }
            observe(&host);
            assert_eq!(density.get(), "compact");
            assert_eq!(
                density_changes.get(),
                1,
                "queued Enter commands must not repeat a radio value change"
            );
            key(&mut host, "Escape");
            assert_eq!(query_value.get(), "");
            assert_eq!(
                query(host.app(), Query::Focus(trigger)),
                QueryValue::Focus(true)
            );
            host.app().clear().unwrap();
            assert_eq!(host.app().stats().nodes, 0);
        }
    }
}
