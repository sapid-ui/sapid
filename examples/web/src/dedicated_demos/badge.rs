use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(10.).fill((
        sapid::badge()
            .theme(theme.clone())
            .variant(sapid::BadgeVariant::Dot)
            .color(Color(34, 139, 90, 255))
            .content("Live"),
        sapid::badge()
            .theme(theme.clone())
            .left_section(sapid::Icon::Check)
            .content("Verified"),
        sapid::badge()
            .theme(theme.clone())
            .variant(sapid::BadgeVariant::Outline)
            .right_section("8")
            .content("New comments"),
    ))
}
