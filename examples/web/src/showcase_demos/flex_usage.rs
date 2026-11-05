use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::flex()
        .theme(theme.clone())
        .wrap(glory::style::FlexWrap::Wrap)
        .gap(12.)
        .fill(Vec::from(["Draft", "In review", "Published"].map(
            |label| sapid::badge().theme(theme.clone()).content(label),
        )))
}
