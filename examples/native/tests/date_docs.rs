#[path = "../../web/src/showcase_dates_docs/calendar_custom_day.rs"]
mod custom_calendar;
#[path = "../../web/src/dedicated_demos/date_input.rs"]
mod date_input_example;
#[path = "../../web/src/showcase_dates_docs/calendar_full_width.rs"]
mod full_calendar;
#[path = "../../web/src/dedicated_demos/inline_date_time_picker.rs"]
mod inline_example;
#[path = "../../web/src/dedicated_demos/month_picker.rs"]
mod month_example;
#[path = "../../web/src/showcase_dates_docs/time_display_reactive.rs"]
mod time_display;
#[path = "../../web/src/dedicated_demos/year_picker.rs"]
mod year_example;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
fn observe(host: &WindowApp) {
    for _ in 0..6 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = vec![];
    while let Some(id) = todo.pop() {
        todo.extend(tree.node(id).unwrap().children.iter().rev());
        out.push(id);
    }
    out
}
fn classes(app: &App, class: &str) -> Vec<NodeId> {
    nodes(app)
        .into_iter()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|v| v == class)
        })
        .collect()
}
fn named(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            let tree = app.tree();
            let v = tree.node(*id).unwrap().descriptor.view();
            v.name.as_deref() == Some(name) || matches!(v.props,Props::Button(p) if p.label==name)
        })
        .unwrap_or_else(|| panic!("missing {name}"))
}
fn bounds(app: &App, node: NodeId) -> Rect {
    let out = Rc::new(RefCell::new(None));
    let saved = out.clone();
    app.request(Query::Bounds(node), move |v| {
        *saved.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(r) = out.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    r
}
fn physical(host: &WindowApp, node: NodeId) -> apace_dom::NodeId {
    host.renderer()
        .engine_id(node, glory::html::Part::Main)
        .unwrap()
}
#[test]
fn actual_date_input_example_has_an_inset_calendar_and_retains_the_editor_after_selection() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 900, 1.).unwrap();
    host.mount(
        glory::ui::view()
            .style(Style::new().width(Length::Px(300.)))
            .fill(date_input_example::demo(sapid::Theme::blue_light())),
    )
    .unwrap();
    observe(&host);
    let app = host.app();
    let editor = classes(app, "sapid-date-input")[0];
    let first = physical(&host, editor);
    let input_rect = bounds(app, editor);
    let trigger = named(app, "Choose appointment date");
    let action_rect = bounds(app, trigger);
    assert!(
        action_rect.x + action_rect.width <= input_rect.x + input_rect.width + 0.5
            && action_rect.y >= input_rect.y
            && action_rect.y + action_rect.height <= input_rect.y + input_rect.height + 0.5
    );
    app.dispatch(trigger, Event::Activate).unwrap();
    observe(&host);
    let day = named(app, "Thursday, November 12, 2026");
    app.dispatch(day, Event::Activate).unwrap();
    observe(&host);
    assert_eq!(physical(&host, editor), first);
    assert!(
        matches!(app.tree().node(editor).unwrap().descriptor.view().props,Props::Input(p) if p.value=="2026-11-12")
    );
    assert_eq!(host.renderer().inspect(|d| d.top_layers().len()), 0);
    app.clear().unwrap();
    assert_eq!(app.stats().handlers, 0);
}
#[test]
fn actual_calendar_full_width_and_custom_day_sources_keep_real_narrow_cells_and_selection() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 700, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(full_calendar::demo(theme.clone())),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let days = classes(app, "sapid-calendar-day");
        assert_eq!(days.len(), 42);
        assert_eq!(classes(app, "sapid-calendar-week-number").len(), 6);
        for id in days {
            let r = bounds(app, id);
            assert!(r.width >= 24. && r.x + r.width <= 300.5, "{r:?}");
        }
        app.clear().unwrap();
        host.mount(custom_calendar::demo(theme)).unwrap();
        observe(&host);
        let day = named(app, "Monday, November 16, 2026");
        let before = physical(&host, day);
        app.dispatch(day, Event::Activate).unwrap();
        observe(&host);
        assert_eq!(physical(&host, day), before);
        assert!(nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content=="16 •")));
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn actual_month_and_year_examples_select_their_own_civil_domains_without_replacing_cells() {
    for (kind, name, example) in [
        (
            "month",
            "November 2026",
            Box::new(month_example::demo(sapid::Theme::green_light())) as Box<dyn Widget>,
        ),
        (
            "year",
            "2026",
            Box::new(year_example::demo(sapid::Theme::green_light())) as Box<dyn Widget>,
        ),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 700, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(300.)))
                .fill(example),
        )
        .unwrap();
        observe(&host);
        let app = host.app();
        let target = named(app, name);
        let id = physical(&host, target);
        let r = bounds(app, target);
        assert!(r.width >= 40. && r.height >= 24. && r.x + r.width <= 300.5);
        app.dispatch(target, Event::Activate).unwrap();
        observe(&host);
        assert_eq!(physical(&host, target), id);
        let expected = if kind == "month" {
            "Planning month: 2026-11"
        } else {
            "Planning year: 2026"
        };
        assert!(nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content==expected)));
        app.clear().unwrap();
    }
}
#[test]
fn actual_inline_primary_keeps_calendar_and_time_controls_in_a_narrow_column() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 1000, 1.).unwrap();
    host.mount(
        glory::ui::view()
            .style(Style::new().width(Length::Px(300.)))
            .fill(inline_example::demo(sapid::Theme::blue_dark())),
    )
    .unwrap();
    observe(&host);
    let app = host.app();
    let root = classes(app, "sapid-inline-date-time-picker")[0];
    assert!(bounds(app, root).width <= 300.5);
    let calendar = classes(app, "sapid-calendar-grid")[0];
    let hour = classes(app, "sapid-time-hour")[0];
    let c = bounds(app, calendar);
    let h = bounds(app, hour);
    assert!(h.y >= c.y + c.height && h.width >= 40.);
    assert!(h.x + h.width <= 300.5);
    let clear_rect = bounds(app, named(app, "Clear date and time"));
    let done_rect = bounds(app, named(app, "Done"));
    assert!(
        (clear_rect.y - done_rect.y).abs() < 1.,
        "inline actions must share a row: {clear_rect:?}, {done_rect:?}"
    );
    let identity = physical(&host, hour);
    let minute = classes(app, "sapid-time-minute")[0];
    for (node, value) in [(hour, "10"), (minute, "45")] {
        let revision = {
            let tree = app.tree();
            let Props::Input(props) = tree.node(node).unwrap().descriptor.view().props else {
                panic!("input")
            };
            props.revision
        };
        app.dispatch(
            node,
            Event::InputEdit(glory::render::InputEdit::Value {
                value: value.into(),
                revision,
                selection: glory::render::TextSelection {
                    start: 2,
                    end: 2,
                    ..Default::default()
                },
                before: Some(glory::render::TextSelection::default()),
                intent: glory::render::InputEditIntent::Replace,
                composing: false,
            }),
        )
        .unwrap();
        observe(&host);
    }
    app.dispatch(named(app, "Done"), Event::Activate).unwrap();
    observe(&host);
    assert!(nodes(app).into_iter().any(|id| matches!(app.tree().node(id).unwrap().descriptor.view().props, Props::Text(p) if p.content=="Last confirmed review: 2026-11-04T10:45:00")));
    assert_eq!(physical(&host, hour), identity);
    app.clear().unwrap();
    assert_eq!(app.runtime_stats().tasks, 0);
}
#[test]
fn reactive_time_display_keeps_one_physical_text_and_has_no_editor() {
    let host = WindowApp::new();
    host.mount(time_display::demo(sapid::Theme::green_dark()))
        .unwrap();
    observe(&host);
    let app = host.app();
    let display = classes(app, "time-display-retained")[0];
    let before = physical(&host, display);
    let toggle = named(app, "Toggle displayed time");
    app.dispatch(toggle, Event::Activate).unwrap();
    observe(&host);
    assert!(
        matches!(app.tree().node(display).unwrap().descriptor.view().props,Props::Text(p) if p.content=="No time selected")
    );
    app.dispatch(toggle, Event::Activate).unwrap();
    observe(&host);
    assert_eq!(physical(&host, display), before);
    assert!(
        matches!(app.tree().node(display).unwrap().descriptor.view().props,Props::Text(p) if p.content=="18:45")
    );
    assert!(!nodes(app).into_iter().any(|id| matches!(
        app.tree().node(id).unwrap().descriptor.view().props,
        Props::Input(_)
    )));
    app.clear().unwrap();
}
#[test]
fn twelve_hour_seconds_clear_and_dropdown_keep_editors_and_actions_inside_a_phone_column() {
    let host = WindowApp::new();
    host.renderer().set_viewport(320, 900, 1.).unwrap();
    let width = host.app().scope().cage(300.);
    host.mount(
        glory::ui::view()
            .style(width.map(|value| Style::new().width(Length::Px(*value))))
            .fill(
                sapid::time_picker("compact-full-time")
                    .theme(sapid::Theme::blue_light())
                    .format(sapid::TimeFormat::new(
                        sapid::HourCycle::H12,
                        sapid::TimePrecision::Second,
                        sapid::DateLocale::En,
                    ))
                    .clearable(true)
                    .with_dropdown(true)
                    .default_value(Some(sapid::LocalTime::new(18, 45, 30).unwrap())),
            ),
    )
    .unwrap();
    observe(&host);
    let app = host.app();
    for class in [
        "sapid-time-hour",
        "sapid-time-minute",
        "sapid-time-second",
        "sapid-time-picker-clear",
        "sapid-time-picker-preset-trigger",
    ] {
        let id = classes(app, class)[0];
        let r = bounds(app, id);
        assert!(
            r.x >= 0.
                && r.x + r.width <= 300.5
                && r.width
                    >= if class.starts_with("sapid-time-picker-") {
                        24.
                    } else {
                        48.
                    }
                && r.height >= 24.,
            "{class}: {r:?}"
        );
    }
    let hour = classes(app, "sapid-time-hour")[0];
    let identity = physical(&host, hour);
    let trigger = classes(app, "sapid-time-picker-preset-trigger")[0];
    let narrow = bounds(app, trigger);
    assert!(narrow.y >= bounds(app, hour).y + bounds(app, hour).height);
    host.renderer().set_viewport(700, 900, 1.).unwrap();
    width.revise(|value| *value = 640.);
    observe(&host);
    assert!((bounds(app, trigger).y - bounds(app, hour).y).abs() < 1.);
    assert_eq!(physical(&host, hour), identity);
    app.dispatch(trigger, Event::Activate).unwrap();
    observe(&host);
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
    app.dispatch(trigger, Event::Activate).unwrap();
    observe(&host);
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 0);
    width.revise(|value| *value = 300.);
    observe(&host);
    app.dispatch(classes(app, "sapid-time-picker-clear")[0], Event::Activate)
        .unwrap();
    observe(&host);
    assert!(classes(app, "sapid-time-picker-clear").is_empty());
    assert_eq!(physical(&host, hour), identity);
    assert!(
        matches!(app.tree().node(hour).unwrap().descriptor.view().props, Props::Input(p) if p.value.is_empty())
    );
    app.clear().unwrap();
}
