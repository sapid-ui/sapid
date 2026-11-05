#![cfg(feature = "dates")]
use glory::prelude::*;
use glory::render::{
    DismissReason, InputEdit, InputEditIntent, InputProps, NodeId, Props, Semantics, TextSelection,
};
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
fn t(s: &str) -> LocalTime {
    LocalTime::parse_iso(s).unwrap()
}
fn props(app: &App, id: NodeId) -> InputProps {
    let tree = app.tree();
    let Props::Input(p) = tree.node(id).unwrap().descriptor.view().props else {
        panic!()
    };
    p.clone()
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
fn raw_edit(app: &App, id: NodeId, value: &str, composing: bool) {
    let end = value.encode_utf16().count();
    app.dispatch(
        id,
        Event::InputEdit(InputEdit::Value {
            value: value.into(),
            revision: props(app, id).revision,
            selection: TextSelection {
                start: end,
                end,
                ..Default::default()
            },
            before: Some(TextSelection::default()),
            intent: InputEditIntent::Replace,
            composing,
        }),
    )
    .unwrap();
}

#[test]
fn captured_numeric_edits_and_terminal_ime_echo_update_the_time_once() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(t("09:30")));
    let calls = Rc::new(RefCell::new(Vec::new()));
    let output = calls.clone();
    app.mount(
        time_picker("raw-numeric")
            .value(value)
            .hour_ref(hour)
            .on_value_change(move |next| {
                output.borrow_mut().push(next);
                value.revise(|old| *old = next);
            }),
    )
    .unwrap();
    let hour = hour.get().unwrap();
    raw_edit(&app, hour, "10", false);
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("10:30")));
    app.scope()
        .batch(|| {
            app.dispatch(hour, Event::Composition { active: true })
                .unwrap();
            raw_edit(&app, hour, "11", true);
            app.dispatch(hour, Event::Composition { active: false })
                .unwrap();
            raw_edit(&app, hour, "11", false);
            raw_edit(&app, hour, "11", false);
        })
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("11:30")));
    assert_eq!(&*calls.borrow(), &[Some(t("10:30")), Some(t("11:30"))]);
    assert_eq!(props(&app, hour).value, "11");
    app.clear().unwrap();
}

#[test]
fn full_time_capture_is_atomic_and_reasserts_equal_pastes_without_numeric_rejection() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(None);
    let rejects = Rc::new(RefCell::new(Vec::new()));
    let errors = rejects.clone();
    app.mount(
        time_picker("raw-whole")
            .value(value)
            .hour_ref(hour)
            .paste_split(|raw| (raw == "noon").then(|| t("12:00")))
            .on_value_change(move |next| value.revise(|old| *old = next))
            .on_reject(move |error| errors.borrow_mut().push(error)),
    )
    .unwrap();
    let hour = hour.get().unwrap();
    for _ in 0..2 {
        raw_edit(&app, hour, "noon", false);
        app.flush().unwrap();
        assert_eq!(value.get(), Some(t("12:00")));
        assert_eq!(props(&app, hour).value, "12");
    }
    assert!(rejects.borrow().is_empty());
    app.clear().unwrap();
}

