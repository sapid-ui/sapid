use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let collision_theme = theme.clone();
    sapid::stack()
        .theme(collision_theme.clone())
        .gap(20.)
        .fill((
            sapid::range_slider("showcase-range-push")
                .theme(collision_theme.clone())
                .default_value([25., 65.])
                .minimum_gap(20.)
                .maximum_gap(50.)
                .labels(["Push lower".into(), "Push upper".into()]),
            sapid::range_slider("showcase-range-hold")
                .theme(collision_theme)
                .default_value([25., 65.])
                .minimum_gap(20.)
                .push_on_overlap(false)
                .labels(["Hold lower".into(), "Hold upper".into()]),
            sapid::range_slider("showcase-range-clamp")
                .theme(theme.clone())
                .default_value([25., 65.])
                .minimum_gap(20.)
                .collision(sapid::RangeCollision::Clamp)
                .labels(["Clamp lower".into(), "Clamp upper".into()]),
        ))
}
