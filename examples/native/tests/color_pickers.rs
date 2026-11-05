use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{Action, DragPhase, Event, NodeId, Query, QueryValue, RangeCommand, Rect};
use std::{cell::RefCell, rc::Rc};
fn bounds(app: &App, id: NodeId) -> Rect {
    let out = Rc::new(RefCell::new(None));
    let seen = out.clone();
    app.request(Query::Bounds(id), move |value| {
        *seen.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = out.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            out.push(id);
        }
        todo.extend(&node.children);
    }
    out
}
fn pixel(raster: &Raster, x: f64, y: f64) -> [u8; 4] {
    let offset = (y as usize * raster.frame().unwrap().width as usize + x as usize) * 4;
    raster.pixels()[offset..offset + 4].try_into().unwrap()
}
#[test]
fn native_palette_columns_fit_the_real_popup_and_shrink_without_replacing_swatches() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(
            sapid::color_input("palette-columns")
                .theme(theme.clone())
                .swatches(["#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf"])
                .swatches_per_row(5)
                .default_open(true),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let mut squares: Vec<_> = nodes(app, "sapid-color-swatch")
            .into_iter()
            .filter(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    .is_some_and(|name| name.starts_with('#'))
            })
            .map(|id| bounds(app, id))
            .collect();
        squares.sort_by(|a, b| a.x.total_cmp(&b.x));
        assert_eq!(squares.len(), 5);
        assert!(
            squares
                .iter()
                .all(|rect| (rect.y - squares[0].y).abs() < 1.),
            "{squares:?}"
        );
        let popup = bounds(app, nodes(app, "sapid-popover")[0]);
        assert!(
            squares.last().unwrap().x + squares.last().unwrap().width
                <= popup.x + popup.width - 10.
        );
        app.clear().unwrap();
        let width = app.scope().cage(420.);
        host.mount(
            glory::ui::view()
                .style(width.map(|v| Style::new().width(Length::Px(*v))))
                .fill(
                    sapid::color_picker("responsive-palette")
                        .theme(theme)
                        .show_picker(false)
                        .full_width(true)
                        .plane_size(420.)
                        .swatches([
                            "#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf", "#12b886",
                            "#fd7e14",
                        ])
                        .swatches_per_row(7),
                ),
        )
        .unwrap();
        app.settle(16).unwrap();
        let ids = nodes(app, "sapid-color-picker-swatch");
        let identity = host
            .renderer()
            .engine_id(ids[0], glory::html::Part::Main)
            .unwrap();
        width.revise(|v| *v = 280.);
        app.settle(16).unwrap();
        let rects: Vec<_> = ids.iter().map(|id| bounds(app, *id)).collect();
        assert!(
            rects.iter().all(|r| (r.y - rects[0].y).abs() < 1.
                && (r.width - r.height).abs() < 1.
                && r.x + r.width <= 281.),
            "{rects:?}"
        );
        assert_eq!(
            host.renderer()
                .engine_id(ids[0], glory::html::Part::Main)
                .unwrap(),
            identity
        );
    }
}
#[test]
fn native_swatch_close_returns_to_the_same_editor_and_the_inset_action_can_reopen() {
    for (theme, with_picker) in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ]
    .into_iter()
    .flat_map(|theme| [true, false].map(move |picker| (theme.clone(), picker)))
    {
        let host = WindowApp::new();
        let app = host.app();
        let editor = app.scope().cage(None);
        let opened = app.scope().cage(false);
        host.mount(
            sapid::color_input("swatch-focus")
                .theme(theme)
                .show_picker(with_picker)
                .open(opened)
                .on_open_change(move |value| opened.revise(|old| *old = value))
                .default_value("#228be6")
                .swatches(["#fa5252"])
                .close_on_color_swatch_click(true)
                .node_ref(editor),
        )
        .unwrap();
        app.settle(16).unwrap();
        let editor = editor.get().unwrap();
        let identity = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        app.scope().action(Action::Focus(editor)).unwrap();
        app.settle(16).unwrap();
        assert_eq!(
            nodes(app, "sapid-color-plane").len(),
            usize::from(with_picker)
        );
        let swatch = nodes(app, "sapid-color-swatch")
            .into_iter()
            .find(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some("#fa5252")
            })
            .unwrap();
        app.scope().action(Action::Focus(swatch)).unwrap();
        app.settle(16).unwrap();
        app.dispatch(swatch, Event::Activate).unwrap();
        app.settle(16).unwrap();
        let panel = nodes(app, "sapid-popover")[0];
        assert!(
            app.tree()
                .node(panel)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        let focus = Rc::new(RefCell::new(None));
        let out = focus.clone();
        app.request(Query::Focus(editor), move |value| {
            *out.borrow_mut() = Some(value.unwrap())
        })
        .unwrap();
        app.pump().unwrap();
        assert_eq!(*focus.borrow(), Some(QueryValue::Focus(true)));
        assert_eq!(
            host.renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        let preview = nodes(app, "sapid-color-input-preview")[0];
        let frame = bounds(app, editor);
        let action = bounds(app, preview);
        assert!(
            action.width <= 22.
                && action.height <= 22.
                && action.x >= frame.x
                && action.x + action.width <= frame.x + frame.width
                && action.y >= frame.y
                && action.y + action.height <= frame.y + frame.height
        );
        app.dispatch(preview, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert!(
            !app.tree()
                .node(panel)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn native_picker_four_themes_has_real_plane_pixels_two_axis_mapping_focus_and_stable_identity() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        let app = host.app();
        let source = app.scope().cage(sapid::Hsva {
            hue: 120.,
            saturation: 0.5,
            value: 0.5,
            alpha: 0.4,
        });
        let editor = app.scope().cage(None);
        host.mount(
            sapid::stack().fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Preserved notes")
                    .node_ref(editor)
                    .default_value("caret"),
                sapid::color_picker("native")
                    .theme(theme)
                    .value(source)
                    .on_value_change(move |next| source.revise(|value| *value = next)),
            )),
        )
        .unwrap();
        app.settle(16).unwrap();
        let plane = nodes(app, "sapid-color-plane")[0];
        let identity = host
            .renderer()
            .engine_id(plane, glory::html::Part::Main)
            .unwrap();
        let rect = bounds(app, plane);
        assert_eq!((rect.width, rect.height), (240., 240.));
        let indicator = bounds(app, nodes(app, "sapid-color-plane-indicator")[0]);
        assert!((indicator.x + indicator.width / 2. - (rect.x + rect.width * 0.5)).abs() < 1.);
        assert!((indicator.y + indicator.height / 2. - (rect.y + rect.height * 0.5)).abs() < 1.);
        let hue_layer = nodes(app, "sapid-color-plane-hue")[0];
        let initial_hue = app
            .tree()
            .node(hue_layer)
            .unwrap()
            .descriptor
            .view()
            .style
            .background;
        assert_eq!(nodes(app, "sapid-color-plane-saturation").len(), 1);
        assert_eq!(nodes(app, "sapid-color-plane-value").len(), 1);
        assert!(nodes(app, "sapid-color-plane-cell").is_empty());
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let white = pixel(&raster, rect.x + 3., rect.y + 3.);
        let green = pixel(&raster, rect.x + rect.width - 3., rect.y + 3.);
        let black = pixel(&raster, rect.x + rect.width / 2., rect.y + rect.height - 3.);
        assert!(white[0] > 240 && white[1] > 240 && white[2] > 240);
        assert!(green[1] > 240 && green[0] < 20 && green[2] < 20);
        assert!(black[0] < 20 && black[1] < 20 && black[2] < 20);
        let editor = editor.get().unwrap();
        app.scope().action(Action::Focus(editor)).unwrap();
        app.flush().unwrap();
        app.dispatch(
            plane,
            Event::Drag {
                from: plane,
                phase: DragPhase::Start,
                x: rect.x + rect.width * 0.25,
                y: rect.y + rect.height * 0.75,
            },
        )
        .unwrap();
        app.dispatch(
            plane,
            Event::Drag {
                from: plane,
                phase: DragPhase::Drop,
                x: rect.x + rect.width + 100.,
                y: rect.y - 100.,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(source.get().saturation, 1.);
        assert_eq!(source.get().value, 1.);
        assert_eq!(source.get().alpha, 0.4);
        assert_eq!(app.stats().queries, 0);
        let hue = nodes(app, "sapid-hue-slider")[0];
        app.dispatch(
            hue,
            Event::Range {
                from: hue,
                command: RangeCommand::Set(240.),
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        raster.draw(host.renderer()).unwrap();
        let now = app
            .tree()
            .node(hue_layer)
            .unwrap()
            .descriptor
            .view()
            .style
            .background;
        assert_ne!(now, initial_hue);
        assert_eq!(
            host.renderer()
                .engine_id(plane, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.stats().queries, 0);
        host.mount(sapid::color_picker("small").plane_size(128.))
            .unwrap();
        app.settle(16).unwrap();
        for id in nodes(app, "sapid-slider") {
            assert!((bounds(app, id).width - 128.).abs() < 1.);
        }
        app.clear().unwrap();
    }
}
#[test]
fn native_color_input_preserves_hex_editor_identity_and_error_metadata() {
    let host = WindowApp::new();
    let app = host.app();
    let value = app.scope().cage("#bad?".to_owned());
    host.mount(
        sapid::field("color")
            .label("Accent color")
            .required(true)
            .form_name("accent")
            .control(move |field| {
                sapid::color_input("hex")
                    .format(sapid::ColorFormat::Hexa)
                    .show_picker(false)
                    .field(field)
                    .value(value)
            }),
    )
    .unwrap();
    app.settle(16).unwrap();
    let id = nodes(app, "sapid-color-editor")[0];
    let identity = host
        .renderer()
        .engine_id(id, glory::html::Part::Main)
        .unwrap();
    assert!(
        app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .invalid
    );
    assert!(bounds(app, id).height > 20.);
    value.revise(|value| *value = "#1234".into());
    app.settle(16).unwrap();
    assert!(
        !app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .invalid
    );
    assert_eq!(
        host.renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap(),
        identity
    );
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn native_default_color_popup_has_real_anchor_geometry_preserves_editor_and_returns_focus() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(800, 700, 1.).unwrap();
        let app = host.app();
        let value = app.scope().cage("#ff0000".to_owned());
        host.mount(
            sapid::color_input("popup")
                .theme(theme)
                .value(value)
                .on_value_change(move |v| value.revise(|old| *old = v)),
        )
        .unwrap();
        app.settle(16).unwrap();
        let editor = nodes(app, "sapid-color-editor")[0];
        let trigger = editor;
        assert!(nodes(app, "sapid-popover-trigger").is_empty());
        let identity = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        app.scope().action(Action::Focus(trigger)).unwrap();
        app.settle(16).unwrap();
        let panel = nodes(app, "sapid-popover")[0];
        let anchor = bounds(app, trigger);
        let pop = bounds(app, panel);
        assert!(pop.width > 200. && pop.height > 150.);
        assert!(
            pop.x >= 0.
                && pop.y >= 0.
                && pop.x + pop.width <= 800. + 1.
                && pop.y + pop.height <= 700. + 1.
        );
        assert!(pop.y >= anchor.y + anchor.height - 1. || pop.y + pop.height <= anchor.y + 1.);
        let hue = nodes(app, "sapid-hue-slider")[0];
        app.dispatch(
            hue,
            Event::Range {
                from: hue,
                command: RangeCommand::Set(120.),
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_ne!(value.get(), "#ff0000");
        assert_eq!(
            host.renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert!(raster.pixels().as_chunks::<4>().0.iter().any(|p| p[3] > 0));
        app.dispatch(panel, Event::Dismiss(glory::render::DismissReason::Escape))
            .unwrap();
        app.settle(16).unwrap();
        assert!(
            app.tree()
                .node(panel)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        let focused = Rc::new(RefCell::new(None));
        let out = focused.clone();
        app.request(Query::Focus(editor), move |reply| {
            *out.borrow_mut() = Some(reply.unwrap())
        })
        .unwrap();
        app.pump().unwrap();
        assert_eq!(*focused.borrow(), Some(QueryValue::Focus(true)));
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.stats().queries, 0);
    }
}
