use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("DateTimePicker", theme)
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

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "DateTimePicker" => Box::new(
                date_time_picker(
                    "showcase-date-time",
                    DatesProvider::new(date("2024-02-29")),
                    DateTimePolicy::Civil,
                )
                .theme(theme)
                .name("Appointment date and time")
                .default_value(Some(datetime("2024-02-29T10:20"))),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
