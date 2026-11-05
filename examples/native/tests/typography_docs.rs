#[path = "../../web/src/dedicated_demos/typography.rs"]
mod article;
#[path = "../../web/src/dedicated_demos/table.rs"]
mod project_table;
#[path = "../../web/src/showcase_typography_docs/quote_icon.rs"]
mod quote_icon;
#[path = "../../web/src/showcase_typography_docs/table_scroll.rs"]
mod sticky;
#[path = "../../web/src/showcase_typography_docs/table_structure.rs"]
mod structure;
#[path = "../../web/src/showcase_typography_docs/text_truncation.rs"]
mod text_truncation;
#[path = "../../web/src/showcase_typography_docs/title_truncation.rs"]
mod title_truncation;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{
        Event, NodeId, Props, Query, QueryValue, Rect, ScrollCommand, ScrollMetrics,
        ScrollMovement, TextKind,
    },
};
use std::{cell::RefCell, rc::Rc};
fn observe(host: &WindowApp) {
    for _ in 0..6 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|v| v == class) {
            out.push(id);
        }
        todo.extend(node.children.iter().rev());
    }
    out
}
fn query(app: &App, query: Query) -> QueryValue {
    let result = Rc::new(RefCell::new(None));
    let saved = result.clone();
    app.request(query, move |value| {
        *saved.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    result.borrow_mut().take().unwrap()
}
fn bounds(app: &App, node: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(node)) else {
        panic!("bounds")
    };
    rect
}
fn metrics(app: &App, node: NodeId) -> ScrollMetrics {
    let QueryValue::ScrollMetrics(value) = query(app, Query::ScrollMetrics(node)) else {
        panic!("scroll metrics")
    };
    value
}
#[test]
fn actual_public_article_and_icon_quote_wrap_in_a_narrow_native_column_in_four_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 900, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(article::demo(theme.clone())),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        assert_eq!(nodes(app, "sapid-title").len(), 3);
        assert_eq!(nodes(app, "sapid-text-list-item").len(), 3);
        for id in nodes(app, "sapid-text")
            .into_iter()
            .chain(nodes(app, "sapid-title"))
        {
            let r = bounds(app, id);
            assert!(r.width <= 300.5 && r.height > 0., "{r:?}");
        }
        let headings = nodes(app, "sapid-title");
        assert!(
            matches!(app.tree().node(headings[0]).unwrap().descriptor.view().props,Props::Text(p) if p.kind==TextKind::Heading(2))
        );
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(280.)))
                .fill(quote_icon::demo(theme)),
        )
        .unwrap();
        observe(&host);
        let icon = bounds(app, nodes(app, "sapid-theme-icon")[0]);
        let text = bounds(app, nodes(app, "sapid-text")[0]);
        assert!((icon.y - text.y).abs() < 1.);
        assert!(text.x >= icon.x + icon.width + 10.);
        assert!(text.height > icon.height);
    }
}
#[test]
fn actual_public_table_uses_local_native_horizontal_scrolling_and_retains_its_cells() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 360, 1.).unwrap();
    host.mount(
        glory::ui::view()
            .style(Style::new().width(Length::Px(300.)))
            .fill(project_table::demo(sapid::Theme::blue_light())),
    )
    .unwrap();
    observe(&host);
    let app = host.app();
    let viewport = nodes(app, "sapid-scroll-area")[0];
    let initial = metrics(app, viewport);
    assert!(initial.content_width >= 560.);
    assert!(initial.viewport_width <= 300.5);
    let cell = nodes(app, "sapid-table-cell")[0];
    let physical = host
        .renderer()
        .engine_id(cell, glory::html::Part::Main)
        .unwrap();
    app.dispatch(
        viewport,
        Event::ScrollKeyboard {
            from: viewport,
            command: ScrollCommand {
                axis: Orientation::Horizontal,
                movement: ScrollMovement::StepPositive,
            },
        },
    )
    .unwrap();
    observe(&host);
    assert!(metrics(app, viewport).x > initial.x);
    assert_eq!(
        host.renderer()
            .engine_id(cell, glory::html::Part::Main)
            .unwrap(),
        physical
    );
}
#[test]
fn native_text_clipping_and_sticky_samples_keep_explicit_unsupported_boundaries() {
    for example in [
        Box::new(text_truncation::demo(sapid::Theme::blue_light())) as Box<dyn Widget>,
        Box::new(title_truncation::demo(sapid::Theme::blue_light())),
        Box::new(sticky::demo(sapid::Theme::blue_light())),
        Box::new(
            sapid::text()
                .line_clamp(2)
                .content("The native line clamp boundary is explicit."),
        ),
    ] {
        let host = WindowApp::new();
        assert!(host.mount(example).is_err());
        assert!(host.app().tree().is_empty());
    }
}

#[test]
fn actual_public_bottom_caption_appears_after_the_footer_in_four_native_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 400, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(structure::demo(theme)),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let caption = bounds(app, nodes(app, "sapid-table-caption")[0]);
        let foot = bounds(app, nodes(app, "sapid-table-foot")[0]);
        assert!(
            caption.y >= foot.y + foot.height - 0.5,
            "{caption:?} {foot:?}"
        );
    }
}
