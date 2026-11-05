use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::blockquote()
        .theme(theme.clone())
        .fill(
            sapid::text()
                .theme(theme.clone())
                .content("Good design gives people room to focus on what matters."),
        )
        .citation(
            sapid::text()
                .theme(theme.clone())
                .variant(sapid::TextVariant::Small)
                .content("— Ava Patel, Mountain studio design notes"),
        )
}
