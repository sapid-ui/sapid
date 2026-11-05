use glory::apace::shell::WindowApp;
use glory::prelude::*;
use glory::render::{Event, Query, QueryValue, ScrollCommand, ScrollMetrics, ScrollMovement};
use glory::style::{Length, Style};
use sapid::{ScrollHandle, scroll_area, scroller};
use std::{cell::RefCell, rc::Rc};
fn metrics(app: &App, node: glory::render::NodeId) -> ScrollMetrics {
    let result = Rc::new(RefCell::new(None));
    let saved = result.clone();
    app.request(Query::ScrollMetrics(node), move |value| {
        *saved.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::ScrollMetrics(metrics) = result.borrow_mut().take().unwrap() else {
        panic!("metrics")
    };
    metrics
}
#[test]
fn native_scroll_area_four_themes_reads_real_extents_and_applies_ordered_queued_intents() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        let app = host.app();
        let viewport = app.scope().cage(None);
        let input = app.scope().cage(None);
        let output = Rc::new(RefCell::new(None::<ScrollHandle>));
        let saved = output.clone();
        host.mount(
            scroll_area("native")
                .theme(theme)
                .node_ref(viewport)
                .name("Native documents")
                .style(
                    Style::new()
                        .width(Length::Px(300.0))
                        .height(Length::Px(180.0)),
                )
                .on_ready(move |handle| *saved.borrow_mut() = Some(handle))
                .content(move |_| {
                    glory::ui::view()
                        .style(Style::new().height(Length::Px(1000.0)))
                        .fill(glory::ui::input().value("Kept editor").node_ref(input))
                }),
        )
        .unwrap();
        app.settle(16).unwrap();
        let viewport = viewport.get().unwrap();
        let input = input.get().unwrap();
        let engine = host
            .renderer()
            .engine_id(input, glory::html::Part::Main)
            .unwrap();
        let initial = metrics(app, viewport);
        assert!(initial.content_height >= 1000.0);
        assert!(initial.viewport_height <= 180.0);
        let handle = output.borrow().as_ref().unwrap().clone();
        handle.scroll_by(0.0, -100.0).unwrap();
        handle.scroll_by(0.0, 50.0).unwrap();
        app.settle(16).unwrap();
        assert_eq!(metrics(app, viewport).y, 50.0);
        app.dispatch(
            viewport,
            Event::ScrollKeyboard {
                from: viewport,
                command: ScrollCommand {
                    axis: Orientation::Vertical,
                    movement: ScrollMovement::PagePositive,
                },
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(metrics(app, viewport).y, 50.0 + initial.viewport_height);
        assert_eq!(
            host.renderer()
                .engine_id(input, glory::html::Part::Main)
                .unwrap(),
            engine
        );
        app.clear().unwrap();
        assert!(handle.refresh().is_err());
    }
}
#[test]
fn native_scroller_page_buttons_use_real_metrics_and_respect_end_boundaries() {
    let host = WindowApp::new();
    let app = host.app();
    let viewport = app.scope().cage(None);
    host.mount(
        scroller("pages")
            .axis(Orientation::Vertical)
            .area(move |area| {
                area.node_ref(viewport).style(
                    Style::new()
                        .width(Length::Px(250.0))
                        .height(Length::Px(150.0)),
                )
            })
            .content(|_| glory::ui::view().style(Style::new().height(Length::Px(750.0)))),
    )
    .unwrap();
    app.settle(16).unwrap();
    let find = |name: &str| {
        let tree = app.tree();
        let mut ids = tree.roots().to_vec();
        while let Some(id) = ids.pop() {
            let node = tree.node(id).unwrap();
            if node.descriptor.view().name.as_deref() == Some(name) {
                return id;
            }
            ids.extend(node.children.iter().copied());
        }
        panic!("missing control {name}")
    };
    let previous = find("Previous scroll page");
    let next = find("Next scroll page");
    let viewport = viewport.get().unwrap();
    assert!(!app.tree().enabled(previous));
    assert!(app.tree().enabled(next));
    app.dispatch(next, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(metrics(app, viewport).y, 150.0);
    assert!(app.tree().enabled(previous));
    for _ in 0..8 {
        if !app.tree().enabled(next) {
            break;
        }
        app.dispatch(next, Event::Activate).unwrap();
        app.settle(16).unwrap();
    }
    assert_eq!(metrics(app, viewport).y, 600.0);
    assert!(!app.tree().enabled(next));
    app.dispatch(previous, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(metrics(app, viewport).y, 450.0);
    app.clear().unwrap();
}

#[test]
fn scroller_overlay_controls_fit_narrow_viewports_and_fixed_steps_keep_the_editor() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(600, 180, 1.).unwrap();
        let app = host.app();
        let viewport = app.scope().cage(None);
        let editor = app.scope().cage(None);
        host.mount(
            scroller("compact")
                .scroll_step(96.)
                .control_size(28.)
                .area(move |area| {
                    area.theme(theme.clone()).node_ref(viewport).style(
                        Style::new()
                            .width(Length::Percent(50.))
                            .height(Length::Px(140.)),
                    )
                })
                .content(move |_| {
                    glory::ui::view()
                        .style(
                            Style::new()
                                .width(Length::Px(900.))
                                .min_width(Length::Px(900.)),
                        )
                        .fill(glory::ui::input().node_ref(editor).name("Kept draft"))
                }),
        )
        .unwrap();
        app.settle(16).unwrap();
        let viewport = viewport.get().unwrap();
        assert!(
            (metrics(app, viewport).viewport_width - 300.).abs() < 0.2,
            "percentage width resolves once on the frame"
        );
        let editor = editor.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        let root = app.tree().roots()[0];
        let children = app.tree().node(root).unwrap().children.clone();
        let controls = children
            .into_iter()
            .filter(|id| {
                matches!(
                    app.tree().node(*id).unwrap().descriptor.view().props,
                    glory::render::Props::Button(_)
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(controls.len(), 2);
        for control in &controls {
            let reply = Rc::new(RefCell::new(None));
            let out = reply.clone();
            app.request(Query::Bounds(*control), move |value| {
                *out.borrow_mut() = Some(value.unwrap())
            })
            .unwrap();
            app.pump().unwrap();
            let Some(QueryValue::Bounds(rect)) = reply.borrow_mut().take() else {
                panic!("bounds")
            };
            assert!((rect.width - 28.).abs() < 0.2 && (rect.height - 28.).abs() < 0.2);
            assert!(
                rect.x >= 0.
                    && rect.x + rect.width <= 300.1
                    && rect.y >= 0.
                    && rect.y + rect.height <= 140.1
            );
        }
        let next = controls
            .into_iter()
            .find(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some("Next scroll page")
            })
            .unwrap();
        app.dispatch(next, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(metrics(app, viewport).x, 96.);
        assert_eq!(
            host.renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        app.clear().unwrap();
    }
}
