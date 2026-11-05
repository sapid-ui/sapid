#[path = "../../web/src/dedicated_demos/agenda_view.rs"]
mod agenda_view;
#[path = "../../web/src/dedicated_demos/day_view.rs"]
mod day_view;
#[path = "../../web/src/dedicated_demos/events_data.rs"]
mod events_data;
#[path = "../../web/src/dedicated_demos/mobile_month_view.rs"]
mod mobile_month_view;
#[path = "../../web/src/dedicated_demos/month_view.rs"]
mod month_view;
#[path = "../../web/src/dedicated_demos/recurring_events.rs"]
mod recurring_events;
#[path = "../../web/src/dedicated_demos/resources_day_view.rs"]
mod resources_day_view;
#[path = "../../web/src/showcase_schedule_docs/resources_day_view_editing.rs"]
mod resources_day_view_editing;
#[path = "../../web/src/dedicated_demos/resources_month_view.rs"]
mod resources_month_view;
#[path = "../../web/src/showcase_schedule_docs/resources_month_view_editing.rs"]
mod resources_month_view_editing;
#[path = "../../web/src/dedicated_demos/resources_schedule.rs"]
mod resources_schedule;
#[path = "../../web/src/showcase_schedule_docs/resources_schedule_editing.rs"]
mod resources_schedule_editing;
#[path = "../../web/src/dedicated_demos/resources_week_view.rs"]
mod resources_week_view;
#[path = "../../web/src/showcase_schedule_docs/resources_week_view_editing.rs"]
mod resources_week_view_editing;
#[path = "../../web/src/dedicated_demos/schedule.rs"]
mod schedule;
#[path = "../../web/src/showcase_schedule_docs/schedule_scroll.rs"]
mod schedule_scroll;
#[path = "../../web/src/dedicated_demos/week_view.rs"]
mod week_view;
#[path = "../../web/src/dedicated_demos/year_view.rs"]
mod year_view;
use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{Event, NodeId, Props},
};
fn nodes(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = vec![];
    while let Some(id) = todo.pop() {
        todo.extend(&tree.node(id).unwrap().children);
        out.push(id);
    }
    out
}
fn observe(host: &WindowApp) {
    for _ in 0..6 {
        host.observe(&std::task::Context::from_waker(std::task::Waker::noop()))
            .unwrap();
    }
}
#[test]
fn actual_schedule_scroll_source_scrolls_the_timeline_instead_of_an_agenda() {
    let host = WindowApp::new();
    host.renderer().set_viewport(360, 900, 1.).unwrap();
    host.mount(
        glory::ui::view()
            .style(Style::new().width(Length::Px(340.)))
            .fill(schedule_scroll::demo(sapid::Theme::blue_light())),
    )
    .unwrap();
    observe(&host);
    let app = host.app();
    let viewport = nodes(app)
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|class| class == "sapid-schedule-viewport")
        })
        .expect("the scroll example must mount a timeline");
    let action=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label=="Show around 10:00")).unwrap();
    app.dispatch(action, Event::Activate).unwrap();
    observe(&host);
    let result = std::rc::Rc::new(std::cell::RefCell::new(None));
    let saved = result.clone();
    app.request(
        glory::render::Query::ScrollMetrics(viewport),
        move |value| *saved.borrow_mut() = Some(value.unwrap()),
    )
    .unwrap();
    app.pump().unwrap();
    assert!(
        matches!(result.borrow().as_ref(),Some(glory::render::QueryValue::ScrollMetrics(metrics)) if metrics.y>=175.)
    );
    app.clear().unwrap();
    assert_eq!(app.stats().handlers, 0);
}
#[test]
fn actual_recurrence_source_changes_the_current_occurrences_and_event_schema_retires() {
    let host = WindowApp::new();
    host.renderer().set_viewport(360, 1200, 1.).unwrap();
    host.mount(recurring_events::demo(sapid::Theme::green_dark()))
        .unwrap();
    observe(&host);
    let count = || {
        nodes(host.app()).into_iter().filter(|id| matches!(host.app().tree().node(*id).unwrap().descriptor.view().props, Props::Button(p) if p.label.starts_with("Daily standup"))).count()
    };
    assert_eq!(count(), 6);
    let toggle = nodes(host.app())
        .into_iter()
        .find(|id| {
            host.app()
                .tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Skip Thursday standup")
        })
        .unwrap();
    host.app()
        .dispatch(toggle, Event::CheckedChanged(false))
        .unwrap();
    observe(&host);
    assert_eq!(count(), 7);
    host.app().clear().unwrap();
    assert_eq!(host.app().stats().handlers, 0);
    host.mount(events_data::demo(sapid::Theme::blue_light()))
        .unwrap();
    observe(&host);
    assert!(nodes(host.app()).into_iter().any(|id| matches!(host.app().tree().node(id).unwrap().descriptor.view().props, Props::Text(p) if p.content == "Civil: no implicit UTC offset")));
    host.app().clear().unwrap();
    assert_eq!(host.app().stats().handlers, 0);
}
#[test]
fn actual_schedule_primary_sources_mount_select_real_events_and_retire() {
    for name in [
        "schedule",
        "day_view",
        "week_view",
        "month_view",
        "year_view",
        "mobile_month_view",
        "agenda_view",
    ] {
        let example: Box<dyn Widget> = match name {
            "schedule" => Box::new(schedule::demo(sapid::Theme::blue_light())),
            "day_view" => Box::new(day_view::demo(sapid::Theme::blue_light())),
            "week_view" => Box::new(week_view::demo(sapid::Theme::blue_light())),
            "month_view" => Box::new(month_view::demo(sapid::Theme::blue_light())),
            "year_view" => Box::new(year_view::demo(sapid::Theme::blue_light())),
            "mobile_month_view" => Box::new(mobile_month_view::demo(sapid::Theme::blue_light())),
            "agenda_view" => Box::new(agenda_view::demo(sapid::Theme::blue_light())),
            _ => unreachable!(),
        };
        let host = WindowApp::new();
        host.renderer().set_viewport(340, 900, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(320.)))
                .fill(example),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
        observe(&host);
        let app = host.app();
        if name == "mobile_month_view" {
            let day = nodes(app)
                .into_iter()
                .find(|id| {
                    let d = app.tree();
                    let n = d.node(*id).unwrap().descriptor.view();
                    n.classes.iter().any(|c| c == "sapid-schedule-period-slot")
                        && n.name
                            .as_ref()
                            .is_some_and(|label| label.starts_with("Wednesday, November 4, 2026"))
                })
                .unwrap();
            app.dispatch(day, Event::Activate).unwrap();
            observe(&host);
        }
        if name == "year_view" {
            let more=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label=="Show all events")).unwrap();
            app.dispatch(more, Event::Activate).unwrap();
            observe(&host);
        }
        let event=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label.contains("Project planning"))).unwrap_or_else(||panic!("{name}: missing planning"));
        app.dispatch(event, Event::Activate).unwrap();
        observe(&host);
        assert!(nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content.contains("Selected: Project planning"))),"{name}");
        app.clear().unwrap();
        assert_eq!(app.stats().handlers, 0);
    }
}

