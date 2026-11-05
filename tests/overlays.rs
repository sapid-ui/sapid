use glory::render::{DismissReason, Event, LayerPlacement, NodeId, Semantics};
use glory::ui::App;
use sapid::{
    AnchorAlign, AnchorArrow, AnchorArrowPosition, AnchorPlacement, AnchorSide, ContentPolicy,
    DrawerSide, PopoverWidth, dialog, drawer, popover,
};
use std::cell::RefCell;
use std::rc::Rc;

fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    fn visit(app: &App, id: NodeId, class: &str, result: &mut Vec<NodeId>) {
        let tree = app.tree();
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|s| s == class) {
            result.push(id);
        }
        for child in &node.children {
            visit(app, *child, class, result);
        }
    }
    let mut result = Vec::new();
    for id in app.tree().roots() {
        visit(app, *id, class, &mut result);
    }
    result
}
fn metadata(app: &App, id: NodeId) -> Semantics {
    *app.tree()
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .clone()
        .unwrap()
}
#[test]
fn trigger_style_is_reactive_and_does_not_style_the_panel_or_replace_the_trigger() {
    use glory::style::{Length, Style};
    let app = App::memory();
    let style = app.scope().cage(Style::new().width(Length::Px(36.)));
    app.mount(
        dialog("trigger-style")
            .trigger_style(style)
            .style(Style::new().width(Length::Px(280.))),
    )
    .unwrap();
    let trigger = nodes(&app, "sapid-dialog-trigger")[0];
    let panel = nodes(&app, "sapid-dialog")[0];
    assert_eq!(
        app.tree()
            .node(trigger)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(36.))
    );
    assert_eq!(
        app.tree()
            .node(panel)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(280.))
    );
    style.revise(|old| *old = Style::new().width(Length::Px(44.)));
    app.flush().unwrap();
    assert_eq!(nodes(&app, "sapid-dialog-trigger"), [trigger]);
    assert_eq!(
        app.tree()
            .node(trigger)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(44.))
    );
    assert_eq!(
        app.tree()
            .node(panel)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(280.))
    );
}
#[test]
fn shared_layer_state_relations_and_local_dismissals() {
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let copy = calls.clone();
    app.mount(
        dialog("settings")
            .label("Settings")
            .title("Preferences")
            .description("Configure this app")
            .on_open_change(move |v| copy.borrow_mut().push(v))
            .content(glory::ui::input),
    )
    .unwrap();
    let trigger = nodes(&app, "sapid-dialog-trigger")[0];
    let panel = nodes(&app, "sapid-dialog")[0];
    assert!(metadata(&app, panel).hidden);
    assert!(nodes(&app, "sapid-layer-close").len() == 1);
    app.dispatch(trigger, Event::Activate).unwrap();
    app.flush().unwrap();
    let m = metadata(&app, panel);
    assert!(!m.hidden && m.layer.as_ref().unwrap().modal);
    assert_eq!(metadata(&app, trigger).controls, m.id);
    assert_eq!(m.layer.unwrap().return_focus, metadata(&app, trigger).id);
    app.dispatch(panel, Event::Dismiss(DismissReason::Escape))
        .unwrap();
    app.flush().unwrap();
    assert!(metadata(&app, panel).hidden);
    assert_eq!(*calls.borrow(), [true, false]);
    assert_eq!(nodes(&app, "sapid-dialog")[0], panel);
}

