use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let swatches_theme = theme.clone();
    sapid::color_input("showcase-color-with-swatches")
        .theme(swatches_theme)
        .swatches(["#fa5252", "#e64980", "#7950f2", "#228be6", "#15aabf"])
        .swatches_per_row(5)
}
