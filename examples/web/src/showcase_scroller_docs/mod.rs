use super::showcase_examples::DetailSection;
mod controls;
mod programmatic;
mod vertical;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "scroller-controls",
            title: "Scroll distance, control size and icons",
            note: "scroll_step uses a fixed 1–100000px distance; the default pages by the actual viewport. control_size accepts 20–64px. previous_icon and next_icon replace passive content while keeping the full accessible action names. Controls use actual measured start/end boundaries. Give horizontal items a non-shrinking width; a Scroller does not resize or virtualize application items.",
            example: Box::new(controls::demo(theme.clone())),
            source: include_str!("controls.rs"),
        },
        DetailSection {
            id: "scroller-vertical",
            title: "Vertical content and native scrolling",
            note: "axis changes both the viewport and controls. area configures the underlying ScrollArea, including its height, scrollbar presentation and keyboard policy. The viewport owns Arrow, Home/End and Page keys only when it has actual focus. Wheel, trackpad and touch remain native. Mouse drag-to-scroll is not implemented; selecting text and editing descendants retain their ordinary behavior.",
            example: Box::new(vertical::demo(theme.clone())),
            source: include_str!("vertical.rs"),
        },
        DetailSection {
            id: "scroller-programmatic",
            title: "Programmatic scrolling without controls",
            note: "controls(false) keeps the real viewport and ScrollHandle. on_ready runs after construction; scroll_to_fraction resolves against current measured extents. Requests are fallible and Owner-scoped. Scrolling does not replace descendants: type a draft, move to the end and back, and inspect it again. Explicit programmatic destinations use the documented nonnegative LTR scroll-coordinate contract.",
            example: Box::new(programmatic::demo(theme)),
            source: include_str!("programmatic.rs"),
        },
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Controls", "#scroller-controls"),
        ("Vertical scrolling", "#scroller-vertical"),
        ("ScrollHandle", "#scroller-programmatic"),
    ]
}
