use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .gap(12.)
        .fill(Vec::from(["Save draft", "Preview", "Publish"].map(
            |label| sapid::button().theme(theme.clone()).label(label),
        )))
}
