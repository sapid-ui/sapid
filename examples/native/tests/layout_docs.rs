use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/showcase_layout_docs/flex_responsive.rs"]
mod flex;
#[path = "../../web/src/showcase_layout_docs/grid_responsive.rs"]
mod grid;
#[path = "../../web/src/showcase_layout_docs/group_growth.rs"]
mod group;
#[path = "../../web/src/showcase_layout_docs/simple_responsive.rs"]
mod simple;
fn named(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().name.as_deref() == Some(name)
            || matches!(node.descriptor.view().props, glory::render::Props::Button(p) if p.label == name)
        {
            return id;
        }
        todo.extend(node.children.iter().copied());
    }
    panic!("missing {name}")
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let saved = Rc::new(RefCell::new(None));
    let output = saved.clone();
    app.request(Query::Bounds(id), move |v| {
        *output.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(value) = saved.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    value
}
#[test]
fn responsive_documentation_reflows_same_native_editors_with_explicit_placement_in_four_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in ["grid", "simple", "flex"] {
            let host = WindowApp::new();
            let app = host.app();
            let width = app.scope().cage(640.);
            let content: Box<dyn Widget> = match kind {
                "grid" => Box::new(grid::demo(theme.clone())),
                "simple" => Box::new(simple::demo(theme.clone())),
                _ => Box::new(flex::demo(theme.clone())),
            };
            host.mount(
                glory::ui::view()
                    .style(width.map(|v| Style::new().width(Length::Px(*v))))
                    .fill(content),
            )
            .unwrap();
            app.settle(16).unwrap();
            let (first, second) = match kind {
                "grid" => ("Grid project 1 note", "Grid project 2 note"),
                "simple" => ("Simple grid note 1", "Simple grid note 2"),
                _ => ("Responsive project name", "Create project"),
            };
            let first = named(app, first);
            let second = named(app, second);
            let engine = host
                .renderer()
                .engine_id(first, glory::html::Part::Main)
                .unwrap();
            let wide_a = bounds(app, first);
            let wide_b = bounds(app, second);
            assert!(wide_b.x > wide_a.x + 20., "{kind}: {wide_a:?} {wide_b:?}");
            width.revise(|v| *v = 280.);
            app.settle(20).unwrap();
            let narrow_a = bounds(app, first);
            let narrow_b = bounds(app, second);
            assert!(
                narrow_b.y > narrow_a.y + 20.,
                "{kind}: {narrow_a:?} {narrow_b:?}"
            );
            assert!(narrow_a.x >= 0. && narrow_a.x + narrow_a.width <= 281.);
            assert!(narrow_b.x >= 0. && narrow_b.x + narrow_b.width <= 281.);
            assert_eq!(
                host.renderer()
                    .engine_id(first, glory::html::Part::Main)
                    .unwrap(),
                engine
            );
            width.revise(|v| *v = 640.);
            app.settle(20).unwrap();
            assert!(bounds(app, second).x > bounds(app, first).x + 20.);
            app.clear().unwrap();
            assert!(app.tree().is_empty());
        }
    }
}
#[test]
fn native_long_group_content_shrinks_beside_a_stable_action() {
    let host = WindowApp::new();
    host.mount(
        glory::ui::view()
            .style(Style::new().width(Length::Px(280.)))
            .fill(group::demo(sapid::Theme::blue_dark())),
    )
    .unwrap();
    let app = host.app();
    app.settle(16).unwrap();
    let open = named(app, "Open");
    let rect = bounds(app, open);
    assert!(rect.width > 30. && rect.x + rect.width <= 281.);
    assert!(rect.height < 60.);
}
