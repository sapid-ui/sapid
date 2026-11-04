#[path = "../../web/src/progress_form.rs"]
mod progress_form;
use glory::apace::Raster;
use glory::apace::shell::WindowApp;
use glory::render::{Event, NodeId, Props, Query, QueryValue, Rect};
use glory::ui::App;
use std::{cell::RefCell, rc::Rc};
fn find(app: &App, class: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node
            .descriptor
            .view()
            .classes
            .iter()
            .any(|value| value == class)
        {
            return id;
        }
        pending.extend(&node.children);
    }
    panic!("missing {class}")
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let result = Rc::new(RefCell::new(None));
    let reply = result.clone();
    app.request(Query::Bounds(id), move |value| {
        *reply.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = result.borrow_mut().take().unwrap() else {
        panic!()
    };
    rect
}
fn pixel(raster: &Raster, x: f64, y: f64) -> [u8; 4] {
    let width = raster.frame().unwrap().width as usize;
    let offset = (y.round() as usize * width + x.round() as usize) * 4;
    raster.pixels()[offset..offset + 4].try_into().unwrap()
}
fn rgba(color: glory::style::Color) -> [u8; 4] {
    [color.0, color.1, color.2, color.3]
}
#[test]
fn native_progress_has_real_fraction_geometry_circular_widgets_and_stable_updates_in_all_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(progress_form::progress_form(theme.clone()))
            .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let line = find(app, "progress-linear");
        let fill = app.tree().node(line).unwrap().children[0];
        let ring = find(app, "progress-ring");
        let arc = app.tree().node(ring).unwrap().children[1];
        let semi = find(app, "progress-semicircle");
        assert!((bounds(app, line).width - 240.).abs() < 0.1);
        assert!((bounds(app, fill).width - 60.).abs() < 0.1);
        assert!((bounds(app, ring).width - 100.).abs() < 0.1);
        assert!((bounds(app, semi).height - 66.).abs() < 0.1);
        let physical = host
            .renderer()
            .engine_id(arc, glory::html::Part::Main)
            .unwrap();
        host.renderer().inspect(|doc| {
            assert!(
                doc.get_node(physical)
                    .unwrap()
                    .element_data()
                    .unwrap()
                    .custom_widget_data()
                    .is_some()
            )
        });
        let ring_box = bounds(app, ring);
        let semi_box = bounds(app, semi);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            pixel(&raster, ring_box.x + 82., ring_box.y + 18.),
            rgba(theme.accent)
        );
        assert_eq!(
            pixel(&raster, ring_box.x + 50., ring_box.y + 95.),
            rgba(theme.border)
        );
        assert_eq!(
            pixel(&raster, semi_box.x + 10., semi_box.y + 39.),
            rgba(theme.accent)
        );
        assert_eq!(
            pixel(&raster, semi_box.x + 98., semi_box.y + 22.),
            rgba(theme.border)
        );
        let root = app.tree().roots()[0];
        let advance = app.tree().node(root).unwrap().children[3];
        app.dispatch(advance, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert!((bounds(app, fill).width - 180.).abs() < 0.1);
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            pixel(&raster, ring_box.x + 50., ring_box.y + 95.),
            rgba(theme.accent)
        );
        assert_eq!(
            pixel(&raster, semi_box.x + 60., semi_box.y + 6.),
            rgba(theme.accent)
        );
        assert!(
            matches!(app.tree().node(arc).unwrap().descriptor.view().props,Props::Graphic(props) if props.commands.len()==4)
        );
        assert_eq!(
            host.renderer()
                .engine_id(arc, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        let toggle = app.tree().node(root).unwrap().children[4];
        app.dispatch(toggle, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert!(
            app.tree()
                .node(line)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .progress
                .as_ref()
                .unwrap()
                .current
                .is_none()
        );
        assert!((bounds(app, fill).width - 72.).abs() < 0.1);
        app.dispatch(toggle, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert!((bounds(app, fill).width - 180.).abs() < 0.1);
        app.clear().unwrap();
        assert_eq!(host.renderer().stats().nodes, 0);
    }
}

#[test]
fn native_vertical_progress_uses_real_bottom_up_geometry_and_pixels_in_all_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(
            sapid::progress(60.)
                .theme(theme.clone())
                .name("Vertical upload")
                .orientation(sapid::ProgressOrientation::Vertical)
                .length(glory::style::Length::Px(200.))
                .thickness(20.),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let root = find(app, "sapid-progress-vertical");
        let fill = find(app, "sapid-progress-fill");
        let root_box = bounds(app, root);
        let fill_box = bounds(app, fill);
        assert!((root_box.width - 20.).abs() < 0.1);
        assert!((root_box.height - 200.).abs() < 0.1);
        assert!((fill_box.height - 120.).abs() < 0.1);
        assert!((fill_box.y - (root_box.y + 80.)).abs() < 0.1);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            pixel(&raster, root_box.x + 10., root_box.y + 190.),
            rgba(theme.accent)
        );
        assert_eq!(
            pixel(&raster, root_box.x + 10., root_box.y + 10.),
            rgba(theme.border)
        );
        app.clear().unwrap();
        assert_eq!(host.renderer().stats().nodes, 0);
    }
}

#[test]
fn native_semicircle_down_direction_and_segment_colors_are_real_pixels() {
    let filled = glory::style::Color(240, 62, 62, 255);
    let empty = glory::style::Color(64, 192, 87, 255);
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(
            sapid::semi_circle_progress(50.)
                .theme(theme)
                .size(120.)
                .thickness(12.)
                .semi_orientation(sapid::SemiCircleOrientation::Down)
                .fill_direction(sapid::SemiCircleFillDirection::LeftToRight)
                .filled_segment_color(filled)
                .empty_segment_color(empty),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let root = find(app, "sapid-progress-semicircle");
        let root_box = bounds(app, root);
        assert!((root_box.width - 120.).abs() < 0.1);
        assert!((root_box.height - 66.).abs() < 0.1);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            pixel(&raster, root_box.x + 22., root_box.y + 44.),
            rgba(filled)
        );
        assert_eq!(
            pixel(&raster, root_box.x + 98., root_box.y + 44.),
            rgba(empty)
        );
        app.clear().unwrap();
        assert_eq!(host.renderer().stats().nodes, 0);
    }
}

#[test]
fn native_ring_start_angle_section_gap_and_root_color_are_real_pixels() {
    let red = glory::style::Color(240, 62, 62, 255);
    let blue = glory::style::Color(34, 139, 230, 255);
    let green = glory::style::Color(64, 192, 87, 255);
    let root_color = glory::style::Color(134, 142, 150, 255);
    let host = WindowApp::new();
    host.mount(
        sapid::ring_progress(60.)
            .size(120.)
            .thickness(12.)
            .sections([
                sapid::ProgressSection::new(20., red),
                sapid::ProgressSection::new(30., blue),
                sapid::ProgressSection::new(10., green),
            ])
            .start_angle(0.)
            .section_gap(18.)
            .root_color(root_color),
    )
    .unwrap();
    let app = host.app();
    app.settle(16).unwrap();
    let root = find(app, "sapid-progress-ring");
    let root_box = bounds(app, root);
    let mut raster = Raster::new();
    raster.draw(host.renderer()).unwrap();
    assert_eq!(
        pixel(&raster, root_box.x + 108., root_box.y + 85.),
        rgba(red)
    );
    assert_eq!(
        pixel(&raster, root_box.x + 85., root_box.y + 108.),
        rgba(root_color)
    );
    app.clear().unwrap();
    assert_eq!(host.renderer().stats().nodes, 0);
}
