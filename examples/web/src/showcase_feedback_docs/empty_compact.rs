use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::empty_state()
        .theme(theme.clone())
        .size(sapid::Size::Small)
        .heading_level(3)
        .style(
            Style::new()
                .padding(Edges::all(12.))
                .width(Length::Percent(100.)),
        )
        .title("No activity")
        .description("Activity appears here when your team starts working.")
        .illustration(
            glory::ui::graphic()
                .drawing(sapid::Icon::Info.drawing(theme.muted))
                .style(Style::new().width(Length::Px(24.)).height(Length::Px(24.))),
        )
}
