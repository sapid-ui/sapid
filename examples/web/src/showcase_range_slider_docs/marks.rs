use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let marks_theme = theme.clone();
    sapid::range_slider("showcase-range-marks")
        .theme(marks_theme)
        .default_value([20., 80.])
        .minimum_gap(20.)
        .marks([
            sapid::SliderMark::new(0., "0"),
            sapid::SliderMark::new(20., "20"),
            sapid::SliderMark::new(50., "50"),
            sapid::SliderMark::new(80., "80"),
            sapid::SliderMark::new(100., "100"),
        ])
        .restrict_to_marks(true)
        .labels(["Restricted minimum".into(), "Restricted maximum".into()])
}
