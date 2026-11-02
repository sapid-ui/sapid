use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("ActionBar", theme)
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
            "ActionBar" => Box::new(widget(move |scope| {
                let count = scope.cage(2usize);
                action_bar("mantine-action-bar")
                    .theme(theme.clone())
                    .name("Selected item actions")
                    .selected_count(count)
                    .group(toolbar_group(&theme, "selected-actions"))
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
