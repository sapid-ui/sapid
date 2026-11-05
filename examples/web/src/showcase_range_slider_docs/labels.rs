use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let labels_theme = theme.clone();
    sapid::stack().theme(labels_theme.clone()).gap(22.).fill((
        sapid::range_slider("showcase-range-labels")
            .theme(labels_theme.clone())
            .default_value([20., 60.])
            .value_text([Some("20 °C".into()), Some("60 °C".into())])
            .value_labels_always_on(true)
            .labels(["Temperature from".into(), "Temperature to".into()]),
        sapid::range_slider("showcase-range-no-labels")
            .theme(labels_theme)
            .default_value([30., 70.])
            .show_value_labels(false)
            .labels(["Quiet lower".into(), "Quiet upper".into()]),
        sapid::range_slider("showcase-range-scaled")
            .theme(theme.clone())
            .default_value([20., 60.])
            .scale(10.)
            .value_labels_always_on(true)
            .labels(["Scaled lower".into(), "Scaled upper".into()]),
    ))
}
