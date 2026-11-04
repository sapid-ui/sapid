use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("List", theme)
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
            "List" => Box::new(
                sapid::stack().theme(theme.clone()).gap(20.0).fill((
                    text_list()
                        .theme(theme.clone())
                        .name("With icons")
                        .item(text_list_item().fill((
                            text().content("✓ "),
                            text().content("Clone or download the repository"),
                        )))
                        .item(
                            text_list_item().fill((
                                text().content("✓ "),
                                text().content("Install dependencies"),
                            )),
                        )
                        .item(text_list_item().fill((
                            text().content("✓ "),
                            text().content("Run the development server"),
                        ))),
                    text_list()
                        .theme(theme.clone())
                        .ordered(true)
                        .name("Nested list")
                        .item(text_list_item().fill(text().content("Create the application")))
                        .item(
                            text_list_item().fill((
                                text().content("Compose widgets"),
                                text_list()
                                    .theme(theme.clone())
                                    .ordered(true)
                                    .item(text_list_item().fill(text().content("Keep keys stable")))
                                    .item(
                                        text_list_item().fill((
                                            text().content("Nest one more level"),
                                            text_list().theme(theme.clone()).item(
                                                text_list_item()
                                                    .fill(text().content("Use semantic controls")),
                                            ),
                                        )),
                                    ),
                            )),
                        )
                        .item(text_list_item().fill(text().content("Choose a host"))),
                )),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
