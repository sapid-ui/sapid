use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Text", theme)
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
        "Text" => Box::new(
            sapid::stack().theme(theme.clone()).gap(14.0).fill((
                sapid::text().theme(theme.clone()).content("Usage"),
                sapid::text().theme(theme.clone()).content("Default text"),
                sapid::text().theme(theme.clone()).variant(TextVariant::Lead).content("Lead text"),
                sapid::text().theme(theme.clone()).variant(TextVariant::Muted).content("Muted text"),
                sapid::text().theme(theme.clone()).content("Single-line truncate"),
                sapid::text()
                    .theme(theme.clone())
                    .class("showcase-text-truncate")
                    .truncate(true)
                    .style(Style::new().width(Length::Px(320.0)))
                    .content("This sentence is deliberately too long for the available single line width."),
                sapid::text().theme(theme.clone()).content("Line clamp"),
                sapid::text()
                    .theme(theme.clone())
                    .class("showcase-text-clamp")
                    .line_clamp(2)
                    .style(Style::new().width(Length::Px(340.0)))
                    .content("This paragraph demonstrates a real two-line clamp. The complete value remains in the text node for copying and accessibility even though additional visual lines are clipped by the host."),
                sapid::text().theme(theme.clone()).content("Gradient"),
                sapid::text()
                    .theme(theme)
                    .class("showcase-text-gradient")
                    .font_size(30.0)
                    .weight(700.0)
                    .gradient(glory::style::LinearGradient {
                        angle: 90.0,
                        stops: vec![Color(30, 110, 240, 255), Color(190, 40, 210, 255)],
                    })
                    .content("Gradient text"),
            )),
        ),
        _ => unreachable!("unsupported named demo: {name}"),
    }
    }
}
