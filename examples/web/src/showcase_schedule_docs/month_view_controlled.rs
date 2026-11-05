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
fn sample(view: ScheduleView) -> ScheduleData {
    ScheduleData::new(
        ScheduleZone::Civil,
        window(view),
        [
            ScheduleEvent::new(
                ScheduleKey::new("planning").unwrap(),
                "Project planning",
                ScheduleInterval::new(dt("2026-11-04T09:00"), dt("2026-11-04T10:30")).unwrap(),
            )
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("review").unwrap(),
                "Design review",
                ScheduleInterval::new(dt("2026-11-04T09:30"), dt("2026-11-04T11:00")).unwrap(),
            )
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("standup").unwrap(),
                "Daily standup",
                ScheduleInterval::new(dt("2026-11-02T11:30"), dt("2026-11-02T12:00")).unwrap(),
            )
            .unwrap()
            .recurring(
                ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Count(7))
                    .unwrap(),
            )
            .unwrap()
            .except_dates([day("2026-11-05")])
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("release").unwrap(),
                "Release day",
                ScheduleInterval::all_day(day("2026-11-04"), day("2026-11-05")).unwrap(),
            )
            .unwrap()
            .all_day()
            .unwrap(),
        ],
    )
    .unwrap()
}
fn accept_window(source: Cage<ScheduleData>, request: ScheduleRequest) {
    source.revise(|current| {
        *current = ScheduleData::new(
            request.zone,
            request.window,
            current.events().iter().cloned(),
        )
        .unwrap()
    });
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
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(sample(ScheduleView::Month));
        let selected = scope.cage(None::<ScheduleSelection>);
        let accept = scope.cage(false);
        let readonly = scope.cage(false);
        let status = scope
            .bond(move || data.with(|model| selected.with(|value| selection_label(model, value))));
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::switch()
                        .theme(theme.clone())
                        .label("Accept schedule changes")
                        .checked(accept)
                        .on_checked_change(move |value| accept.revise(|old| *old = value)),
                    sapid::switch()
                        .theme(theme.clone())
                        .label("Readonly schedule")
                        .checked(readonly)
                        .on_checked_change(move |value| readonly.revise(|old| *old = value)),
                )),
                month_view("month-view-controlled", data)
                    .theme(theme.clone())
                    .selection(selected)
                    .readonly(readonly)
                    .viewport_height(300.)
                    .today(day("2026-11-04"))
                    .max_events_per_day(2)
                    .on_selection_change(move |value| {
                        if accept.get() {
                            selected.revise(|old| *old = Some(value));
                        }
                    })
                    .on_window_request(move |request| {
                        if accept.get() {
                            accept_window(data, request);
                        }
                    }),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
