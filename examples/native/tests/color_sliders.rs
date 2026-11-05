use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{Action, Event, NodeId, Query, QueryValue, RangeCommand, Rect};
use glory::style::Color;
use std::{cell::RefCell, rc::Rc};
fn bounds(app: &App, node: NodeId) -> Rect {
    let out = Rc::new(RefCell::new(None));
    let result = out.clone();
    app.request(Query::Bounds(node), move |value| {
        *result.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = out.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
fn classes(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut ids = Vec::new();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            ids.push(id)
        }
        pending.extend(&node.children)
    }
    ids
}
fn pixel(raster: &Raster, x: f64, y: f64) -> [u8; 4] {
    let offset = (y as usize * raster.frame().unwrap().width as usize + x as usize) * 4;
    raster.pixels()[offset..offset + 4].try_into().unwrap()
}
fn close(actual: [u8; 4], expected: [u8; 4]) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!(a.abs_diff(b) <= 2, "{actual:?} != {expected:?}")
    }
}
#[test]
fn native_hue_and_alpha_four_themes_have_real_color_pixels_and_shared_control_identity() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let checker = theme.checker_dark;
        let host = WindowApp::new();
        let app = host.app();
        let hue = app.scope().cage(None);
        let alpha = app.scope().cage(None);
        let color = app.scope().cage(Color(100, 150, 200, 255));
        host.mount(
            sapid::stack().gap(16.).fill((
                sapid::hue_slider("hue")
                    .theme(theme.clone())
                    .default_value(180.)
                    .node_ref(hue),
                sapid::alpha_slider("alpha")
                    .theme(theme)
                    .color(color)
                    .default_value(0.5)
                    .node_ref(alpha),
            )),
        )
        .unwrap();
        app.settle(16).unwrap();
        let id = hue.get().unwrap();
        let identity = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        app.scope().action(Action::Focus(id)).unwrap();
        app.flush().unwrap();
        app.dispatch(
            id,
            Event::Range {
                from: id,
                command: RangeCommand::Increase,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(
            app.tree()
                .node(id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .slider
                .as_ref()
                .unwrap()
                .range
                .value,
            181.
        );
        let mut overlays = classes(app, "sapid-color-slider-overlay");
        overlays.sort_by(|a, b| {
            let ar = bounds(app, *a);
            let br = bounds(app, *b);
            ar.y.partial_cmp(&br.y)
                .unwrap()
                .then_with(|| ar.x.partial_cmp(&br.x).unwrap())
        });
        assert_eq!(overlays.len(), 2);
        assert!(classes(app, "sapid-color-slider-segment").is_empty());
        assert_eq!(classes(app, "sapid-alpha-slider-background").len(), 1);
        assert!(classes(app, "sapid-alpha-checker-cell").is_empty());
        let hue_rect = bounds(app, overlays[0]);
        let alpha_rect = bounds(app, overlays[1]);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        close(
            pixel(&raster, hue_rect.x + 0.1, hue_rect.y + 0.5),
            [255, 3, 0, 255],
        );
        close(
            pixel(&raster, alpha_rect.x + 1., alpha_rect.y + 0.5),
            [checker.0, checker.1, checker.2, checker.3],
        );
        close(
            pixel(
                &raster,
                alpha_rect.x + alpha_rect.width - 2.,
                alpha_rect.y + 0.5,
            ),
            [100, 150, 200, 255],
        );
        color.revise(|old| *old = Color(200, 50, 100, 3));
        app.settle(16).unwrap();
        raster.draw(host.renderer()).unwrap();
        close(
            pixel(
                &raster,
                alpha_rect.x + alpha_rect.width - 2.,
                alpha_rect.y + 0.5,
            ),
            [200, 50, 100, 255],
        );
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        let rect = bounds(app, alpha.get().unwrap());
        assert!(rect.width > 0. && rect.height > 0.);
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.stats().queries, 0);
    }
}

#[test]
fn alpha_background_alternates_in_both_axes_instead_of_a_flat_gray_gradient() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        let dark = theme.checker_dark;
        let light = theme.checker_light;
        host.mount(
            sapid::alpha_slider("checker-pixels")
                .theme(theme)
                .color(light)
                .default_value(1.)
                .style(Style::new().width(Length::Px(240.))),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let layer = classes(app, "sapid-alpha-slider-background")[0];
        let rect = bounds(app, layer);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        close(
            pixel(&raster, rect.x + 2., rect.y + 1.),
            [dark.0, dark.1, dark.2, 255],
        );
        close(
            pixel(&raster, rect.x + 2., rect.y + 3.),
            [light.0, light.1, light.2, 255],
        );
        close(
            pixel(&raster, rect.x + 7., rect.y + 1.),
            [light.0, light.1, light.2, 255],
        );
        close(
            pixel(&raster, rect.x + 7., rect.y + 3.),
            [dark.0, dark.1, dark.2, 255],
        );
    }
}

#[test]
fn hue_middle_represents_180_degree_cyan() {
    let host = WindowApp::new();
    host.mount(
        sapid::hue_slider("cyan-middle")
            .default_value(0.)
            .style(Style::new().width(Length::Px(240.))),
    )
    .unwrap();
    let app = host.app();
    app.settle(16).unwrap();
    let rect = bounds(app, classes(app, "sapid-color-slider-overlay")[0]);
    let mut raster = Raster::new();
    raster.draw(host.renderer()).unwrap();
    let actual = pixel(&raster, rect.x + rect.width / 2., rect.y + 0.5);
    for (actual, expected) in actual.into_iter().zip([0, 255, 255, 255]) {
        assert!(
            actual.abs_diff(expected) <= 4,
            "midpoint must represent cyan, not 170-degree green-cyan"
        );
    }
}
