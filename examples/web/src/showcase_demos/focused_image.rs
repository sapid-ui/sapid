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
            "Image" => {
                let pattern = media("image-pattern.png");
                let fallback = media("image-fallback.png");
                Box::new(
                    sapid::stack().theme(theme.clone()).gap(14.0).fill((
                        sapid::text().theme(theme.clone()).content("Usage"),
                        image(pattern.clone())
                            .theme(theme.clone())
                            .class("showcase-image-usage")
                            .alt("Abstract color pattern")
                            .width(Length::Px(320.0)),
                        sapid::text().theme(theme.clone()).content("Image height"),
                        image(pattern.clone())
                            .theme(theme.clone())
                            .class("showcase-image-height")
                            .alt("Fixed height color pattern")
                            .width(Length::Px(320.0))
                            .height(Length::Px(160.0)),
                        sapid::text().theme(theme.clone()).content("Image fit"),
                        image(pattern.clone())
                            .theme(theme.clone())
                            .class("showcase-image-fit")
                            .alt("Contained color pattern")
                            .width(Length::Px(320.0))
                            .height(Length::Px(160.0))
                            .fit(ObjectFit::Contain),
                        sapid::text().theme(theme.clone()).content("Fallback image"),
                        image(media("image-invalid.png"))
                            .theme(theme.clone())
                            .class("showcase-image-fallback")
                            .alt("Fallback color pattern")
                            .fallback_src(fallback.clone())
                            .width(Length::Px(320.0))
                            .height(Length::Px(160.0))
                            .fit(ObjectFit::Cover),
                        sapid::text()
                            .theme(theme.clone())
                            .content("Responsive source and loading"),
                        image(pattern.clone())
                            .theme(theme)
                            .class("showcase-image-responsive")
                            .alt("Responsive color pattern")
                            .srcset([
                                ImageCandidate {
                                    source: pattern,
                                    width: 320,
                                },
                                ImageCandidate {
                                    source: fallback,
                                    width: 960,
                                },
                            ])
                            .sizes([
                                ImageSizeHint {
                                    max_viewport_width: Some(600),
                                    slot: ImageSlotSize::ViewportWidth(100),
                                },
                                ImageSizeHint {
                                    max_viewport_width: None,
                                    slot: ImageSlotSize::Pixels(480),
                                },
                            ])
                            .loading(ImageLoading::Lazy)
                            .width(Length::Px(480.0))
                            .height(Length::Px(180.0))
                            .fit(ObjectFit::Cover),
                    )),
                )
            }
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
