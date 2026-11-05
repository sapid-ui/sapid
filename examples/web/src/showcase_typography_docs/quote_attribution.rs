use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::blockquote()
        .theme(theme.clone())
        .fill(sapid::text().theme(theme.clone()).content(TEXT_1))
        .citation(
            sapid::group().theme(theme.clone()).gap(6.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Small)
                    .content("Mountain studio ·"),
                sapid::anchor()
                    .theme(theme.clone())
                    .label("Read the article example")
                    .href("?widget=typography"),
            )),
        )
}

const TEXT_1: &str = "Small, well understood patterns make a large system easier to use.";
