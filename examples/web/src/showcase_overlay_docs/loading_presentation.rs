use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(
        [
            sapid::LoaderType::Oval,
            sapid::LoaderType::Bars,
            sapid::LoaderType::Dots,
        ]
        .into_iter()
        .map(|kind| {
            sapid::loading_overlay(true)
                .theme(theme.clone())
                .label(format!("Loading {kind:?} region"))
                .loader_type(kind)
                .loader_size(sapid::Size::Large)
                .loader_color(theme.warning)
                .opacity(0.75)
                .animated(false)
                .fill(
                    sapid::card().theme(theme.clone()).fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content(format!("A region using the {kind:?} indicator")),
                    ),
                )
        })
        .collect::<Vec<_>>(),
    )
}
