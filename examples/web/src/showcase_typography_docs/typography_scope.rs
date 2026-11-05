use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::typography()
        .theme(theme.clone())
        .size(sapid::Size::Large)
        .content(|cx| {
            (
                cx.title(3).weight(400.).content("A shared content scope"),
                cx.text()
                    .content("These builders use the same explicit theme and size."),
                cx.anchor()
                    .label("Return to the article example")
                    .href("#typography-article"),
                sapid::code()
                    .theme(cx.theme.clone())
                    .size(cx.size)
                    .content("context.text().content(message)"),
            )
        })
}
