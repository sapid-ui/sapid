#![cfg(feature = "dates")]
use glory::prelude::*;
use glory::render::{Event, EventKind, NodeId, Props};
use sapid::*;
use std::{cell::RefCell, rc::Rc};
fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::new(2024, 2, 29).unwrap())
}
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        out.push(id);
    }
    out
}
fn button(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let node = tree.node(*id).unwrap().descriptor.view();
            matches!(node.props, Props::Button(_)) && node.name.as_deref() == Some(name)
        })
        .unwrap_or_else(|| panic!("missing {name}"))
}
fn value(app: &App, id: NodeId) -> String {
    let tree = app.tree();
    let Props::Input(input) = tree.node(id).unwrap().descriptor.view().props else {
        panic!("input")
    };
    input.value.clone()
}
fn submitted(app: &App, name: &str) -> String {
    let controls = nodes(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .and_then(|value| value.form_control.as_ref())
                .is_some_and(|control| control.name == name)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        controls.len(),
        1,
        "exactly one successful control for {name}"
    );
    value(app, controls[0])
}

#[test]
fn clear_preserves_multiple_and_range_domains_and_clears_iso_submission() {
    use std::boxed::Box;
    for mode in [DatePickerMode::Multiple, DatePickerMode::Range] {
        let separator = if mode == DatePickerMode::Multiple {
            ","
        } else {
            "/"
        };
        let dates = format!("2024-02-29{separator}2024-03-01");
        let months = format!("2024-02{separator}2024-03");
        let years = format!("2024{separator}2025");
        let cases: Vec<(Box<dyn Widget>, &str, String)> = vec![
            (
                Box::new(
                    date_picker_input("date-modes", provider())
                        .mode(mode)
                        .clearable(true)
                        .form_name("value")
                        .default_value(DateSelection::parse_iso(mode, &dates).unwrap()),
                ),
                "Clear dates",
                dates,
            ),
            (
                Box::new(
                    month_picker_input("month-modes", provider())
                        .mode(mode)
                        .clearable(true)
                        .form_name("value")
                        .display_format(PeriodInputFormat::Localized)
                        .default_value(MonthSelection::parse_iso(mode, &months).unwrap()),
                ),
                "Clear months",
                months,
            ),
            (
                Box::new(
                    year_picker_input("year-modes", provider())
                        .mode(mode)
                        .clearable(true)
                        .form_name("value")
                        .display_format(PeriodInputFormat::Localized)
                        .default_value(YearSelection::parse_iso(mode, &years).unwrap()),
                ),
                "Clear years",
                years,
            ),
        ];
        for (widget, clear_name, initial) in cases {
            let app = App::memory();
            app.mount(widget).unwrap();
            assert_eq!(submitted(&app, "value"), initial);
            let clear = button(&app, clear_name);
            app.dispatch(clear, Event::Activate).unwrap();
            assert_eq!(submitted(&app, "value"), "");
            assert!(!app.tree().enabled(clear));
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}

#[test]
fn formatted_date_display_and_iso_submission_update_together_and_clear_can_be_refused() {
    let app = App::memory();
    let reference = app.scope().cage(None);
    let date = CivilDate::new(2024, 2, 29).unwrap();
    let source = app.scope().cage(DateSelection::Single(Some(date)));
    let accept = app.scope().cage(false);
    let readonly = app.scope().cage(false);
    let events = Rc::new(RefCell::new(Vec::new()));
    let log = events.clone();
    app.mount(
        date_picker_input("date", provider())
            .name("Appointment")
            .form_name("appointment")
            .display_format(DateInputFormat::DayMonthYear)
            .clearable(true)
            .value(source)
            .readonly(readonly)
            .node_ref(reference)
            .on_value_change(move |next| {
                log.borrow_mut().push(next);
                if accept.get() {
                    source.revise(|old| *old = next);
                }
            }),
    )
    .unwrap();
    let id = reference.get().unwrap();
    assert_eq!(value(&app, id), "29/02/2024");
    assert_eq!(submitted(&app, "appointment"), "2024-02-29");
    let clear = button(&app, "Clear dates");
    app.dispatch(clear, Event::Activate).unwrap();
    assert_eq!(events.borrow().len(), 1);
    assert_eq!(source.get(), DateSelection::Single(Some(date)));
    assert_eq!(value(&app, id), "29/02/2024");
    readonly.revise(|old| *old = true);
    app.flush().unwrap();
    assert!(!app.dispatch(clear, Event::Activate).unwrap());
    assert_eq!(events.borrow().len(), 1);
    readonly.revise(|old| *old = false);
    accept.revise(|old| *old = true);
    app.flush().unwrap();
    app.dispatch(clear, Event::Activate).unwrap();
    assert_eq!(value(&app, id), "");
    assert_eq!(submitted(&app, "appointment"), "");
    assert_eq!(reference.get(), Some(id));
}

#[test]
fn localized_month_and_year_display_do_not_change_the_submitted_period_domain() {
    let app = App::memory();
    let month = app.scope().cage(None);
    let year = app.scope().cage(None);
    let locale = app.scope().cage(provider().locale(DateLocale::En));
    app.mount(
        glory::ui::group().fill((
            month_picker_input("month", locale)
                .name("Billing month")
                .form_name("month")
                .clearable(true)
                .display_format(PeriodInputFormat::Localized)
                .default_value(MonthSelection::Single(Some(
                    YearMonth::new(2024, 2).unwrap(),
                )))
                .node_ref(month),
            year_picker_input("year", locale)
                .name("Billing year")
                .form_name("year")
                .clearable(true)
                .display_format(PeriodInputFormat::Localized)
                .default_value(YearSelection::Single(Some(CivilYear::new(2024).unwrap())))
                .node_ref(year),
        )),
    )
    .unwrap();
    let mid = month.get().unwrap();
    let yid = year.get().unwrap();
    assert_eq!(value(&app, mid), "February 2024");
    assert_eq!(submitted(&app, "month"), "2024-02");
    locale.revise(|old| *old = provider().locale(DateLocale::ZhCn));
    app.flush().unwrap();
    assert_eq!(value(&app, mid), "2024年2月");
    assert_eq!(value(&app, yid), "2024年");
    assert_eq!(submitted(&app, "month"), "2024-02");
    assert_eq!(submitted(&app, "year"), "2024");
    for name in ["Clear months", "Clear years"] {
        app.dispatch(button(&app, name), Event::Activate).unwrap();
    }
    assert_eq!(value(&app, mid), "");
    assert_eq!(value(&app, yid), "");
    assert_eq!(month.get(), Some(mid));
    assert_eq!(year.get(), Some(yid));
}

#[test]
fn date_time_display_format_and_clear_preserve_one_iso_value_and_current_editor() {
    let app = App::memory();
    let reference = app.scope().cage(None);
    let time = parse_local_date_time("2024-02-29T09:30:00").unwrap();
    app.mount(
        date_time_picker("time", provider(), DateTimePolicy::Civil)
            .name("Meeting")
            .form_name("meeting")
            .display_format(DateTimeDisplayFormat::DayMonthYear)
            .default_value(Some(time))
            .clearable(true)
            .node_ref(reference),
    )
    .unwrap();
    let id = reference.get().unwrap();
    assert_eq!(value(&app, id), "29/02/2024 09:30");
    assert_eq!(submitted(&app, "meeting"), "2024-02-29T09:30:00");
    app.dispatch(button(&app, "Clear date and time"), Event::Activate)
        .unwrap();
    assert_eq!(value(&app, id), "");
    assert_eq!(submitted(&app, "meeting"), "");
    assert_eq!(reference.get(), Some(id));
}

#[test]
fn clear_owner_retirement_stops_focus_work_and_releases_display_refs() {
    let app = Rc::new(App::memory());
    let reference = app.scope().cage(None);
    let weak = Rc::downgrade(&app);
    app.mount(
        month_picker_input("retire", provider())
            .node_ref(reference)
            .clearable(true)
            .default_value(MonthSelection::Single(Some(
                YearMonth::new(2024, 2).unwrap(),
            )))
            .on_value_change(move |_| weak.upgrade().unwrap().clear().unwrap()),
    )
    .unwrap();
    app.dispatch(button(&app, "Clear months"), Event::Activate)
        .unwrap();
    assert!(reference.get().is_none());
    assert_eq!(app.stats().nodes, 0);
    assert_eq!(app.stats().handlers, 0);
}

#[test]
fn select_inset_clear_preserves_typed_before_generic_and_the_real_combobox_node() {
    let app = App::memory();
    let reference = app.scope().cage(None);
    let log = Rc::new(RefCell::new(Vec::new()));
    let typed = log.clone();
    let generic = log.clone();
    app.mount(
        select("clear")
            .label("Language")
            .clearable(true)
            .node_ref(reference)
            .default_value(Some("rust".into()))
            .option(select_option("rust", "Rust"))
            .on_value_change(move |_| typed.borrow_mut().push("typed"))
            .on(EventKind::Activate, move |_| {
                generic.borrow_mut().push("generic")
            }),
    )
    .unwrap();
    let id = reference.get().unwrap();
    app.dispatch(button(&app, "Clear selection"), Event::Activate)
        .unwrap();
    assert_eq!(*log.borrow(), ["typed", "generic"]);
    assert_eq!(reference.get(), Some(id));
    assert!(!app.tree().enabled(button(&app, "Clear selection")));
}
