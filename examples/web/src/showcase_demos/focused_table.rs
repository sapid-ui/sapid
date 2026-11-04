use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Table", theme)
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

    const ELEMENTS: &[(&str, &str, &str, &str)] = &[
        ("6", "Carbon", "C", "12.011"),
        ("7", "Nitrogen", "N", "14.007"),
        ("39", "Yttrium", "Y", "88.906"),
        ("56", "Barium", "Ba", "137.335"),
        ("58", "Cerium", "Ce", "140.12"),
    ];

    fn element_table(theme: &Theme, key: &str) -> Table {
        table(key)
            .theme(theme.clone())
            .caption("Some elements from the periodic table")
            .head(table_head().row(table_row("head").cells([
                table_header_cell("position").text("Element position"),
                table_header_cell("name").text("Element name"),
                table_header_cell("symbol").text("Symbol"),
                table_header_cell("mass").text("Atomic mass"),
            ])))
            .body(
                table_body().rows(ELEMENTS.iter().map(|(position, name, symbol, mass)| {
                    table_row(*name).cells([
                        table_cell("position").text(*position),
                        table_row_header("name").text(*name),
                        table_cell("symbol").text(*symbol),
                        table_cell("mass").text(*mass),
                    ])
                })),
            )
    }

    fn table_demo(theme: Theme) -> impl Widget {
        let caption = element_table(&theme, "elements-caption")
            .caption_side(TableCaptionSide::Bottom)
            .foot(table_foot().row(table_row("foot").cells([
                table_row_header("position").text("Element position"),
                table_row_header("name").text("Element name"),
                table_row_header("symbol").text("Symbol"),
                table_row_header("mass").text("Atomic mass"),
            ])));
        let styled = element_table(&theme, "elements-styled")
            .striped(true)
            .highlight_on_hover(true)
            .with_table_border(true)
            .with_column_borders(true)
            .with_row_borders(true);
        let spaced = element_table(&theme, "elements-spaced")
            .horizontal_spacing(24.0)
            .vertical_spacing(3.0);
        let vertical = table("vertical-details")
            .theme(theme.clone())
            .caption("Vertical variant")
            .with_row_borders(true)
            .body(
                table_body()
                    .row(table_row("name").cells([
                        table_row_header("label").text("Epic name"),
                        table_cell("value").text("7.x migration"),
                    ]))
                    .row(table_row("status").cells([
                        table_row_header("label").text("Status"),
                        table_cell("value").text("Open"),
                    ]))
                    .row(table_row("issues").cells([
                        table_row_header("label").text("Total issues"),
                        table_cell("value").text("135"),
                    ]))
                    .row(table_row("points").cells([
                        table_row_header("label").text("Total story points"),
                        table_cell("value").text("874"),
                    ])),
            );
        let scroll_theme = theme.clone();
        let sticky = scroll_area("elements-scroll")
            .theme(theme.clone())
            .name("Scrollable periodic table")
            .style(
                Style::new()
                    .height(Length::Px(240.0))
                    .max_width(Limit::Percent(100.0)),
            )
            .content(move |_| {
                element_table(&scroll_theme, "elements-sticky")
                    .class("site-sticky-table")
                    .sticky_header(true)
                    .sticky_header_offset(Length::Px(0.0))
            });
        sapid::stack().theme(theme.clone()).gap(24.0).fill((
            sapid::title(3)
                .theme(theme.clone())
                .content("Caption, tfoot and sticky header"),
            caption,
            sapid::title(3)
                .theme(theme.clone())
                .content("Scroll container"),
            sticky,
            sapid::title(3).theme(theme.clone()).content("Spacing"),
            spaced,
            sapid::title(3)
                .theme(theme.clone())
                .content("Striped, hover and borders"),
            styled,
            sapid::title(3)
                .theme(theme.clone())
                .content("Vertical variant"),
            vertical,
            widget(move |scope| {
                let selected = scope.cage(Vec::<String>::new());
                let rows = ELEMENTS
                    .iter()
                    .map(|(position, name, symbol, mass)| {
                        let key = (*name).to_owned();
                        let checked = selected.map({
                            let key = key.clone();
                            move |values| values.contains(&key)
                        });
                        table_row(*name).cells([
                            table_cell("select").content({
                                let key = key.clone();
                                let theme = theme.clone();
                                move || {
                                    sapid::checkbox()
                                        .theme(theme.clone())
                                        .label(format!("Select {name}"))
                                        .checked(checked)
                                        .on_checked_change({
                                            let key = key.clone();
                                            move |next| {
                                                selected.revise(|values| {
                                                    if next && !values.contains(&key) {
                                                        values.push(key.clone());
                                                    } else if !next {
                                                        values.retain(|value| value != &key);
                                                    }
                                                })
                                            }
                                        })
                                }
                            }),
                            table_cell("position").text(*position),
                            table_row_header("name").text(*name),
                            table_cell("symbol").text(*symbol),
                            table_cell("mass").text(*mass),
                        ])
                    })
                    .collect::<Vec<_>>();
                sapid::stack()
                    .theme(theme.clone())
                    .gap(12.0)
                    .fill((
                        sapid::title(3)
                            .theme(theme.clone())
                            .content("Row selection"),
                        table("element-selection")
                            .theme(theme.clone())
                            .caption("Select elements to include in the report")
                            .highlight_on_hover(true)
                            .with_row_borders(true)
                            .head(table_head().row(table_row("head").cells([
                                table_header_cell("select").text("Select"),
                                table_header_cell("position").text("Position"),
                                table_header_cell("name").text("Name"),
                                table_header_cell("symbol").text("Symbol"),
                                table_header_cell("mass").text("Atomic mass"),
                            ])))
                            .body(table_body().rows(rows)),
                        sapid::text()
                            .theme(theme.clone())
                            .content(selected.map(|values| {
                                if values.is_empty() {
                                    "No elements selected".to_owned()
                                } else {
                                    format!("Selected: {}", values.join(", "))
                                }
                            })),
                    ))
                    .show_in(scope)
            }),
        ))
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Table" => Box::new(table_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