#[test]
fn capture_budget_rejection_keeps_the_value_without_calling_the_full_value_parser() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(t("09:30")));
    let parsed = Rc::new(std::cell::Cell::new(0));
    let parser = parsed.clone();
    let rejects = Rc::new(RefCell::new(Vec::new()));
    let errors = rejects.clone();
    app.mount(
        time_picker("raw-budget")
            .value(value)
            .hour_ref(hour)
            .paste_split(move |_| {
                parser.set(parser.get() + 1);
                None
            })
            .on_value_change(move |next| value.revise(|old| *old = next))
            .on_reject(move |error| errors.borrow_mut().push(error)),
    )
    .unwrap();
    let hour = hour.get().unwrap();
    app.dispatch(
        hour,
        Event::InputEdit(InputEdit::Rejected {
            revision: props(&app, hour).revision,
            reason: glory::render::InputEditRejection::TooLong,
        }),
    )
    .unwrap();
    app.flush().unwrap();
    assert_eq!(parsed.get(), 0);
    assert_eq!(value.get(), Some(t("09:30")));
    assert_eq!(&*rejects.borrow(), &[TimePickerStatus::TooLong]);
    assert_eq!(props(&app, hour).value, "09");
    app.clear().unwrap();
}
fn commit_edit(app: &App, id: NodeId, value: &str) {
    app.dispatch(
        id,
        Event::Commit {
            value: value.into(),
            revision: props(app, id).revision,
        },
    )
    .unwrap();
}
fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut result = vec![];
    while let Some(id) = todo.pop() {
        todo.extend(tree.node(id).unwrap().children.iter().copied());
        result.push(id)
    }
    result
}
fn button(app: &App, name: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let d = tree.node(*id).unwrap().descriptor.view();
            matches!(d.props,Props::Button(p)if p.label==name) || d.name.as_deref() == Some(name)
        })
        .unwrap()
}
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    all(app)
        .into_iter()
        .filter(|id| {
            app.tree().node(*id).is_ok_and(|node| {
                node.descriptor
                    .view()
                    .classes
                    .iter()
                    .any(|item| item == class)
            })
        })
        .collect()
}
fn semantics(app: &App, id: NodeId) -> Semantics {
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
fn real_segment_spinbuttons_keep_partial_empty_and_complete_time_values_distinct() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let m = app.scope().cage(None);
    let v = app.scope().cage(None);
    let drafts = Rc::new(RefCell::new(Vec::new()));
    let calls = drafts.clone();
    app.mount(
        time_picker("segments")
            .hour_ref(h)
            .minute_ref(m)
            .value(v)
            .on_value_change(move |next| v.revise(|old| *old = next))
            .on_draft_change(move |draft| calls.borrow_mut().push(draft.status)),
    )
    .unwrap();
    let h = h.get().unwrap();
    let m = m.get().unwrap();
    let tree = app.tree();
    let hd = tree.node(h).unwrap().descriptor.view();
    assert_eq!(hd.effective_role(), Some(Role::SpinButton));
    assert_eq!(
        hd.semantics
            .as_ref()
            .unwrap()
            .spinbutton
            .as_ref()
            .unwrap()
            .max,
        Some(23.)
    );
    drop(tree);
    edit(&app, h, "12");
    app.flush().unwrap();
    assert_eq!(v.get(), None);
    assert_eq!(drafts.borrow().last(), Some(&TimePickerStatus::Incomplete));
    edit(&app, m, "34");
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("12:34")));
    assert_eq!(props(&app, h).value, "12");
    assert_eq!(props(&app, m).value, "34");
    edit(&app, h, "");
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("12:34")));
    edit(&app, m, "");
    app.flush().unwrap();
    assert_eq!(v.get(), None);
}
#[test]
fn noon_midnight_afternoon_toggle_and_second_segments_keep_full_wall_clock_precision() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let m = app.scope().cage(None);
    let s = app.scope().cage(None);
    let v = app.scope().cage(Some(t("12:00:00")));
    app.mount(
        time_picker("twelve")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_ref(h)
            .minute_ref(m)
            .second_ref(s)
            .value(v)
            .on_value_change(move |next| v.revise(|old| *old = next)),
    )
    .unwrap();
    let toggle = button(&app, "AM/PM");
    assert_eq!(props(&app, h.get().unwrap()).value, "12");
    app.dispatch(toggle, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("00:00:00")));
    app.scope()
        .batch(|| {
            edit(&app, h.get().unwrap(), "01");
            edit(&app, m.get().unwrap(), "02");
            edit(&app, s.get().unwrap(), "03");
        })
        .unwrap();
    assert_eq!(v.get(), Some(t("01:02:03")));
    app.dispatch(toggle, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("13:02:03")));
}
#[test]
fn same_tick_segments_compose_and_commit_pads_without_resetting_unedited_segments() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let m = app.scope().cage(None);
    let s = app.scope().cage(None);
    let v = app.scope().cage(None);
    let commits = app.scope().cage(0usize);
    app.mount(
        time_picker("batch")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_ref(h)
            .minute_ref(m)
            .second_ref(s)
            .value(v)
            .on_value_change(move |next| v.revise(|old| *old = next))
            .on_commit(move |_| commits.revise(|n| *n += 1)),
    )
    .unwrap();
    app.scope()
        .batch(|| {
            edit(&app, h.get().unwrap(), "23");
            edit(&app, m.get().unwrap(), "59");
            edit(&app, s.get().unwrap(), "8");
        })
        .unwrap();
    assert_eq!(v.get(), Some(t("23:59:08")));
    assert_eq!(props(&app, s.get().unwrap()).value, "8");
    commit_edit(&app, s.get().unwrap(), "8");
    app.flush().unwrap();
    assert_eq!(props(&app, s.get().unwrap()).value, "08");
    assert_eq!(props(&app, m.get().unwrap()).value, "59");
    assert_eq!(commits.get(), 1);
}
#[test]
fn controlled_refusal_invalid_source_recovery_and_overbudget_segment_use_last_accepted_state() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let m = app.scope().cage(None);
    let v = app.scope().cage(Some(t("10:20")));
    let calls = app.scope().cage(0usize);
    let rejects = app.scope().cage(0usize);
    app.mount(
        time_picker("controlled")
            .hour_ref(h)
            .minute_ref(m)
            .value(v)
            .on_value_change(move |_| calls.revise(|n| *n += 1))
            .on_reject(move |_| rejects.revise(|n| *n += 1)),
    )
    .unwrap();
    let h = h.get().unwrap();
    edit(&app, h, "11");
    app.flush().unwrap();
    assert_eq!(props(&app, h).value, "10");
    assert_eq!(calls.get(), 1);
    v.revise(|v| *v = Some(t("10:20:01")));
    assert!(app.flush().is_err());
    assert_eq!(props(&app, h).value, "10");
    v.revise(|v| *v = Some(t("12:30")));
    app.flush().unwrap();
    assert_eq!(props(&app, h).value, "12");
    assert_eq!(props(&app, m.get().unwrap()).value, "30");
    assert_eq!(calls.get(), 1);
    edit(&app, h, "123");
    app.flush().unwrap();
    assert_eq!(props(&app, h).value, "12");
    assert_eq!(rejects.get(), 1);
}
#[test]
fn grid_selection_and_unavailable_typed_time_share_one_value_without_clamping() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let m = app.scope().cage(None);
    let v = app.scope().cage(Some(t("09:00")));
    let status = app.scope().cage(TimePickerStatus::Empty);
    let rejects = app.scope().cage(0usize);
    let spec = TimeGridSpec::new(t("09:00"), t("10:00"), 1800, 2).unwrap();
    app.mount(
        time_picker("grid")
            .grid(spec)
            .hour_ref(h)
            .minute_ref(m)
            .value(v)
            .disable_time(t("09:30"))
            .on_value_change(move |next| v.revise(|old| *old = next))
            .on_draft_change(move |draft| status.revise(|s| *s = draft.status))
            .on_reject(move |_| rejects.revise(|n| *n += 1)),
    )
    .unwrap();
    assert!(
        !app.tree()
            .node(button(&app, "09:30"))
            .unwrap()
            .descriptor
            .view()
            .props
            .enabled()
    );
    edit(&app, m.get().unwrap(), "30");
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("09:00")));
    assert_eq!(status.get(), TimePickerStatus::Unavailable(t("09:30")));
    commit_edit(&app, m.get().unwrap(), "30");
    app.flush().unwrap();
    assert_eq!(rejects.get(), 1);
    app.dispatch(button(&app, "10:00"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(v.get(), Some(t("10:00")));
    assert_eq!(props(&app, h.get().unwrap()).value, "10");
    assert_eq!(props(&app, m.get().unwrap()).value, "00");
    assert_eq!(status.get(), TimePickerStatus::Valid(t("10:00")));
}
#[test]
fn field_label_targets_real_hour_and_successful_names_are_explicit_independent_segments() {
    let app = App::memory();
    let root = app.scope().cage(None);
    let hour = app.scope().cage(None);
    let minute = app.scope().cage(None);
    app.mount(
        field("time-field")
            .label("Meeting segments")
            .required(true)
            .form_name("legacy-hour")
            .control(move |f| {
                time_picker("field")
                    .field(f)
                    .form_name("meeting")
                    .node_ref(root)
                    .hour_ref(hour)
                    .minute_ref(minute)
            }),
    )
    .unwrap();
    let tree = app.tree();
    let group_view = tree.node(root.get().unwrap()).unwrap().descriptor.view();
    assert_eq!(group_view.role, Some(Role::Group));
    assert!(group_view.semantics.as_ref().unwrap().labelled_by.is_some());
    let hour = tree.node(hour.get().unwrap()).unwrap().descriptor.view();
    let hs = hour.semantics.as_ref().unwrap();
    assert!(hs.required);
    assert_eq!(hs.form_control.as_ref().unwrap().name, "meeting_hour");
    assert!(hs.id.is_some());
    let minute = tree.node(minute.get().unwrap()).unwrap().descriptor.view();
    assert!(minute.semantics.as_ref().unwrap().required);
    assert_eq!(
        minute
            .semantics
            .as_ref()
            .unwrap()
            .form_control
            .as_ref()
            .unwrap()
            .name,
        "meeting_minute"
    );
}
#[test]
fn readonly_disabled_and_owner_retirement_control_real_spin_and_period_actions() {
    let app = App::memory();
    let h = app.scope().cage(None);
    let root = app.scope().cage(None);
    let readonly = app.scope().cage(false);
    let enabled = app.scope().cage(true);
    let visible = app.scope().cage(true);
    let v = app.scope().cage(Some(t("10:20")));
    app.mount(glory::ui::when(visible, move || {
        time_picker("owner")
            .hour_ref(h)
            .node_ref(root)
            .value(v)
            .readonly(readonly)
            .enabled(enabled)
            .on_value_change(move |next| v.revise(|old| *old = next))
    }))
    .unwrap();
    let old = h.get().unwrap();
    readonly.revise(|v| *v = true);
    app.flush().unwrap();
    assert!(
        !app.dispatch(
            old,
            Event::ValueChanged {
                value: "11".into(),
                revision: props(&app, old).revision,
                composing: false
            }
        )
        .unwrap()
    );
    assert_eq!(v.get(), Some(t("10:20")));
    enabled.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(!app.tree().enabled(old));
    visible.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(root.get().is_none() && h.get().is_none());
    assert_eq!(app.stats().queries, 0);
    assert!(!app.dispatch(old, Event::Activate).unwrap());
}

#[test]
fn explicit_segment_steps_use_real_spin_protocol_and_reject_zero_before_building() {
    let app = App::memory();
    let minute = app.scope().cage(None);
    let value = app.scope().cage(Some(t("10:20")));
    let calls = app.scope().cage(0usize);
    app.mount(
        time_picker("steps")
            .minute_step(15)
            .minute_ref(minute)
            .value(value)
            .on_value_change(move |v| {
                value.revise(|old| *old = v);
                calls.revise(|n| *n += 1)
            }),
    )
    .unwrap();
    let id = minute.get().unwrap();
    app.dispatch(
        id,
        Event::Spin {
            from: id,
            command: glory::render::RangeCommand::Increase,
        },
    )
    .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("10:35")));
    assert_eq!(calls.get(), 1);
    assert_eq!(
        app.tree()
            .node(id)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .spinbutton
            .as_ref()
            .unwrap()
            .step,
        15.
    );
    let bad = App::memory();
    assert!(bad.mount(time_picker("bad-step").minute_step(0)).is_err());
    assert_eq!(bad.stats().nodes, 0);
}

