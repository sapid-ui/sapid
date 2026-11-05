use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Orientation, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
fn class(app: &App, name: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut found = Vec::new();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        todo.extend(node.children.iter().copied());
        if node
            .descriptor
            .view()
            .classes
            .iter()
            .any(|value| value == name)
        {
            found.push(id);
        }
    }
    found
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    app.request(Query::Bounds(id), move |value| {
        *out.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let Some(QueryValue::Bounds(rect)) = saved.borrow_mut().take() else {
        panic!("bounds")
    };
    rect
}
#[test]
fn five_sizes_keep_real_circular_markers_and_joined_tracks_in_both_orientations() {
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
            for orientation in [Orientation::Horizontal, Orientation::Vertical] {
                let host = WindowApp::new();
                host.renderer().set_viewport(600, 480, 1.).unwrap();
                let app = host.app();
                let active = app.scope().cage(1_usize);
                host.mount(
                    sapid::stepper("native-workflow")
                        .theme(theme.clone())
                        .size(size)
                        .orientation(orientation)
                        .name("Setup progress")
                        .active(active)
                        .style(Style::new().width(Length::Px(560.)))
                        .steps([
                            sapid::step("first")
                                .label("First")
                                .description("Create an account"),
                            sapid::step("second")
                                .label("Second")
                                .description("Confirm email"),
                            sapid::step("third")
                                .label("Third")
                                .description("Start a project"),
                        ]),
                )
                .unwrap();
                app.settle(16).unwrap();
                let mut buttons = class(app, "sapid-stepper-trigger");
                let mut lines = class(app, "sapid-stepper-connector");
                buttons.sort_by(|a, b| {
                    let a = bounds(app, *a);
                    let b = bounds(app, *b);
                    if orientation == Orientation::Horizontal {
                        a.x.total_cmp(&b.x)
                    } else {
                        a.y.total_cmp(&b.y)
                    }
                });
                lines.sort_by(|a, b| {
                    let a = bounds(app, *a);
                    let b = bounds(app, *b);
                    if orientation == Orientation::Horizontal {
                        a.x.total_cmp(&b.x)
                    } else {
                        a.y.total_cmp(&b.y)
                    }
                });
                assert_eq!(buttons.len(), 3);
                assert_eq!(lines.len(), 2);
                let side = theme.tokens.control(size).min_height;
                for button in &buttons {
                    let rect = bounds(app, *button);
                    assert!(
                        (rect.width - side).abs() < 1. && (rect.height - side).abs() < 1.,
                        "{size:?} {orientation:?} {rect:?}"
                    );
                }
                for (index, line) in lines.iter().enumerate() {
                    let rect = bounds(app, *line);
                    let before = bounds(app, buttons[index]);
                    let after = bounds(app, buttons[index + 1]);
                    if orientation == Orientation::Vertical {
                        assert!((rect.width - 2.).abs() < 1.);
                        assert!(
                            (rect.x + rect.width / 2. - before.x - before.width / 2.).abs() < 1.
                        );
                        assert!((rect.y - before.y - before.height).abs() < 1.);
                        assert!((rect.y + rect.height - after.y).abs() < 1.);
                    } else {
                        assert!((rect.height - 2.).abs() < 1. && rect.width >= 15.);
                        assert!(
                            (rect.y + rect.height / 2. - before.y - before.height / 2.).abs() < 1.
                        );
                        assert!(
                            rect.x >= before.x + before.width
                                && rect.x + rect.width <= after.x + 1.
                        );
                    }
                }
                let physical = host
                    .renderer()
                    .engine_id(buttons[1], glory::html::Part::Main)
                    .unwrap();
                active.revise(|value| *value = 2);
                app.settle(16).unwrap();
                assert_eq!(
                    host.renderer()
                        .engine_id(buttons[1], glory::html::Part::Main)
                        .unwrap(),
                    physical
                );
                assert_eq!(
                    app.tree()
                        .node(lines[1])
                        .unwrap()
                        .descriptor
                        .view()
                        .style
                        .background,
                    Some(theme.accent)
                );
            }
        }
    }
}
