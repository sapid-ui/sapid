use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::title(3)
            .theme(theme.clone())
            .font_size(30.)
            .line_height(39.)
            .weight(400.)
            .content("A large level-three heading"),
        sapid::title(2)
            .theme(theme.clone())
            .font_size(18.)
            .line_height(27.)
            .weight(400.)
            .content("A quiet level-two heading"),
    ))
}
