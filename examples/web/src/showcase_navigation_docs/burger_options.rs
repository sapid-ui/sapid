use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(16.).fill((
        sapid::burger()
            .theme(theme.clone())
            .size(sapid::Size::Small),
        sapid::burger().theme(theme.clone()).default_open(true),
        sapid::burger()
            .theme(theme.clone())
            .size(sapid::Size::Large),
        sapid::burger().theme(theme).enabled(false),
    ))
}
