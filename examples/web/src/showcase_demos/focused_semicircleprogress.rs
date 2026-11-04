use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("SemiCircleProgress", theme)
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

    fn progress_example(name: &str, theme: Theme) -> Box<dyn Widget> {
        let name = name.to_owned();
        Box::new(widget(move |scope| {
            let value = scope.cage(64.0);
            let value_text = value.map(|value| format!("{value:.0}%"));
            let preview: Box<dyn Widget> = match name.as_str() {
                "Progress" => Box::new(
                    progress(value)
                        .theme(theme.clone())
                        .name("File upload")
                        .value_text(value_text)
                        .size(240.0)
                        .thickness(10.0),
                ),
                "RingProgress" => Box::new(
                    ring_progress(value)
                        .theme(theme.clone())
                        .name("Circular upload")
                        .value_text(value_text)
                        .size(120.0)
                        .thickness(10.0),
                ),
                _ => Box::new(
                    semi_circle_progress(value)
                        .theme(theme.clone())
                        .name("Semicircular upload")
                        .value_text(value_text)
                        .size(160.0)
                        .thickness(10.0),
                ),
            };
            sapid::stack()
                .theme(theme.clone())
                .gap(16.0)
                .fill((
                    preview,
                    sapid::slider("progress-value")
                        .theme(theme.clone())
                        .name("Progress value")
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .value(value)
                        .on_value_change(move |next| value.revise(|current| *current = next)),
                ))
                .show_in(scope)
        }))
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Progress" | "RingProgress" | "SemiCircleProgress" => progress_example(name, theme),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
