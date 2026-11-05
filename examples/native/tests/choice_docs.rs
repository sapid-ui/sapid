use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props},
};
#[path = "../../web/src/showcase_choice_docs/chip_modes.rs"]
mod chip;
#[path = "../../web/src/showcase_choice_docs/fieldset_disabled.rs"]
mod fieldset;
#[path = "../../web/src/showcase_choice_docs/checkbox_group.rs"]
mod group;
#[path = "../../web/src/showcase_choice_docs/radio_controlled.rs"]
mod radio;
fn find(app: &App, label: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if matches!(node.descriptor.view().props, Props::Check(value) if value.label == label)
            || matches!(node.descriptor.view().props, Props::Input(value) if value.value == label)
        {
            return id;
        }
        pending.extend(node.children.iter().copied());
    }
    panic!("missing {label}")
}
fn checked(app: &App, node: NodeId) -> bool {
    let tree = app.tree();
    let Props::Check(value) = tree.node(node).unwrap().descriptor.view().props else {
        panic!("check")
    };
    value.checked
}
#[test]
fn actual_native_group_transitions_between_mixed_checked_and_empty_without_replacing_controls() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(group::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let all = find(app, "All available channels");
        let email = find(app, "Email");
        let sms = find(app, "SMS");
        let identity = host
            .renderer()
            .engine_id(email, glory::html::Part::Control)
            .unwrap();
        assert!(
            matches!(app.tree().node(all).unwrap().descriptor.view().props, Props::Check(value) if value.mode == glory::render::CheckMode::Mixed)
        );
        app.dispatch(all, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        assert!(checked(app, email) && checked(app, sms));
        assert!(!checked(app, find(app, "Push notifications")));
        app.dispatch(all, Event::CheckedChanged(false)).unwrap();
        app.settle(16).unwrap();
        assert!(!checked(app, email) && !checked(app, sms));
        assert_eq!(
            host.renderer()
                .engine_id(email, glory::html::Part::Control)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn disabled_native_fieldset_retains_the_same_editor_and_rejects_descendant_changes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.mount(fieldset::demo(theme)).unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let toggle = find(app, "Disable notification settings");
        let email = find(app, "Send email updates");
        let editor = find(app, "Alex");
        let identity = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        assert!(!app.dispatch(email, Event::CheckedChanged(true)).unwrap());
        assert!(!checked(app, email));
        app.dispatch(toggle, Event::CheckedChanged(false)).unwrap();
        app.settle(16).unwrap();
        assert!(app.dispatch(email, Event::CheckedChanged(true)).unwrap());
        app.settle(16).unwrap();
        assert!(checked(app, email));
        app.dispatch(toggle, Event::CheckedChanged(true)).unwrap();
        app.settle(16).unwrap();
        assert!(!app.dispatch(email, Event::CheckedChanged(false)).unwrap());
        assert!(checked(app, email));
        assert_eq!(
            host.renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        assert_eq!(find(app, "Alex"), editor);
    }
}
#[test]
fn native_radio_and_chip_documentation_mount_real_controls_and_retire_them() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for (label, content) in [
            (
                "Standard",
                Box::new(radio::demo(theme.clone())) as Box<dyn Widget>,
            ),
            (
                "Rust",
                Box::new(chip::demo(theme.clone())) as Box<dyn Widget>,
            ),
        ] {
            let host = WindowApp::new();
            host.mount(content).unwrap();
            host.app().settle(16).unwrap();
            let node = find(host.app(), label);
            assert!(
                host.renderer()
                    .engine_id(node, glory::html::Part::Control)
                    .is_ok()
            );
            host.app().clear().unwrap();
            assert!(host.app().tree().is_empty());
        }
    }
}
