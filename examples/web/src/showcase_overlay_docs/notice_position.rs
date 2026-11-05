use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [(24., 90.), (120., 240.)]
            .into_iter()
            .enumerate()
            .map(|(i, (x, y))| {
                sapid::floating_dialog(format!("notice-position-{i}"))
                    .theme(theme.clone())
                    .label(format!("Show notice position {}", i + 1))
                    .title("Background task completed")
                    .description("The page remains available while this notice is shown.")
                    .position(glory::render::ViewportPlacement {
                        x,
                        y,
                        ..Default::default()
                    })
                    .style(Style::new().width(Length::Px(280.)))
            })
            .collect::<Vec<_>>(),
    )
}
