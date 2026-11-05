use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [
            (sapid::ButtonVariant::Primary, "Filled"),
            (sapid::ButtonVariant::Secondary, "Default"),
            (sapid::ButtonVariant::Light, "Light"),
            (sapid::ButtonVariant::Outline, "Outline"),
            (sapid::ButtonVariant::Subtle, "Subtle"),
            (sapid::ButtonVariant::Danger, "Delete"),
        ]
        .into_iter()
        .map(|(variant, label)| {
            sapid::button()
                .theme(theme.clone())
                .variant(variant)
                .label(label)
        })
        .collect::<Vec<_>>(),
    )
}
