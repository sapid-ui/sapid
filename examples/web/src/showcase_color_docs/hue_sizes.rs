use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(vec![
        sapid::hue_slider("hue-small")
            .theme(theme.clone())
            .name("Small hue control")
            .size(sapid::Size::Small)
            .default_value(60.),
        sapid::hue_slider("hue-large")
            .theme(theme.clone())
            .name("Large hue control")
            .size(sapid::Size::Large)
            .default_value(240.),
        sapid::hue_slider("hue-disabled")
            .theme(theme)
            .name("Disabled hue control")
            .default_value(120.)
            .enabled(false),
    ])
}
