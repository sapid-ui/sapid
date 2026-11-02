use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Image", theme)
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

    fn media(name: &str) -> String {
        format!(
            "{}/{}",
            option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets"),
            name
        )
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Image" => Box::new(
                image(media("image-pattern.png"))
                    .theme(theme)
                    .alt("Abstract color pattern")
                    .fallback_src(media("image-fallback.png"))
                    .width(Length::Px(320.0))
                    .height(Length::Px(180.0))
                    .fit(ObjectFit::Cover),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
