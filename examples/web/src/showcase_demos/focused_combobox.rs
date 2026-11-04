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
            stack()
                .theme(theme.clone())
                .gap(20.0)
                .fill((
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
                                .on_value_change(move |next| {
                                    value.revise(|current| *current = next)
                                })
                                .on_search_change(move |next| {
                                    search.revise(|current| *current = next)
                                })
                        }),
                    field("combobox-prefix-limit")
                        .theme(theme.clone())
                        .label("Prefix filter with visible limit")
                        .description("Four declarations match A, but only two results are mounted.")
                        .control(move |field| {
                            combobox("combobox-prefix-limit")
                                .field(field)
                                .default_search("a")
                                .default_open(true)
                                .filter(ComboboxFilter::Prefix)
                                .limit(2)
                                .loading(true)
                                .loading_left(true)
                                .options([
                                    select_option("angular", "Angular"),
                                    select_option("astro", "Astro"),
                                    select_option("alpine", "Alpine"),
                                    select_option("ark", "Ark UI"),
                                    select_option("react", "React"),
                                ])
                        }),
                    field("combobox-groups")
                        .theme(theme.clone())
                        .label("Grouped options")
                        .description(
                            "Groups preserve declaration order and disabled options are skipped.",
                        )
                        .control(move |field| {
                            combobox("combobox-groups")
                                .field(field)
                                .group(select_group("Frontend").options([
                                    select_option("vue", "Vue"),
                                    select_option("svelte", "Svelte"),
                                ]))
                                .group(select_group("Backend").options([
                                    select_option("axum", "Axum"),
                                    select_option("django", "Django").disabled(true),
                                ]))
                                .placeholder("Search grouped options")
                        }),
                ))
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
