use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P12-01", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::prelude::*;

    fn date(value: &str) -> CivilDate {
        CivilDate::parse_iso(value).expect("showcase date")
    }

    fn datetime(value: &str) -> LocalDateTime {
        parse_local_date_time(value).expect("showcase date-time")
    }

    fn window(view: ScheduleView) -> ScheduleWindow {
        match view {
            ScheduleView::Day => ScheduleWindow::days(date("2024-02-29"), 1),
            ScheduleView::Week | ScheduleView::Agenda => {
                ScheduleWindow::days(date("2024-02-26"), 7)
            }
            ScheduleView::Month | ScheduleView::MobileMonth => {
                ScheduleWindow::month(YearMonth::new(2024, 2).expect("showcase month"))
            }
            ScheduleView::Year => {
                ScheduleWindow::year(CivilYear::new(2024).expect("showcase year"))
            }
        }
        .expect("showcase schedule window")
    }

    fn data(view: ScheduleView) -> ScheduleData {
        ScheduleData::new(
            ScheduleZone::Civil,
            window(view),
            [
                ScheduleEvent::new(
                    ScheduleKey::new("planning").expect("showcase event key"),
                    "Planning",
                    ScheduleInterval::new(
                        datetime("2024-02-29T09:00"),
                        datetime("2024-02-29T10:00"),
                    )
                    .expect("showcase event interval"),
                )
                .expect("showcase event"),
                ScheduleEvent::new(
                    ScheduleKey::new("standup").expect("showcase event key"),
                    "Daily standup",
                    ScheduleInterval::new(
                        datetime("2024-02-29T09:30"),
                        datetime("2024-02-29T10:00"),
                    )
                    .expect("showcase event interval"),
                )
                .expect("showcase event")
                .recurring(
                    ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Never)
                        .expect("showcase recurrence"),
                )
                .expect("showcase recurring event"),
            ],
        )
        .expect("showcase schedule data")
    }

    fn view_label(view: ScheduleView) -> &'static str {
        match view {
            ScheduleView::Agenda => "Agenda",
            ScheduleView::Day => "Day",
            ScheduleView::Week => "Week",
            ScheduleView::Month => "Month",
            ScheduleView::Year => "Year",
            ScheduleView::MobileMonth => "Mobile month",
        }
    }

    fn heading(theme: &Theme, id: &'static str, title: &'static str) -> impl Widget {
        glory::ui::view()
            .semantics(Some(std::boxed::Box::new(glory::render::Semantics {
                id: Some(glory::render::SemanticId::new(id).expect("showcase section id")),
                ..Default::default()
            })))
            .fill(sapid::title(3).theme(theme.clone()).content(title))
    }

    pub fn demo_for(identity: &str, theme: Theme) -> impl Widget {
        assert_eq!(identity, "P12-01");
        widget(move |scope| {
            let selected_view = scope.cage(ScheduleView::Week);
            let schedule_data = scope.cage(data(ScheduleView::Week));
            let current_view =
                selected_view.map(|value| format!("Current view: {}", view_label(*value)));
            let current_window = schedule_data.map(|value| {
                format!(
                    "Loaded window: {} — {}",
                    value.window().start().date(),
                    value.window().end().date()
                )
            });
            let update_scope = scope.downgrade();
            let requested_data = schedule_data;

            sapid::stack()
            .theme(theme.clone())
            .gap(24.)
            .fill((
                heading(&theme, "schedule-usage", "Usage and views"),
                schedule("showcase-schedule", schedule_data)
                    .theme(theme.clone())
                    .name("Project schedule")
                    .view(ScheduleView::Week)
                    .view_value(selected_view)
                    .available_views([
                        ScheduleView::Day,
                        ScheduleView::Week,
                        ScheduleView::Month,
                        ScheduleView::Year,
                    ])
                    .viewport_height(360.)
                    .on_view_change(move |next| {
                        if let Some(scope) = update_scope.upgrade() {
                            let _ = scope.batch(|| {
                                schedule_data.revise(|current| *current = data(next));
                                selected_view.revise(|current| *current = next);
                            });
                        }
                    })
                    .on_window_request(move |request| {
                        requested_data.revise(|current| {
                            *current = ScheduleData::new(
                                request.zone,
                                request.window,
                                current.events().iter().cloned(),
                            )
                            .expect("requested showcase window");
                        });
                    }),
                heading(&theme, "schedule-controlled-view", "Controlled view"),
                sapid::text()
                    .theme(theme.clone())
                    .class("site-schedule-current-view")
                    .content(current_view),
                heading(
                    &theme,
                    "schedule-current-window",
                    "Application-owned current window",
                ),
                sapid::text()
                    .theme(theme.clone())
                    .class("site-schedule-current-window")
                    .content(current_window),
                sapid::text().theme(theme.clone()).content(
                    "Previous and next propose a finite window; this example accepts the request and supplies matching event data.",
                ),
                heading(&theme, "schedule-recurrence", "Recurring event data"),
                data_list()
                    .theme(theme.clone())
                    .name("Recurring event model")
                    .item(
                        data_list_item()
                            .term("Event")
                            .fill(sapid::text().theme(theme.clone()).content("Daily standup")),
                    )
                    .item(
                        data_list_item()
                            .term("Frequency")
                            .fill(sapid::text().theme(theme.clone()).content("Daily")),
                    )
                    .item(
                        data_list_item()
                            .term("Expansion")
                            .fill(sapid::text().theme(theme.clone()).content("Current window only")),
                    ),
            ))
            .show_in(scope)
        })
    }
}
