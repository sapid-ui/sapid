use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        (1..=6)
            .map(|level| {
                sapid::title(level)
                    .theme(theme.clone())
                    .weight(400.)
                    .content(format!("Heading level {level}"))
            })
            .collect::<Vec<_>>(),
    )
}
