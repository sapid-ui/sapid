use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let bounds_theme = theme.clone();
    sapid::group()
        .theme(bounds_theme.clone())
        .gap(18.)
        .wrap(glory::style::FlexWrap::Wrap)
        .fill((
            sapid::slider("showcase-slider-step")
                .theme(bounds_theme.clone())
                .min(-20.)
                .max(20.)
                .step(5.)
                .default_value(5.)
                .name("Minus twenty to twenty"),
            sapid::slider("showcase-slider-decimal")
                .theme(bounds_theme.clone())
                .min(0.)
                .max(1.)
                .step(0.05)
                .default_value(0.35)
                .name("Decimal value"),
        ))
}
