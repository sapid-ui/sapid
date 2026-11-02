use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Combobox", theme)
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

    fn combobox_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let value = scope.cage(None::<String>);
            let search = scope.cage(String::new());
            field("combobox-framework")
                .theme(theme.clone())
                .label("Your favorite framework")
                .description("Type to filter options, then choose a suggestion.")
                .control(move |field| {
                    combobox("combobox-framework")
                        .field(field)
                        .value(value)
                        .search(search)
                        .options(framework_options())
                        .empty_message("Nothing found")
                        .placeholder("Search or pick value")
                        .on_value_change(move |next| value.revise(|current| *current = next))
                        .on_search_change(move |next| search.revise(|current| *current = next))
                })
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Combobox" => Box::new(combobox_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
