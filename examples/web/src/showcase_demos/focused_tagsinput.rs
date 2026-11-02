use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("TagsInput", theme)
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
            "PillsInput" | "TagsInput" => {
                let pills = name == "PillsInput";
                Box::new(widget(move |scope| {
                    let state = scope.cage(TagsInputState {
                        values: vec!["Rust".into(), "WebAssembly".into()],
                        search: String::new(),
                    });
                    field("mantine-token-input")
                    .theme(theme.clone())
                    .label(if pills { "Labels" } else { "Skills" })
                    .description("Press Enter to add a value. Backspace removes the last value when search is empty.")
                    .control(move |field| {
                        let input: Box<dyn Widget> = if pills {
                            Box::new(
                                pills_input("mantine-pills-input")
                                    .field(field)
                                    .state(state)
                                    .placeholder("Add a value")
                                    .on_change(move |next| state.revise(|current| *current = next)),
                            )
                        } else {
                            Box::new(
                                tags_input("mantine-tags-input")
                                    .field(field)
                                    .state(state)
                                    .placeholder("Add a value")
                                    .on_change(move |next| state.revise(|current| *current = next)),
                            )
                        };
                        input
                    })
                    .show_in(scope)
                }))
            }
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
