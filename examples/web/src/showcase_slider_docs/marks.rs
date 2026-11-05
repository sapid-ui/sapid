use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let marks_theme = theme.clone();
    sapid::stack().theme(marks_theme.clone()).gap(28.).fill((
        sapid::slider("showcase-slider-marks")
            .theme(marks_theme.clone())
            .default_value(40.)
            .marks([
                sapid::SliderMark::new(0., "0"),
                sapid::SliderMark::new(20., "20"),
                sapid::SliderMark::new(40., "40"),
                sapid::SliderMark::new(70., "70"),
                sapid::SliderMark::new(100., "100"),
            ]),
        sapid::slider("showcase-slider-restricted")
            .theme(marks_theme)
            .default_value(40.)
            .marks([
                sapid::SliderMark::new(0., "Small"),
                sapid::SliderMark::new(40., "Medium"),
                sapid::SliderMark::new(100., "Large"),
            ])
            .restrict_to_marks(true)
            .name("Restricted to marks"),
    ))
}
