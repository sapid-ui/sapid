use glory::prelude::*;
use sapid::prelude::*;

fn date(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).expect("showcase date")
}

fn datetime(value: &str) -> LocalDateTime {
    parse_local_date_time(value).expect("showcase date-time")
}

fn data(view: ScheduleView) -> ScheduleData {
    let start = if view == ScheduleView::Day {
        date("2024-02-29")
    } else {
        date("2024-02-26")
    };
    let days = if view == ScheduleView::Day { 1 } else { 7 };
    ScheduleData::new(
        ScheduleZone::Civil,
        ScheduleWindow::days(start, days).expect("showcase window"),
        [
            ScheduleEvent::new(
                ScheduleKey::new("planning").expect("event key"),
                "Planning",
                ScheduleInterval::new(datetime("2024-02-29T09:00"), datetime("2024-02-29T10:30"))
                    .expect("event interval"),
            )
            .expect("event"),
            ScheduleEvent::new(
                ScheduleKey::new("review").expect("event key"),
                "Design review",
                ScheduleInterval::new(datetime("2024-02-29T09:30"), datetime("2024-02-29T11:00"))
                    .expect("event interval"),
            )
            .expect("event"),
            ScheduleEvent::new(
                ScheduleKey::new("release").expect("event key"),
                "Release day",
                ScheduleInterval::all_day(date("2024-02-29"), date("2024-03-01"))
                    .expect("all-day interval"),
            )
            .expect("event")
            .all_day()
            .expect("all-day event"),
        ],
    )
    .expect("showcase schedule")
}

fn heading(theme: &Theme, id: &'static str, title: &'static str) -> impl Widget {
    glory::ui::view()
        .semantics(Some(std::boxed::Box::new(glory::render::Semantics {
            id: Some(glory::render::SemanticId::new(id).expect("section id")),
            ..Default::default()
        })))
        .fill(sapid::title(3).theme(theme.clone()).content(title))
}

pub fn demo_for(identity: &str, theme: Theme) -> std::boxed::Box<dyn Widget> {
    let view = match identity {
        "P12-02" => ScheduleView::Day,
        "P12-02-weekview" => ScheduleView::Week,
        _ => unreachable!("unsupported timeline showcase: {identity}"),
    };
    let noun = if view == ScheduleView::Day { "day" } else { "week" };
    std::boxed::Box::new(sapid::stack().theme(theme.clone()).gap(24.).fill((
        heading(&theme, "timeline-usage", "Usage and exact time range"),
        schedule(format!("showcase-{noun}"), data(view))
            .theme(theme.clone())
            .name(format!("Configured {noun} schedule"))
            .view(view)
            .time_window(ScheduleTimeWindow::new(8 * 60, 14 * 60, 30).expect("time window"))
            .slot_height(44.)
            .viewport_height(340.),
        heading(&theme, "timeline-overlap", "Overlap lanes and all-day events"),
        sapid::text().theme(theme.clone()).content(
            "Planning and Design review overlap in independent lanes; Release day stays in the all-day region.",
        ),
        heading(&theme, "timeline-presentation", "Presentation options"),
        schedule(format!("showcase-{noun}-compact"), data(view))
            .theme(theme.clone())
            .class("site-schedule-timeline-compact")
            .name(format!("Compact read-only {noun} schedule"))
            .view(view)
            .with_header(false)
            .with_all_day(false)
            .readonly(true)
            .time_window(ScheduleTimeWindow::new(9 * 60, 12 * 60, 60).expect("time window"))
            .slot_height(60.)
            .viewport_height(240.),
        heading(&theme, "timeline-contract", "Controlled finite contract"),
        sapid::text().theme(theme).content(
            "Selection and window changes are typed proposals. The application owns accepted data; no clock, route, network or implicit event mutation is installed.",
        ),
    )))
}
