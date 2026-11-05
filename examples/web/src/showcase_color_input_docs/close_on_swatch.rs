use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let close_theme = theme.clone();
    sapid::color_input("showcase-color-close-swatch")
        .theme(close_theme)
        .swatches(["#fa5252", "#12b886", "#228be6"])
        .close_on_color_swatch_click(true)
}
