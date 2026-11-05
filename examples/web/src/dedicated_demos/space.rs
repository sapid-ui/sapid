use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(0.).fill((
        sapid::text()
            .theme(theme.clone())
            .content("Before a 32px vertical space"),
        sapid::space().theme(theme.clone()).height(Length::Px(32.)),
        sapid::text().theme(theme).content("After the space"),
    ))
}
