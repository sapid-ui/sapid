use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill((
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content("Create state with "),
        sapid::code().theme(theme.clone()).content("scope.cage(0)"),
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content(" and read it with "),
        sapid::code().theme(theme.clone()).content("count.get()"),
        sapid::text().theme(theme.clone()).inline(true).content("."),
    ))
}
