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
        let moved = scope.cage(false);
        let current = scope.cage(window(ScheduleView::Week));
        let data = scope.bond(move || {
            let model = repeated(
                day("2026-11-02"),
                current.get(),
                RecurrenceFrequency::Daily,
                1,
                RecurrenceEnd::Count(7),
                if moved.get() {
                    vec![day("2026-11-04")]
                } else {
                    Vec::new()
                },
            );
            let mut events = model.events().to_vec();
            if moved.get() {
                events.push(
                    ScheduleEvent::new(
                        ScheduleKey::new("standup-20261104-replacement").unwrap(),
                        "Rescheduled standup",
                        ScheduleInterval::new(dt("2026-11-04T14:00"), dt("2026-11-04T15:00"))
                            .unwrap(),
                    )
                    .unwrap(),
                );
            }
            ScheduleData::new(ScheduleZone::Civil, current.get(), events).unwrap()
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Move Wednesday standup to afternoon")
                    .checked(moved)
                    .on_checked_change(move |value| moved.revise(|old| *old = value)),
                week_view("one-occurrence-replacement", data)
                    .theme(theme.clone())
                    .time_window(ScheduleTimeWindow::new(8 * 60, 16 * 60, 30).unwrap())
                    .viewport_height(340.)
                    .on_window_request(move |request| current.revise(|old| *old = request.window)),
            ))
            .show_in(scope)
    })
}