#[test]
fn clearable_uses_the_controlled_value_protocol_and_respects_control_state() {
    let app = App::memory();
    let value = app.scope().cage(Some(t("10:20")));
    let readonly = app.scope().cage(false);
    let calls = app.scope().cage(0usize);
    app.mount(
        time_picker("clearable")
            .value(value)
            .readonly(readonly)
            .clearable(true)
            .clear_label("Clear appointment time")
            .on_value_change(move |next| {
                value.revise(|old| *old = next);
                calls.revise(|count| *count += 1);
            }),
    )
    .unwrap();
    let clear = button(&app, "Clear appointment time");
    app.dispatch(clear, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), None);
    assert_eq!(calls.get(), 1);
    assert!(!all(&app).contains(&clear));

    value.revise(|value| *value = Some(t("11:30")));
    readonly.revise(|value| *value = true);
    app.flush().unwrap();
    let clear = button(&app, "Clear appointment time");
    assert!(!app.tree().enabled(clear));
    assert!(!app.dispatch(clear, Event::Activate).unwrap());
    assert_eq!(value.get(), Some(t("11:30")));

    let refused = App::memory();
    let fixed = refused.scope().cage(Some(t("09:00")));
    let proposals = refused.scope().cage(0usize);
    refused
        .mount(
            time_picker("clear-refused")
                .value(fixed)
                .clearable(true)
                .on_value_change(move |_| proposals.revise(|count| *count += 1)),
        )
        .unwrap();
    let clear = button(&refused, "Clear time");
    refused.dispatch(clear, Event::Activate).unwrap();
    refused.flush().unwrap();
    assert_eq!(fixed.get(), Some(t("09:00")));
    assert_eq!(proposals.get(), 1);
    assert_eq!(
        props(
            &refused,
            all(&refused)
                .into_iter()
                .find(|id| {
                    refused
                        .tree()
                        .node(*id)
                        .is_ok_and(|node| node.descriptor.view().name.as_deref() == Some("Hour"))
                })
                .unwrap()
        )
        .value,
        "09"
    );

    let partial = App::memory();
    let partial_value = partial.scope().cage(None);
    let partial_hour = partial.scope().cage(None);
    partial
        .mount(
            time_picker("clear-partial")
                .value(partial_value)
                .hour_ref(partial_hour)
                .clearable(true)
                .on_value_change(move |next| partial_value.revise(|old| *old = next)),
        )
        .unwrap();
    let hour = partial_hour.get().unwrap();
    assert_eq!(
        partial
            .tree()
            .node(hour)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .placeholder
            .as_deref(),
        Some("--")
    );
    edit(&partial, hour, "7");
    partial.flush().unwrap();
    assert_eq!(partial_value.get(), None);
    let clear = button(&partial, "Clear time");
    partial.dispatch(clear, Event::Activate).unwrap();
    partial.flush().unwrap();
    assert_eq!(props(&partial, hour).value, "");
    assert!(!all(&partial).contains(&clear));

    let invalid = App::memory();
    assert!(
        invalid
            .mount(
                time_picker("bad-clear-label")
                    .clearable(true)
                    .clear_label("bad\nlabel")
            )
            .is_err()
    );
    assert_eq!(invalid.stats().nodes, 0);
}

