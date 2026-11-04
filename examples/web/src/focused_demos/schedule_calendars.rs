use glory::prelude::*;
use sapid::prelude::*;

fn date(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).expect("showcase date")
}

fn datetime(value: &str) -> LocalDateTime {
    parse_local_date_time(value).expect("showcase date-time")
}

fn view_for(identity: &str) -> ScheduleView {
    match identity {
        "P12-03" => ScheduleView::Month,
        "P12-03-yearview" => ScheduleView::Year,
        "P12-03-mobilemonthview" => ScheduleView::MobileMonth,
        _ => unreachable!("unsupported calendar showcase: {identity}"),
    }
}

fn data(view: ScheduleView) -> ScheduleData {
    let window = match view {
        ScheduleView::Year => ScheduleWindow::year(CivilYear::new(2024).expect("year")),
        _ => ScheduleWindow::month(YearMonth::new(2024, 2).expect("month")),
    }
    .expect("showcase window");
    ScheduleData::new(
        ScheduleZone::Civil,
        window,
        (0..5).map(|index| {
            ScheduleEvent::new(
                ScheduleKey::new(format!("calendar-{index}")).expect("event key"),
                format!("Calendar event {}", index + 1),
                ScheduleInterval::new(datetime("2024-02-29T09:00"), datetime("2024-02-29T10:00"))
                    .expect("event interval"),
            )
            .expect("event")
        }),
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
    let view = view_for(identity);
    let name = match view {
        ScheduleView::Month => "month",
        ScheduleView::Year => "year",
        ScheduleView::MobileMonth => "mobile month",
        _ => unreachable!(),
    };
    std::boxed::Box::new(sapid::stack().theme(theme.clone()).gap(24.).fill((
        heading(&theme, "calendar-usage", "Usage and event disclosure"),
        schedule(format!("showcase-{}", name.replace(' ', "-")), data(view))
            .theme(theme.clone())
            .name(format!("Configured {name} schedule"))
            .view(view)
            .today(date("2024-02-29"))
            .max_events_per_day(2),
        heading(&theme, "calendar-presentation", "Calendar presentation"),
        schedule(
            format!("showcase-{}-compact", name.replace(' ', "-")),
            data(view),
        )
        .theme(theme.clone())
        .class("site-schedule-calendar-compact")
        .name(format!("Compact read-only {name} schedule"))
        .view(view)
        .with_header(false)
        .with_weekdays(false)
        .show_outside_days(false)
        .max_events_per_day(1)
        .readonly(true),
        heading(&theme, "calendar-locale", "Week start, locale and reading direction"),
        data_list()
            .theme(theme.clone())
            .name("Calendar localization options")
            .item(data_list_item().term("Week start").fill(
                sapid::text().theme(theme.clone()).content("Explicit weekday"),
            ))
            .item(data_list_item().term("Locale").fill(
                sapid::text().theme(theme.clone()).content("Finite date labels"),
            ))
            .item(data_list_item().term("Direction").fill(
                sapid::text().theme(theme.clone()).content("LTR or RTL grid navigation"),
            )),
        heading(&theme, "calendar-contract", "Controlled finite contract"),
        sapid::text().theme(theme).content(
            "Paging proposes an exact replacement window. Selection, disclosure and current data remain bounded; no clock or unbounded event history is retained.",
        ),
    )))
}
