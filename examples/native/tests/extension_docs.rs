#[path = "../../web/src/dedicated_demos/code_highlight.rs"]
mod code_highlight;
#[path = "../../web/src/dedicated_demos/json_viewer.rs"]
mod json_viewer;
#[path = "../../web/src/dedicated_demos/lightbox.rs"]
mod lightbox;
#[path = "../../web/src/dedicated_demos/modals_manager.rs"]
mod modals_manager;
#[path = "../../web/src/dedicated_demos/navigation_progress.rs"]
mod navigation_progress;
#[path = "../../web/src/dedicated_demos/notifications_system.rs"]
mod notifications_system;
#[path = "../../web/src/dedicated_demos/spotlight.rs"]
mod spotlight;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props},
};
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut out = vec![];
    while let Some(id) = pending.pop() {
        pending.extend(&tree.node(id).unwrap().children);
        out.push(id)
    }
    out
}
fn observe(host: &WindowApp) {
    for _ in 0..6 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
fn button(app: &App, label: &str) -> NodeId {
    nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label==label)).unwrap_or_else(||panic!("missing {label}"))
}
fn edit(app: &App, id: NodeId, value: &str) {
    let revision = match app.tree().node(id).unwrap().descriptor.view().props {
        Props::Input(p) => p.revision,
        _ => panic!("input"),
    };
    app.dispatch(
        id,
        Event::ValueChanged {
            value: value.into(),
            revision,
            composing: false,
        },
    )
    .unwrap();
}
fn has_text(app: &App, value: &str) -> bool {
    nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content.contains(value)))
}
#[test]
fn actual_extension_primary_sources_mount_in_four_themes_and_retire_their_graphs() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for name in [
            "code",
            "json",
            "notifications",
            "spotlight",
            "lightbox",
            "navigation",
            "modals",
        ] {
            let example: Box<dyn Widget> = match name {
                "code" => Box::new(code_highlight::demo(theme.clone())),
                "json" => Box::new(json_viewer::demo(theme.clone())),
                "notifications" => Box::new(notifications_system::demo(theme.clone())),
                "spotlight" => Box::new(spotlight::demo(theme.clone())),
                "lightbox" => Box::new(lightbox::demo(theme.clone())),
                "navigation" => Box::new(navigation_progress::demo(theme.clone())),
                _ => Box::new(modals_manager::demo(theme.clone())),
            };
            let host = WindowApp::new();
            host.renderer().set_viewport(360, 900, 1.).unwrap();
            host.mount(
                glory::ui::view()
                    .style(Style::new().width(Length::Px(340.)))
                    .fill(example),
            )
            .unwrap_or_else(|error| panic!("{name}: {error}"));
            observe(&host);
            assert!(host.app().stats().nodes > 0);
            host.app().clear().unwrap();
            assert_eq!(host.app().stats().handlers, 0);
            assert_eq!(host.app().stats().nodes, 0);
        }
    }
}
#[test]
fn actual_save_notification_and_managed_form_publish_current_editor_values() {
    let host = WindowApp::new();
    host.renderer().set_viewport(400, 900, 1.).unwrap();
    host.mount(notifications_system::demo(sapid::Theme::green_dark()))
        .unwrap();
    observe(&host);
    let title = nodes(host.app())
        .into_iter()
        .find(|id| {
            host.app()
                .tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Draft title")
        })
        .unwrap();
    edit(host.app(), title, "Native release notes");
    observe(&host);
    host.app()
        .dispatch(button(host.app(), "Save draft"), Event::Activate)
        .unwrap();
    observe(&host);
    assert!(has_text(
        host.app(),
        "Native release notes is saved in this example."
    ));
    host.app().clear().unwrap();
    host.mount(modals_manager::demo(sapid::Theme::blue_light()))
        .unwrap();
    observe(&host);
    host.app()
        .dispatch(button(host.app(), "Edit project"), Event::Activate)
        .unwrap();
    observe(&host);
    let editor = nodes(host.app())
        .into_iter()
        .find(|id| {
            host.app()
                .tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Project name")
        })
        .unwrap();
    edit(host.app(), editor, "Native workspace");
    observe(&host);
    host.app()
        .dispatch(button(host.app(), "Save project"), Event::Activate)
        .unwrap();
    observe(&host);
    assert!(has_text(host.app(), "Project: Native workspace"));
    host.app().clear().unwrap();
    assert_eq!(host.app().stats().handlers, 0);
}