#[test]
fn full_screen_dialog_keeps_modal_protocol_and_fills_the_viewport_contract() {
    use glory::style::{Length, Limit};
    let app = App::memory();
    app.mount(
        dialog("full")
            .title("Full screen editor")
            .default_open(true)
            .full_screen(true)
            .content(|| glory::ui::text().content("Scrollable content")),
    )
    .unwrap();
    let panel = nodes(&app, "sapid-dialog-full-screen")[0];
    let tree = app.tree();
    let view = tree.node(panel).unwrap().descriptor.view();
    assert_eq!(view.style.width, Some(Length::Percent(100.)));
    assert_eq!(view.style.height, Some(Length::Percent(100.)));
    assert_eq!(view.style.max_width, Some(Limit::None));
    assert_eq!(view.style.max_height, Some(Limit::None));
    assert_eq!(view.style.radius, Some(0.));
    let semantics = view.semantics.as_deref().unwrap();
    assert!(!semantics.hidden);
    assert!(semantics.layer.as_ref().unwrap().modal);
    assert_eq!(
        semantics.layer.as_ref().unwrap().placement,
        LayerPlacement::Center
    );
}
#[test]
fn controlled_rejection_and_external_state_changes_are_silent() {
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let copy = calls.clone();
    let state = app.scope().cage(true);
    app.mount(
        popover("controlled")
            .open(state)
            .on_open_change(move |v| copy.borrow_mut().push(v)),
    )
    .unwrap();
    let panel = nodes(&app, "sapid-popover")[0];
    app.dispatch(panel, Event::Dismiss(DismissReason::Outside))
        .unwrap();
    app.flush().unwrap();
    assert!(!metadata(&app, panel).hidden);
    assert_eq!(*calls.borrow(), [false]);
    state.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(metadata(&app, panel).hidden);
    assert_eq!(*calls.borrow(), [false]);
}
#[test]
fn popover_slots_keep_intrinsic_buttons_accessible_names_controlled_refusal_and_owner_cleanup() {
    use glory::render::Props;
    let app = App::memory();
    let state = app.scope().cage(true);
    let capture = Rc::new(());
    let slot = capture.clone();
    app.mount(
        popover("slots")
            .label("Details")
            .close_label("Dismiss details")
            .open(state)
            .trigger_content(move || {
                let _ = &slot;
                glory::ui::text().content("Custom trigger")
            })
            .close_content(|| glory::ui::text().content("Custom close")),
    )
    .unwrap();
    for (class, name) in [
        ("sapid-popover-trigger", "Details"),
        ("sapid-layer-close", "Dismiss details"),
    ] {
        let id = nodes(&app, class)[0];
        let tree = app.tree();
        let d = tree.node(id).unwrap().descriptor.view();
        assert!(matches!(d.props, Props::Button(_)));
        assert_eq!(d.name.as_deref(), Some(name));
    }
    let close = nodes(&app, "sapid-layer-close")[0];
    app.dispatch(close, Event::Activate).unwrap();
    assert!(!metadata(&app, nodes(&app, "sapid-popover")[0]).hidden);
    assert!(state.get());
    app.clear().unwrap();
    assert_eq!(Rc::strong_count(&capture), 1);
}

#[test]
fn button_slots_reject_interactive_descendants_through_transparent_groups() {
    for close in [false, true] {
        let app = App::memory();
        let p = popover("invalid-slot");
        let p = if close {
            p.close_content(|| glory::ui::group().fill(glory::ui::input()))
        } else {
            p.trigger_content(|| glory::ui::group().fill(glory::ui::button()))
        };
        assert!(app.mount(p).is_err());
    }
}

