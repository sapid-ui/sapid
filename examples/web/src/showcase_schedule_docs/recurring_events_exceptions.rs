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
    widget(move |scope| {
        let skip = scope.cage(true);
        let current = scope.cage(window(ScheduleView::Month));
        let data = scope.bond(move || {
            repeated(
                day("2026-11-02"),
                current.get(),
                RecurrenceFrequency::Daily,
                1,
                RecurrenceEnd::Count(7),
                if skip.get() {
                    vec![day("2026-11-05")]
                } else {
                    Vec::new()
                },
            )
        });
        let status = scope.bond(move || {
            data.with(|model| {
                format!(
                    "{} current occurrences",
                    model.expand(MAX_SCHEDULE_OCCURRENCES).unwrap().len()
                )
            })
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Skip Thursday standup")
                    .checked(skip)
                    .on_checked_change(move |value| skip.revise(|old| *old = value)),
                month_view("repeated-exceptions", data)
                    .theme(theme.clone())
                    .today(day("2026-11-04"))
                    .max_events_per_day(2)
                    .on_window_request(move |request| current.revise(|old| *old = request.window)),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
