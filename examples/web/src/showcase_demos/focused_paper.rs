use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Paper", theme)
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
            "Paper" => Box::new(
                group()
                    .theme(theme.clone())
                    .gap(16.0)
                    .wrap(FlexWrap::Wrap)
                    .fill(
                        [
                            (ShadowSize::None, "Flat paper"),
                            (ShadowSize::Small, "Small shadow"),
                            (ShadowSize::Large, "Large shadow"),
                        ]
                        .into_iter()
                        .map(|(shadow, label)| {
                            paper()
                                .theme(theme.clone())
                                .bordered(shadow != ShadowSize::None)
                                .shadow(shadow)
                                .style(
                                    Style::new()
                                        .padding(Edges::all(20.0))
                                        .radius(theme.radius)
                                        .width(Length::Px(180.0)),
                                )
                                .fill(sapid::text().theme(theme.clone()).content(label))
                        })
                        .collect::<Vec<_>>(),
                    ),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
