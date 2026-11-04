use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Marquee", theme)
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
            "Marquee" => Box::new(
                marquee()
                    .theme(theme)
                    .text(
                        "Latest updates: native and browser motion preserve stable accessible text",
                    )
                    .period_millis(1200u32)
                    .style(
                        Style::new()
                            .width(Length::Px(320.0))
                            .max_width(Limit::Percent(100.0)),
                    ),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
