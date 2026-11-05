use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [240., 360., 520.]
            .into_iter()
            .map(|width| {
                sapid::dialog(format!("panel-{width:.0}"))
                    .theme(theme.clone())
                    .label(format!("Open {width:.0}px modal"))
                    .title("A longer workspace title that wraps beside the close icon")
                    .style(Style::new().width(Length::Px(width)))
                    .description("The title and close control retain their own layout space.")
            })
            .collect::<Vec<_>>(),
    )
}
