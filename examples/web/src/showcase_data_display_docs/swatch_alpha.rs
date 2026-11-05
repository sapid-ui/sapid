use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(14.).fill(
        [255, 160, 64, 0]
            .into_iter()
            .map(|alpha| {
                sapid::stack().theme(theme.clone()).gap(6.).fill((
                    sapid::color_swatch(Color(35, 115, 210, alpha))
                        .theme(theme.clone())
                        .side(42.)
                        .name(format!("Blue, alpha {alpha} of 255")),
                    sapid::text()
                        .theme(theme.clone())
                        .content(format!("Alpha {alpha}")),
                ))
            })
            .collect::<Vec<_>>(),
    )
}
