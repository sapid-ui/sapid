use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Query, QueryValue, Rect},
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
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|v| v == name) {
            return id;
        }
        pending.extend(&node.children);
    }
    panic!("missing {name}")
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let result = Rc::new(RefCell::new(None));
    let saved = result.clone();
    app.request(Query::Bounds(id), move |value| {
        *saved.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = result.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
#[test]
fn default_layer_headers_have_compact_vector_closes_and_wrapping_titles_in_real_layout() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for width in [390, 1280] {
            for kind in ["dialog", "popover", "floating-dialog"] {
                let host = WindowApp::new();
                host.renderer().set_viewport(width, 600, 1.).unwrap();
                let title = "Workspace settings with a longer descriptive title";
                let widget: Box<dyn Widget> = match kind {
                    "dialog" => Box::new(
                        sapid::dialog("header")
                            .theme(theme.clone())
                            .title(title)
                            .default_open(true),
                    ),
                    "drawer" => Box::new(
                        sapid::drawer("header")
                            .theme(theme.clone())
                            .title(title)
                            .default_open(true),
                    ),
                    "popover" => Box::new(
                        sapid::popover("header")
                            .theme(theme.clone())
                            .title(title)
                            .default_open(true),
                    ),
                    _ => Box::new(
                        sapid::floating_dialog("header")
                            .theme(theme.clone())
                            .title(title)
                            .default_open(true)
                            .position(glory::render::ViewportPlacement {
                                x: 20.,
                                y: 60.,
                                ..Default::default()
                            }),
                    ),
                };
                host.mount(widget).unwrap();
                observe(&host);
                let app = host.app();
                let close = bounds(app, class(app, "sapid-layer-close"));
                let title = bounds(app, class(app, "sapid-layer-title"));
                let header = bounds(app, class(app, "sapid-layer-header"));
                assert_eq!((close.width, close.height), (24., 24.));
                assert!(close.x >= title.x + title.width - 1.);
                assert!((close.y + close.height / 2. - header.y - header.height / 2.).abs() < 1.);
                assert!(close.x + close.width <= f64::from(width) + 1.);
                assert!(
                    matches!(app.tree().node(class(app,"sapid-layer-close")).unwrap().descriptor.view().props,glory::render::Props::Button(props) if props.label.is_empty())
                );
            }
        }
    }
}

