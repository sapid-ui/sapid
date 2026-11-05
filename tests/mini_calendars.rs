#![cfg(feature = "dates")]
use glory::render::{Event, GridCommand, NodeId, Props};
use glory::ui::App;
use sapid::*;
use std::{cell::RefCell, rc::Rc};
fn d(s: &str) -> CivilDate {
    CivilDate::parse_iso(s).unwrap()
}
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        result.push(id);
        pending.extend(tree.node(id).unwrap().children.iter().copied());
    }
    result
}
fn named(app: &App, name: &str) -> NodeId {
    nodes(app)
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some(name)
        })
        .unwrap()
}
fn days(app: &App) -> Vec<NodeId> {
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
                .any(|c| c == "sapid-mini-calendar-day")
        })
        .collect()
}
fn day(app: &App, date: CivilDate) -> NodeId {
    named(app, &DateLocale::En.date_label(date))
}
#[test]
fn strip_defaults_to_seven_days_and_reuses_slots_across_leap_day_and_two_same_tick_pages() {
    let app = App::memory();
    let date = app.scope().cage(d("2024-02-27"));
    let value = app.scope().cage(None);
    app.mount(
        mini_calendar("strip", DatesProvider::new(date.get()))
            .date(date)
            .value(value)
            .on_date_change(move |v| date.revise(|old| *old = v))
            .on_value_change(move |v| value.revise(|old| *old = v)),
    )
    .unwrap();
    let mut ids = days(&app);
    ids.sort();
    assert_eq!(ids.len(), 7);
    app.dispatch(day(&app, d("2024-02-29")), Event::Activate)
        .unwrap();
    assert_eq!(value.get(), Some(d("2024-02-29")));
    let from = day(&app, d("2024-02-28"));
    let boundary = nodes(&app)
        .into_iter()
        .find(|id| {
            app.tree().node(*id).unwrap().descriptor.view().role == Some(glory::render::Role::Grid)
        })
        .unwrap();
    app.scope()
        .batch(|| {
            for _ in 0..2 {
                app.dispatch(
                    boundary,
                    Event::GridNavigate {
                        from,
                        command: GridCommand::Right,
                    },
                )
                .unwrap();
            }
            app.dispatch(from, Event::Activate).unwrap();
        })
        .unwrap();
    assert_eq!(value.get(), Some(d("2024-03-01")));
    let next = named(&app, "Next range");
    app.dispatch(next, Event::Activate).unwrap();
    app.dispatch(next, Event::Activate).unwrap();
    assert_eq!(date.get(), d("2024-03-12"));
    app.settle(16).unwrap();
    let mut after = days(&app);
    after.sort();
    assert_eq!(ids, after);
    assert!(days(&app).contains(&day(&app, d("2024-03-12"))));
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn controlled_range_refusal_and_same_tick_readonly_do_not_write_sources() {
    let app = App::memory();
    let date = app.scope().cage(d("2024-02-27"));
    let value = app.scope().cage(None);
    let readonly = app.scope().cage(false);
    let proposals = Rc::new(RefCell::new(Vec::new()));
    let seen = proposals.clone();
    let root = app.scope().cage(None);
    app.mount(
        mini_calendar("controlled", DatesProvider::new(date.get()))
            .date(date)
            .value(value)
            .readonly(readonly)
            .node_ref(root)
            .on_date_change(move |v| seen.borrow_mut().push(v)),
    )
    .unwrap();
    app.dispatch(named(&app, "Next range"), Event::Activate)
        .unwrap();
    assert_eq!(date.get(), d("2024-02-27"));
    assert_eq!(*proposals.borrow(), vec![d("2024-03-05")]);
    let last = day(&app, d("2024-03-04"));
    app.dispatch(
        root.get().unwrap(),
        Event::GridNavigate {
            from: last,
            command: GridCommand::Right,
        },
    )
    .unwrap();
    app.settle(16).unwrap();
    assert_eq!(date.get(), d("2024-02-27"));
    readonly.revise(|old| *old = true);
    app.dispatch(last, Event::Activate).unwrap();
    assert_eq!(value.get(), None);
    app.clear().unwrap();
}
#[test]
fn bounds_exclusions_weekends_localization_and_finite_edges_are_explicit() {
    let app = App::memory();
    let provider = app.scope().cage(
        DatesProvider::new(d("2024-02-27"))
            .bounds(DateBounds::new(Some(d("2024-02-28")), Some(d("2024-03-03"))).unwrap()),
    );
    let value = app.scope().cage(None);
    app.mount(
        mini_calendar("bounds", provider)
            .value(value)
            .on_value_change(move |v| value.revise(|old| *old = v))
            .exclude_date(d("2024-02-29"))
            .disable_weekday(Weekday::Saturday)
            .day_style(|_| glory::style::Style {
                font_weight: Some(600.),
                ..Default::default()
            }),
    )
    .unwrap();
    for date in [
        d("2024-02-27"),
        d("2024-02-29"),
        d("2024-03-02"),
        d("2024-03-04"),
    ] {
        assert!(
            matches!(app.tree().node(day(&app,date)).unwrap().descriptor.view().props,Props::Button(p) if !p.enabled)
        );
    }
    let identity = day(&app, d("2024-03-01"));
    app.dispatch(identity, Event::Activate).unwrap();
    assert_eq!(value.get(), Some(d("2024-03-01")));
    provider.revise(|p| *p = p.locale(DateLocale::ZhCn));
    app.settle(16).unwrap();
    assert_eq!(named(&app, "2024年3月1日 星期五"), identity);
    app.clear().unwrap();
    for edge in [CivilDate::MIN, CivilDate::MAX] {
        app.mount(mini_calendar("edge", DatesProvider::new(edge)).number_of_days(31))
            .unwrap();
        assert_eq!(days(&app).len(), 31);
        app.clear().unwrap();
    }
    for count in [0, 32, usize::MAX] {
        assert!(
            app.mount(
                mini_calendar("invalid", DatesProvider::new(CivilDate::MIN)).number_of_days(count)
            )
            .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
