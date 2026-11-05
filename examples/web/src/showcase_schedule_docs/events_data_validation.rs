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
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let status = scope.cage("The current event data is valid.".to_owned());
        let key = sapid::button()
            .theme(theme.clone())
            .label("Validate a key with spaces")
            .on_press(move || {
                let error = ScheduleKey::new("room a").unwrap_err();
                status.revise(|old| *old = format!("Key rejected: {error}"));
            });
        let interval = sapid::button()
            .theme(theme.clone())
            .label("Validate a reversed interval")
            .on_press(move || {
                let error = ScheduleInterval::new(dt("2026-11-04T10:00"), dt("2026-11-04T09:00"))
                    .unwrap_err();
                status.revise(|old| *old = format!("Interval rejected: {error}"));
            });
        let resource = sapid::button()
            .theme(theme.clone())
            .label("Validate an unknown room")
            .on_press(move || {
                let event = ScheduleEvent::new(
                    ScheduleKey::new("unknown-room-event").unwrap(),
                    "Proposed room booking",
                    ScheduleInterval::new(dt("2026-11-04T09:00"), dt("2026-11-04T10:00")).unwrap(),
                )
                .unwrap()
                .resource(ScheduleKey::new("room-z").unwrap());
                let model =
                    ScheduleData::new(ScheduleZone::Civil, window(ScheduleView::Agenda), [event])
                        .unwrap();
                let rooms = ScheduleResources::new([ScheduleResource::new(
                    ScheduleKey::new("room-a").unwrap(),
                    "Room A",
                )
                .unwrap()])
                .unwrap();
                let error = model.validate_resources(&rooms).unwrap_err();
                status.revise(|old| *old = format!("Resource rejected: {error}"));
            });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group()
                    .theme(theme.clone())
                    .fill((key, interval, resource)),
                sapid::text().theme(theme.clone()).content(status),
                agenda_view("validated-current-events", sample(ScheduleView::Agenda))
                    .theme(theme.clone())
                    .viewport_height(260.),
            ))
            .show_in(scope)
    })
}
