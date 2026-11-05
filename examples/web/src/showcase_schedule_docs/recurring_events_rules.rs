use glory::prelude::*;
use sapid::prelude::*;
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
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
fn repeated(
    anchor: CivilDate,
    current: ScheduleWindow,
    frequency: RecurrenceFrequency,
    every: u16,
    end: RecurrenceEnd,
    exceptions: Vec<CivilDate>,
) -> ScheduleData {
    let event = ScheduleEvent::new(
        ScheduleKey::new("standup").unwrap(),
        match frequency {
            RecurrenceFrequency::Daily => "Daily standup",
            RecurrenceFrequency::Weekly { .. } => "Weekly planning",
            RecurrenceFrequency::Monthly { .. } => "Month-end review",
            RecurrenceFrequency::Yearly { .. } => "Annual review",
        },
        ScheduleInterval::new(
            LocalDateTime::new(anchor, LocalTime::new(9, 0, 0).unwrap()),
            LocalDateTime::new(anchor, LocalTime::new(10, 0, 0).unwrap()),
        )
        .unwrap(),
    )
    .unwrap()
    .recurring(ScheduleRecurrence::new(frequency, every, end).unwrap())
    .unwrap()
    .except_dates(exceptions)
    .unwrap();
    ScheduleData::new(ScheduleZone::Civil, current, [event]).unwrap()
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let current = window(ScheduleView::Month);
        let weekdays = (1 << Weekday::Monday.index())
            | (1 << Weekday::Wednesday.index())
            | (1 << Weekday::Friday.index());
        let daily = repeated(
            day("2026-11-02"),
            current,
            RecurrenceFrequency::Daily,
            2,
            RecurrenceEnd::Until(day("2026-11-08")),
            Vec::new(),
        );
        let weekly = repeated(
            day("2026-11-02"),
            current,
            RecurrenceFrequency::Weekly {
                weekdays,
                week_start: Weekday::Monday,
            },
            1,
            RecurrenceEnd::Count(6),
            Vec::new(),
        );
        let summary = format!(
            "Every 2 days through November 8: {} · Weekly COUNT 6: {}",
            daily.expand(64).unwrap().len(),
            weekly.expand(64).unwrap().len()
        );
        sapid::stack().theme(theme.clone()).fill((
            sapid::text().theme(theme.clone()).content(summary),
            agenda_view("daily-until", daily)
                .theme(theme.clone())
                .name("Every two days until November 8")
                .viewport_height(260.),
            agenda_view("weekly-count", weekly)
                .theme(theme)
                .name("Monday Wednesday Friday")
                .viewport_height(260.),
        ))
    }
}
