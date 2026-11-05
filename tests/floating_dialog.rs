use glory::render::{DismissReason, Event, LayerFocus, NodeId};
use glory::ui::App;
use sapid::{ContentPolicy, floating_dialog};
fn find(app: &App, class: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let n = tree.node(id).unwrap();
        if n.descriptor
            .view()
            .classes
            .iter()
            .any(|value| value == class)
        {
            return id;
        }
        pending.extend(&n.children)
    }
    panic!("missing {class}")
}
#[test]
fn floating_notice_is_named_nonmodal_preserves_focus_and_has_no_default_outside_dismissal() {
    let app = App::memory();
    app.mount(
        floating_dialog("notice")
            .title("Sync complete")
            .description("Editing can continue."),
    )
    .unwrap();
    let panel = find(&app, "sapid-floating-dialog");
    let tree = app.tree();
    let d = tree.node(panel).unwrap().descriptor.view();
    let metadata = d.semantics.as_ref().unwrap();
    let layer = metadata.layer.as_ref().unwrap();
    assert!(!layer.modal && !layer.outside && metadata.hidden);
    assert_eq!(layer.focus, LayerFocus::Preserve);
    assert!(layer.initial_focus.is_none() && layer.return_focus.is_none());
    assert!(metadata.labelled_by.is_some() && metadata.described_by.is_some());
}
#[test]
fn controlled_refusal_is_silent_on_external_writes_and_keeps_retained_content() {
    let app = App::memory();
    let open = app.scope().cage(true);
    app.mount(
        floating_dialog("held")
            .title("Background task")
            .open(open)
            .content_policy(ContentPolicy::KeepMounted)
            .content(|| glory::ui::input().name("Task note")),
    )
    .unwrap();
    let panel = find(&app, "sapid-floating-dialog");
    let before = app.tree().len();
    app.dispatch(panel, Event::Dismiss(DismissReason::Escape))
        .unwrap();
    assert!(open.get());
    assert!(
        !app.tree()
            .node(panel)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .hidden
    );
    open.revise(|value| *value = false);
    app.flush().unwrap();
    assert_eq!(app.tree().len(), before);
    open.revise(|value| *value = true);
    app.flush().unwrap();
    assert_eq!(find(&app, "sapid-floating-dialog"), panel);
}
#[test]
fn floating_dialog_validates_names_before_nodes_and_uses_escaped_ssr_dialog_markup() {
    for widget in [
        floating_dialog("bad"),
        floating_dialog("bad").title(" "),
        floating_dialog("bad").name(""),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().is_empty());
    }
    let html = glory::ssr::render(
        floating_dialog("ssr")
            .title("Task <complete>")
            .description("A & B")
            .default_open(true),
    )
    .unwrap();
    assert!(html.contains("role=\"dialog\""));
    assert!(html.contains("aria-modal=\"false\""));
    assert!(html.contains("Task &lt;complete&gt;"));
    assert!(html.contains("A &amp; B"));
}

#[test]
fn viewport_position_uses_shared_constraints_without_changing_focus_policy() {
    let app = App::memory();
    let position = glory::render::ViewportPlacement {
        x: 24.,
        y: 90.,
        ..Default::default()
    };
    app.mount(
        floating_dialog("positioned")
            .title("Background task")
            .position(position),
    )
    .unwrap();
    let panel = find(&app, "sapid-floating-dialog");
    let tree = app.tree();
    let layer = tree
        .node(panel)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap()
        .layer
        .as_ref()
        .unwrap();
    assert_eq!(
        layer.placement,
        glory::render::LayerPlacement::Viewport(position)
    );
    assert_eq!(layer.focus, LayerFocus::Preserve);
    drop(tree);
    for position in [
        glory::render::ViewportPlacement {
            x: f64::NAN,
            ..Default::default()
        },
        glory::render::ViewportPlacement {
            viewport_padding: 4097.,
            ..Default::default()
        },
    ] {
        let app = App::memory();
        assert!(
            app.mount(
                floating_dialog("invalid")
                    .title("Notice")
                    .position(position)
            )
            .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
