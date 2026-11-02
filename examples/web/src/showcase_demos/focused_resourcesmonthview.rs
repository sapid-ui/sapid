use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("ResourcesMonthView", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{FileConfig, Orientation};
    use glory::style::{FlexWrap, ObjectFit};
    use glory::ui::text;
    use sapid::prelude::*;
    use sapid::{
        Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button,
        file_input, group, image, indicator, progress, ring_progress, select_group, select_option,
        semi_circle_progress, theme_icon,
    };
    use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

    fn date(value: &str) -> CivilDate {
        CivilDate::parse_iso(value).unwrap()
    }

    fn datetime(value: &str) -> LocalDateTime {
        parse_local_date_time(value).unwrap()
    }

    fn resource_data(window: ScheduleWindow) -> ResourceScheduleData {
        let key = |value: &str| ScheduleKey::new(value).unwrap();
        let event = |id: &str, title: &str, resource: &str| {
            ScheduleEvent::new(
                key(id),
                title,
                ScheduleInterval::new(datetime("2024-02-29T09:00"), datetime("2024-02-29T10:00"))
                    .unwrap(),
            )
            .unwrap()
            .resource(key(resource))
        };
        ResourceScheduleData::new(
            ScheduleData::new(
                ScheduleZone::Civil,
                window,
                [
                    event("planning", "Planning", "room-a"),
                    event("review", "Review", "room-b"),
                ],
            )
            .unwrap(),
            ScheduleResources::new([
                ScheduleResource::new(key("room-a"), "Room A").unwrap(),
                ScheduleResource::new(key("room-b"), "Room B").unwrap(),
            ])
            .unwrap(),
            ScheduleResourceWindow::new(0, 2).unwrap(),
        )
        .unwrap()
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "ResourcesDayView" | "ResourcesWeekView" | "ResourcesMonthView"
            | "ResourcesSchedule" => {
                let window = if name == "ResourcesMonthView" {
                    ScheduleWindow::month(YearMonth::new(2024, 2).unwrap()).unwrap()
                } else if name == "ResourcesWeekView" {
                    ScheduleWindow::days(date("2024-02-26"), 7).unwrap()
                } else {
                    ScheduleWindow::days(date("2024-02-29"), 1).unwrap()
                };
                let data = resource_data(window);
                let widget: Box<dyn Widget> = match name {
                    "ResourcesWeekView" => Box::new(
                        resources_week_view("mantine-resource-week", data)
                            .theme(theme)
                            .name("Resources this week")
                            .viewport_height(400.0),
                    ),
                    "ResourcesMonthView" => Box::new(
                        resources_month_view("mantine-resource-month", data)
                            .theme(theme)
                            .name("Resources this month"),
                    ),
                    "ResourcesSchedule" => Box::new(
                        resources_schedule("mantine-resources", data)
                            .theme(theme)
                            .name("Resource schedule")
                            .viewport_height(400.0),
                    ),
                    _ => Box::new(
                        resources_day_view("mantine-resource-day", data)
                            .theme(theme)
                            .name("Resources today")
                            .viewport_height(400.0),
                    ),
                };
                widget
            }
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
