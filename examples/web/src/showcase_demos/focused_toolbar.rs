use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Toolbar", theme)
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

    fn toolbar_group(theme: &Theme, key: &str) -> ToolbarGroup {
        sapid::toolbar_group(key).name("Formatting").buttons([
            sapid::button().theme(theme.clone()).label("Bold"),
            sapid::button().theme(theme.clone()).label("Italic"),
        ])
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Toolbar" => Box::new(
                toolbar("mantine-toolbar")
                    .theme(theme.clone())
                    .name("Editor toolbar")
                    .group(toolbar_group(&theme, "editor-actions")),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
