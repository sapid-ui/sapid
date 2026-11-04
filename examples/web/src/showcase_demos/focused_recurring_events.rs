use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Recurring events", theme)
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

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Recurring events" => Box::new(
                data_list()
                    .theme(theme)
                    .name("Recurrence rule")
                    .item(
                        data_list_item()
                            .term("Frequency")
                            .fill(text().content("Daily")),
                    )
                    .item(
                        data_list_item()
                            .term("Interval")
                            .fill(text().content("Every day")),
                    )
                    .item(data_list_item().term("End").fill(text().content("Never"))),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