#[test]
fn preset_popup_is_anchored_selects_through_the_value_protocol_and_optionally_closes() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(t("09:00")));
    let open = app.scope().cage(false);
    let open_calls = Rc::new(RefCell::new(Vec::new()));
    let opened = open_calls.clone();
    app.mount(
        time_picker("presets")
            .hour_ref(hour)
            .value(value)
            .open(open)
            .presets(vec![t("09:00"), t("12:30"), t("17:45")])
            .close_on_preset_select(true)
            .on_value_change(move |next| value.revise(|old| *old = next))
            .on_open_change(move |next| {
                open.revise(|old| *old = next);
                opened.borrow_mut().push(next);
            }),
    )
    .unwrap();
    let trigger = nodes(&app, "sapid-time-picker-preset-trigger")[0];
    let panel = nodes(&app, "sapid-popover")[0];
    let relation = semantics(&app, trigger);
    assert_eq!(relation.controls, semantics(&app, panel).id);
    assert_eq!(relation.expanded, Some(false));
    assert!(semantics(&app, panel).hidden);

    app.dispatch(hour.get().unwrap(), Event::FocusChanged(true))
        .unwrap();
    app.flush().unwrap();
    assert!(!semantics(&app, panel).hidden);
    assert_eq!(semantics(&app, trigger).expanded, Some(true));
    app.dispatch(button(&app, "12:30"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("12:30")));
    assert!(semantics(&app, panel).hidden);
    assert_eq!(*open_calls.borrow(), [true, false]);
}

