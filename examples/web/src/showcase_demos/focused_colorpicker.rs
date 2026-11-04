use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("ColorPicker", theme)
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
            "ColorPicker" => Box::new(widget(move |scope| {
                let value = scope.cage(Hsva {
                    hue: 210.0,
                    saturation: 0.8,
                    value: 0.9,
                    alpha: 1.0,
                });
                color_picker("showcase-color-picker")
                    .theme(theme.clone())
                    .name("Accent color")
                    .value(value)
                    .show_precision_sliders(false)
                    .on_value_change(move |next| value.revise(|current| *current = next))
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
