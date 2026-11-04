use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Scroller", theme)
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

    fn tile(theme: &Theme, label: &str) -> sapid::Box {
        sapid::box_()
            .theme(theme.clone())
            .style(Style {
                background: Some(theme.accent_soft),
                color: Some(theme.accent_text),
                ..Style::new().padding(Edges::all(16.0)).radius(theme.radius)
            })
            .fill(text().content(label.to_owned()))
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "ScrollArea" | "Scroller" => {
                let horizontal = name == "Scroller";
                Box::new(widget(move |scope| {
                    let content_theme = theme.clone();
                    let content = move |_| {
                        let count = if horizontal { 16 } else { 24 };
                        let items = (1..=count)
                            .map(|index| tile(&content_theme, &format!("Item {index}")))
                            .collect::<Vec<_>>();
                        if horizontal {
                            Box::new(
                                group()
                                    .theme(content_theme.clone())
                                    .wrap(FlexWrap::NoWrap)
                                    .fill(items),
                            ) as Box<dyn Widget>
                        } else {
                            Box::new(sapid::stack().theme(content_theme.clone()).fill(items))
                        }
                    };
                    let area: Box<dyn Widget> = if horizontal {
                        Box::new(
                            scroller("showcase-scroller")
                                .area(|area| area.theme(theme.clone()))
                                .content(content),
                        )
                    } else {
                        Box::new(
                            scroll_area("showcase-scroll-area")
                                .theme(theme.clone())
                                .name("Scrollable content")
                                .content(content),
                        )
                    };
                    view()
                        .style(
                            Style::new()
                                .height(Length::Px(220.0))
                                .max_width(Limit::Px(560.0)),
                        )
                        .fill(area)
                        .show_in(scope)
                }))
            }
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