#[test]
fn grouped_presets_format_selected_and_unavailable_items_and_support_dismissal() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(Some(t("13:15:00")));
    let open = app.scope().cage(false);
    app.mount(
        time_picker("grouped-presets")
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_ref(hour)
            .value(value)
            .open(open)
            .bounds(TimeBounds::new(Some(t("08:00:00")), Some(t("18:00:00"))).unwrap())
            .disable_time(t("13:15:00"))
            .presets(vec![
                TimePickerPresetGroup::new("Morning", [t("07:00:00"), t("09:30:00")]),
                TimePickerPresetGroup::new("Afternoon", [t("13:15:00"), t("17:45:00")]),
            ])
            .on_value_change(move |next| value.revise(|old| *old = next))
            .on_open_change(move |next| open.revise(|old| *old = next)),
    )
    .unwrap();
    app.dispatch(hour.get().unwrap(), Event::FocusChanged(true))
        .unwrap();
    app.flush().unwrap();
    assert_eq!(nodes(&app, "sapid-time-picker-preset-group-label").len(), 2);
    assert!(!app.tree().enabled(button(&app, "07:00:00 AM")));
    assert!(!app.tree().enabled(button(&app, "01:15:00 PM")));
    assert!(app.tree().enabled(button(&app, "05:45:00 PM")));
    let selected = semantics(&app, button(&app, "01:15:00 PM"));
    assert_eq!(selected.pressed, Some(true));
    let panel = nodes(&app, "sapid-popover")[0];
    app.dispatch(panel, Event::Dismiss(DismissReason::Escape))
        .unwrap();
    app.flush().unwrap();
    assert!(semantics(&app, panel).hidden);
}

