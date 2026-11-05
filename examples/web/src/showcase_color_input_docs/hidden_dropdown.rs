use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let hidden_theme = theme.clone();
    sapid::color_input("showcase-color-hide-dropdown")
        .theme(hidden_theme)
        .show_picker(false)
}
