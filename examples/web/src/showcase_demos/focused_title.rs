use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Title", theme)
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
            "Title" => Box::new(sapid::stack().theme(theme.clone()).gap(8.0).fill((
                title(1).theme(theme.clone()).content("Heading level 1"),
                title(2).theme(theme.clone()).content("Heading level 2"),
                title(3).theme(theme).content("Heading level 3"),
            ))),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
