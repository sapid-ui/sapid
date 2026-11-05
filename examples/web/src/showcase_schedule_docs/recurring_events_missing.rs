use glory::prelude::*;
use sapid::prelude::*;
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn dt(value: &str) -> LocalDateTime {
    parse_local_date_time(value).unwrap()
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
        let months = ScheduleWindow::new(dt("2026-01-01T00:00"), dt("2026-07-01T00:00")).unwrap();
        let year = ScheduleWindow::year(CivilYear::new(2025).unwrap()).unwrap();
        let skip = repeated(
            day("2026-01-31"),
            months,
            RecurrenceFrequency::Monthly {
                missing: MissingScheduleDay::Skip,
            },
            1,
            RecurrenceEnd::Never,
            Vec::new(),
        );
        let clamp = repeated(
            day("2026-01-31"),
            months,
            RecurrenceFrequency::Monthly {
                missing: MissingScheduleDay::Clamp,
            },
            1,
            RecurrenceEnd::Never,
            Vec::new(),
        );
        let leap_skip = repeated(
            day("2024-02-29"),
            year,
            RecurrenceFrequency::Yearly {
                missing: MissingScheduleDay::Skip,
            },
            1,
            RecurrenceEnd::Never,
            Vec::new(),
        );
        let leap_clamp = repeated(
            day("2024-02-29"),
            year,
            RecurrenceFrequency::Yearly {
                missing: MissingScheduleDay::Clamp,
            },
            1,
            RecurrenceEnd::Never,
            Vec::new(),
        );
        let summary = format!(
            "Month-end Skip: {} · Clamp: {}. Leap-year Skip: {} · Clamp: {}.",
            skip.expand(64).unwrap().len(),
            clamp.expand(64).unwrap().len(),
            leap_skip.expand(64).unwrap().len(),
            leap_clamp.expand(64).unwrap().len()
        );
        sapid::stack().theme(theme.clone()).fill((
            sapid::text().theme(theme.clone()).content(summary),
            agenda_view("month-end-skip", skip)
                .theme(theme.clone())
                .name("Missing month-end skip")
                .viewport_height(200.),
            agenda_view("month-end-clamp", clamp)
                .theme(theme.clone())
                .name("Missing month-end clamp")
                .viewport_height(200.),
            agenda_view("leap-skip", leap_skip)
                .theme(theme.clone())
                .name("Missing leap day skip")
                .viewport_height(140.),
            agenda_view("leap-clamp", leap_clamp)
                .theme(theme)
                .name("Missing leap day clamp")
                .viewport_height(140.),
        ))
    }
}
