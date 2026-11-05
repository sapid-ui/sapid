use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [(4., 12.), (12., 24.)]
            .into_iter()
            .map(|(radius, padding)| {
                sapid::paper()
                    .theme(theme.clone())
                    .bordered(true)
                    .style(Style::new().radius(radius).padding(Edges::all(padding)))
                    .fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content(format!("Radius {radius}px · Padding {padding}px")),
                    )
            })
            .collect::<Vec<_>>(),
    )
}
