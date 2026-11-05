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
pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let events = [
            ScheduleEvent::new(
                ScheduleKey::new("first").unwrap(),
                "Morning review",
                ScheduleInterval::new(dt("2026-11-04T09:00"), dt("2026-11-04T10:00")).unwrap(),
            )
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("second").unwrap(),
                "Team check-in",
                ScheduleInterval::new(dt("2026-11-04T10:00"), dt("2026-11-04T11:00")).unwrap(),
            )
            .unwrap(),
            ScheduleEvent::new(
                ScheduleKey::new("release").unwrap(),
                "Release day",
                ScheduleInterval::all_day(day("2026-11-05"), day("2026-11-06")).unwrap(),
            )
            .unwrap()
            .all_day()
            .unwrap(),
        ];
        let model =
            ScheduleData::new(ScheduleZone::Civil, window(ScheduleView::Agenda), events).unwrap();
        let count = model
            .conflicts(ScheduleConflictPolicy::AllResources, 64)
            .unwrap()
            .len();
        sapid::stack().theme(theme.clone()).fill((
            sapid::text().theme(theme.clone()).content(format!(
                "Overlapping pairs: {count}. Release day ends at November 6, 00:00 (exclusive)."
            )),
            agenda_view("half-open-events", model)
                .theme(theme)
                .viewport_height(320.),
        ))
    }
}
