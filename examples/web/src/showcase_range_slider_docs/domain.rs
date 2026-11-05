use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let domain_theme = theme.clone();
    sapid::range_slider("showcase-range-domain")
        .theme(domain_theme)
        .domain([0., 100.])
        .min(10.)
        .max(90.)
        .default_value([25., 75.])
        .marks([
            sapid::SliderMark::new(10., "min"),
            sapid::SliderMark::new(90., "max"),
        ])
        .labels(["Domain minimum".into(), "Domain maximum".into()])
}
