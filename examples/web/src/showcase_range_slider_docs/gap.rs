use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let gap_theme = theme.clone();
    sapid::stack().theme(gap_theme.clone()).gap(18.).fill((
        sapid::range_slider("showcase-range-gap")
            .theme(gap_theme.clone())
            .default_value([30., 70.])
            .minimum_gap(20.)
            .labels(["Minimum with gap".into(), "Maximum with gap".into()]),
        sapid::range_slider("showcase-range-decimal")
            .theme(gap_theme)
            .min(0.)
            .max(1.)
            .step(0.05)
            .default_value([0.25, 0.8])
            .labels(["Decimal minimum".into(), "Decimal maximum".into()]),
    ))
}
