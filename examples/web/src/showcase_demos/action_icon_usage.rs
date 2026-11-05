use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::action_icon("Confirm selection")
        .theme(theme.clone())
        .variant(sapid::ActionIconVariant::Light)
        .icon(
            glory::ui::graphic()
                .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
        )
}
