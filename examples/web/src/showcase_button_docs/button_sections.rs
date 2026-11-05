use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::button()
            .theme(theme.clone())
            .label("Save document")
            .left_section(
                glory::ui::graphic()
                    .drawing(sapid::Icon::Check.drawing(theme.on_accent))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
            )
            .right_section(text().content("Ctrl S").selectable(false)),
        sapid::button()
            .theme(theme.clone())
            .label("Continue with this selected workspace and its settings")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .white_space(glory::style::WhiteSpace::Normal),
            ),
    ))
}
