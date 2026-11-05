use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::title(2)
        .theme(theme.clone())
        .weight(400.)
        .content("Build with a clear sense of direction")
}
