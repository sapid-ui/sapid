use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        glory::ui::view().fill((
            sapid::text()
                .theme(theme.clone())
                .inline(true)
                .content("One sentence with "),
            sapid::text()
                .theme(theme.clone())
                .inline(true)
                .weight(500.)
                .color(theme.accent_text)
                .content("a clear emphasis"),
            sapid::text()
                .theme(theme.clone())
                .inline(true)
                .content(" and a quiet ending."),
        )),
        sapid::group()
            .theme(theme.clone())
            .justify(Justify::Between)
            .fill((
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Caption)
                    .content("Last updated"),
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Caption)
                    .content("A moment ago"),
            )),
    ))
}
