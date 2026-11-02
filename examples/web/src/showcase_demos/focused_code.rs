use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Code", theme)
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
            "Code" => Box::new(
                sapid::stack().theme(theme.clone()).gap(16.0).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .content("Inline code: let answer = 42;"),
                    code().theme(theme.clone()).content("let answer = 42;"),
                    code_block()
                        .theme(theme.clone())
                        .source("fn main() {\n    println!(\"Hello\");\n}"),
                    group().theme(theme.clone()).fill((
                        code()
                            .theme(theme.clone())
                            .content("Accent code")
                            .style(Style {
                                background: Some(theme.accent),
                                color: Some(theme.on_accent),
                                ..Style::new().padding(Edges::all(4.0)).radius(theme.radius)
                            }),
                        code()
                            .theme(theme.clone())
                            .content("Soft accent code")
                            .style(Style {
                                background: Some(theme.accent_soft),
                                color: Some(theme.accent_text),
                                ..Style::new().padding(Edges::all(4.0)).radius(theme.radius)
                            }),
                    )),
                )),
            ),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
