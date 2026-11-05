use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            sapid::AnchorSide::Top,
            sapid::AnchorSide::Right,
            sapid::AnchorSide::Bottom,
            sapid::AnchorSide::Left,
        ]
        .into_iter()
        .map(|side| {
            sapid::popover(format!("side-{side:?}"))
                .theme(theme.clone())
                .label(format!("{side:?} popover"))
                .title(format!("{side:?} placement"))
                .placement(sapid::AnchorPlacement::new(
                    side,
                    sapid::AnchorAlign::Center,
                ))
                .offset(10.)
                .with_arrow(true)
                .width(sapid::PopoverWidth::Px(240.))
                .description("The host may flip this requested side at the viewport boundary.")
        })
        .collect::<Vec<_>>(),
    )
}
