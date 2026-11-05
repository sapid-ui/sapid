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
            sapid::tooltip(format!("tip-{side:?}"))
                .theme(theme.clone())
                .label(format!("{side:?} help"))
                .text("A short helpful description.")
                .placement(glory::render::LayerPlacement::Anchor(
                    sapid::AnchorPlacement {
                        side,
                        align: sapid::AnchorAlign::Center,
                        arrow: Some(Default::default()),
                        ..Default::default()
                    },
                ))
        })
        .collect::<Vec<_>>(),
    )
}
