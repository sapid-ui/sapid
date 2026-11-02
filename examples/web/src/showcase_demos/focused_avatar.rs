use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Avatar", theme)
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
            "Avatar" => Box::new(
                sapid::stack().theme(theme.clone()).gap(18.0).fill((
                    group().theme(theme.clone()).gap(16.0).fill((
                        avatar("Ada Lovelace")
                            .theme(theme.clone())
                            .fallback("AL")
                            .source(media("image-pattern.png")),
                        avatar("John Doe").theme(theme.clone()).fallback("JD"),
                        avatar("Square avatar")
                            .theme(theme.clone())
                            .fallback("SQ")
                            .shape(AvatarShape::Square)
                            .size(Size::Large),
                        avatar("Large avatar")
                            .theme(theme.clone())
                            .fallback("LG")
                            .size(Size::ExtraLarge),
                    )),
                    sapid::title(3).theme(theme.clone()).content("Avatar.Group"),
                    avatar_group("showcase-team")
                        .theme(theme.clone())
                        .name("Project team")
                        .max_visible(2)
                        .avatar("ada", avatar("Ada Lovelace").fallback("AL"))
                        .avatar("grace", avatar("Grace Hopper").fallback("GH"))
                        .avatar("katherine", avatar("Katherine Johnson").fallback("KJ"))
                        .avatar("margaret", avatar("Margaret Hamilton").fallback("MH")),
                )),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
