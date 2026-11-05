use glory::prelude::*;
use glory::render::{DragPhase, Event, EventKind, NodeId, RangeCommand, SliderValue};
use sapid::{RangeThumb, SliderMark, range_slider};
use std::{cell::RefCell, rc::Rc};
fn range(app: &App, id: NodeId) -> SliderValue {
    app.tree()
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap()
        .slider
        .clone()
        .unwrap()
}
fn key(app: &App, id: NodeId, command: RangeCommand) {
    app.dispatch(id, Event::Range { from: id, command })
        .unwrap();
}
fn class_count(app: &App, class: &str) -> usize {
    let mut pending = app.tree().roots().to_vec();
    let mut count = 0;
    while let Some(id) = pending.pop() {
        let tree = app.tree();
        let node = tree.node(id).unwrap();
        count += usize::from(
            node.descriptor
                .view()
                .classes
                .iter()
                .any(|value| value == class),
        );
        pending.extend(node.children.iter().copied());
    }
    count
}

#[test]
fn current_step_and_unavailable_state_apply_to_the_next_key_in_the_same_packet() {
    let app = App::memory();
    let step = app.scope().cage(10.);
    let enabled = app.scope().cage(true);
    let lower = app.scope().cage(None);
    app.mount(
        range_slider("live-key-config")
            .default_value([20., 80.])
            .step(step)
            .enabled(enabled)
            .thumb_ref(RangeThumb::Lower, lower),
    )
    .unwrap();
    let lower = lower.get().unwrap();
    key(&app, lower, RangeCommand::Increase);
    app.scope()
        .batch(|| {
            step.revise(|v| *v = 1.);
            key(&app, lower, RangeCommand::Increase);
        })
        .unwrap();
    assert_eq!(range(&app, lower).range.value, 31.);
    app.scope()
        .batch(|| {
            enabled.revise(|v| *v = false);
            key(&app, lower, RangeCommand::Increase);
        })
        .unwrap();
    assert_eq!(range(&app, lower).range.value, 31.);
}

#[test]
fn refreshing_event_configuration_does_not_accept_a_pending_external_pair() {
    let app = App::memory();
    let value = app.scope().cage([25., 75.]);
    let step = app.scope().cage(10.);
    let lower = app.scope().cage(None);
    let proposals = app.scope().cage(Vec::new());
    app.mount(
        range_slider("pending-pair")
            .value(value)
            .step(step)
            .thumb_ref(RangeThumb::Lower, lower)
            .on_value_change(move |v| proposals.revise(|values| values.push(v))),
    )
    .unwrap();
    app.scope()
        .batch(|| {
            value.revise(|v| *v = [50., 90.]);
            step.revise(|v| *v = 1.);
            key(&app, lower.get().unwrap(), RangeCommand::Increase);
        })
        .unwrap();
    assert_eq!(proposals.get(), [[26., 75.]]);
    assert_eq!(value.get(), [50., 90.]);
}

#[test]
fn restricted_marks_keep_both_thumbs_on_declared_values_and_respect_the_gap() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    let upper = app.scope().cage(None);
    app.mount(
        range_slider("marks")
            .collision(sapid::RangeCollision::Clamp)
            .default_value([20., 80.])
            .minimum_gap(20.)
            .marks([
                SliderMark::new(0., "0"),
                SliderMark::new(20., "20"),
                SliderMark::new(80., "80"),
                SliderMark::new(100., "100"),
            ])
            .restrict_to_marks(true)
            .thumb_ref(RangeThumb::Lower, lower)
            .thumb_ref(RangeThumb::Upper, upper),
    )
    .unwrap();
    assert_eq!(class_count(&app, "sapid-range-slider-mark"), 4);
    assert_eq!(class_count(&app, "sapid-range-slider-mark-label"), 4);
    key(&app, lower.get().unwrap(), RangeCommand::Set(59.));
    assert_eq!(range(&app, lower.get().unwrap()).range.value, 20.);
    key(&app, lower.get().unwrap(), RangeCommand::Increase);
    assert_eq!(range(&app, lower.get().unwrap()).range.value, 20.);
    key(&app, upper.get().unwrap(), RangeCommand::Decrease);
    assert_eq!(range(&app, upper.get().unwrap()).range.value, 80.);
    key(&app, upper.get().unwrap(), RangeCommand::Increase);
    assert_eq!(range(&app, upper.get().unwrap()).range.value, 100.);
}

