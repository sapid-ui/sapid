use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let free_theme = theme.clone();
    sapid::color_input("showcase-color-disable-free-input")
        .theme(free_theme)
        .disallow_input(true)
}