#[test]
fn selection_actions_wrap_without_squeezing_labels_and_keep_theme_and_identity() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(220, 360, 1.).unwrap();
        host.mount(
            sapid::action_bar("bulk-documents")
                .theme(theme.clone())
                .selected_count(2usize)
                .group(
                    sapid::toolbar_group("bulk-actions")
                        .button(
                            sapid::button()
                                .class("bulk-archive")
                                .label("Archive selected"),
                        )
                        .button(
                            sapid::button()
                                .class("bulk-export")
                                .label("Export selected"),
                        ),
                ),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let archive = class(app, "bulk-archive");
        let export = class(app, "bulk-export");
        let first = bounds(app, archive);
        let next = bounds(app, export);
        assert!(first.width > 110. && first.height <= 40.);
        assert!(next.width > 110. && next.height <= 40.);
        assert!(first.x >= 0. && first.x + first.width <= 221.);
        assert!(next.x >= 0. && next.x + next.width <= 221.);
        assert!(next.y > first.y);
        let root = class(app, "sapid-action-bar");
        assert_eq!(
            app.tree().node(root).unwrap().descriptor.view().style.color,
            Some(theme.foreground)
        );
        let physical = host
            .renderer()
            .engine_id(archive, glory::html::Part::Main)
            .unwrap();
        host.renderer().set_viewport(720, 360, 1.).unwrap();
        observe(&host);
        assert_eq!(
            host.renderer()
                .engine_id(archive, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        let first = bounds(app, archive);
        let next = bounds(app, export);
        assert!((first.y - next.y).abs() < 1.);
    }
}
#[test]
fn native_drawer_edges_still_reject_before_leaving_a_partial_widget() {
    for side in [
        sapid::DrawerSide::Left,
        sapid::DrawerSide::Right,
        sapid::DrawerSide::Top,
        sapid::DrawerSide::Bottom,
    ] {
        let host = WindowApp::new();
        assert!(matches!(
            host.mount(
                sapid::drawer("unsupported-edge")
                    .side(side)
                    .title("Workspace")
            ),
            Err(glory::ui::Error::Render(glory::render::Error::Unsupported(
                "native top layers"
            )))
        ));
        assert_eq!(host.app().stats().nodes, 0);
    }
}

#[test]
fn hover_card_preserves_real_background_focus_while_tooltip_retains_explicit_native_refusal() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 480, 1.).unwrap();
        let app = host.app();
        let outside = app.scope().cage(None);
        let open = app.scope().cage(false);
        let body = theme.clone();
        host.mount(
            sapid::stack().theme(theme.clone()).gap(12.).fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Background hover draft")
                    .node_ref(outside)
                    .default_value("Keep editing"),
                sapid::hover_card("native-profile")
                    .theme(theme.clone())
                    .label("Ada profile trigger")
                    .name("Ada details")
                    .open(open)
                    .on_open_change(move |v| open.revise(|old| *old = v))
                    .content(move || {
                        sapid::button()
                            .theme(body.clone())
                            .label("Follow native Ada")
                    }),
            )),
        )
        .unwrap();
        observe(&host);
        let outside = outside.get().unwrap();
        app.scope()
            .action(glory::render::Action::Focus(outside))
            .unwrap();
        observe(&host);
        let physical = host
            .renderer()
            .engine_id(outside, glory::html::Part::Main)
            .unwrap();
        open.revise(|v| *v = true);
        observe(&host);
        let panel = class(app, "sapid-hover-card");
        assert!(bounds(app, panel).height > 30.);
        let result = Rc::new(RefCell::new(None));
        let saved = result.clone();
        app.request(Query::Focus(outside), move |v| {
            *saved.borrow_mut() = Some(v.unwrap())
        })
        .unwrap();
        app.pump().unwrap();
        assert_eq!(*result.borrow(), Some(QueryValue::Focus(true)));
        assert_eq!(
            host.renderer()
                .engine_id(outside, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        let host = WindowApp::new();
        assert!(matches!(
            host.mount(sapid::tooltip("native-tip").text("Explanation")),
            Err(glory::ui::Error::Render(glory::render::Error::Unsupported(
                "native top layers"
            )))
        ));
        assert_eq!(host.app().stats().nodes, 0);
    }
}

#[test]
fn tour_has_separate_text_rows_and_themed_navigation_in_real_narrow_layout() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 480, 1.).unwrap();
        let context = sapid::tour_context(host.app().scope(), "native-guide-layout").unwrap();
        host.mount(
            sapid::stack().theme(theme.clone()).gap(12.).fill((
                sapid::tour_target(&context, "draft").fill(
                    sapid::text_field()
                        .theme(theme.clone())
                        .name("Guide layout draft"),
                ),
                sapid::tour(&context, "guide-layout")
                    .theme(theme.clone())
                    .default_open(true)
                    .step(sapid::tour_step(
                        "draft",
                        "Name your workspace",
                        "Choose a name that your team will recognize.",
                    )),
            )),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let title = bounds(app, class(app, "sapid-tour-title"));
        let description = bounds(app, class(app, "sapid-tour-description"));
        let progress = bounds(app, class(app, "sapid-tour-progress"));
        let navigation = bounds(app, class(app, "sapid-tour-navigation"));
        assert!(title.width > 0. && title.height > 0.);
        assert!(description.y >= title.y + title.height);
        assert!(progress.y >= description.y + description.height);
        assert!(navigation.y >= progress.y + progress.height);
        let nav = class(app, "sapid-tour-navigation");
        let children = app.tree().node(nav).unwrap().children.clone();
        assert_eq!(children.len(), 3);
        for button in children {
            let rect = bounds(app, button);
            assert!(rect.height >= 30. && rect.height <= 36.);
            assert!(rect.x + rect.width <= 391.);
            assert!(
                app.tree()
                    .node(button)
                    .unwrap()
                    .descriptor
                    .view()
                    .style
                    .font_family
                    .is_some()
            );
        }
    }
}
