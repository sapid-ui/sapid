use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill((
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content("A useful review makes "),
        sapid::mark()
            .theme(theme.clone())
            .content("important details"),
        sapid::text()
            .theme(theme.clone())
            .inline(true)
            .content(" easier to find."),
    ))
}
