use glory::prelude::*;
use sapid::prelude::*;
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn dt(value: &str) -> LocalDateTime {
    parse_local_date_time(value).unwrap()
}
fn window(view: ScheduleView) -> ScheduleWindow {
    match view {
        ScheduleView::Day => ScheduleWindow::days(day("2026-11-04"), 1),
        ScheduleView::Week | ScheduleView::Agenda => ScheduleWindow::days(day("2026-11-02"), 7),
        ScheduleView::Month | ScheduleView::MobileMonth => {
            ScheduleWindow::month(YearMonth::new(2026, 11).unwrap())
        }
        ScheduleView::Year => ScheduleWindow::year(CivilYear::new(2026).unwrap()),
    }
    .unwrap()
}
fn interval_label(span: ScheduleInterval) -> String {
    format!(
        "{} to {} (end exclusive)",
        format_local_date_time(span.start()),
        format_local_date_time(span.end())
    )
}
fn selection_label(data: &ScheduleData, value: &Option<ScheduleSelection>) -> String {
    match value {
        Some(ScheduleSelection::Event { key, interval, .. }) => format!(
            "Selected: {} · {}",
            data.events()
                .iter()
                .find(|event| event.key() == key)
                .map_or("Event", |event| event.title()),
            interval_label(*interval)
        ),
        Some(ScheduleSelection::Slot { interval, .. }) => {
            format!("Selected time: {}", interval_label(*interval))
        }
        None => "Choose an event or an available time.".to_owned(),
    }
}
fn resource_window(view: ResourceScheduleView) -> ScheduleWindow {
    window(match view {
        ResourceScheduleView::Day => ScheduleView::Day,
        ResourceScheduleView::Week => ScheduleView::Week,
        ResourceScheduleView::Month => ScheduleView::Month,
    })
}
fn resource_sample(view: ResourceScheduleView) -> ResourceScheduleData {
    let rooms = [
        ("room-a", "Room A", "Project planning"),
        ("room-b", "Room B", "Design review"),
        ("room-c", "Room C", "Workshop"),
        ("room-d", "Room D", "Testing"),
        ("room-e", "Room E", "Interview"),
        ("room-f", "Room F", "Research"),
    ];
    let permission = SchedulePermissions {
        select: true,
        move_event: true,
        resize_event: true,
    };
    let mut events = rooms
        .iter()
        .enumerate()
        .map(|(index, (room, _, title))| {
            ScheduleEvent::new(
                ScheduleKey::new(format!("booking-{index}")).unwrap(),
                *title,
                ScheduleInterval::new(dt("2026-11-04T09:00"), dt("2026-11-04T10:00")).unwrap(),
            )
            .unwrap()
            .resource(ScheduleKey::new(*room).unwrap())
            .permissions(permission)
        })
        .collect::<Vec<_>>();
    events.push(
        ScheduleEvent::new(
            ScheduleKey::new("overlap").unwrap(),
            "Client review",
            ScheduleInterval::new(dt("2026-11-04T09:30"), dt("2026-11-04T10:30")).unwrap(),
        )
        .unwrap()
        .resource(ScheduleKey::new("room-a").unwrap())
        .permissions(permission),
    );
    events.push(
        ScheduleEvent::new(
            ScheduleKey::new("release").unwrap(),
            "Release day",
            ScheduleInterval::all_day(day("2026-11-04"), day("2026-11-05")).unwrap(),
        )
        .unwrap()
        .all_day()
        .unwrap()
        .resource(ScheduleKey::new("room-a").unwrap()),
    );
    ResourceScheduleData::new(
        ScheduleData::new(ScheduleZone::Civil, resource_window(view), events).unwrap(),
        ScheduleResources::new(rooms.iter().map(|(room, title, _)| {
            ScheduleResource::new(ScheduleKey::new(*room).unwrap(), *title).unwrap()
        }))
        .unwrap(),
        ScheduleResourceWindow::new(0, 2).unwrap(),
    )
    .unwrap()
}
fn resource_request(source: Cage<ResourceScheduleData>, request: ResourceScheduleRequest) {
    source.revise(|current| {
        *current = ResourceScheduleData::new(
            ScheduleData::new(
                request.zone,
                request.window,
                current.data().events().iter().cloned(),
            )
            .unwrap(),
            current.resources().clone(),
            request.resource_window,
        )
        .unwrap()
    });
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(resource_sample(ResourceScheduleView::Day));
        let selected = scope.cage(None::<ScheduleSelection>);
        let window_status = data.map(|model| {
            format!(
                "Rooms: {} · {} to {} (end exclusive)",
                model
                    .visible_resources()
                    .iter()
                    .map(|room| room.title())
                    .collect::<Vec<_>>()
                    .join(", "),
                model.data().window().start().date(),
                model.data().window().end().date()
            )
        });
        let selection_status = scope.bond(move || {
            data.with(|model| selected.with(|value| selection_label(model.data(), value)))
        });
        let bookings = resources_day_view("resources-day-view-primary", data)
            .theme(theme.clone())
            .name("Room bookings")
            .viewport_height(320.)
            .column_width(160.)
            .resource_page_size(2)
            .time_window(ScheduleTimeWindow::new(8 * 60, 16 * 60, 30).unwrap())
            .selection(selected)
            .on_selection_change(move |value| selected.revise(|old| *old = Some(value)))
            .on_window_request(move |request| {
                resource_request(data, request);
                selected.revise(|old| *old = None);
            });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                bookings,
                sapid::text().theme(theme.clone()).content(window_status),
                sapid::text().theme(theme.clone()).content(selection_status),
            ))
            .show_in(scope)
    })
}
