use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Typography", theme)
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
            "Typography" => Box::new(typography().theme(theme).content(|context| {
                (
                    context.title(2).content("Rich typography"),
                    context
                        .text()
                        .content("Text and headings inherit the same theme and size."),
                    context
                        .anchor()
                        .label("Read the documentation")
                        .href("https://sapid.taidge.org/"),
                )
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