#[test]
fn invalid_range_marks_and_non_mark_restricted_values_are_rejected_atomically() {
    for widget in [
        range_slider("empty").restrict_to_marks(true),
        range_slider("order").marks([SliderMark::unlabeled(80.), SliderMark::unlabeled(20.)]),
        range_slider("value")
            .default_value([20., 75.])
            .marks([SliderMark::unlabeled(20.), SliderMark::unlabeled(80.)])
            .restrict_to_marks(true),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().is_empty());
    }
    for maximum in [-1., 5., 101., f64::INFINITY] {
        let app = App::memory();
        assert!(
            app.mount(
                range_slider("invalid-maximum-gap")
                    .minimum_gap(10.)
                    .maximum_gap(maximum)
            )
            .is_err()
        );
        assert!(app.tree().is_empty());
    }
    for domain in [
        [0., 0.],
        [100., 0.],
        [f64::NAN, 100.],
        [10., 100.],
        [0., 90.],
    ] {
        let app = App::memory();
        assert!(
            app.mount(range_slider("invalid-domain").domain(domain))
                .is_err()
        );
        assert!(app.tree().is_empty());
    }
}
#[test]
fn maximum_gap_allows_existing_wide_values_to_narrow_and_hidden_marks_still_snap() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    let upper = app.scope().cage(None);
    app.mount(
        range_slider("maximum-gap")
            .collision(sapid::RangeCollision::Clamp)
            .default_value([20., 80.])
            .maximum_gap(50.)
            .marks([
                SliderMark::new(0., "0"),
                SliderMark::unlabeled(20.).hidden(true),
                SliderMark::unlabeled(30.).hidden(true),
                SliderMark::unlabeled(70.).hidden(true),
                SliderMark::unlabeled(80.).hidden(true),
                SliderMark::new(100., "100"),
            ])
            .restrict_to_marks(true)
            .thumb_ref(RangeThumb::Lower, lower)
            .thumb_ref(RangeThumb::Upper, upper),
    )
    .unwrap();
    assert_eq!(class_count(&app, "sapid-range-slider-mark"), 2);
    assert_eq!(class_count(&app, "sapid-range-slider-mark-label"), 2);
    let low = lower.get().unwrap();
    let high = upper.get().unwrap();
    assert_eq!(range(&app, low).range.min, 20.);
    assert_eq!(range(&app, high).range.max, 80.);
    key(&app, low, RangeCommand::Decrease);
    key(&app, high, RangeCommand::Increase);
    assert_eq!(range(&app, low).range.value, 20.);
    assert_eq!(range(&app, high).range.value, 80.);
    key(&app, low, RangeCommand::Increase);
    assert_eq!(range(&app, low).range.value, 30.);
    assert_eq!(range(&app, high).range.max, 80.);
    key(&app, low, RangeCommand::Decrease);
    assert_eq!(range(&app, low).range.value, 30.);
}

#[test]
fn value_labels_follow_dragging_support_always_on_formatting_and_can_be_disabled() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    app.mount(
        range_slider("labels")
            .value_text([Some("25 °C".into()), Some("75 °C".into())])
            .thumb_ref(RangeThumb::Lower, lower),
    )
    .unwrap();
    assert_eq!(class_count(&app, "sapid-range-slider-value-label"), 2);
    let label_opacities = || {
        let mut pending = app.tree().roots().to_vec();
        let mut values = Vec::new();
        while let Some(id) = pending.pop() {
            let tree = app.tree();
            let node = tree.node(id).unwrap();
            if node
                .descriptor
                .view()
                .classes
                .iter()
                .any(|class| class == "sapid-range-slider-value-label")
            {
                values.push(node.descriptor.view().style.opacity);
            }
            pending.extend(node.children.iter().copied());
        }
        values
    };
    assert_eq!(label_opacities(), [Some(0.), Some(0.)]);
    let low = lower.get().unwrap();
    app.dispatch(
        low,
        Event::Drag {
            from: low,
            phase: DragPhase::Start,
            x: 10.,
            y: 10.,
        },
    )
    .unwrap();
    app.flush().unwrap();
    assert!(label_opacities().contains(&Some(1.)));
    app.dispatch(
        low,
        Event::Drag {
            from: low,
            phase: DragPhase::Cancel,
            x: 10.,
            y: 10.,
        },
    )
    .unwrap();
    app.flush().unwrap();
    assert_eq!(label_opacities(), [Some(0.), Some(0.)]);

    let always = App::memory();
    always
        .mount(range_slider("always").value_labels_always_on(true))
        .unwrap();
    assert_eq!(class_count(&always, "sapid-range-slider-value-label"), 2);
    let hidden = App::memory();
    hidden
        .mount(range_slider("hidden").show_value_labels(false))
        .unwrap();
    assert_eq!(class_count(&hidden, "sapid-range-slider-value-label"), 0);
}

