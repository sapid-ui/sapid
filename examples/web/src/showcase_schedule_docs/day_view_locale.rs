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
    day_view("day-view-locale", sample(ScheduleView::Day))
        .theme(theme)
        .locale(DateLocale::ZhCn)
        .week_start(Weekday::Sunday)
        .direction(glory::render::Direction::Rtl)
        .viewport_height(280.)
        .time_window(ScheduleTimeWindow::new(8 * 60, 16 * 60, 30).unwrap())
        .slot_height(44.)
}
