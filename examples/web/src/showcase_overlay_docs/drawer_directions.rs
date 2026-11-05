use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            sapid::DrawerSide::Left,
            sapid::DrawerSide::Right,
            sapid::DrawerSide::Top,
            sapid::DrawerSide::Bottom,
        ]
        .into_iter()
        .map(|side| {
            let style = if matches!(side, sapid::DrawerSide::Top | sapid::DrawerSide::Bottom) {
                Style::new()
                    .width(Length::Percent(100.))
                    .height(Length::Px(240.))
            } else {
                Style::new().width(Length::Px(300.))
            };
            sapid::drawer(format!("edge-{side:?}"))
                .theme(theme.clone())
                .label(format!("Open {side:?} drawer"))
                .title(format!("{side:?} workspace drawer"))
                .side(side)
                .style(style)
                .description("Close or press Escape to return to the invoking control.")
        })
        .collect::<Vec<_>>(),
    )
}
