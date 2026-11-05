use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .gap(20.)
        .align(Align::Start)
        .fill((
            sapid::angle_slider("angle-ltr")
                .theme(theme.clone())
                .name("Left-to-right rotation")
                .default_value(45.),
            sapid::angle_slider("angle-rtl")
                .theme(theme.clone())
                .name("Right-to-left rotation")
                .direction(glory::render::Direction::Rtl)
                .default_value(45.),
            sapid::angle_slider("angle-readonly")
                .theme(theme.clone())
                .name("Read-only rotation")
                .readonly(true)
                .default_value(180.),
            sapid::angle_slider("angle-disabled")
                .theme(theme)
                .name("Disabled rotation")
                .enabled(false)
                .default_value(270.),
        ))
}
