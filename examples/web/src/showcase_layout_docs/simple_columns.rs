use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .gap(16.)
        .fill(Vec::from([2u16, 3].map(|columns| {
            sapid::simple_grid(format!("docs-simple-{columns}"))
                .theme(theme.clone())
                .columns(columns)
                .gap(if columns == 2 { 8. } else { 20. })
                .class(format!("docs-simple-columns-{columns}"))
                .items(
                    [
                        "Short",
                        "A longer project summary wraps inside the same column width",
                        "Final item",
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(i, label)| {
                        sapid::grid_item(i.to_string()).fill(
                            sapid::card()
                                .theme(theme.clone())
                                .fill(sapid::text().theme(theme.clone()).content(label)),
                        )
                    }),
                )
        })))
}
