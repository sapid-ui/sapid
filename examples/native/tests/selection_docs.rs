use glory::{
    apace::{Raster, shell::WindowApp},
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/showcase_choice_docs/segments_layout.rs"]
mod layouts;
#[path = "../../web/src/showcase_choice_docs/rating_symbols.rs"]
mod symbols;
#[path = "../../web/src/showcase_choice_docs/segments_width.rs"]
mod widths;
fn themes() -> [sapid::Theme; 4] {
    [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ]
}
fn nodes(app: &App, predicate: impl Fn(glory::render::DescriptorView<'_>) -> bool) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending: Vec<_> = tree.roots().iter().rev().copied().collect();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if predicate(node.descriptor.view()) {
            result.push(id);
        }
        pending.extend(node.children.iter().rev().copied());
    }
    result
}
fn named(app: &App, name: &str) -> NodeId {
    nodes(
        app,
        |view| matches!(view.props,Props::Check(p) if p.label==name),
    )[0]
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    app.request(Query::Bounds(id), move |v| {
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
fn native_segments_switch_between_content_and_equal_width_without_replacing_members() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(widths::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let ids = ["Day", "Two weeks", "Calendar month"].map(|label| named(app, label));
        let identity = host
            .renderer()
            .engine_id(ids[0], glory::html::Part::Control)
            .unwrap();
        let before = ids.map(|id| bounds(app, id));
        assert!(before[2].width > before[0].width + 10., "{before:?}");
        let toggle = named(app, "Use equal full-width segments");
        app.dispatch(toggle, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        let after = ids.map(|id| bounds(app, id));
        assert!(
            after
                .iter()
                .all(|r| (r.width - after[0].width).abs() < 1. && r.x + r.width <= 321.),
            "{after:?}"
        );
        assert!(after[0].width > 80.);
        app.dispatch(toggle, Event::CheckedChanged(false)).unwrap();
        app.settle(16).unwrap();
        assert!(bounds(app, ids[2]).width > bounds(app, ids[0]).width + 10.);
        assert_eq!(
            host.renderer()
                .engine_id(ids[0], glory::html::Part::Control)
                .unwrap(),
            identity
        );
    }
}
#[test]
fn native_vertical_and_long_label_segments_fit_their_parent_and_keep_disabled_policy() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(layouts::demo(theme)),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let daily = bounds(app, named(app, "Daily"));
        let weekly = bounds(app, named(app, "Weekly"));
        assert!(
            (daily.width - weekly.width).abs() < 1. && weekly.y >= daily.y + daily.height,
            "{daily:?} {weekly:?}"
        );
        assert!(
            !app.dispatch(named(app, "Monthly"), Event::CheckedChanged(true))
                .unwrap()
        );
        let rects = ["Team settings", "Project reports", "Access control"]
            .map(|label| bounds(app, named(app, label)));
        assert!(
            rects.iter().all(|r| (r.width - rects[0].width).abs() < 1.
                && r.x + r.width <= 321.
                && r.height < 100.),
            "{rects:?}"
        );
    }
}
#[test]
fn native_custom_half_rating_paints_declared_symbols_and_reuses_the_same_graphics() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(symbols::demo(theme.clone())).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let graphics = nodes(app, |view| {
            view.classes.iter().any(|c| c == "sapid-rating-star")
        });
        assert_eq!(graphics.len(), 6);
        let last = *graphics.last().unwrap();
        let identity = host
            .renderer()
            .engine_id(last, glory::html::Part::Main)
            .unwrap();
        let rect = bounds(app, last);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let sample = |raster: &Raster| {
            let offset = ((rect.y + rect.height * 0.5) as usize
                * raster.frame().unwrap().width as usize
                + (rect.x + rect.width * 0.7) as usize)
                * 4;
            <[u8; 4]>::try_from(&raster.pixels()[offset..offset + 4]).unwrap()
        };
        assert_eq!(
            sample(&raster),
            [
                theme.surface.0,
                theme.surface.1,
                theme.surface.2,
                theme.surface.3
            ]
        );
        let members = nodes(app, |view| matches!(view.props, Props::Check(_)));
        app.dispatch(*members.last().unwrap(), Event::CheckedChanged(true))
            .unwrap();
        app.settle(16).unwrap();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            sample(&raster),
            [
                theme.accent.0,
                theme.accent.1,
                theme.accent.2,
                theme.accent.3
            ]
        );
        assert_eq!(
            host.renderer()
                .engine_id(last, glory::html::Part::Main)
                .unwrap(),
            identity
        );
    }
}

#[test]
fn native_linear_thumbs_use_the_control_contrast_color_in_light_and_dark_themes() {
    for theme in themes() {
        let host = WindowApp::new();
        host.mount(
            glory::ui::view()
                .style(Style {
                    background: Some(theme.surface),
                    ..Style::new()
                        .width(Length::Px(320.))
                        .padding(Edges::all(16.))
                })
                .fill(
                    sapid::stack().theme(theme.clone()).gap(24.).fill((
                        sapid::slider("contrast-scalar")
                            .theme(theme.clone())
                            .default_value(50.),
                        sapid::range_slider("contrast-pair")
                            .theme(theme.clone())
                            .default_value([25., 75.]),
                    )),
                ),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let thumbs = nodes(app, |view| {
            view.classes
                .iter()
                .any(|c| c == "sapid-slider-thumb" || c == "sapid-range-slider-thumb")
        });
        assert_eq!(thumbs.len(), 3);
        let rects: Vec<_> = thumbs.iter().map(|id| bounds(app, *id)).collect();
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        for rect in rects {
            let offset = ((rect.y + rect.height / 2.) as usize
                * raster.frame().unwrap().width as usize
                + (rect.x + rect.width / 2.) as usize)
                * 4;
            assert_eq!(
                &raster.pixels()[offset..offset + 4],
                &[
                    theme.control_thumb.0,
                    theme.control_thumb.1,
                    theme.control_thumb.2,
                    theme.control_thumb.3
                ]
            );
        }
    }
}
