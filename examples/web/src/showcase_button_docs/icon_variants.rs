use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            sapid::ActionIconVariant::Default,
            sapid::ActionIconVariant::Filled,
            sapid::ActionIconVariant::Light,
            sapid::ActionIconVariant::Outline,
            sapid::ActionIconVariant::Subtle,
        ]
        .into_iter()
        .enumerate()
        .map(|(i, variant)| {
            sapid::action_icon(format!("Confirm variant {}", i + 1))
                .theme(theme.clone())
                .variant(variant)
                .icon(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::Check.drawing(if i == 1 {
                            theme.on_accent
                        } else {
                            theme.accent_text
                        }))
                        .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                )
        })
        .collect::<Vec<_>>(),
    )
}
