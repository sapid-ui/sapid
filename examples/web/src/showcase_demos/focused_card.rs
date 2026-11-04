use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Card", theme)
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
        "Card" => Box::new(
            card()
                .theme(theme.clone())
                .title("Norway Fjord Adventures")
                .description("Explore the magical fjord landscapes with guided tours and outdoor activities.")
                .header(
                    image(media("image-pattern.png"))
                        .theme(theme.clone())
                        .alt("Abstract mountain and fjord landscape")
                        .width(Length::Percent(100.0))
                        .height(Length::Px(160.0))
                        .fit(ObjectFit::Cover),
                )
                .fill(
                    sapid::stack().theme(theme.clone()).gap(12.0).fill((
                        sapid::badge().theme(theme.clone()).content("On sale"),
                        sapid::text().theme(theme.clone()).content("Book a small-group tour with a local guide, travel insurance and flexible cancellation."),
                    )),
                )
                .footer(
                    group().theme(theme.clone()).fill((
                        sapid::button().theme(theme.clone()).label("Book tour"),
                        sapid::button().theme(theme).variant(sapid::ButtonVariant::Secondary).label("View details"),
                    )),
                ),
        ),
        _ => unreachable!("unsupported named demo: {name}"),
    }
    }
}
