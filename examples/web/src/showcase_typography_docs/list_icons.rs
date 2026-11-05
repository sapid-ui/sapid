use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::text_list()
        .theme(theme.clone())
        .name("Review results")
        .items(
            [
                "Brief approved",
                "Prototype reviewed",
                "Documentation ready",
            ]
            .into_iter()
            .map(|label| {
                sapid::text_list_item().fill(
                    sapid::group().theme(theme.clone()).gap(8.).fill((
                        sapid::theme_icon(sapid::Icon::Check)
                            .theme(theme.clone())
                            .size(sapid::Size::ExtraSmall),
                        sapid::text()
                            .theme(theme.clone())
                            .inline(true)
                            .content(label),
                    )),
                )
            }),
        )
}
