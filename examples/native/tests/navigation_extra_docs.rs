use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Props, Query, QueryValue, Role},
};
use std::{cell::RefCell, rc::Rc};
#[path = "../../web/src/dedicated_demos/table_of_contents.rs"]
mod article;
#[path = "../../web/src/dedicated_demos/tree.rs"]
mod files;
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = todo.pop() {
        todo.extend(tree.node(id).unwrap().children.iter().copied());
        out.push(id);
    }
    out
}
fn query(app: &App, query: Query) -> QueryValue {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(query, move |value| {
        *saved.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    out.borrow_mut().take().unwrap()
}
fn observe(host: &WindowApp) {
    for _ in 0..4 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
#[test]
fn article_links_scroll_the_owned_document_and_retain_live_heading_links_in_four_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 850, 1.).unwrap();
        let app = host.app();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(360.)))
                .fill(article::demo(theme)),
        )
        .unwrap();
        observe(&host);
        let ids = all(app);
        let document = *ids
            .iter()
            .find(|id| {
                app.tree()
                    .node(**id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some("Getting started article")
            })
            .unwrap();
        let links = ids
            .iter()
            .filter(|id| {
                matches!(
                    app.tree().node(**id).unwrap().descriptor.view().props,
                    Props::Link(_)
                )
            })
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 4);
        let link=*links.iter().find(|id|matches!(app.tree().node(**id).unwrap().descriptor.view().props,Props::Link(p) if p.label=="Share with your team")).unwrap();
        let physical = host
            .renderer()
            .engine_id(link, glory::html::Part::Main)
            .unwrap();
        let before = query(app, Query::Bounds(document));
        assert!(app.dispatch(link, glory::render::Event::Activate).unwrap());
        observe(&host);
        let QueryValue::ScrollMetrics(metrics) = query(app, Query::ScrollMetrics(document)) else {
            panic!("scroll metrics")
        };
        assert!(metrics.y > 0., "{metrics:?}");
        assert_eq!(metrics.viewport_height, 300.);
        assert_eq!(query(app, Query::Bounds(document)), before);
        assert_eq!(
            host.renderer()
                .engine_id(link, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        assert!(links.iter().any(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .is_some_and(|s| s.current == Some(glory::render::Current::Location))
        }));
    }
}
#[test]
fn project_file_example_selects_a_real_tree_item_and_updates_readable_feedback() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 500, 1.).unwrap();
        let app = host.app();
        host.mount(files::demo(theme)).unwrap();
        observe(&host);
        let readme = all(app)
            .into_iter()
            .find(|id| {
                let d = app.tree().node(*id).unwrap().descriptor.view().to_owned();
                d.role == Some(Role::TreeItem) && d.name.as_deref() == Some("README.md")
            })
            .unwrap();
        let physical = host
            .renderer()
            .engine_id(readme, glory::html::Part::Main)
            .unwrap();
        app.dispatch(readme, glory::render::Event::Activate)
            .unwrap();
        observe(&host);
        assert!(all(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content=="Selected file key: readme")));
        assert_eq!(
            host.renderer()
                .engine_id(readme, glory::html::Part::Main)
                .unwrap(),
            physical
        );
    }
}
