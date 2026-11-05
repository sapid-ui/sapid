use glory::{
    apace::{Config, Raster, shell::WindowApp},
    prelude::*,
    render::{Action, MotionPreference, NodeId, Query, QueryValue},
};
use std::{cell::RefCell, rc::Rc};
fn observe(host: &WindowApp) {
    for _ in 0..4 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
fn class(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|v| v == name) {
            return id;
        }
        todo.extend(node.children.iter().copied());
    }
    panic!("missing {name}")
}
fn query(app: &App, query: Query) -> QueryValue {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(query, move |v| *saved.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    out.borrow_mut().take().unwrap()
}
#[test]
fn skeleton_wraps_intrinsic_content_fills_its_box_and_retains_the_real_editor() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::with_config(Config {
            motion: MotionPreference::NoPreference,
            ..Default::default()
        });
        host.renderer().set_viewport(360, 200, 1.).unwrap();
        let app = host.app();
        let loading = app.scope().cage(true);
        let input = app.scope().cage(None);
        let root = app.scope().cage(None);
        host.mount(
            sapid::skeleton()
                .theme(theme.clone())
                .loading(loading)
                .node_ref(root)
                .width(Length::Px(320.))
                .fill(
                    sapid::text_field()
                        .theme(theme.clone())
                        .name("Project note")
                        .node_ref(input)
                        .value("Retained draft"),
                ),
        )
        .unwrap();
        observe(&host);
        let input = input.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(input, glory::html::Part::Main)
            .unwrap();
        let root = root.get().unwrap();
        let QueryValue::Bounds(rect) = query(app, Query::Bounds(root)) else {
            panic!("bounds")
        };
        assert_eq!(rect.width, 320.);
        assert!(rect.height >= 36.);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let writes = app.runtime_stats().source_writes;
        host.renderer().advance_time(0.75).unwrap();
        raster.draw(host.renderer()).unwrap();
        assert_eq!(app.runtime_stats().source_writes, writes);
        let at = |x: usize, y: usize| {
            let width = raster.frame().unwrap().width as usize;
            let offset = (y * width + x) * 4;
            raster.pixels()[offset..offset + 4].to_vec()
        };
        assert_eq!(
            at(310, 12),
            vec![
                theme.border.0,
                theme.border.1,
                theme.border.2,
                theme.border.3
            ]
        );
        assert!(matches!(
            app.scope().action(Action::Focus(input)),
            Err(glory::ui::Error::Render(glory::render::Error::Unsupported(
                _
            )))
        ));
        observe(&host);
        assert_eq!(query(app, Query::Focus(input)), QueryValue::Focus(false));
        loading.revise(|v| *v = false);
        observe(&host);
        assert_eq!(
            host.renderer()
                .engine_id(input, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        app.scope().action(Action::Focus(input)).unwrap();
        observe(&host);
        assert_eq!(query(app, Query::Focus(input)), QueryValue::Focus(true));
        assert!(!host.renderer().inspect(|doc| doc.is_animating()));
    }
}
#[test]
fn circle_placeholder_clips_its_real_native_pixels_and_reduce_is_static() {
    let host = WindowApp::new();
    host.renderer().set_viewport(48, 48, 1.).unwrap();
    host.mount(sapid::skeleton().circle(40.)).unwrap();
    observe(&host);
    let mut raster = Raster::new();
    raster.draw(host.renderer()).unwrap();
    let width = raster.frame().unwrap().width as usize;
    assert_eq!(&raster.pixels()[0..4], &[0, 0, 0, 0]);
    let offset = (20 * width + 20) * 4;
    assert!(raster.pixels()[offset + 3] > 0);
    let revision = host.renderer().stats().visual_revision;
    host.renderer().advance_time(1.).unwrap();
    assert_eq!(host.renderer().stats().visual_revision, revision);
    assert!(!raster.draw(host.renderer()).unwrap().rendered);
}
#[test]
fn segmented_loaders_animate_real_pixels_without_widget_frame_writes_and_honor_reduce() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in [sapid::LoaderType::Bars, sapid::LoaderType::Dots] {
            let host = WindowApp::with_config(Config {
                motion: MotionPreference::NoPreference,
                ..Default::default()
            });
            host.renderer().set_viewport(48, 48, 1.).unwrap();
            host.mount(
                sapid::loader()
                    .theme(theme.clone())
                    .kind(kind)
                    .diameter(32.)
                    .period_millis(900),
            )
            .unwrap();
            observe(&host);
            let mut raster = Raster::new();
            raster.draw(host.renderer()).unwrap();
            let before = raster.pixels().to_vec();
            let writes = host.app().runtime_stats().source_writes;
            host.renderer().advance_time(0.3).unwrap();
            assert!(raster.draw(host.renderer()).unwrap().rendered);
            assert_ne!(raster.pixels(), before);
            assert_eq!(host.app().runtime_stats().source_writes, writes);
            host.renderer()
                .set_motion_preference(MotionPreference::Reduce);
            raster.draw(host.renderer()).unwrap();
            let revision = host.renderer().stats().visual_revision;
            host.renderer().advance_time(0.6).unwrap();
            assert_eq!(host.renderer().stats().visual_revision, revision);
            assert!(!raster.draw(host.renderer()).unwrap().rendered);
        }
    }
}
#[test]
fn notification_loading_and_close_have_real_compact_geometry_and_retained_custom_icon() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 300, 1.).unwrap();
        let app = host.app();
        let loading = app.scope().cage(false);
        let icon = app.scope().cage(None);
        host.mount(
            sapid::notification("native-job")
                .theme(theme.clone())
                .title("Workspace upload")
                .content("Your project is being saved.")
                .loading(loading)
                .icon(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::Info.drawing(theme.accent))
                        .node_ref(icon)
                        .style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))),
                ),
        )
        .unwrap();
        observe(&host);
        let icon = icon.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(icon, glory::html::Part::Main)
            .unwrap();
        let close = class(app, "sapid-alert-close");
        let QueryValue::Bounds(rect) = query(app, Query::Bounds(close)) else {
            panic!("bounds")
        };
        assert_eq!((rect.width, rect.height), (24., 24.));
        loading.revise(|v| *v = true);
        observe(&host);
        assert!(!app.dispatch(close, glory::render::Event::Activate).unwrap());
        loading.revise(|v| *v = false);
        observe(&host);
        assert_eq!(
            host.renderer()
                .engine_id(icon, glory::html::Part::Main)
                .unwrap(),
            physical
        );
    }
}

