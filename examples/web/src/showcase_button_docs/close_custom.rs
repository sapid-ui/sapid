use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill((
        sapid::close_button()
            .theme(theme.clone())
            .name("Close preview")
            .variant(sapid::ActionIconVariant::Outline),
        sapid::close_button()
            .theme(theme.clone())
            .name("Confirm and close")
            .icon(
                glory::ui::graphic()
                    .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
            ),
    ))
}
