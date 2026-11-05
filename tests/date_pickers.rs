#![cfg(feature = "dates")]
use glory::prelude::*;
use glory::render::{EventKind, NodeId, Props};
use sapid::prelude::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
fn d(s: &str) -> CivilDate {
    CivilDate::parse_iso(s).unwrap()
}
fn p() -> DatesProvider {
    DatesProvider::new(d("2024-02-29")).today(Some(d("2024-02-29")))
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
fn class(app: &App, class: &str) -> Vec<NodeId> {
    all(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|c| c == class)
        })
        .collect()
}
fn day(app: &App, date: CivilDate) -> NodeId {
    let name = DateLocale::En.date_label(date);
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let view = tree.node(*id).unwrap().descriptor.view();
            matches!(view.props, Props::Button(_)) && view.name.as_deref() == Some(name.as_str())
        })
        .unwrap_or_else(|| panic!("missing {date}"))
}

fn button(app: &App, label: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let view = tree.node(*id).unwrap().descriptor.view();
            matches!(view.props, Props::Button(value) if value.label == label)
                || matches!(view.props, Props::Button(_)) && view.name.as_deref() == Some(label)
        })
        .unwrap_or_else(|| panic!("missing button {label}"))
}

fn selected(app: &App) -> usize {
    class(app, "sapid-calendar-cell")
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .selected
                == Some(true)
        })
        .count()
}

fn named_buttons(app: &App, name: &str) -> Vec<NodeId> {
    all(app)
        .into_iter()
        .filter(|id| {
            let tree = app.tree();
            let view = tree.node(*id).unwrap().descriptor.view();
            matches!(view.props, Props::Button(_)) && view.name.as_deref() == Some(name)
        })
        .collect()
}

