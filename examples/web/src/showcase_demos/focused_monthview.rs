use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("MonthView", theme)
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

    fn datetime(value: &str) -> LocalDateTime {
        parse_local_date_time(value).unwrap()
    }

    fn schedule_data(window: ScheduleWindow) -> ScheduleData {
        ScheduleData::new(
            ScheduleZone::Civil,
            window,
            [
                ScheduleEvent::new(
                    ScheduleKey::new("planning").unwrap(),
                    "Planning",
                    ScheduleInterval::new(
                        datetime("2024-02-29T09:00"),
                        datetime("2024-02-29T10:00"),
                    )
                    .unwrap(),
                )
                .unwrap(),
                ScheduleEvent::new(
                    ScheduleKey::new("standup").unwrap(),
                    "Daily standup",
                    ScheduleInterval::new(
                        datetime("2024-02-29T09:30"),
                        datetime("2024-02-29T10:00"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .recurring(
                    ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Never)
                        .unwrap(),
                )
                .unwrap(),
            ],
        )
        .unwrap()
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "MonthView" => Box::new(
                month_view(
                    "mantine-month-view",
                    schedule_data(ScheduleWindow::month(YearMonth::new(2024, 2).unwrap()).unwrap()),
                )
                .theme(theme)
                .name("Month schedule"),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
