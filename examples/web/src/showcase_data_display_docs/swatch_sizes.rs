use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(14.).fill(
        [false, true]
            .into_iter()
            .flat_map(|round| [20., 32., 48.].into_iter().map(move |side| (round, side)))
            .map(|(round, side)| {
                sapid::color_swatch(Color(158, 70, 180, 175))
                    .theme(theme.clone())
                    .side(side)
                    .round(round)
                    .name(format!("Purple {side}px, round {round}"))
            })
            .collect::<Vec<_>>(),
    )
}
