use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Grid", theme)
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
            "Grid" => Box::new(
                grid("showcase-grid")
                    .theme(theme.clone())
                    .columns(3)
                    .gap(12.0)
                    .items((1..=6).map(|index| {
                        grid_item(format!("showcase-grid-{index}"))
                            .theme(theme.clone())
                            .span(if index == 1 { 2 } else { 1 })
                            .fill(tile(&theme, &format!("Grid item {index}")))
                    })),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
