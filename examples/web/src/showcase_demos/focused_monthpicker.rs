use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("MonthPicker", theme)
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

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "MonthPicker" => Box::new(
                month_picker("mantine-month", DatesProvider::new(date("2024-02-29")))
                    .theme(theme)
                    .name("Billing month")
                    .default_value(MonthSelection::Single(Some(
                        YearMonth::new(2024, 2).unwrap(),
                    ))),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
