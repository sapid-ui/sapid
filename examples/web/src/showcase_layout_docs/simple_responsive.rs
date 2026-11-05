use glory::prelude::*;
use glory::style::{GridPlacement, GridTrack, GridTracks};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::responsive_root(640.).content(move |context| {
        widget({
            let theme = theme.clone();
            move |scope| {
                let style = |count, gap| {
                    Style::new()
                        .grid_columns(GridTracks::Tracks(vec![GridTrack::Fr(1.); count]))
                        .gap(gap)
                };
                let columns = context.styles(sapid::ResponsiveValues::new(
                    style(1, 8.),
                    style(2, 12.),
                    style(3, 16.),
                    style(3, 20.),
                    style(3, 24.),
                ))?;
                sapid::simple_grid("docs-responsive-simple")
                    .theme(theme.clone())
                    .columns(3)
                    .style(columns)
                    .class("docs-responsive-simple")
                    .items((1..=4).map(|i| {
                        sapid::grid_item(i.to_string())
                            .style(
                                Style::new()
                                    .grid_column(GridPlacement::Auto)
                                    .grid_row(GridPlacement::Auto),
                            )
                            .fill(
                                sapid::card().theme(theme.clone()).fill(
                                    sapid::text_field()
                                        .theme(theme.clone())
                                        .name(format!("Simple grid note {i}"))
                                        .placeholder("Write a note…")
                                        .style(Style::new().width(Length::Percent(100.))),
                                ),
                            )
                    }))
                    .show_in(scope)
            }
        })
    })
}
