use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::container().theme(theme.clone()).max_width(Limit::Px(420.))
        .fill(sapid::card().theme(theme.clone()).fill(sapid::text().theme(theme)
            .content("A centered reading area with a 420px maximum content width. The container keeps its side padding when the page narrows.")))
}
