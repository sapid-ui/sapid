use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-color-state-disabled")
        .theme(theme.clone())
        .label("Disabled")
        .enabled(false)
        .control(|field| sapid::color_input("showcase-color-state-disabled").field(field))
}
