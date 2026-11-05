use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill((
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content("Read a reactive value with "),
        sapid::code().theme(theme.clone()).content("count.get()"),
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content(" and update it with an explicit revision."),
    ))
}
