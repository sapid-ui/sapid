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
    {
        let model = sample(ScheduleView::Agenda);
        let range = format!(
            "{} to {} (end exclusive)",
            model.window().start().date(),
            model.window().end().date()
        );
        let count = model.events().len().to_string();
        sapid::stack().theme(theme.clone()).fill((
            data_list()
                .theme(theme.clone())
                .name("Event data contract")
                .item(
                    data_list_item().term("Zone").fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("Civil: no implicit UTC offset"),
                    ),
                )
                .item(
                    data_list_item()
                        .term("Window")
                        .fill(sapid::text().theme(theme.clone()).content(range)),
                )
                .item(
                    data_list_item()
                        .term("Declared events")
                        .fill(sapid::text().theme(theme.clone()).content(count)),
                ),
            agenda_view("event-data-overview", model)
                .theme(theme)
                .name("Current event data")
                .viewport_height(320.),
        ))
    }
}
