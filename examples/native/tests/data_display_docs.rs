use glory::{
    apace::{Raster, shell::WindowApp},
    prelude::*,
    render::{NodeId, Query, QueryValue, Rect},
    style::{Color, Length},
};
use std::{cell::RefCell, rc::Rc};
fn observe(host: &WindowApp) {
    for _ in 0..4 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}

#[test]
fn overflow_reserves_the_unwrapped_summary_width_instead_of_measuring_its_squeezed_box() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(300, 200, 1.).unwrap();
        let app = host.app();
        let handle = app.scope().cage(None);
        host.mount(
            sapid::overflow_list("natural-summary")
                .theme(theme.clone())
                .overflow_label("More project pages")
                .style(Style::new().width(Length::Px(260.)))
                .initial_visible(1)
                .handle_ref(handle)
                .items(
                    ["Overview", "Settings", "Archive", "Documents"]
                        .into_iter()
                        .map(|name| {
                            let theme = theme.clone();
                            sapid::overflow_item(name)
                                .width(80.)
                                .content(move || sapid::button().theme(theme.clone()).label(name))
                        }),
                ),
        )
        .unwrap();
        for _ in 0..4 {
            observe(&host);
        }
        let trigger = find(app, "sapid-popover-trigger");
        assert_eq!(handle.get().unwrap().visible_count().get(), 0);
        assert!((bounds(app, trigger).height - 36.).abs() < 0.5);
        assert!(bounds(app, trigger).width <= 260.5);
    }
}

#[test]
fn rolling_number_large_type_uses_actual_matching_line_boxes_and_preserves_reels_on_resize() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 160, 1.).unwrap();
    let app = host.app();
    let style = app.scope().cage(Style {
        font_size: Some(38.),
        ..Style::new()
    });
    host.mount(sapid::rolling_number().value(19.5).style(style))
        .unwrap();
    observe(&host);
    let digit = find(app, "sapid-rolling-number-digit");
    let literal = find(app, "sapid-rolling-number-literal");
    let reel = find(app, "sapid-rolling-number-reel");
    let physical = host
        .renderer()
        .engine_id(reel, glory::html::Part::Main)
        .unwrap();
    assert!((bounds(app, digit).height - 58.9).abs() < 0.5);
    assert!((bounds(app, literal).height - 58.9).abs() < 0.5);
    style.revise(|v| v.font_size = Some(24.));
    observe(&host);
    assert!((bounds(app, digit).height - 37.2).abs() < 0.5);
    assert!((bounds(app, literal).height - 37.2).abs() < 0.5);
    assert_eq!(
        host.renderer()
            .engine_id(reel, glory::html::Part::Main)
            .unwrap(),
        physical
    );
}

