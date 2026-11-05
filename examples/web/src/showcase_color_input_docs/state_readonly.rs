use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-color-state-readonly")
        .theme(theme.clone())
        .label("Read only")
        .readonly(true)
        .control(|field| sapid::color_input("showcase-color-state-readonly").field(field))
}