#[test]
fn default_push_moves_the_other_thumb_atomically_and_hold_rejects_overlap() {
    let pushed = App::memory();
    let pushed_value = pushed.scope().cage([25., 65.]);
    let pushed_lower = pushed.scope().cage(None);
    let pushed_upper = pushed.scope().cage(None);
    pushed
        .mount(
            range_slider("push")
                .value(pushed_value)
                .minimum_gap(20.)
                .maximum_gap(40.)
                .thumb_ref(RangeThumb::Lower, pushed_lower)
                .thumb_ref(RangeThumb::Upper, pushed_upper)
                .on_value_change(move |next| pushed_value.revise(|value| *value = next)),
        )
        .unwrap();
    key(&pushed, pushed_lower.get().unwrap(), RangeCommand::Minimum);
    assert_eq!(pushed_value.get(), [0., 40.]);
    key(&pushed, pushed_upper.get().unwrap(), RangeCommand::Maximum);
    assert_eq!(pushed_value.get(), [60., 100.]);

    let held = App::memory();
    let held_value = held.scope().cage([25., 65.]);
    let held_lower = held.scope().cage(None);
    held.mount(
        range_slider("hold")
            .value(held_value)
            .minimum_gap(20.)
            .push_on_overlap(false)
            .thumb_ref(RangeThumb::Lower, held_lower)
            .on_value_change(move |next| held_value.revise(|value| *value = next)),
    )
    .unwrap();
    key(&held, held_lower.get().unwrap(), RangeCommand::Maximum);
    assert_eq!(held_value.get(), [25., 65.]);

    let marked = App::memory();
    let marked_value = marked.scope().cage([20., 40.]);
    let marked_lower = marked.scope().cage(None);
    marked
        .mount(
            range_slider("marked-push")
                .value(marked_value)
                .minimum_gap(20.)
                .marks([0., 20., 40., 60., 80., 100.].map(SliderMark::unlabeled))
                .restrict_to_marks(true)
                .thumb_ref(RangeThumb::Lower, marked_lower)
                .on_value_change(move |next| marked_value.revise(|value| *value = next)),
        )
        .unwrap();
    key(&marked, marked_lower.get().unwrap(), RangeCommand::Increase);
    assert_eq!(marked_value.get(), [40., 60.]);
}
#[test]
fn local_pair_clamps_neighbours_keeps_tab_order_and_unfreezes_singleton_thumb() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    let upper = app.scope().cage(None);
    app.mount(
        range_slider("local")
            .collision(sapid::RangeCollision::Clamp)
            .minimum_gap(10.)
            .thumb_ref(RangeThumb::Lower, lower)
            .thumb_ref(RangeThumb::Upper, upper),
    )
    .unwrap();
    let low = lower.get().unwrap();
    let high = upper.get().unwrap();
    assert_eq!(range(&app, low).range.max, 65.);
    key(&app, low, RangeCommand::Maximum);
    assert_eq!(range(&app, low).range.value, 65.);
    assert_eq!(range(&app, high).range.min, 75.);
    key(&app, low, RangeCommand::Minimum);
    key(&app, high, RangeCommand::Minimum);
    assert_eq!(range(&app, low).range.min, 0.);
    assert_eq!(range(&app, low).range.max, 0.);
    assert!(
        app.tree()
            .node(low)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .readonly
    );
    assert_eq!(
        app.tree().node(low).unwrap().descriptor.view().focus,
        glory::render::FocusMode::TabStop
    );
    assert!(
        !app.dispatch(
            low,
            Event::Range {
                from: low,
                command: RangeCommand::Increase
            }
        )
        .unwrap()
    );
    key(&app, high, RangeCommand::Maximum);
    assert!(
        !app.tree()
            .node(low)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .readonly
    );
    assert_eq!(lower.get(), Some(low));
    assert_eq!(upper.get(), Some(high));
}
#[test]
fn controlled_commit_only_two_thumb_events_accumulate_without_overwriting_the_other_value() {
    let app = App::memory();
    let value = app.scope().cage([25., 75.]);
    let lower = app.scope().cage(None);
    let upper = app.scope().cage(None);
    app.mount(
        range_slider("controlled")
            .collision(sapid::RangeCollision::Clamp)
            .value(value)
            .minimum_gap(10.)
            .thumb_ref(RangeThumb::Lower, lower)
            .thumb_ref(RangeThumb::Upper, upper)
            .on_value_commit(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    app.scope()
        .batch(|| {
            key(&app, lower.get().unwrap(), RangeCommand::Increase);
            key(&app, upper.get().unwrap(), RangeCommand::Decrease)
        })
        .unwrap();
    assert_eq!(value.get(), [26., 74.]);
}
#[test]
fn controlled_rejection_and_invalid_source_use_accepted_bounds_and_typed_order() {
    let app = App::memory();
    let value = app.scope().cage([25., 75.]);
    let lower = app.scope().cage(None);
    let calls = Rc::new(RefCell::new(Vec::new()));
    let changes = calls.clone();
    let committed = calls.clone();
    let observers = calls.clone();
    app.mount(
        range_slider("rejected")
            .collision(sapid::RangeCollision::Clamp)
            .value(value)
            .minimum_gap(10.)
            .thumb_ref(RangeThumb::Lower, lower)
            .on_value_change(move |v| changes.borrow_mut().push(format!("change:{v:?}")))
            .on_value_commit(move |v| committed.borrow_mut().push(format!("commit:{v:?}")))
            .on(EventKind::Range, move |_| {
                observers.borrow_mut().push("observer".into())
            }),
    )
    .unwrap();
    let low = lower.get().unwrap();
    key(&app, low, RangeCommand::Increase);
    assert_eq!(value.get(), [25., 75.]);
    assert_eq!(
        &*calls.borrow(),
        &["change:[26.0, 75.0]", "commit:[26.0, 75.0]", "observer"]
    );
    value.revise(|v| *v = [90., 10.]);
    assert!(app.flush().is_err());
    assert_eq!(range(&app, low).range.value, 25.);
    key(&app, low, RangeCommand::Maximum);
    assert_eq!(calls.borrow().get(3).unwrap(), "change:[65.0, 75.0]");
    value.revise(|v| *v = [20., 80.]);
    app.flush().unwrap();
    assert_eq!(range(&app, low).range.value, 20.);
}
#[test]
fn invalid_initial_pairs_gaps_steps_and_unbounded_names_create_no_nodes() {
    for widget in [
        range_slider("bad").default_value([90., 10.]),
        range_slider("bad").minimum_gap(101.),
        range_slider("bad").minimum_gap(-1.),
        range_slider("bad").step(0.),
        range_slider("bad").labels(["x".repeat(1_000_000), "Maximum".into()]),
        range_slider("bad").value_text([Some("x".repeat(1_000_000)), None]),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().is_empty());
    }
    for scale in [f64::NAN, f64::INFINITY, 1e13] {
        let app = App::memory();
        assert!(app.mount(range_slider("bad-scale").scale(scale)).is_err());
        assert!(app.tree().is_empty());
    }
}
#[test]
fn field_uses_two_real_controls_with_explicit_repeated_snapshot_names() {
    let app = App::memory();
    let low = app.scope().cage(None);
    let high = app.scope().cage(None);
    let readonly = app.scope().cage(false);
    let enabled = app.scope().cage(true);
    app.mount(
        sapid::field("price")
            .label("Price")
            .description("Budget")
            .required(true)
            .form_name("price")
            .readonly(readonly)
            .enabled(enabled)
            .control(move |field| {
                range_slider("price")
                    .field(field)
                    .labels(["Minimum price".into(), "Maximum price".into()])
                    .thumb_ref(RangeThumb::Lower, low)
                    .thumb_ref(RangeThumb::Upper, high)
            }),
    )
    .unwrap();
    for (id, expected) in [(low.get().unwrap(), "25"), (high.get().unwrap(), "75")] {
        let tree = app.tree();
        let node = tree.node(id).unwrap();
        let meta = node.descriptor.view().semantics.as_ref().unwrap();
        assert!(meta.required && meta.labelled_by.is_none() && meta.described_by.is_some());
        assert!(
            node.descriptor
                .view()
                .name
                .as_ref()
                .is_some_and(|name| name.ends_with("price"))
        );
        assert_eq!(meta.form_control.as_ref().unwrap().name, "price");
        assert_eq!(
            meta.form_control.as_ref().unwrap().value.as_deref(),
            Some(expected)
        );
    }
    readonly.revise(|v| *v = true);
    app.flush().unwrap();
    assert!(
        !app.dispatch(
            low.get().unwrap(),
            Event::Range {
                from: low.get().unwrap(),
                command: RangeCommand::Increase
            }
        )
        .unwrap()
    );
    enabled.revise(|v| *v = false);
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(high.get().unwrap())
            .unwrap()
            .descriptor
            .view()
            .focus,
        glory::render::FocusMode::Programmatic
    );
}
#[test]
fn pending_track_geometry_is_canceled_on_domain_changes_and_owner_retirement() {
    let app = App::memory();
    let low = app.scope().cage(None);
    let gap = app.scope().cage(10.);
    app.mount(
        range_slider("requests")
            .minimum_gap(gap)
            .thumb_ref(RangeThumb::Lower, low),
    )
    .unwrap();
    let id = low.get().unwrap();
    app.dispatch(
        id,
        Event::Drag {
            from: id,
            phase: DragPhase::Start,
            x: 10.,
            y: 10.,
        },
    )
    .unwrap();
    assert_eq!(app.stats().queries, 1);
    gap.revise(|v| *v = 20.);
    app.flush().unwrap();
    assert_eq!(app.stats().queries, 0);
    app.pump().unwrap();
    app.dispatch(
        id,
        Event::Drag {
            from: id,
            phase: DragPhase::Start,
            x: 10.,
            y: 10.,
        },
    )
    .unwrap();
    assert_eq!(app.stats().queries, 1);
    app.clear().unwrap();
    assert_eq!(app.stats().queries, 0);
    assert!(low.get().is_none());
    app.pump().unwrap();
}
#[test]
fn ssr_has_two_named_sliders_with_neighbour_limits_and_no_separator() {
    let html = glory::ssr::render(
        range_slider("ssr")
            .collision(sapid::RangeCollision::Clamp)
            .minimum_gap(10.)
            .labels(["Lower amount".into(), "Upper amount".into()]),
    )
    .unwrap();
    assert_eq!(html.matches("role=\"slider\"").count(), 2);
    assert!(html.contains("aria-label=\"Lower amount\""));
    assert!(html.contains("aria-valuemax=\"65\""));
    assert!(html.contains("aria-valuemin=\"35\""));
    assert!(!html.contains("role=\"separator\""));
}