#[test]
fn preset_admission_and_controlled_open_refusal_are_finite_and_transactional() {
    let refused = App::memory();
    let hour = refused.scope().cage(None);
    let open = refused.scope().cage(false);
    let proposals = refused.scope().cage(0usize);
    refused
        .mount(
            time_picker("refused-presets")
                .hour_ref(hour)
                .open(open)
                .presets(vec![t("09:00")])
                .on_open_change(move |_| proposals.revise(|count| *count += 1)),
        )
        .unwrap();
    refused
        .dispatch(hour.get().unwrap(), Event::FocusChanged(true))
        .unwrap();
    refused.flush().unwrap();
    assert_eq!(proposals.get(), 1);
    assert!(!open.get());
    assert!(semantics(&refused, nodes(&refused, "sapid-popover")[0]).hidden);

    for widget in [
        time_picker("duplicate-presets").presets(vec![t("09:00"), t("09:00")]),
        time_picker("empty-presets").presets(Vec::<LocalTime>::new()),
        time_picker("grid-and-presets")
            .grid(TimeGridSpec::new(t("09:00"), t("10:00"), 1800, 2).unwrap())
            .presets(vec![t("09:00")]),
    ] {
        let invalid = App::memory();
        assert!(invalid.mount(widget).is_err());
        assert_eq!(invalid.stats().nodes, 0);
    }
    let too_many = App::memory();
    assert!(
        too_many
            .mount(
                time_picker("too-many-presets")
                    .presets(vec![t("09:00"); MAX_TIME_PICKER_PRESETS + 1])
            )
            .is_err()
    );
    assert_eq!(too_many.stats().nodes, 0);
}