#[test]
fn actual_resource_primary_sources_select_bookings_without_mounting_the_edit_workbench() {
    for name in [
        "resources_day_view",
        "resources_week_view",
        "resources_month_view",
        "resources_schedule",
    ] {
        let example: Box<dyn Widget> = match name {
            "resources_day_view" => Box::new(resources_day_view::demo(sapid::Theme::blue_light())),
            "resources_week_view" => {
                Box::new(resources_week_view::demo(sapid::Theme::blue_light()))
            }
            "resources_month_view" => {
                Box::new(resources_month_view::demo(sapid::Theme::blue_light()))
            }
            _ => Box::new(resources_schedule::demo(sapid::Theme::blue_light())),
        };
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 1000, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(340.)))
                .fill(example),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
        observe(&host);
        let app = host.app();
        let planning=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label.starts_with("Project planning"))).unwrap_or_else(||panic!("{name}: planning missing"));
        app.dispatch(planning, Event::Activate).unwrap();
        observe(&host);
        assert!(nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content.contains("Selected: Project planning"))),"{name}");
        assert!(!nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Button(p) if p.label=="Move later")),"{name}: edit controls belong to their separate source section");
        app.clear().unwrap();
        assert_eq!(app.stats().handlers, 0);
    }
}

#[test]
fn actual_resource_editing_sections_validate_accept_and_publish_the_updated_booking() {
    for name in ["day", "week", "month", "unified"] {
        let example: Box<dyn Widget> = match name {
            "day" => Box::new(resources_day_view_editing::demo(sapid::Theme::blue_light())),
            "week" => Box::new(resources_week_view_editing::demo(sapid::Theme::blue_light())),
            "month" => Box::new(resources_month_view_editing::demo(
                sapid::Theme::blue_light(),
            )),
            _ => Box::new(resources_schedule_editing::demo(sapid::Theme::blue_light())),
        };
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 1000, 1.).unwrap();
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(340.)))
                .fill(example),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
        observe(&host);
        let app = host.app();
        let edit=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label.starts_with("Edit Design review"))).unwrap();
        app.dispatch(edit, Event::Activate).unwrap();
        observe(&host);
        let later=nodes(app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label=="Move later"&&p.enabled)).unwrap();
        app.dispatch(later, Event::Activate).unwrap();
        observe(&host);
        assert!(nodes(app).into_iter().any(|id|matches!(app.tree().node(id).unwrap().descriptor.view().props,Props::Text(p) if p.content.contains("Saved Design review"))),"{name}");
        assert!(
            matches!(app.tree().node(later).unwrap().descriptor.view().props,Props::Button(p) if p.enabled),
            "{name}: accepted same-window edits must retain the current editor"
        );
        app.dispatch(later, Event::Activate).unwrap();
        observe(&host);

        app.clear().unwrap();
        assert_eq!(app.stats().handlers, 0);
    }
}
