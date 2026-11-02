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
                let value = scope.cage("#228be6".to_owned());
                field("mantine-color-input")
                    .theme(theme.clone())
                    .label("Accent color")
                    .description("Enter a hex color, including an optional alpha channel.")
                    .control(move |field| {
                        color_input("mantine-color-input")
                            .field(field)
                            .value(value)
                            .on_value_change(move |next| value.revise(|current| *current = next))
                    })
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
