use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("ColorInput", theme)
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
            "ColorInput" => Box::new(widget(move |scope| {
                let hex = scope.cage("#228be6".to_owned());
                field("showcase-color-input")
                    .theme(theme.clone())
                    .label("Input label")
                    .description("Input description")
                    .control(move |field| {
                        color_input("showcase-color-input")
                            .field(field)
                            .value(hex)
                            .on_value_change(move |next| hex.revise(|current| *current = next))
                    })
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
