use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme.clone())
        .label("Download report")
        .href("#report")
        .fill(sapid::group().theme(theme).gap(8.).fill((
            text().content("↓"),
            text().content("Download report"),
            text().content("PDF"),
        )))
}
