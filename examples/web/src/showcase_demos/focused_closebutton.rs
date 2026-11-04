use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("CloseButton", theme)
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
            "CloseButton" => Box::new(
                group().theme(theme.clone()).fill(
                    [
                        Size::ExtraSmall,
                        Size::Small,
                        Size::Medium,
                        Size::Large,
                        Size::ExtraLarge,
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(index, size)| {
                        close_button()
                            .theme(theme.clone())
                            .size(size)
                            .name(format!("Close size {}", index + 1))
                    })
                    .collect::<Vec<_>>(),
                ),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
