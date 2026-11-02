use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Select", theme)
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

    fn framework_options() -> Vec<sapid::NativeOption> {
        ["React", "Angular", "Vue", "Svelte"]
            .into_iter()
            .map(|label| select_option(label.to_lowercase(), label))
            .collect()
    }

    fn select_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let value = scope.cage(None::<String>);
            field("select-framework")
                .theme(theme.clone())
                .label("Your favorite framework")
                .description("Choose one option. Keyboard typing moves through matching labels.")
                .control(move |field| {
                    select("select-framework")
                        .field(field)
                        .value(value)
                        .options(framework_options())
                        .group(
                            select_group("Other libraries")
                                .option(select_option("solid", "Solid"))
                                .option(select_option("qwik", "Qwik")),
                        )
                        .clearable(true)
                        .placeholder("Pick value")
                        .on_value_change(move |next| value.revise(|current| *current = next))
                })
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Select" => Box::new(select_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