#[test]
fn finite_scale_changes_visual_and_accessible_text_without_changing_raw_values() {
    let html = glory::ssr::render(
        range_slider("scaled")
            .scale(10.)
            .value_labels_always_on(true),
    )
    .unwrap();
    assert!(html.contains("aria-valuenow=\"25\""));
    assert!(html.contains("aria-valuenow=\"75\""));
    assert!(html.contains("aria-valuetext=\"250\""));
    assert!(html.contains("aria-valuetext=\"750\""));
    assert!(html.contains(">250<"));
    assert!(html.contains(">750<"));
}

#[test]
fn decimal_gap_boundaries_round_inwards_and_never_publish_a_too_small_distance() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    let upper = app.scope().cage(None);
    app.mount(
        range_slider("decimal")
            .collision(sapid::RangeCollision::Clamp)
            .min(0.)
            .max(1.)
            .step(0.1)
            .minimum_gap(0.1)
            .default_value([0.2, 0.7])
            .thumb_ref(RangeThumb::Lower, lower)
            .thumb_ref(RangeThumb::Upper, upper),
    )
    .unwrap();
    let low = lower.get().unwrap();
    let high = upper.get().unwrap();
    key(&app, low, RangeCommand::Maximum);
    let lower = value(&app, low);
    let upper = value(&app, high);
    assert!(upper - lower >= 0.1, "{lower}/{upper}");
    assert!(lower <= 0.6);
    key(&app, high, RangeCommand::Minimum);
    assert!(value(&app, high) - value(&app, low) >= 0.1);
}
fn value(app: &App, id: NodeId) -> f64 {
    range(app, id).range.value
}

#[test]
fn focused_thumb_uses_a_theme_ring_instead_of_the_default_outline() {
    let app = App::memory();
    let lower = app.scope().cage(None);
    let theme = sapid::Theme::blue_light();
    let ring = theme.focus_ring;
    app.mount(
        range_slider("focus-ring")
            .theme(theme)
            .thumb_ref(RangeThumb::Lower, lower),
    )
    .unwrap();
    let id = lower.get().unwrap();
    app.dispatch(id, Event::FocusChanged(true)).unwrap();
    let tree = app.tree();
    let style = &tree.node(id).unwrap().descriptor.view().style;
    assert_eq!(style.border_width, Some(glory::style::Edges::all(1.)));
    assert_eq!(style.border_color, Some(ring));
    drop(tree);
    app.dispatch(id, Event::FocusChanged(false)).unwrap();
    assert_eq!(
        app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .style
            .border_width,
        None
    );
}
