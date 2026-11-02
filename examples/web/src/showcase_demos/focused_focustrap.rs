use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("FocusTrap", theme)
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
            "FocusTrap" => Box::new(widget(move |scope| {
                let active = scope.cage(true);
                sapid::stack()
                    .theme(theme.clone())
                    .gap(12.0)
                    .fill((
                        focus_scope().active(active).fill((
                            sapid::text_field().theme(theme.clone()).name("First field"),
                            sapid::text_field()
                                .theme(theme.clone())
                                .name("Second field"),
                        )),
                        sapid::button()
                            .theme(theme.clone())
                            .label(active.map(|value| {
                                if *value {
                                    "Deactivate focus trap".to_owned()
                                } else {
                                    "Activate focus trap".to_owned()
                                }
                            }))
                            .on_press(move || active.revise(|current| *current = !*current)),
                    ))
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
