use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Autocomplete", theme)
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

    fn autocomplete_options() -> Vec<sapid::NativeOption> {
        ["React", "Angular", "Vue", "Svelte"]
            .into_iter()
            .map(|label| autocomplete_option(label.to_lowercase(), label))
            .collect()
    }

    fn autocomplete_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let value = scope.cage(String::new());
            let selected = scope.cage(None::<String>);
            field("autocomplete-framework")
                .theme(theme.clone())
                .label("Your favorite framework")
                .description("Enter any text or pick one of the suggestions.")
                .control(move |field| {
                    autocomplete("autocomplete-framework")
                        .field(field)
                        .value(value)
                        .selected_key(selected)
                        .options(autocomplete_options())
                        .empty_message("Nothing found")
                        .placeholder("Start typing")
                        .on_value_change(move |next| {
                            value.revise(|current| *current = next);
                            selected.revise(|current| *current = None);
                        })
                        .on_option_select(move |next| {
                            selected.revise(|current| *current = Some(next))
                        })
                })
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Autocomplete" => Box::new(autocomplete_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