#[test]
fn avatar_vector_fallback_has_real_clipped_geometry_and_preserves_its_physical_drawing() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(180, 120, 1.).unwrap();
        let app = host.app();
        let style = app.scope().cage(Style::new());
        host.mount(
            sapid::avatar("Ava Patel")
                .theme(theme.clone())
                .size(sapid::Size::ExtraLarge)
                .fallback_icon(sapid::Icon::Info)
                .style(style),
        )
        .unwrap();
        observe(&host);
        let root = find(app, "sapid-avatar");
        let icon = find(app, "sapid-avatar-fallback");
        let outer = bounds(app, root);
        let inner = bounds(app, icon);
        assert_eq!((outer.width, outer.height), (50., 50.));
        assert_eq!((inner.width, inner.height), (30., 30.));
        assert!((inner.x - outer.x - 10.).abs() < 0.5);
        assert!((inner.y - outer.y - 10.).abs() < 0.5);
        let physical = host
            .renderer()
            .engine_id(icon, glory::html::Part::Main)
            .unwrap();
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert!(raster.pixels().as_chunks::<4>().0.iter().any(|px| px[..3]
            == [
                theme.accent_text.0,
                theme.accent_text.1,
                theme.accent_text.2
            ]));
        style.revise(|v| v.color = Some(Color(200, 60, 40, 255)));
        observe(&host);
        assert_eq!(
            host.renderer()
                .engine_id(icon, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        raster.draw(host.renderer()).unwrap();
        assert!(
            raster
                .pixels()
                .as_chunks::<4>()
                .0
                .iter()
                .any(|px| px[..3] == [200, 60, 40])
        );
    }
}
fn find(app: &App, class: &str) -> NodeId {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|v| v == class) {
            return id;
        }
        todo.extend(&node.children);
    }
    panic!("missing {class}")
}
fn bounds(app: &App, node: NodeId) -> Rect {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(Query::Bounds(node), move |v| {
        *saved.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = out.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
#[test]
fn indicator_inset_has_real_geometry_and_its_marker_does_not_block_the_underlying_button() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(240, 160, 1.).unwrap();
        let app = host.app();
        let button = app.scope().cage(None);
        let clicks = app.scope().cage(0usize);
        host.mount(
            sapid::indicator()
                .theme(theme.clone())
                .diameter(20.)
                .offset(10.)
                .label("3")
                .color(Color(255, 225, 65, 255))
                .fill(
                    sapid::button()
                        .theme(theme.clone())
                        .node_ref(button)
                        .label("Inbox")
                        .style(Style::new().width(Length::Px(100.)).height(Length::Px(40.)))
                        .on_press(move || clicks.revise(|v| *v += 1)),
                ),
        )
        .unwrap();
        observe(&host);
        let button = button.get().unwrap();
        let marker = find(app, "sapid-indicator-marker");
        let parent = bounds(app, button);
        let rect = bounds(app, marker);
        assert!((rect.x + rect.width / 2. - parent.x - parent.width + 10.).abs() < 1.);
        assert!((rect.y + rect.height / 2. - parent.y - 10.).abs() < 1.);
        let physical = host
            .renderer()
            .engine_id(button, glory::html::Part::Main)
            .unwrap();
        assert_eq!(
            host.renderer().inspect(|doc| doc
                .hit(
                    (rect.x + rect.width / 2.) as f32,
                    (rect.y + rect.height / 2.) as f32
                )
                .unwrap()
                .node_id),
            physical
        );
        app.dispatch(button, glory::render::Event::Activate)
            .unwrap();
        assert_eq!(clicks.get(), 1);
    }
}
#[test]
fn description_pairs_have_real_horizontal_and_vertical_layout_and_wrap_values_in_all_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for orientation in [
            sapid::DataListOrientation::Horizontal,
            sapid::DataListOrientation::Vertical,
        ] {
            let host = WindowApp::new();
            host.renderer().set_viewport(280, 300, 1.).unwrap();
            let app = host.app();
            let value=app.scope().cage("The project workspace contains shared notes and planning documents for the whole product team.".to_owned());
            host.mount(
                sapid::data_list()
                    .theme(theme.clone())
                    .orientation(orientation)
                    .label_width(96.)
                    .gap(8.)
                    .item(
                        sapid::data_list_item()
                            .term("Project summary")
                            .fill(glory::ui::text().content(value)),
                    ),
            )
            .unwrap();
            observe(&host);
            let term = bounds(app, find(app, "sapid-data-list-term"));
            let definition = find(app, "sapid-data-list-definition");
            let rect = bounds(app, definition);
            assert!(rect.height > 30.);
            assert!(rect.x + rect.width <= 281.);
            if orientation == sapid::DataListOrientation::Horizontal {
                assert!((rect.x - term.x - 104.).abs() < 1.);
                assert!((rect.y - term.y).abs() < 1.);
            } else {
                assert!((rect.x - term.x).abs() < 1.);
                assert!(rect.y >= term.y + term.height + 7.);
            }
            let physical = host
                .renderer()
                .engine_id(definition, glory::html::Part::Main)
                .unwrap();
            value.revise(|v| *v = "Updated shared project summary".to_owned());
            observe(&host);
            assert_eq!(
                host.renderer()
                    .engine_id(definition, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
        }
    }
}
#[test]
fn badge_dot_paints_actual_color_and_icon_size_changes_retain_physical_graphics() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 160, 1.).unwrap();
        let app = host.app();
        let extent = app.scope().cage(18.);
        host.mount(
            sapid::stack().theme(theme.clone()).gap(12.).fill((
                sapid::badge()
                    .theme(theme.clone())
                    .variant(sapid::BadgeVariant::Dot)
                    .color(Color(200, 60, 40, 255))
                    .content("Available"),
                sapid::theme_icon(sapid::Icon::Check)
                    .theme(theme.clone())
                    .icon_size(extent),
            )),
        )
        .unwrap();
        observe(&host);
        let dot = bounds(app, find(app, "sapid-badge-dot-marker"));
        assert_eq!((dot.width, dot.height), (6., 6.));
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let width = raster.frame().unwrap().width as usize;
        let offset = (((dot.y + dot.height / 2.).round() as usize) * width
            + (dot.x + dot.width / 2.).round() as usize)
            * 4;
        assert_eq!(&raster.pixels()[offset..offset + 4], &[200, 60, 40, 255]);
        let graphic = find(app, "sapid-theme-icon-graphic");
        let physical = host
            .renderer()
            .engine_id(graphic, glory::html::Part::Main)
            .unwrap();
        extent.revise(|v| *v = 28.);
        observe(&host);
        assert_eq!(bounds(app, graphic).width, 28.);
        assert_eq!(
            host.renderer()
                .engine_id(graphic, glory::html::Part::Main)
                .unwrap(),
            physical
        );
    }
}
