use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [260., 420.]
            .into_iter()
            .map(|width| {
                sapid::drawer(format!("drawer-width-{width:.0}"))
                    .theme(theme.clone())
                    .label(format!("Open {width:.0}px drawer"))
                    .title("Workspace details and notification preferences")
                    .close_label("Close workspace details")
                    .style(Style::new().width(Length::Px(width)))
                    .description("The panel remains constrained by the host viewport.")
            })
            .collect::<Vec<_>>(),
    )
}
