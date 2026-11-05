use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Props, Query, QueryValue, Rect},
};
use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Waker},
};
fn themes() -> [sapid::Theme; 4] {
    [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ]
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let value = Rc::new(RefCell::new(None));
    let saved = value.clone();
    app.request(Query::Bounds(id), move |v| {
        *saved.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = value.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
fn observe(host: &WindowApp) {
    for _ in 0..8 {
        host.observe(&Context::from_waker(Waker::noop())).unwrap();
    }
}
#[test]
fn icon_and_close_targets_are_square_and_default_vector_is_centered_in_all_sizes() {
    for theme in themes() {
        for (size, side, glyph) in [
            (sapid::Size::ExtraSmall, 24., 12.),
            (sapid::Size::Small, 30., 14.),
            (sapid::Size::Medium, 36., 16.),
            (sapid::Size::Large, 42., 18.),
            (sapid::Size::ExtraLarge, 50., 20.),
        ] {
            let host = WindowApp::new();
            host.renderer().set_viewport(180, 100, 1.).unwrap();
            host.mount(
                sapid::close_button()
                    .theme(theme.clone())
                    .size(size)
                    .name("Close settings"),
            )
            .unwrap();
            let app = host.app();
            app.settle(16).unwrap();
            let id = app.tree().roots()[0];
            let rect = bounds(app, id);
            assert_eq!((rect.width, rect.height), (side, side));
            let icon = app.tree().node(id).unwrap().children[0];
            let ir = bounds(app, icon);
            assert_eq!((ir.width, ir.height), (glyph, glyph));
            assert!((ir.x + glyph / 2. - rect.x - side / 2.).abs() < 0.1);
            assert!((ir.y + glyph / 2. - rect.y - side / 2.).abs() < 0.1);
            app.clear().unwrap();
            let host = WindowApp::new();
            host.mount(
                sapid::action_icon("Accept settings")
                    .theme(theme.clone())
                    .size(size)
                    .icon(text().content("+")),
            )
            .unwrap();
            host.app().settle(16).unwrap();
            let id = host.app().tree().roots()[0];
            let rect = bounds(host.app(), id);
            assert_eq!((rect.width, rect.height), (side, side));
            host.app().clear().unwrap();
        }
    }
}
#[test]
fn loading_suspends_and_resumes_the_same_caller_icon_and_button() {
    for theme in themes() {
        let host = WindowApp::new();
        host.renderer().set_viewport(180, 100, 1.).unwrap();
        let loading = host.app().scope().cage(false);
        let icon = host.app().scope().cage(None);
        host.mount(
            sapid::action_icon("Refresh settings")
                .theme(theme.clone())
                .loading(loading)
                .icon(
                    glory::ui::graphic()
                        .node_ref(icon)
                        .drawing(sapid::Icon::Check.drawing(theme.accent))
                        .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                ),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let button = app.tree().roots()[0];
        let icon = icon.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(icon, glory::html::Part::Main)
            .unwrap();
        let button_physical = host
            .renderer()
            .engine_id(button, glory::html::Part::Main)
            .unwrap();
        for _ in 0..3 {
            loading.revise(|v| *v = true);
            observe(&host);
            let tree = app.tree();
            let d = tree.node(button).unwrap().descriptor.view();
            assert!(matches!(d.props,Props::Button(p)if !p.enabled));
            assert_eq!(d.semantics.as_ref().unwrap().busy, Some(true));
            assert_eq!(d.name.as_deref(), Some("Refresh settings"));
            drop(tree);
            loading.revise(|v| *v = false);
            observe(&host);
            assert_eq!(
                host.renderer()
                    .engine_id(icon, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
            assert_eq!(
                host.renderer()
                    .engine_id(button, glory::html::Part::Main)
                    .unwrap(),
                button_physical
            );
            assert_eq!(bounds(app, icon).width, 16.);
        }
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.runtime_stats().tasks, 0);
    }
}
