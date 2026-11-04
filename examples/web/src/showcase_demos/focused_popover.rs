use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Popover", theme)
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
            "Popover" => Box::new(widget(move |scope| {
                let controlled = scope.cage(false);
                let controlled_copy = controlled;
                sapid::stack()
                .theme(theme.clone())
                .gap(18.0)
                .fill((
                    sapid::text().theme(theme.clone()).content("Basic usage"),
                    popover("showcase-popover-basic")
                        .theme(theme.clone())
                        .class("showcase-popover-basic")
                        .label("Toggle popover")
                        .title("Popover title")
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("Additional information stays beside its trigger.")
                        }),
                    sapid::text().theme(theme.clone()).content("Same width as target"),
                    popover("showcase-popover-target-width")
                        .theme(theme.clone())
                        .class("showcase-popover-target-width")
                        .label("Open matching panel")
                        .width(PopoverWidth::Target)
                        .trigger_content(|| text().content("A deliberately wide target control"))
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("This panel follows the live trigger width.")
                        }),
                    sapid::text().theme(theme.clone()).content("Arrow, offset and placement"),
                    popover("showcase-popover-arrow")
                        .theme(theme.clone())
                        .class("showcase-popover-arrow")
                        .label("Open positioned panel")
                        .placement(AnchorPlacement {
                            side: AnchorSide::Top,
                            align: AnchorAlign::End,
                            gap: 12.0,
                            arrow: Some(AnchorArrow {
                                size: 10.0,
                                radius: 2.0,
                                offset: 14.0,
                                position: AnchorArrowPosition::Side,
                            }),
                            ..AnchorPlacement::default()
                        })
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("The arrow follows the side selected after collision handling.")
                        }),
                    sapid::text().theme(theme.clone()).content("Controlled state"),
                    popover("showcase-popover-controlled")
                        .theme(theme.clone())
                        .class("showcase-popover-controlled")
                        .label("Toggle controlled popover")
                        .open(controlled)
                        .on_open_change(move |next| controlled_copy.revise(|value| *value = next))
                        .dismiss_on_outside(false)
                        .content({
                            let theme = theme.clone();
                            move || sapid::text().theme(theme.clone()).content("Outside clicks are disabled; use the close action or Escape.")
                        }),
                ))
                .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
