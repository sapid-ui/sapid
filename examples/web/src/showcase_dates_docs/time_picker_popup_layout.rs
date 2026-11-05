use glory::prelude::*;
use sapid::{AnchorAlign, AnchorPlacement, AnchorSide, LocalTime, time_picker};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |hour, minute, second| LocalTime::new(hour, minute, second).unwrap();
    time_picker("showcase-time-picker-popup-layout")
        .theme(theme.clone())
        .name("Top aligned presets")
        .presets(vec![at(8, 0, 0), at(10, 0, 0), at(12, 0, 0)])
        .popup_placement(AnchorPlacement {
            side: AnchorSide::Top,
            align: AnchorAlign::Start,
            gap: 8.,
            viewport_padding: 12.,
            flip: false,
            shift: true,
            ..AnchorPlacement::default()
        })
        .popup_width(320.)
}
