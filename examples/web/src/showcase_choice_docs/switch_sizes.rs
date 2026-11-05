use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(Vec::from(
        [
            (sapid::Size::ExtraSmall, "Extra small"),
            (sapid::Size::Small, "Small"),
            (sapid::Size::Medium, "Medium"),
            (sapid::Size::Large, "Large"),
            (sapid::Size::ExtraLarge, "Extra large"),
        ]
        .map(|(size, label)| {
            sapid::switch()
                .theme(theme.clone())
                .size(size)
                .label(label)
                .with_marks(true)
                .default_checked(true)
        }),
    ))
}