#[test]
fn popover_anchor_loss_is_verified_by_the_shared_model_before_request_callbacks() {
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let out = calls.clone();
    app.mount(
        popover("anchor-loss")
            .default_open(true)
            .on_dismiss(move |reason| out.borrow_mut().push(reason)),
    )
    .unwrap();
    let panel = nodes(&app, "sapid-popover")[0];
    let trigger = nodes(&app, "sapid-popover-trigger")[0];
    app.dispatch(panel, Event::Dismiss(DismissReason::AnchorLost))
        .unwrap();
    assert!(calls.borrow().is_empty());
    app.scope()
        .patch(
            trigger,
            glory::render::Patch::Style(glory::render::Change::Set(Box::new(
                glory::style::Style::new().display(glory::style::Display::None),
            ))),
        )
        .unwrap();
    app.dispatch(panel, Event::Dismiss(DismissReason::AnchorLost))
        .unwrap();
    assert_eq!(*calls.borrow(), [DismissReason::AnchorLost]);
    assert!(metadata(&app, panel).hidden);
}
#[test]
fn drawer_edges_popover_alignment_and_dismiss_policy_use_typed_metadata() {
    for side in [
        DrawerSide::Left,
        DrawerSide::Right,
        DrawerSide::Top,
        DrawerSide::Bottom,
    ] {
        let app = App::memory();
        app.mount(
            drawer("edge")
                .side(side)
                .default_open(true)
                .dismiss_on_escape(false),
        )
        .unwrap();
        let panel = nodes(&app, "sapid-drawer")[0];
        let l = metadata(&app, panel).layer.unwrap();
        assert!(l.modal && !l.escape && l.outside);
        assert_eq!(
            l.placement,
            match side {
                DrawerSide::Left => LayerPlacement::Left,
                DrawerSide::Right => LayerPlacement::Right,
                DrawerSide::Top => LayerPlacement::Top,
                DrawerSide::Bottom => LayerPlacement::Bottom,
            }
        );
        assert!(
            app.dispatch(panel, Event::Dismiss(DismissReason::Escape))
                .is_err()
        );
    }
    let app = App::memory();
    app.mount(
        popover("align").placement(AnchorPlacement::new(AnchorSide::Bottom, AnchorAlign::End)),
    )
    .unwrap();
    let l = metadata(&app, nodes(&app, "sapid-popover")[0])
        .layer
        .unwrap();
    assert!(!l.modal);
    assert_eq!(
        l.placement,
        LayerPlacement::Anchor(glory::render::AnchorPlacement::new(
            glory::render::AnchorSide::Bottom,
            glory::render::AnchorAlign::End
        ))
    );
    assert!(l.anchor.is_some());
}

#[test]
fn popover_width_offset_and_arrow_are_finite_host_contracts() {
    let app = App::memory();
    app.mount(
        popover("geometry")
            .width(PopoverWidth::Target)
            .offset(15.0)
            .arrow(AnchorArrow {
                size: 10.0,
                radius: 2.0,
                offset: 12.0,
                position: AnchorArrowPosition::Side,
            }),
    )
    .unwrap();
    let layer = metadata(&app, nodes(&app, "sapid-popover")[0])
        .layer
        .unwrap();
    let LayerPlacement::Anchor(placement) = layer.placement else {
        panic!("popover must remain anchored");
    };
    assert!(placement.match_width);
    assert_eq!(placement.gap, 15.0);
    assert_eq!(placement.arrow.unwrap().size, 10.0);

    assert!(
        App::memory()
            .mount(popover("invalid-width").width(PopoverWidth::Px(f64::NAN)))
            .is_err()
    );
    assert!(
        App::memory()
            .mount(popover("invalid-arrow").arrow(AnchorArrow {
                size: 4.0,
                radius: 3.0,
                ..AnchorArrow::default()
            }))
            .is_err()
    );
}
#[test]
fn content_policy_retires_or_preserves_child_owners_and_hidden_events() {
    for policy in [ContentPolicy::Unmount, ContentPolicy::KeepMounted] {
        let app = App::memory();
        let value = app.scope().cage(None);
        app.mount(
            dialog("life")
                .content_policy(policy)
                .content(move || glory::ui::input().node_ref(value)),
        )
        .unwrap();
        assert_eq!(value.get().is_some(), policy == ContentPolicy::KeepMounted);
        let trigger = nodes(&app, "sapid-dialog-trigger")[0];
        app.dispatch(trigger, Event::Activate).unwrap();
        app.flush().unwrap();
        let first = value.get().unwrap();
        let panel = nodes(&app, "sapid-dialog")[0];
        app.dispatch(panel, Event::Dismiss(DismissReason::Outside))
            .unwrap();
        app.flush().unwrap();
        assert_eq!(value.get().is_some(), policy == ContentPolicy::KeepMounted);
        app.dispatch(trigger, Event::Activate).unwrap();
        app.flush().unwrap();
        assert_eq!(
            value.get() == Some(first),
            policy == ContentPolicy::KeepMounted
        );
    }
}
#[test]
fn ssr_layer_names_escaping_and_open_state_are_deterministic() {
    let a = glory::ssr::snapshot(
        dialog("ssr")
            .title("<unsafe>")
            .description("details")
            .default_open(true),
    )
    .unwrap();
    let b = glory::ssr::snapshot(
        dialog("ssr")
            .title("<unsafe>")
            .description("details")
            .default_open(true),
    )
    .unwrap();
    assert_eq!(a.html, b.html);
    for text in [
        "popover=\"manual\"",
        "aria-modal=\"true\"",
        "aria-haspopup=\"dialog\"",
        "aria-describedby=",
        "&lt;unsafe&gt;",
    ] {
        assert!(a.html.contains(text), "{text}: {}", a.html);
    }
}