#[test]
fn unknown_progress_animates_without_source_writes_and_retains_geometry_when_paused() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::with_config(Config {
            motion: MotionPreference::NoPreference,
            ..Default::default()
        });
        host.renderer().set_viewport(320, 360, 1.).unwrap();
        let app = host.app();
        let animated = app.scope().cage(true);
        host.mount(
            sapid::stack().theme(theme.clone()).gap(16.).fill((
                sapid::progress(40.)
                    .theme(theme.clone())
                    .width(Length::Px(240.))
                    .thickness(16.)
                    .indeterminate(true)
                    .animated(animated),
                sapid::ring_progress(40.)
                    .theme(theme.clone())
                    .size(96.)
                    .thickness(8.)
                    .indeterminate(true)
                    .animated(animated),
                sapid::semi_circle_progress(40.)
                    .theme(theme.clone())
                    .size(120.)
                    .thickness(10.)
                    .indeterminate(true)
                    .animated(animated),
                sapid::progress(60.)
                    .theme(theme.clone())
                    .width(Length::Px(240.))
                    .thickness(26.)
                    .sections([
                        sapid::ProgressSection::new(40., theme.accent),
                        sapid::ProgressSection::new(20., theme.warning),
                    ])
                    .section_label(0, "Documents")
                    .section_label(1, "Images"),
            )),
        )
        .unwrap();
        observe(&host);
        let arc = class(app, "sapid-progress-arc");
        let physical = host
            .renderer()
            .engine_id(arc, glory::html::Part::Main)
            .unwrap();
        let label = class(app, "sapid-progress-section-label");
        let QueryValue::Bounds(rect) = query(app, Query::Bounds(label)) else {
            panic!("bounds")
        };
        assert!(rect.width > 0. && rect.width <= 96. && rect.height <= 26.);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let before = raster.pixels().to_vec();
        let writes = app.runtime_stats().source_writes;
        host.renderer().advance_time(0.3).unwrap();
        assert!(raster.draw(host.renderer()).unwrap().rendered);
        assert_ne!(raster.pixels(), before);
        assert_eq!(app.runtime_stats().source_writes, writes);
        animated.revise(|v| *v = false);
        observe(&host);
        raster.draw(host.renderer()).unwrap();
        assert_eq!(
            host.renderer()
                .engine_id(arc, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        let revision = host.renderer().stats().visual_revision;
        host.renderer().advance_time(0.5).unwrap();
        assert_eq!(host.renderer().stats().visual_revision, revision);
        assert!(!raster.draw(host.renderer()).unwrap().rendered);
        animated.revise(|v| *v = true);
        observe(&host);
        host.renderer()
            .set_motion_preference(MotionPreference::Reduce);
        raster.draw(host.renderer()).unwrap();
        let revision = host.renderer().stats().visual_revision;
        host.renderer().advance_time(0.5).unwrap();
        assert_eq!(host.renderer().stats().visual_revision, revision);
        assert!(!raster.draw(host.renderer()).unwrap().rendered);
    }
}

#[test]
fn empty_state_actions_center_as_a_row_and_after_narrow_wrapping() {
    for width in [320, 720] {
        let host = WindowApp::new();
        host.renderer().set_viewport(width, 320, 1.).unwrap();
        host.mount(
            sapid::empty_state().title("No projects yet").actions((
                sapid::button()
                    .class("empty-first")
                    .label("Create a project"),
                sapid::button()
                    .class("empty-second")
                    .label("Use a sample project"),
            )),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let QueryValue::Bounds(first) = query(app, Query::Bounds(class(app, "empty-first"))) else {
            panic!("bounds")
        };
        let QueryValue::Bounds(second) = query(app, Query::Bounds(class(app, "empty-second")))
        else {
            panic!("bounds")
        };
        let center = f64::from(width) / 2.;
        if width == 320 {
            assert!(second.y > first.y);
            assert!((first.x + first.width / 2. - center).abs() < 1.);
            assert!((second.x + second.width / 2. - center).abs() < 1.);
        } else {
            assert!((first.y - second.y).abs() < 1.);
            assert!(((first.x + second.x + second.width) / 2. - center).abs() < 1.);
        }
    }
}
