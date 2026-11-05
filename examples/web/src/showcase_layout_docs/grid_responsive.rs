use glory::prelude::*;
use glory::style::GridPlacement;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::responsive_root(640.).content(move |context| {
        widget({
            let theme = theme.clone();
            move |scope| {
                let small = context.select(sapid::ResponsiveValues::new(
                    true, false, false, false, false,
                ))?;
                sapid::grid("docs-responsive-grid")
                    .theme(theme.clone())
                    .columns(12)
                    .gap(12.)
                    .class("docs-responsive-grid")
                    .items((0..4).map(|i| {
                        sapid::grid_item(i.to_string())
                            .style(scope.bond(move || {
                                Style::new()
                                    .grid_column(GridPlacement::Span(if small.get() {
                                        12
                                    } else {
                                        6
                                    }))
                                    .grid_row(GridPlacement::Auto)
                            }))
                            .fill(
                                sapid::card().theme(theme.clone()).fill(
                                    sapid::stack().theme(theme.clone()).gap(8.).fill((
                                        sapid::text()
                                            .theme(theme.clone())
                                            .content(format!("Project {}", i + 1)),
                                        sapid::text_field()
                                            .theme(theme.clone())
                                            .name(format!("Grid project {} note", i + 1))
                                            .style(Style::new().width(Length::Percent(100.)))
                                            .placeholder("Keep a note…"),
                                    )),
                                ),
                            )
                    }))
                    .show_in(scope)
            }
        })
    })
}
