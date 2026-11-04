use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P12-05-resourcesschedule", theme)
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

    fn key(value: &str) -> ScheduleKey {
        ScheduleKey::new(value).expect("showcase schedule key")
    }

    fn window(view: ResourceScheduleView) -> ScheduleWindow {
        match view {
            ResourceScheduleView::Day => ScheduleWindow::days(date("2024-02-29"), 1),
            ResourceScheduleView::Week => ScheduleWindow::days(date("2024-02-26"), 7),
            ResourceScheduleView::Month => {
                ScheduleWindow::month(YearMonth::new(2024, 2).expect("showcase month"))
            }
        }
        .expect("showcase resource window")
    }

    fn data(view: ResourceScheduleView) -> ResourceScheduleData {
        let permissions = SchedulePermissions {
            select: true,
            move_event: true,
            resize_event: true,
        };
        ResourceScheduleData::new(
            ScheduleData::new(
                ScheduleZone::Civil,
                window(view),
                [
                    ScheduleEvent::new(
                        key("planning"),
                        "Planning",
                        ScheduleInterval::new(
                            datetime("2024-02-29T09:00"),
                            datetime("2024-02-29T10:00"),
                        )
                        .expect("showcase event interval"),
                    )
                    .expect("showcase event")
                    .resource(key("room-a"))
                    .permissions(permissions),
                    ScheduleEvent::new(
                        key("review"),
                        "Review",
                        ScheduleInterval::new(
                            datetime("2024-02-29T10:00"),
                            datetime("2024-02-29T11:00"),
                        )
                        .expect("showcase event interval"),
                    )
                    .expect("showcase event")
                    .resource(key("room-b"))
                    .permissions(permissions),
                ],
            )
            .expect("showcase resource events"),
            ScheduleResources::new([
                ScheduleResource::new(key("room-a"), "Room A").expect("showcase resource"),
                ScheduleResource::new(key("room-b"), "Room B").expect("showcase resource"),
            ])
            .expect("showcase resources"),
            ScheduleResourceWindow::new(0, 2).expect("showcase resource page"),
        )
        .expect("showcase resource schedule")
    }

    fn view_label(view: ResourceScheduleView) -> &'static str {
        match view {
            ResourceScheduleView::Day => "Day",
            ResourceScheduleView::Week => "Week",
            ResourceScheduleView::Month => "Month",
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
        assert_eq!(identity, "P12-05-resourcesschedule");
        widget(move |scope| {
            let selected_view = scope.cage(ResourceScheduleView::Day);
            let schedule_data = scope.cage(data(ResourceScheduleView::Day));
            let current_view =
                selected_view.map(|value| format!("Current resource view: {}", view_label(*value)));
            let edit_status = scope.cage("No edit proposed".to_owned());
            let update_scope = scope.downgrade();
            let requested_data = schedule_data;

            sapid::stack()
            .theme(theme.clone())
            .gap(24.)
            .fill((
                heading(&theme, "resources-schedule-usage", "Usage and views"),
                resources_schedule("showcase-resources", schedule_data)
                    .theme(theme.clone())
                    .name("Room schedule")
                    .view_value(selected_view)
                    .available_views([
                        ResourceScheduleView::Day,
                        ResourceScheduleView::Week,
                        ResourceScheduleView::Month,
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
                            *current = ResourceScheduleData::new(
                                ScheduleData::new(
                                    request.zone,
                                    request.window,
                                    current.data().events().iter().cloned(),
                                )
                                .expect("requested resource events"),
                                current.resources().clone(),
                                request.resource_window,
                            )
                            .expect("requested resource window");
                        });
                    })
                    .on_edit(move |edit| {
                        edit_status.revise(|status| {
                            *status = format!("Proposed edit: {:?}", edit.kind)
                        });
                    }),
                heading(
                    &theme,
                    "resources-schedule-controlled",
                    "Controlled state",
                ),
                sapid::text()
                    .theme(theme.clone())
                    .class("site-resources-current-view")
                    .content(current_view),
                heading(
                    &theme,
                    "resources-schedule-editing",
                    "Move and resize proposals",
                ),
                sapid::text()
                    .theme(theme.clone())
                    .class("site-resources-edit-status")
                    .content(edit_status),
                sapid::text().theme(theme.clone()).content(
                    "Editing is permission checked and proposes an exact event, resource and half-open interval. The application remains authoritative.",
                ),
                heading(
                    &theme,
                    "resources-schedule-window",
                    "Finite date and resource windows",
                ),
                sapid::text().theme(theme.clone()).content(
                    "Date paging and resource paging request bounded replacement data without retaining visited resources.",
                ),
            ))
            .show_in(scope)
        })
    }
}
