use glory::{
    apace::{Raster, shell::WindowApp},
    prelude::*,
    render::{
        Action, Event, NodeId, Props, Purpose, Query, QueryValue, Rect, SelectionDirection,
        TextSelection,
    },
};
use std::{cell::RefCell, rc::Rc};
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        pending.extend(node.children.iter().copied());
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            found.push(id)
        }
    }
    found
}
fn query(app: &App, request: Query) -> QueryValue {
    let value = Rc::new(RefCell::new(None));
    let out = value.clone();
    app.request(request, move |v| *out.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    value.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    rect
}
#[test]
fn four_themes_and_five_sizes_keep_eye_inset_painted_and_preserve_native_selection() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for size in [
            sapid::Size::ExtraSmall,
            sapid::Size::Small,
            sapid::Size::Medium,
            sapid::Size::Large,
            sapid::Size::ExtraLarge,
        ] {
            let host = WindowApp::new();
            host.renderer().set_viewport(400, 200, 1.).unwrap();
            let app = host.app();
            let reference = app.scope().cage(None);
            host.mount(
                sapid::field("native-password")
                    .theme(theme.clone())
                    .size(size)
                    .label("Secret")
                    .style(Style::new().width(Length::Px(320.)))
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .node_ref(reference)
                            .default_value("秘密😀 sample")
                    }),
            )
            .unwrap();
            app.settle(16).unwrap();
            let editor = reference.get().unwrap();
            let toggle = nodes(app, "sapid-password-toggle")[0];
            let identity = host
                .renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap();
            let input = bounds(app, editor);
            let eye = bounds(app, toggle);
            assert!(eye.width > 0. && eye.width < input.height && eye.height < input.height);
            assert!(eye.x >= input.x && eye.x + eye.width <= input.x + input.width + 0.2);
            assert!((eye.y + eye.height / 2. - input.y - input.height / 2.).abs() < 0.3);
            let mut raster = Raster::new();
            raster.draw(host.renderer()).unwrap();
            let frame = raster.frame().unwrap();
            let color = |x: usize, y: usize| {
                &raster.pixels()
                    [(y * frame.width as usize + x) * 4..(y * frame.width as usize + x) * 4 + 3]
            };
            let base = color(eye.x as usize + 1, eye.y as usize + 1).to_vec();
            let mut changed = 0;
            for y in eye.y.ceil() as usize..(eye.y + eye.height).floor() as usize {
                for x in eye.x.ceil() as usize..(eye.x + eye.width).floor() as usize {
                    if color(x, y)
                        .iter()
                        .zip(&base)
                        .map(|(a, b)| a.abs_diff(*b) as u32)
                        .sum::<u32>()
                        > 48
                    {
                        changed += 1;
                    }
                }
            }
            assert!(changed > 4, "eye must contain actual painted strokes");
            app.scope().action(Action::Focus(editor)).unwrap();
            let revision = match app.tree().node(editor).unwrap().descriptor.view().props {
                Props::Input(p) => p.revision,
                _ => panic!(),
            };
            app.scope()
                .action(Action::SelectText {
                    node: editor,
                    revision,
                    selection: TextSelection {
                        start: 1,
                        end: 4,
                        direction: SelectionDirection::Forward,
                    },
                })
                .unwrap();
            app.settle(16).unwrap();
            for purpose in [Purpose::Text, Purpose::Password] {
                app.scope().action(Action::Focus(toggle)).unwrap();
                app.flush().unwrap();
                app.dispatch(toggle, Event::Activate).unwrap();
                app.settle(16).unwrap();
                assert_eq!(
                    host.renderer()
                        .engine_id(editor, glory::html::Part::Main)
                        .unwrap(),
                    identity
                );
                assert_eq!(
                    query(app, Query::Selection(editor)),
                    QueryValue::Selection { start: 1, end: 4 }
                );
                let tree = app.tree();
                let Props::Input(props) = tree.node(editor).unwrap().descriptor.view().props else {
                    panic!()
                };
                assert_eq!(props.purpose, purpose);
                assert_eq!(props.value, "秘密😀 sample");
                assert_eq!(props.revision, revision);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}
#[test]
fn loading_replaces_only_the_requested_slot_without_retiring_the_editor_or_eye() {
    for position in [
        sapid::InputSectionPosition::Right,
        sapid::InputSectionPosition::Left,
    ] {
        let host = WindowApp::new();
        let app = host.app();
        let loading = app.scope().cage(false);
        let reference = app.scope().cage(None);
        host.mount(
            sapid::password_input()
                .loading(loading)
                .loading_position(position)
                .node_ref(reference)
                .default_value("Loading sample"),
        )
        .unwrap();
        app.settle(16).unwrap();
        let editor = reference.get().unwrap();
        let eye = nodes(app, "sapid-password-toggle")[0];
        let identity = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        loading.revise(|v| *v = true);
        app.settle(16).unwrap();
        assert_eq!(
            app.tree().active(eye),
            position == sapid::InputSectionPosition::Left
        );
        assert_eq!(reference.get(), Some(editor));
        assert_eq!(
            host.renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        loading.revise(|v| *v = false);
        app.settle(16).unwrap();
        assert!(app.tree().active(eye));
        assert_eq!(nodes(app, "sapid-password-toggle"), [eye]);
    }
}
