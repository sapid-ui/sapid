#![cfg(feature = "dates")]

use glory::prelude::*;
use glory::render::{InputProps, NodeId, Props};
use sapid::prelude::*;

fn d(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}

fn dt(value: &str) -> LocalDateTime {
    parse_local_date_time(value).unwrap()
}

fn provider() -> DatesProvider {
    DatesProvider::new(d("2024-02-29"))
}

fn all(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut output = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        output.push(id);
    }
    output
}

fn button(app: &App, label: &str) -> NodeId {
    all(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let value = tree.node(*id).unwrap().descriptor.view();
            matches!(value.props, Props::Button(props) if props.label == label)
                || matches!(value.props, Props::Button(_)) && value.name.as_deref() == Some(label)
        })
        .unwrap_or_else(|| panic!("missing button {label}"))
}

fn input(app: &App, id: NodeId) -> InputProps {
    let tree = app.tree();
    let Props::Input(value) = tree.node(id).unwrap().descriptor.view().props else {
        panic!("not an input")
    };
    value.clone()
}

fn class_count(app: &App, class: &str) -> usize {
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
                .any(|value| value == class)
        })
        .count()
}

#[test]
fn range_variants_forward_levels_columns_and_week_rows() {
    let app = App::memory();
    app.mount(
        inline_date_time_range_picker("range-levels", provider(), DateTimePolicy::Civil)
            .default_month(YearMonth::new(2024, 2).unwrap())
            .default_level(DatePickerLevel::Decade),
    )
    .unwrap();
    app.dispatch(button(&app, "2025"), Event::Activate).unwrap();
    app.flush().unwrap();
    app.dispatch(button(&app, "March 2025"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert!(all(&app).contains(&button(&app, "Saturday, March 1, 2025")));

    let app = App::memory();
    app.mount(
        date_time_range_picker("range-columns", provider(), DateTimePolicy::Civil)
            .default_open(true)
            .default_month(YearMonth::new(2024, 2).unwrap())
            .number_of_columns(2)
            .columns_to_scroll(1)
            .with_week_numbers(true),
    )
    .unwrap();
    assert_eq!(class_count(&app, "sapid-calendar-grid"), 2);
    assert_eq!(class_count(&app, "sapid-calendar-week-number"), 12);
}

#[test]
fn range_value_rejects_end_only_and_reverse() {
    assert!(DateTimeRangeValue::new(None, Some(dt("2024-02-29T10:00"))).is_err());
    assert!(
        DateTimeRangeValue::new(Some(dt("2024-02-29T11:00")), Some(dt("2024-02-29T10:00")))
            .is_err()
    );
    assert!(DateTimeRangeValue::new(Some(dt("2024-02-29T10:00")), None).is_ok());
}

#[test]
fn one_calendar_publishes_partial_then_complete_range_with_endpoint_defaults() {
    let app = App::memory();
    let value = app.scope().cage(DateTimeRangeValue::default());
    let start_hour = app.scope().cage(None);
    let end_hour = app.scope().cage(None);
    app.mount(
        inline_date_time_range_picker("range", provider(), DateTimePolicy::Civil)
            .value(value)
            .default_times(
                LocalTime::new(9, 30, 0).unwrap(),
                LocalTime::new(17, 45, 0).unwrap(),
            )
            .start_hour_ref(start_hour)
            .end_hour_ref(end_hour)
            .on_value_change(move |next| value.revise(|current| *current = next)),
    )
    .unwrap();
    app.dispatch(
        button(&app, &DateLocale::En.date_label(d("2024-02-28"))),
        Event::Activate,
    )
    .unwrap();
    app.flush().unwrap();
    assert_eq!(
        value.get(),
        DateTimeRangeValue::new(Some(dt("2024-02-28T09:30")), None).unwrap()
    );
    app.dispatch(
        button(&app, &DateLocale::En.date_label(d("2024-02-29"))),
        Event::Activate,
    )
    .unwrap();
    app.flush().unwrap();
    assert_eq!(
        value.get(),
        DateTimeRangeValue::new(Some(dt("2024-02-28T09:30")), Some(dt("2024-02-29T17:45")))
            .unwrap()
    );
    assert_eq!(input(&app, start_hour.get().unwrap()).value, "09");
    assert_eq!(input(&app, end_hour.get().unwrap()).value, "17");
}

#[test]
fn range_preset_clear_and_dialog_done_are_atomic() {
    let app = App::memory();
    let value = app.scope().cage(DateTimeRangeValue::default());
    let open = app.scope().cage(false);
    let trigger = app.scope().cage(None);
    let preset =
        DateTimeRangeValue::new(Some(dt("2024-02-28T08:00")), Some(dt("2024-02-29T18:00")))
            .unwrap();
    app.mount(
        date_time_range_picker("dialog-range", provider(), DateTimePolicy::Civil)
            .value(value)
            .open(open)
            .trigger_ref(trigger)
            .clearable(true)
            .presets([DateTimeRangePreset::new("Conference", preset)])
            .on_open_change(move |next| open.revise(|current| *current = next))
            .on_value_change(move |next| value.revise(|current| *current = next)),
    )
    .unwrap();
    app.dispatch(trigger.get().unwrap(), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    app.dispatch(button(&app, "Conference"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), preset);
    app.dispatch(button(&app, "Done"), Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(!open.get());
    app.dispatch(button(&app, "Clear range"), Event::Activate)
        .unwrap();
    app.flush().unwrap();
    assert_eq!(value.get(), DateTimeRangeValue::default());
}

#[test]
fn duplicate_or_partial_presets_are_rejected() {
    let app = App::memory();
    let complete =
        DateTimeRangeValue::new(Some(dt("2024-02-28T08:00")), Some(dt("2024-02-29T18:00")))
            .unwrap();
    assert!(
        app.mount(
            inline_date_time_range_picker("bad", provider(), DateTimePolicy::Civil).presets([
                DateTimeRangePreset::new("Same", complete),
                DateTimeRangePreset::new("Same", complete),
            ])
        )
        .is_err()
    );
    assert!(
        app.mount(
            inline_date_time_range_picker("partial", provider(), DateTimePolicy::Civil).presets([
                DateTimeRangePreset::new(
                    "Partial",
                    DateTimeRangeValue::new(Some(dt("2024-02-28T08:00")), None).unwrap(),
                ),
            ])
        )
        .is_err()
    );
}