#[test]
fn segment_labels_period_labels_focus_boundary_and_popup_geometry_are_configurable() {
    let app = App::memory();
    let root = app.scope().cage(None);
    let hour = app.scope().cage(None);
    let focused = app.scope().cage(0usize);
    let blurred = app.scope().cage(0usize);
    let placement = AnchorPlacement {
        side: AnchorSide::Top,
        align: AnchorAlign::Start,
        gap: 6.,
        viewport_padding: 10.,
        flip: false,
        shift: true,
        ..AnchorPlacement::default()
    };
    app.mount(
        time_picker("labels-placement")
            .node_ref(root)
            .hour_ref(hour)
            .format(TimeFormat::new(
                HourCycle::H12,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_label("Hours")
            .minute_label("Minutes")
            .second_label("Seconds")
            .period_label("Period")
            .period_labels("पूर्वाह्न", "अपराह्न")
            .presets(vec![t("09:00:00")])
            .popup_placement(placement)
            .popup_width(360.)
            .on_focus(move || focused.revise(|count| *count += 1))
            .on_blur(move || blurred.revise(|count| *count += 1)),
    )
    .unwrap();
    let hour = hour.get().unwrap();
    assert_eq!(
        app.tree()
            .node(hour)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref(),
        Some("Hours")
    );
    let period = button(&app, "Period");
    let tree = app.tree();
    let Props::Button(period_props) = tree.node(period).unwrap().descriptor.view().props else {
        panic!()
    };
    assert_eq!(period_props.label, "पूर्वाह्न");
    drop(tree);
    let panel = nodes(&app, "sapid-popover")[0];
    assert_eq!(
        semantics(&app, panel).layer.unwrap().placement,
        glory::render::LayerPlacement::Anchor(placement)
    );
    assert_eq!(
        app.tree()
            .node(panel)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(glory::style::Length::Px(360.))
    );
    app.dispatch(hour, Event::FocusChanged(true)).unwrap();
    app.flush().unwrap();
    assert_eq!(focused.get(), 1);
    app.dispatch(hour, Event::FocusChanged(false)).unwrap();
    app.flush().unwrap();
    assert_eq!(blurred.get(), 1);

    let invalid = App::memory();
    assert!(
        invalid
            .mount(
                time_picker("bad-popup-width")
                    .presets(vec![t("09:00")])
                    .popup_width(20.)
            )
            .is_err()
    );
    assert_eq!(invalid.stats().nodes, 0);
}

#[test]
fn paste_split_populates_all_segments_with_default_and_custom_bounded_parsers() {
    fn paste(app: &App, node: NodeId, value: &str) {
        let end = value.encode_utf16().count();
        app.dispatch(
            node,
            Event::InputEdit(InputEdit::Value {
                value: value.into(),
                revision: props(app, node).revision,
                selection: TextSelection {
                    start: end,
                    end,
                    ..Default::default()
                },
                before: Some(TextSelection::default()),
                intent: InputEditIntent::Insert,
                composing: false,
            }),
        )
        .unwrap();
    }
    let app = App::memory();
    let hour = app.scope().cage(None);
    let minute = app.scope().cage(None);
    let second = app.scope().cage(None);
    let value = app.scope().cage(None);
    app.mount(
        time_picker("paste")
            .format(TimeFormat::new(
                HourCycle::H24,
                TimePrecision::Second,
                DateLocale::En,
            ))
            .hour_ref(hour)
            .minute_ref(minute)
            .second_ref(second)
            .value(value)
            .on_value_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    paste(&app, hour.get().unwrap(), "17:33:43");
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("17:33:43")));
    assert_eq!(props(&app, minute.get().unwrap()).value, "33");
    assert_eq!(props(&app, second.get().unwrap()).value, "43");

    let custom = App::memory();
    let custom_hour = custom.scope().cage(None);
    let custom_value = custom.scope().cage(None);
    custom
        .mount(
            time_picker("custom-paste")
                .hour_ref(custom_hour)
                .value(custom_value)
                .paste_split(|pasted| match pasted {
                    "noon" => LocalTime::new(12, 0, 0).ok(),
                    _ => None,
                })
                .on_value_change(move |next| custom_value.revise(|old| *old = next)),
        )
        .unwrap();
    paste(&custom, custom_hour.get().unwrap(), "noon");
    custom.flush().unwrap();
    assert_eq!(custom_value.get(), Some(t("12:00")));

    let disabled = App::memory();
    let disabled_hour = disabled.scope().cage(None);
    disabled
        .mount(
            time_picker("paste-disabled")
                .hour_ref(disabled_hour)
                .handle_paste(false),
        )
        .unwrap();
    assert!(
        disabled
            .tree()
            .node(disabled_hour.get().unwrap())
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .input_edit
            .is_none()
    );
}

#[test]
fn dropdown_renders_segment_columns_and_selects_through_the_same_controlled_protocol() {
    let app = App::memory();
    let hour = app.scope().cage(None);
    let value = app.scope().cage(None);
    app.mount(
        time_picker("dropdown")
            .hour_ref(hour)
            .value(value)
            .with_dropdown(true)
            .minute_step(15)
            .max_dropdown_content_height(180.)
            .on_value_change(move |next| value.revise(|old| *old = next)),
    )
    .unwrap();
    app.dispatch(hour.get().unwrap(), Event::FocusChanged(true))
        .unwrap();
    app.flush().unwrap();
    assert_eq!(nodes(&app, "sapid-time-picker-control-column").len(), 2);
    let controls = nodes(&app, "sapid-time-picker-controls");
    assert_eq!(controls.len(), 1);
    assert_eq!(
        app.tree()
            .node(controls[0])
            .unwrap()
            .descriptor
            .view()
            .style
            .max_height,
        Some(glory::style::Limit::Px(180.))
    );
    app.dispatch(button(&app, "18"), Event::Activate).unwrap();
    app.dispatch(button(&app, "30"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), Some(t("18:30")));

    let twelve = App::memory();
    let twelve_hour = twelve.scope().cage(None);
    twelve
        .mount(
            time_picker("dropdown-twelve")
                .hour_ref(twelve_hour)
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Second,
                    DateLocale::En,
                ))
                .with_dropdown(true)
                .reverse_time_controls_list(true),
        )
        .unwrap();
    twelve
        .dispatch(twelve_hour.get().unwrap(), Event::FocusChanged(true))
        .unwrap();
    twelve.flush().unwrap();
    assert_eq!(nodes(&twelve, "sapid-time-picker-control-column").len(), 4);
    assert_eq!(nodes(&twelve, "sapid-time-picker-period-control").len(), 2);

    let invalid = App::memory();
    assert!(
        invalid
            .mount(
                time_picker("bad-dropdown-height")
                    .with_dropdown(true)
                    .max_dropdown_content_height(20.)
            )
            .is_err()
    );
    assert_eq!(invalid.stats().nodes, 0);
}
