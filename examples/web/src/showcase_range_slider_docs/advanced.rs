use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let advanced_theme = theme.clone();
    sapid::stack().theme(advanced_theme.clone()).gap(20.).fill((
        sapid::range_slider("showcase-range-maximum")
            .theme(advanced_theme.clone())
            .default_value([30., 70.])
            .minimum_gap(10.)
            .maximum_gap(40.)
            .labels(["Maximum range lower".into(), "Maximum range upper".into()]),
        sapid::range_slider("showcase-range-inverted-hidden")
            .theme(advanced_theme)
            .default_value([25., 75.])
            .inverted(true)
            .marks([
                sapid::SliderMark::new(0., "0%"),
                sapid::SliderMark::unlabeled(25.).hidden(true),
                sapid::SliderMark::new(50., "50%"),
                sapid::SliderMark::unlabeled(75.).hidden(true),
                sapid::SliderMark::new(100., "100%"),
            ])
            .restrict_to_marks(true)
            .labels(["Hidden mark lower".into(), "Hidden mark upper".into()]),
    ))
}
