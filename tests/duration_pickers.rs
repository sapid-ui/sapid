#![cfg(feature = "dates")]
use glory::prelude::*;
use glory::render::{InputProps, NodeId, Props};
use sapid::prelude::*;

fn d(hours: u32, minutes: u8, seconds: u8) -> TimeDuration {
    TimeDuration::new(hours, minutes, seconds).unwrap()
}
fn props(app: &App, id: NodeId) -> InputProps {
    let tree = app.tree();
    let Props::Input(props) = tree.node(id).unwrap().descriptor.view().props else {
        panic!()
    };
    props.clone()
}
fn edit(app: &App, id: NodeId, value: &str) {
    app.dispatch(
        id,
        Event::ValueChanged {
            value: value.into(),
            revision: props(app, id).revision,
            composing: false,
        },
    )
    .unwrap();
}
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        result.push(id);
    }
    result
}
fn button(app: &App, name: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let view = tree.node(*id).unwrap().descriptor.view();
            matches!(view.props, Props::Button(props) if props.label == name)
                || view.name.as_deref() == Some(name)
        })
        .unwrap()
}

#[test]
fn duration_variant_accepts_more_than_twenty_four_hours_and_preserves_typed_control() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let minute = app.scope().cage(None);
    let second = app.scope().cage(None);
    let value = app.scope().cage(Some(d(123, 45, 6)));
    app.mount(
        time_picker("long-duration")
            .duration()
            .precision(TimePrecision::Second)
            .hour_ref(hour)
            .minute_ref(minute)
            .second_ref(second)
            .value(value)
            .on_value_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    assert_eq!(props(&app, hour.get().unwrap()).value, "123");
    assert_eq!(props(&app, minute.get().unwrap()).value, "45");
    assert_eq!(props(&app, second.get().unwrap()).value, "06");
    edit(&app, hour.get().unwrap(), "125");
    app.flush().unwrap();
    assert_eq!(value.get(), Some(d(125, 45, 6)));
    assert_eq!(value.get().unwrap().total_seconds(), 452_706);
}

#[test]
fn minimum_hour_digits_placeholder_dynamic_width_partial_draft_and_clear_are_real() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let minute = app.scope().cage(None);
    let value = app.scope().cage(None);
    app.mount(
        duration_picker("digits")
            .min_hours_digits(4)
            .hour_ref(hour)
            .minute_ref(minute)
            .value(value)
            .clearable(true)
            .on_value_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    let hour = hour.get().unwrap();
    let minute = minute.get().unwrap();
    assert_eq!(
        app.tree()
            .node(hour)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .placeholder
            .as_deref(),
        Some("----")
    );
    let initial_width = app.tree().node(hour).unwrap().descriptor.view().style.width;
    edit(&app, hour, "7");
    app.flush().unwrap();
    assert_eq!(value.get(), None);
    assert!(all(&app).contains(&button(&app, "Clear duration")));
    edit(&app, minute, "05");
    app.flush().unwrap();
    assert_eq!(value.get(), Some(d(7, 5, 0)));
    assert_eq!(props(&app, hour).value, "7");
    edit(&app, hour, "123456");
    app.flush().unwrap();
    assert_eq!(value.get(), Some(d(123_456, 5, 0)));
    assert_ne!(
        app.tree().node(hour).unwrap().descriptor.view().style.width,
        initial_width
    );
    app.dispatch(button(&app, "Clear duration"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), None);
    assert_eq!(props(&app, hour).value, "");
}

#[test]
fn duration_controlled_refusal_and_finite_admission_restore_last_accepted_value() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(d(30, 0, 0)));
    let calls = app.scope().cage(0usize);
    app.mount(
        duration_picker("refused")
            .hour_ref(hour)
            .value(value)
            .on_value_change(move |_| calls.revise(|count| *count += 1)),
    )
    .unwrap();
    edit(&app, hour.get().unwrap(), "40");
    app.flush().unwrap();
    assert_eq!(value.get(), Some(d(30, 0, 0)));
    assert_eq!(props(&app, hour.get().unwrap()).value, "30");
    assert_eq!(calls.get(), 1);
    for widget in [
        duration_picker("bad-digits").min_hours_digits(0),
        duration_picker("bad-step").hour_step(0),
        duration_picker("bad-label").clear_label("bad\nlabel"),
    ] {
        let invalid = App::memory();
        assert!(invalid.mount(widget).is_err());
        assert_eq!(invalid.stats().nodes, 0);
    }
    assert!(TimeDuration::new(MAX_DURATION_HOURS + 1, 0, 0).is_err());
}
