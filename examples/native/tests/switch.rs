use apace_dom::Document;
use apace_traits::events::{
    ApacePointerEvent, ApacePointerId, MouseEventButton, MouseEventButtons, Point, PointerCoords,
    UiEvent,
};
use glory::{
    apace::{Raster, shell::WindowApp},
    prelude::*,
    render::{Event, Query, QueryValue},
};
use std::task::{Context, Waker};
use std::{cell::RefCell, rc::Rc};
#[test]
fn native_switch_paints_a_track_and_moves_its_thumb_without_replacing_the_control() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        let app = host.app();
        let reference = app.scope().cage(None);
        host.mount(
            sapid::switch()
                .theme(theme.clone())
                .name("Native switch")
                .style(Style::new().width(Length::Px(34.)).height(Length::Px(20.)))
                .node_ref(reference),
        )
        .unwrap();
        app.settle(16).unwrap();
        let node = reference.get().unwrap();
        let identity = host
            .renderer()
            .engine_id(node, glory::html::Part::Control)
            .unwrap();
        let saved = Rc::new(RefCell::new(None));
        let output = saved.clone();
        app.request(Query::Bounds(node), move |v| {
            *output.borrow_mut() = Some(v.unwrap())
        })
        .unwrap();
        app.pump().unwrap();
        let QueryValue::Bounds(rect) = saved.borrow_mut().take().unwrap() else {
            panic!("bounds")
        };
        let sample = |raster: &Raster, x: f64| {
            let offset = ((rect.y + 10.) as usize * raster.frame().unwrap().width as usize
                + (rect.x + x) as usize)
                * 4;
            <[u8; 4]>::try_from(&raster.pixels()[offset..offset + 4]).unwrap()
        };
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            sample(&raster, 30.),
            [
                theme.border.0,
                theme.border.1,
                theme.border.2,
                theme.border.3
            ]
        );
        app.dispatch(node, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            sample(&raster, 4.),
            [
                theme.accent.0,
                theme.accent.1,
                theme.accent.2,
                theme.accent.3
            ]
        );
        assert_eq!(
            host.renderer()
                .engine_id(node, glory::html::Part::Control)
                .unwrap(),
            identity
        );
    }
}

#[test]
fn queued_native_pointer_on_the_track_activates_the_real_switch_and_keeps_focus() {
    let mut host = WindowApp::new();
    let value = host.app().scope().cage(false);
    let reference = host.app().scope().cage(None);
    host.mount(
        sapid::switch()
            .name("Pointer switch")
            .with_marks(true)
            .checked(value)
            .node_ref(reference)
            .style(Style::new().width(Length::Px(34.)).height(Length::Px(20.)))
            .on_checked_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    host.app().settle(16).unwrap();
    let node = reference.get().unwrap();
    let main = host
        .renderer()
        .engine_id(node, glory::html::Part::Main)
        .unwrap();
    let rect = host
        .renderer()
        .inspect(|doc| doc.get_client_bounding_rect(main).unwrap());
    let x = (rect.x + 30.) as f32;
    let y = (rect.y + 10.) as f32;
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
    for _ in 0..8 {
        host.observe(&Context::from_waker(Waker::noop())).unwrap();
    }
    assert!(value.get());
    let physical = host
        .renderer()
        .engine_id(node, glory::html::Part::Control)
        .unwrap();
    assert!(host.renderer().inspect(|doc| {
        doc.get_node(physical)
            .unwrap()
            .element_data()
            .unwrap()
            .checkbox_input_checked()
            .unwrap()
    }));
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    host.app()
        .request(Query::Focus(node), move |v| {
            *out.borrow_mut() = Some(v.unwrap())
        })
        .unwrap();
    host.app().pump().unwrap();
    assert_eq!(*saved.borrow(), Some(QueryValue::Focus(true)));
}

fn class_node(app: &App, class: &str) -> glory::render::NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            return id;
        }
        pending.extend(node.children.iter().copied());
    }
    panic!("missing {class}")
}
fn node_bounds(app: &App, node: glory::render::NodeId) -> glory::render::Rect {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    app.request(Query::Bounds(node), move |v| {
        *out.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = saved.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
#[test]
fn native_switch_sizes_marks_and_label_positions_retain_the_same_editor_when_caption_changes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for position in [
            sapid::SwitchLabelPosition::Before,
            sapid::SwitchLabelPosition::After,
        ] {
            for (size, width, height) in [
                (sapid::Size::ExtraSmall, 26., 14.),
                (sapid::Size::Small, 30., 16.),
                (sapid::Size::Medium, 34., 20.),
                (sapid::Size::Large, 42., 24.),
                (sapid::Size::ExtraLarge, 50., 28.),
            ] {
                let host = WindowApp::new();
                let app = host.app();
                let caption = app.scope().cage("Updates".to_owned());
                let reference = app.scope().cage(None);
                host.mount(
                    sapid::switch()
                        .theme(theme.clone())
                        .size(size)
                        .with_marks(true)
                        .label_position(position)
                        .label(caption)
                        .node_ref(reference)
                        .style(Style::new().width(Length::Px(240.))),
                )
                .unwrap();
                app.settle(16).unwrap();
                let node = reference.get().unwrap();
                let identity = host
                    .renderer()
                    .engine_id(node, glory::html::Part::Control)
                    .unwrap();
                let track = node_bounds(app, class_node(app, "sapid-switch-track"));
                let label = node_bounds(app, class_node(app, "sapid-switch-label"));
                assert_eq!((track.width, track.height), (width, height));
                if position == sapid::SwitchLabelPosition::Before {
                    assert!(label.x + label.width <= track.x);
                } else {
                    assert!(track.x + track.width <= label.x);
                }
                app.dispatch(node, Event::CheckedChanged(true)).unwrap();
                caption.revise(|v| *v = "New updates".into());
                app.settle(16).unwrap();
                assert_eq!(
                    app.tree()
                        .node(node)
                        .unwrap()
                        .descriptor
                        .view()
                        .name
                        .as_deref(),
                    Some("New updates")
                );
                caption.revise(String::clear);
                app.settle(16).unwrap();
                let label = class_node(app, "sapid-switch-label");
                assert!(
                    app.tree()
                        .node(label)
                        .unwrap()
                        .descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .unwrap()
                        .hidden
                );
                let track = node_bounds(app, class_node(app, "sapid-switch-track"));
                assert!((track.x - node_bounds(app, node).x).abs() < 1.);
                assert_eq!(
                    host.renderer()
                        .engine_id(node, glory::html::Part::Control)
                        .unwrap(),
                    identity
                );
            }
        }
    }
}
