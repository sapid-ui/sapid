use super::showcase_examples::DetailSection;
mod autoplay;
mod controlled;
mod geometry;
mod retention;
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Controlled pages", "#carousel-controlled"),
        ("Controls and direction", "#carousel-geometry"),
        ("Autoplay policy", "#carousel-autoplay"),
        ("Windowed content", "#carousel-retention"),
    ]
}
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "carousel-controlled",
            title: "Controlled pages and refusal",
            note: "current is a read-only input; on_current_change proposes a zero-based index. Disable acceptance to compare refused arrows, indicators and keyboard commands. Root arrows/Home/End and pointer or touch swipe share the same controller.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "carousel-geometry",
            title: "Control size, direction and visibility",
            note: "with_controls(false) omits arrow buttons while keeping indicators, root keyboard navigation and swipe. control_size accepts finite 20–64px values. RTL reverses horizontal control placement; vertical changes the capture axis and uses up/down icons. Slides remain one page at a time, without arbitrary Embla plugins or fractional multi-slide sizing.",
            example: Box::new(geometry::demo(theme.clone())),
            source: include_str!("geometry.rs"),
        },
        DetailSection {
            id: "carousel-autoplay",
            title: "Explicit autoplay and pause policies",
            note: "Autoplay is opt-in. Enable this example to supply one cancellable two-second browser timer per tick. Focus, mouse/pen hover, the pause button and the external paused source stop the current wait. External pause disables local resume. SSR never constructs the timer; native applications supply their own local timer future.",
            example: Box::new(autoplay::demo(theme.clone())),
            source: include_str!("autoplay.rs"),
        },
        DetailSection {
            id: "carousel-retention",
            title: "Windowed slides and editor lifetime",
            note: "count supports up to 1,000,000 logical pages, overscan 0–3, with at most seven slide Owners. Indicators independently retain at most nine nearby positions. Enter a draft and move to a neighbor to inspect identity retention; jump out of the retained window to retire it. No visited-slide cache is kept.",
            example: Box::new(retention::demo(theme)),
            source: include_str!("retention.rs"),
        },
    ]
}
