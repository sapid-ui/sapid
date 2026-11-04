use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::timeline("custom-release-markers")
        .theme(theme)
        .active("review")
        .bullet_size(28.)
        .line_width(3.)
        .item(
            sapid::timeline_item("design")
                .title("Design agreed")
                .marker(text().content("✓")),
        )
        .item(
            sapid::timeline_item("review")
                .title("Review in progress")
                .marker(text().content("↗")),
        )
        .item(sapid::timeline_item("publish").title("Publish the release"))
}
