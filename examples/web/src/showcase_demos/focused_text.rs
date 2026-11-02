use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Text", theme)
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
            "Text" => Box::new(
                sapid::stack().theme(theme.clone()).gap(10.0).fill((
                    sapid::text().theme(theme.clone()).content("Default text"),
                    sapid::text()
                        .theme(theme.clone())
                        .variant(TextVariant::Lead)
                        .content("Lead text"),
                    sapid::text()
                        .theme(theme)
                        .variant(TextVariant::Muted)
                        .content("Muted text"),
                )),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
