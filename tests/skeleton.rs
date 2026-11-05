use glory::prelude::*;
use glory::render::Props;
use sapid::skeleton;
#[test]
fn placeholder_has_busy_owner_and_one_decorative_unfocusable_graphic() {
    let app = App::memory();
    app.mount(
        skeleton()
            .name("Loading profile")
            .width(Length::Px(220.0))
            .height(20.0),
    )
    .unwrap();
    let tree = app.tree();
    let root = tree.node(tree.roots()[0]).unwrap();
    let d = root.descriptor.view();
    assert_eq!(d.semantics.as_ref().unwrap().busy, Some(true));
    assert!(!d.semantics.as_ref().unwrap().hidden);
    assert_eq!(root.children.len(), 1);
    let child = tree.node(root.children[0]).unwrap().descriptor.view();
    assert!(
        matches!(child.props,Props::Graphic(p)if p.alternative.is_none()&&p.fill.is_some()&&!p.commands.is_empty())
    );
    assert!(child.name.is_none());
    assert_eq!(child.focus, glory::render::FocusMode::Auto);
}
#[test]
fn loading_content_retains_the_real_editor_and_blocks_interaction_until_available() {
    let app = App::memory();
    let loading = app.scope().cage(true);
    let editor = app.scope().cage(None);
    let draft = app.scope().cage("Retained note".to_owned());
    let calls = std::rc::Rc::new(std::cell::Cell::new(0));
    let output = calls.clone();
    app.mount(
        skeleton().loading(loading).fill(
            sapid::stack().fill((
                sapid::text_field()
                    .name("Project note")
                    .node_ref(editor)
                    .value(draft)
                    .on_value_change(move |v| draft.revise(|old| *old = v)),
                sapid::button()
                    .class("loaded-action")
                    .label("Save note")
                    .on_press(move || output.set(output.get() + 1)),
            )),
        ),
    )
    .unwrap();
    let input = editor.get().unwrap();
    let root = app.tree().roots()[0];
    let body = app.tree().node(root).unwrap().children[0];
    let mut todo = vec![body];
    let action = loop {
        let id = todo.pop().unwrap();
        let tree = app.tree();
        let node = tree.node(id).unwrap();
        if node
            .descriptor
            .view()
            .classes
            .iter()
            .any(|s| s == "loaded-action")
        {
            break id;
        }
        todo.extend(node.children.iter().copied());
    };
    assert!(
        !app.dispatch(action, glory::render::Event::Activate)
            .unwrap()
    );
    assert_eq!(calls.get(), 0);
    for value in [false, true, false] {
        loading.revise(|old| *old = value);
        app.flush().unwrap();
        assert_eq!(editor.get(), Some(input));
        let tree = app.tree();
        let body = tree.node(body).unwrap().descriptor.view();
        assert_eq!(body.semantics.as_ref().unwrap().inert, value);
        assert_eq!(
            tree.node(root).unwrap().descriptor.view().style.height,
            None
        );
        assert!(
            !tree
                .node(root)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden
        );
        assert!(
            matches!(tree.node(input).unwrap().descriptor.view().props,Props::Input(p) if p.value=="Retained note")
        );
    }
    app.dispatch(action, glory::render::Event::Activate)
        .unwrap();
    assert_eq!(calls.get(), 1);
    app.clear().unwrap();
    assert_eq!(editor.get(), None);
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn loading_hides_without_replacing_placeholder_and_owner_retirement_clears_ref() {
    let app = App::memory();
    let loading = app.scope().cage(true);
    let reference = app.scope().cage(None);
    let mounted = app
        .scope()
        .mount_with(|| skeleton().loading(loading).node_ref(reference))
        .unwrap();
    app.flush().unwrap();
    let root = reference.get().unwrap();
    let child = app.tree().node(root).unwrap().children[0];
    for value in [false, true] {
        loading.revise(|old| *old = value);
        app.flush().unwrap();
        let tree = app.tree();
        let node = tree.node(root).unwrap();
        assert_eq!(node.children[0], child);
        assert_eq!(
            node.descriptor.view().semantics.as_ref().unwrap().hidden,
            !value
        );
        assert_eq!(
            node.descriptor.view().semantics.as_ref().unwrap().busy,
            Some(value)
        );
    }
    drop(mounted);
    app.flush().unwrap();
    assert_eq!(reference.get(), None);
    assert!(app.tree().is_empty());
}
#[test]
fn dimension_validation_precedes_nodes_and_reactive_rejection_recovers() {
    for widget in [
        skeleton().width(Length::Auto),
        skeleton().height(f64::NAN),
        skeleton().height(0.0),
        skeleton().radius(f64::INFINITY),
        skeleton().circle(-1.0),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().is_empty());
    }
    let app = App::memory();
    let height = app.scope().cage(20.0);
    app.mount(skeleton().height(height)).unwrap();
    let id = app.tree().roots()[0];
    height.revise(|value| *value = f64::NAN);
    assert!(app.flush().is_err());
    assert_eq!(
        app.tree().node(id).unwrap().descriptor.view().style.height,
        Some(Length::Px(20.0))
    );
    height.revise(|value| *value = 32.0);
    app.flush().unwrap();
    assert_eq!(
        app.tree().node(id).unwrap().descriptor.view().style.height,
        Some(Length::Px(32.0))
    );
}
#[test]
fn unstyled_preserves_dimensions_and_explicit_styles_while_ssr_keeps_graphic_decorative() {
    let app = App::memory();
    app.mount(
        skeleton()
            .circle(40.0)
            .unstyled()
            .class("custom")
            .style(Style {
                background: Some(Color(1, 2, 3, 255)),
                ..Style::new()
            }),
    )
    .unwrap();
    let tree = app.tree();
    let root = tree.node(tree.roots()[0]).unwrap();
    let d = root.descriptor.view();
    assert_eq!(d.style.width, Some(Length::Px(40.0)));
    assert_eq!(d.style.height, Some(Length::Px(40.0)));
    assert_eq!(d.style.radius, Some(20.0));
    assert_eq!(d.style.background, Some(Color(1, 2, 3, 255)));
    assert!(
        tree.node(root.children[0])
            .unwrap()
            .descriptor
            .view()
            .style
            .background
            .is_none()
    );
    let html = glory::ssr::render(skeleton().name("Loading profile")).unwrap();
    assert!(html.contains("aria-busy=\"true\""));
    assert!(html.contains("<svg"));
    assert!(html.contains("aria-hidden=\"true\""));
}
