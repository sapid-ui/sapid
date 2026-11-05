use glory::{
    prelude::*,
    render::{Event, NodeId, Props},
};
use std::{cell::Cell, rc::Rc};
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
#[test]
fn loading_retains_custom_icon_and_message_and_rejects_close_in_the_current_packet() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let app = App::memory();
        let loading = app.scope().cage(false);
        let visible = app.scope().cage(true);
        let icon = app.scope().cage(None);
        let label = app.scope().cage("Hide upload".to_owned());
        let calls = Rc::new(Cell::new(0));
        let output = calls.clone();
        app.mount(
            sapid::notification("upload")
                .theme(theme.clone())
                .title("Project upload")
                .content("Files are being checked.")
                .loading(loading)
                .visible(visible)
                .close_label(label)
                .icon(
                    glory::ui::graphic()
                        .node_ref(icon)
                        .drawing(sapid::Icon::Check.drawing(theme.accent)),
                )
                .on_close(move || output.set(output.get() + 1)),
        )
        .unwrap();
        let icon_id = icon.get().unwrap();
        let message = class(&app, "sapid-alert-content");
        let close = class(&app, "sapid-alert-close");
        let root = class(&app, "sapid-notification");
        app.scope()
            .batch(|| {
                loading.revise(|v| *v = true);
                app.dispatch(close, Event::Activate).unwrap();
            })
            .unwrap();
        assert_eq!(calls.get(), 0);
        assert!(visible.get());
        assert_eq!(
            app.tree()
                .node(root)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .busy,
            Some(true)
        );
        assert!(
            matches!(app.tree().node(close).unwrap().descriptor.view().props,Props::Button(p) if !p.enabled)
        );
        assert!(
            app.tree()
                .node(class(&app, "sapid-notification-icon-content"))
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        loading.revise(|v| *v = false);
        label.revise(|v| *v = "Dismiss upload".into());
        app.flush().unwrap();
        assert_eq!(icon.get(), Some(icon_id));
        assert_eq!(class(&app, "sapid-alert-content"), message);
        assert_eq!(class(&app, "sapid-alert-close"), close);
        assert_eq!(
            app.tree()
                .node(close)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref(),
            Some("Dismiss upload")
        );
        app.dispatch(close, Event::Activate).unwrap();
        assert_eq!(calls.get(), 1);
        assert!(visible.get());
        assert!(
            !app.tree()
                .node(root)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        app.clear().unwrap();
        assert_eq!(icon.get(), None);
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn loading_without_a_custom_icon_releases_the_empty_icon_slot_when_finished() {
    let app = App::memory();
    let loading = app.scope().cage(false);
    app.mount(
        sapid::notification("plain")
            .content("A background operation")
            .loading(loading),
    )
    .unwrap();
    let slot = class(&app, "sapid-alert-icon");
    for value in [false, true, false] {
        loading.revise(|v| *v = value);
        app.flush().unwrap();
        assert_eq!(
            app.tree()
                .node(slot)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden,
            !value
        );
    }
}