#[test]
fn dismiss_callbacks_precede_observers_and_hidden_requests_do_not_escape() {
    use glory::render::EventKind;
    let app = App::memory();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let dismiss = calls.clone();
    let change = calls.clone();
    let observed = calls.clone();
    app.mount(
        dialog("order")
            .default_open(true)
            .on_dismiss(move |reason| {
                assert_eq!(reason, DismissReason::Outside);
                dismiss.borrow_mut().push("dismiss");
            })
            .on_open_change(move |_| change.borrow_mut().push("change"))
            .on(EventKind::Dismiss, move |_| {
                observed.borrow_mut().push("observer")
            }),
    )
    .unwrap();
    let panel = nodes(&app, "sapid-dialog")[0];
    app.dispatch(panel, Event::Dismiss(DismissReason::Outside))
        .unwrap();
    assert_eq!(*calls.borrow(), ["dismiss", "change", "observer"]);
    app.dispatch(panel, Event::Dismiss(DismissReason::Outside))
        .unwrap();
    assert_eq!(calls.borrow().len(), 3);
    app.clear().unwrap();
    assert_eq!(app.tree().len(), 0);
}

#[test]
fn pending_trigger_disable_cancels_open_proposals_and_generic_observers() {
    for kind in [
        "dialog",
        "drawer",
        "popover",
        "floating-dialog",
        "alert-dialog",
    ] {
        let app = App::memory();
        let enabled = app.scope().cage(true);
        let open = app.scope().cage(false);
        let proposals = app.scope().cage(0usize);
        let observed = app.scope().cage(0usize);
        macro_rules! configure {
            ($widget:expr) => {
                $widget
                    .enabled(enabled)
                    .open(open)
                    .on_open_change(move |_| proposals.revise(|v| *v += 1))
                    .on(glory::render::EventKind::Activate, move |_| {
                        observed.revise(|v| *v += 1)
                    })
            };
        }
        match kind {
            "dialog" => app.mount(configure!(dialog("guard"))),
            "drawer" => app.mount(configure!(drawer("guard"))),
            "popover" => app.mount(configure!(popover("guard"))),
            "floating-dialog" => {
                app.mount(configure!(sapid::floating_dialog("guard").title("Notice")))
            }
            _ => app.mount(configure!(
                sapid::alert_dialog("guard")
                    .title("Remove draft?")
                    .description("Remove the temporary draft.")
            )),
        }
        .unwrap();
        let trigger = nodes(&app, &format!("sapid-{kind}-trigger"))[0];
        enabled.revise(|v| *v = false);
        app.dispatch(trigger, Event::Activate).unwrap();
        app.flush().unwrap();
        assert_eq!(proposals.get(), 0, "{kind}");
        assert_eq!(observed.get(), 0, "{kind}");
        enabled.revise(|v| *v = true);
        app.flush().unwrap();
        app.dispatch(trigger, Event::Activate).unwrap();
        assert_eq!(proposals.get(), 1);
        assert_eq!(observed.get(), 1);
        assert!(!open.get());
    }
}
