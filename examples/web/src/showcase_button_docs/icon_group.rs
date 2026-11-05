use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(4.).fill((
        sapid::action_icon("Accept invitation")
            .theme(theme.clone())
            .icon(
                glory::ui::graphic()
                    .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
            ),
        sapid::close_button()
            .theme(theme)
            .name("Dismiss invitation"),
    ))
}
