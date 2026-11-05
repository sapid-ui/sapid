use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            sapid::Size::ExtraSmall,
            sapid::Size::Small,
            sapid::Size::Medium,
            sapid::Size::Large,
            sapid::Size::ExtraLarge,
        ]
        .into_iter()
        .enumerate()
        .map(|(i, size)| {
            sapid::action_icon(format!("Settings size {}", i + 1))
                .theme(theme.clone())
                .size(size)
                .icon(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                        .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                )
        })
        .collect::<Vec<_>>(),
    )
}
