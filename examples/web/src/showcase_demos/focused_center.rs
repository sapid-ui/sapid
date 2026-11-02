use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Center", theme)
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

    fn tile(theme: &Theme, label: &str) -> sapid::Box {
        sapid::box_()
            .theme(theme.clone())
            .style(Style {
                background: Some(theme.accent_soft),
                color: Some(theme.accent_text),
                ..Style::new().padding(Edges::all(16.0)).radius(theme.radius)
            })
            .fill(text().content(label.to_owned()))
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Center" => Box::new(
                center()
                    .theme(theme.clone())
                    .inline(true)
                    .height(Length::Px(140.0))
                    .fill(tile(&theme, "Centered content")),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