#[test]
fn picker_uses_real_month_and_year_grids_for_level_navigation() {
    let app = App::memory();
    app.mount(date_picker("levels", p()).default_month(YearMonth::new(2024, 2).unwrap()))
        .unwrap();

    app.dispatch(button(&app, "Choose month or year"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert!(
        app.tree()
            .node(button(&app, "March 2024"))
            .unwrap()
            .descriptor
            .view()
            .props
            .enabled()
    );

    app.dispatch(button(&app, "Choose year"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert!(
        app.tree()
            .node(button(&app, "2025"))
            .unwrap()
            .descriptor
            .view()
            .props
            .enabled()
    );

    app.dispatch(button(&app, "2025"), Event::Activate).unwrap();
    app.flush().unwrap();
    app.dispatch(button(&app, "March 2025"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(named_buttons(&app, "Saturday, March 1, 2025").len(), 1);
}

#[test]
fn controlled_level_refusal_and_max_level_are_preserved() {
    let app = App::memory();
    let level = app.scope().cage(DatePickerLevel::Month);
    let proposals = app.scope().cage(0usize);
    app.mount(
        date_picker("controlled-level", p())
            .level(level)
            .on_level_change(move |_| proposals.revise(|count| *count += 1)),
    )
    .unwrap();
    app.dispatch(button(&app, "Choose month or year"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(proposals.get(), 1);
    assert_eq!(level.get(), DatePickerLevel::Month);
    assert!(
        app.tree()
            .node(day(&app, d("2024-02-29")))
            .unwrap()
            .descriptor
            .view()
            .props
            .enabled()
    );
    assert!(
        !app.tree()
            .node(button(&app, "March 2024"))
            .unwrap()
            .descriptor
            .view()
            .props
            .enabled()
    );

    let app = App::memory();
    app.mount(date_picker("month-only", p()).max_level(DatePickerLevel::Month))
        .unwrap();
    assert!(named_buttons(&app, "Choose month or year").is_empty());
}

#[test]
fn picker_and_input_forward_week_numbers_and_hidden_weekdays() {
    for input in [false, true] {
        let app = App::memory();
        if input {
            app.mount(
                date_picker_input("week-input", p())
                    .default_open(true)
                    .with_week_numbers(true)
                    .hide_weekdays(true),
            )
            .unwrap();
        } else {
            app.mount(
                date_picker("week-picker", p())
                    .with_week_numbers(true)
                    .hide_weekdays(true),
            )
            .unwrap();
        }
        assert_eq!(class(&app, "sapid-calendar-week-number").len(), 6);
        assert!(class(&app, "sapid-calendar-weekdays").is_empty());
    }
}

#[test]
fn consecutive_month_columns_share_selection_and_scroll_one_anchor() {
    let app = App::memory();
    let month = app.scope().cage(YearMonth::new(2024, 2).unwrap());
    let value = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Range));
    app.mount(
        date_picker("range-columns", p())
            .mode(DatePickerMode::Range)
            .value(value)
            .month(month)
            .number_of_columns(2)
            .columns_to_scroll(2)
            .on_value_change(move |next| value.revise(|current| *current = next))
            .on_month_change(move |next| month.revise(|current| *current = next)),
    )
    .unwrap();

    assert_eq!(class(&app, "sapid-calendar-grid").len(), 2);
    assert_eq!(named_buttons(&app, "Thursday, February 29, 2024").len(), 1);
    assert_eq!(named_buttons(&app, "Friday, March 1, 2024").len(), 1);
    assert_eq!(named_buttons(&app, "Previous month").len(), 1);
    assert_eq!(named_buttons(&app, "Next month").len(), 1);

    app.dispatch(day(&app, d("2024-02-28")), Event::Activate)
        .unwrap();
    app.dispatch(day(&app, d("2024-03-02")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(
        value.get(),
        DateSelection::Range(DateRangeSelection::Complete(
            CivilRange::new(d("2024-02-28"), d("2024-03-02")).unwrap()
        ))
    );
    assert_eq!(selected(&app), 4);

    app.dispatch(button(&app, "Next month"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(month.get(), YearMonth::new(2024, 4).unwrap());
    assert_eq!(named_buttons(&app, "Wednesday, May 1, 2024").len(), 1);
}

#[test]
fn controlled_multi_column_month_refusal_restores_the_consecutive_window() {
    let app = App::memory();
    let month = app.scope().cage(YearMonth::new(2024, 2).unwrap());
    let proposals = app.scope().cage(0usize);
    app.mount(
        date_picker("refused-columns", p())
            .month(month)
            .number_of_columns(2)
            .on_month_change(move |_| proposals.revise(|count| *count += 1)),
    )
    .unwrap();

    app.dispatch(button(&app, "Next month"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(proposals.get(), 1);
    assert_eq!(month.get(), YearMonth::new(2024, 2).unwrap());
    assert_eq!(named_buttons(&app, "Friday, March 1, 2024").len(), 1);
    assert!(named_buttons(&app, "Monday, April 1, 2024").is_empty());
}

#[test]
fn finite_presets_publish_atomic_single_multiple_and_range_values() {
    let app = App::memory();
    let single = app.scope().cage(DateSelection::Single(None));
    let multiple = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Multiple));
    let range = app
        .scope()
        .cage(DateSelection::empty(DatePickerMode::Range));
    let multiple_value = DateSelection::Multiple(
        SelectedDates::try_from_iter([d("2024-02-27"), d("2024-02-29")]).unwrap(),
    );
    let range_value = DateSelection::Range(DateRangeSelection::Complete(
        CivilRange::new(d("2024-02-26"), d("2024-02-29")).unwrap(),
    ));
    app.mount(
        glory::ui::view().fill((
            date_picker("single-presets", p())
                .value(single)
                .presets([DatePickerPreset::new(
                    "Today preset",
                    DateSelection::Single(Some(d("2024-02-29"))),
                )])
                .on_value_change(move |next| single.revise(|value| *value = next)),
            date_picker("multiple-presets", p())
                .mode(DatePickerMode::Multiple)
                .value(multiple)
                .presets([DatePickerPreset::new("Two dates preset", multiple_value)])
                .on_value_change(move |next| multiple.revise(|value| *value = next)),
            date_picker("range-presets", p())
                .mode(DatePickerMode::Range)
                .value(range)
                .presets([DatePickerPreset::new("Work week preset", range_value)])
                .on_value_change(move |next| range.revise(|value| *value = next)),
        )),
    )
    .unwrap();
    for (label, value, expected) in [
        (
            "Today preset",
            single,
            DateSelection::Single(Some(d("2024-02-29"))),
        ),
        ("Two dates preset", multiple, multiple_value),
        ("Work week preset", range, range_value),
    ] {
        app.dispatch(button(&app, label), Event::Activate).unwrap();
        app.flush().unwrap();
        assert_eq!(value.get(), expected);
    }
}

#[test]
fn date_picker_input_preset_uses_controlled_protocol_and_closes_complete_value() {
    let app = App::memory();
    let value = app.scope().cage(DateSelection::Single(None));
    let open = app.scope().cage(true);
    app.mount(
        date_picker_input("input-preset", p())
            .value(value)
            .open(open)
            .presets([DatePickerPreset::new(
                "Appointment preset",
                DateSelection::Single(Some(d("2024-02-28"))),
            )])
            .on_value_change(move |next| value.revise(|current| *current = next))
            .on_open_change(move |next| open.revise(|current| *current = next)),
    )
    .unwrap();
    app.dispatch(button(&app, "Appointment preset"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), DateSelection::Single(Some(d("2024-02-28"))));
    assert!(!open.get());
}

#[test]
fn partial_duplicate_and_over_capacity_presets_are_rejected() {
    let app = App::memory();
    assert!(
        app.mount(
            date_picker("partial-preset", p())
                .mode(DatePickerMode::Range)
                .presets([DatePickerPreset::new(
                    "Partial",
                    DateSelection::Range(DateRangeSelection::Start(d("2024-02-28"))),
                )]),
        )
        .is_err()
    );
    let value = DateSelection::Single(Some(d("2024-02-28")));
    assert!(
        app.mount(date_picker("duplicate-preset", p()).presets([
            DatePickerPreset::new("Duplicate", value),
            DatePickerPreset::new("Duplicate", value),
        ]))
        .is_err()
    );
    assert!(
        app.mount(
            date_picker("too-many-presets", p()).presets((0..=MAX_DATE_PICKER_PRESETS).map(
                |index| DatePickerPreset::new(
                    format!("Preset {index}"),
                    DateSelection::Single(Some(d("2024-02-28"))),
                )
            ),)
        )
        .is_err()
    );
}
#[test]
fn multiple_real_grid_selection_updates_in_one_tick_and_typed_precedes_generic() {
    let app = App::memory();
    let root = app.scope().cage(None);
    let calls = app.scope().cage(0usize);
    let seen = app.scope().cage(0usize);
    app.mount(
        date_picker("multi", p())
            .mode(DatePickerMode::Multiple)
            .node_ref(root)
            .on_value_change(move |_| calls.revise(|n| *n += 1))
            .on(EventKind::Activate, move |_| {
                seen.revise(|n| *n = calls.get_untracked())
            }),
    )
    .unwrap();
    assert!(
        app.tree()
            .node(root.get().unwrap())
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .grid_multiple
    );
    let a = day(&app, d("2024-02-28"));
    let b = day(&app, d("2024-02-29"));
    app.scope()
        .batch(|| {
            app.dispatch(a, Event::Activate).unwrap();
            app.dispatch(b, Event::Activate).unwrap();
        })
        .unwrap();
    assert_eq!(selected(&app), 2);
    assert_eq!(calls.get(), 2);
    assert_eq!(seen.get(), 2);
    app.dispatch(a, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(selected(&app), 1);
}
#[test]
fn range_keeps_partial_then_orders_endpoints_and_explicitly_allows_disabled_interior() {
    let app = App::memory();
    let proposals = Rc::new(RefCell::new(Vec::new()));
    let out = proposals.clone();
    app.mount(
        date_picker("range", p())
            .mode(DatePickerMode::Range)
            .exclude_date(d("2024-02-28"))
            .on_value_change(move |v| out.borrow_mut().push(v)),
    )
    .unwrap();
    app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(selected(&app), 1);
    app.dispatch(day(&app, d("2024-02-26")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(selected(&app), 4);
    assert_eq!(
        proposals.borrow().last().unwrap().format_iso(),
        "2024-02-26/2024-02-29"
    );
    app.dispatch(day(&app, d("2024-02-27")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(
        proposals.borrow().last().unwrap().format_iso(),
        "2024-02-27/"
    );
    assert_eq!(selected(&app), 1);
}
#[test]
fn controlled_refusal_and_invalid_mode_preserve_accepted_selection_and_recover() {
    let app = App::memory();
    let value = app
        .scope()
        .cage(DateSelection::Single(Some(d("2024-02-28"))));
    let calls = app.scope().cage(0usize);
    app.mount(
        date_picker("controlled", p())
            .value(value)
            .on_value_change(move |_| calls.revise(|n| *n += 1)),
    )
    .unwrap();
    app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(calls.get(), 1);
    assert_eq!(selected(&app), 1);
    value.revise(|v| *v = DateSelection::empty(DatePickerMode::Multiple));
    assert!(app.flush().is_err());
    assert_eq!(selected(&app), 1);
    value.revise(|v| *v = DateSelection::Single(None));
    app.flush().unwrap();
    assert_eq!(selected(&app), 0);
    assert_eq!(calls.get(), 1);
}
#[test]
fn multiple_full_selection_rejects_once_without_fault_or_growing_and_can_deselect() {
    let app = App::memory();
    let start = d("2024-02-01");
    let values =
        SelectedDates::try_from_iter((0..32).map(|i| start.checked_add_days(i).unwrap())).unwrap();
    let rejected = app.scope().cage(0usize);
    let accepted = app.scope().cage(0usize);
    app.mount(
        date_picker("full", p())
            .mode(DatePickerMode::Multiple)
            .default_value(DateSelection::Multiple(values))
            .on_reject(move |e| {
                assert_eq!(e, DateSelectionError::TooManyDates);
                rejected.revise(|n| *n += 1);
            })
            .on_value_change(move |_| accepted.revise(|n| *n += 1)),
    )
    .unwrap();
    app.dispatch(day(&app, d("2024-03-04")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(rejected.get(), 1);
    assert_eq!(accepted.get(), 0);
    app.dispatch(day(&app, start), Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(accepted.get(), 1);
}
#[test]
fn readonly_disabled_and_owner_retirement_do_not_publish_business_changes() {
    let app = App::memory();
    let visible = app.scope().cage(true);
    let readonly = app.scope().cage(true);
    let enabled = app.scope().cage(true);
    let calls = app.scope().cage(0usize);
    let reference = app.scope().cage(None);
    app.mount(glory::ui::when(visible, move || {
        date_picker("owned", p())
            .readonly(readonly)
            .enabled(enabled)
            .node_ref(reference)
            .on_value_change(move |_| calls.revise(|n| *n += 1))
    }))
    .unwrap();
    let id = day(&app, d("2024-02-29"));
    app.dispatch(id, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(calls.get(), 0);
    readonly.revise(|v| *v = false);
    enabled.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(!app.dispatch(id, Event::Activate).unwrap());
    visible.revise(|v| *v = false);
    app.flush().unwrap();
    assert!(reference.get().is_none());
    assert_eq!(calls.get(), 0);
    assert!(!app.dispatch(id, Event::Activate).unwrap());
}
#[test]
fn picker_input_is_readonly_successful_input_and_real_dialog_with_stable_shells() {
    let app = App::memory();
    let input = app.scope().cage(None);
    let trigger = app.scope().cage(None);
    let panel = app.scope().cage(None);
    let grid = app.scope().cage(None);
    app.mount(
        date_picker_input("input", p())
            .name("Appointment")
            .form_name("appointment")
            .node_ref(input)
            .trigger_ref(trigger)
            .panel_ref(panel)
            .calendar_ref(grid),
    )
    .unwrap();
    let input = input.get().unwrap();
    let trigger = trigger.get().unwrap();
    let panel = panel.get().unwrap();
    let root = grid.get().unwrap();
    let t = app.tree();
    let v = t.node(input).unwrap().descriptor.view();
    assert!(matches!(v.props, Props::Input(_)));
    assert!(v.readonly());
    assert!(
        v.semantics.as_ref().unwrap().form_control.is_none(),
        "the display spelling is not submitted twice"
    );
    assert_eq!(
        all(&app)
            .into_iter()
            .filter(|id| app
                .tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .and_then(|value| value.form_control.as_ref())
                .is_some_and(|control| control.name == "appointment"))
            .count(),
        1
    );
    assert_eq!(
        t.node(panel).unwrap().descriptor.view().role,
        Some(Role::Dialog)
    );
    drop(t);
    app.dispatch(trigger, Event::Activate).unwrap();
    app.flush().unwrap();
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
    app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert!(
        app.tree()
            .node(panel)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .hidden
    );
    let t = app.tree();
    let Props::Input(v) = t.node(input).unwrap().descriptor.view().props else {
        panic!()
    };
    assert_eq!(v.value, "2024-02-29");
    drop(t);
    app.dispatch(trigger, Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(app.tree().node(root).is_ok());
    app.dispatch(panel, Event::Dismiss(glory::render::DismissReason::Escape))
        .unwrap();
    app.flush().unwrap();
    assert!(app.tree().node(root).is_ok());
}
#[test]
fn input_controlled_selection_and_open_refusal_remain_independent_and_field_metadata_applies() {
    let app = App::memory();
    let value = app.scope().cage(DateSelection::Single(None));
    let open = app.scope().cage(true);
    let reference = app.scope().cage(None);
    let calls = app.scope().cage(0usize);
    let close = app.scope().cage(0usize);
    app.mount(
        field("booking")
            .label("Booking")
            .description("Civil dates")
            .error("Unavailable")
            .required(true)
            .form_name("booking")
            .control(move |f| {
                date_picker_input("field", p())
                    .field(f)
                    .value(value)
                    .open(open)
                    .node_ref(reference)
                    .on_value_change(move |_| calls.revise(|n| *n += 1))
                    .on_open_change(move |_| close.revise(|n| *n += 1))
            }),
    )
    .unwrap();
    let id = reference.get().unwrap();
    let t = app.tree();
    let s = t
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_ref()
        .unwrap();
    assert!(
        s.required
            && s.invalid
            && s.readonly
            && s.labelled_by.is_some()
            && s.described_by.is_some()
            && s.error_message.is_some()
    );
    drop(t);
    app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(calls.get(), 1);
    assert_eq!(close.get(), 0);
    assert_eq!(value.get(), DateSelection::Single(None));
    assert!(open.get());
}
#[test]
fn invalid_configuration_is_rejected_before_business_callback_and_exclusion_iterator_is_bounded() {
    let app = App::memory();
    assert!(
        app.mount(
            date_picker("wrong", p())
                .mode(DatePickerMode::Range)
                .default_value(DateSelection::Single(None))
        )
        .is_err()
    );
    let app = App::memory();
    let count = Rc::new(Cell::new(0));
    let out = count.clone();
    let picker = date_picker_input("budget", p()).exclude_dates((0..1_000_000).map(move |_| {
        out.set(out.get() + 1);
        CivilDate::MIN
    }));
    assert_eq!(count.get(), MAX_CALENDAR_EXCLUDED_DATES + 1);
    assert!(app.mount(picker).is_err());
    let app = App::memory();
    assert!(
        app.mount(date_picker_input("x".repeat(1_000_000), p()))
            .is_err()
    );
}

#[test]
fn range_and_multiple_inputs_preserve_partial_and_multi_open_policy_and_exact_successful_text() {
    for mode in [DatePickerMode::Range, DatePickerMode::Multiple] {
        let app = App::memory();
        let input = app.scope().cage(None);
        let panel = app.scope().cage(None);
        app.mount(
            date_picker_input("mode", p())
                .mode(mode)
                .form_name("dates")
                .default_open(true)
                .node_ref(input)
                .panel_ref(panel),
        )
        .unwrap();
        let input = input.get().unwrap();
        let panel = panel.get().unwrap();
        app.dispatch(day(&app, d("2024-02-26")), Event::Activate)
            .unwrap();
        app.flush().unwrap();
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
        let t = app.tree();
        let Props::Input(v) = t.node(input).unwrap().descriptor.view().props else {
            panic!()
        };
        assert_eq!(
            v.value,
            if mode == DatePickerMode::Range {
                "2024-02-26 –"
            } else {
                "2024-02-26"
            }
        );
        drop(t);
        app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
            .unwrap();
        app.flush().unwrap();
        let t = app.tree();
        let Props::Input(v) = t.node(input).unwrap().descriptor.view().props else {
            panic!()
        };
        assert_eq!(
            v.value,
            if mode == DatePickerMode::Range {
                "2024-02-26 – 2024-02-29"
            } else {
                "2024-02-26, 2024-02-29"
            }
        );
        assert_eq!(
            t.node(panel)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .hidden,
            mode == DatePickerMode::Range
        );
    }
}
