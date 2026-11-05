use super::showcase_examples::DetailSection;
mod affix_offsets;
mod affix_sticky;
mod affix_viewport;
mod background_fallback;
mod background_fit;
mod time_bounds;
mod time_intervals;
mod time_states;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident, $id:literal, $title:literal, $note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    match slug {
        "affix" => vec![
            section!(
                affix_viewport,
                "affix-viewport",
                "Viewport positioning and visibility",
                "affix defaults to the viewport with 16px right/bottom offsets. Open this example deliberately, then scroll the page: the action stays at the viewport corner. visible hides the same subtree without retiring its editor or Owner. No global scroll or breakpoint service is installed. The usage preview above uses an explicit AffixContext to keep its action local."
            ),
            section!(
                affix_offsets,
                "affix-offsets",
                "Reactive offsets within a container",
                "AffixOffset selects one horizontal and one vertical side with finite pixel offsets. offset accepts Cage/Bond/Vary; this example moves the same action between corners. within requires a live construction ancestor and uses absolute positioning. Container dimensions remain responsive; content width never forces the page wider."
            ),
            section!(
                affix_sticky,
                "affix-sticky",
                "Sticky content and platform support",
                "sticky keeps a normal flow slot and follows the scroll container's CSS sticky constraints. Scroll inside this example. It differs from an action fixed to the viewport. Browser/SSR declarations are supported; native Sticky is explicitly rejected until the native engine implements it. Ordinary within and viewport positioning remain supported by native layout."
            ),
        ],
        "background-image" => vec![
            section!(
                background_fit,
                "background-fit",
                "Cover, contain and responsive dimensions",
                "BackgroundImage mounts a decorative real image behind ordinary Widget content. fit selects Cover, Contain, Fill, None or ScaleDown; the default is Cover. Image content is centered. Width, height and explicit style are independent; the default max width prevents a fixed-width image from widening a narrow container. Foreground contrast is application-owned: this example uses a dark translucent layer and white text. Object-position customization is not yet supported."
            ),
            section!(
                background_fallback,
                "background-fallback",
                "Reactive sources and fallback",
                "The source accepts Cage/Bond/Vary and updates the same image request identity. fallback_src is tried once for each primary source revision; on_load/on_error identify completions by request revision. This example toggles a bundled decode failure to demonstrate the fallback, retaining foreground content and its action. Decorative image metadata does not supply the region's accessible name. Native hosts require an explicit resource provider."
            ),
        ],
        "time-grid" => vec![
            section!(
                time_intervals,
                "time-grid-intervals",
                "Intervals, columns and time format",
                "TimeGridSpec defines inclusive start/end, positive step seconds and 1–12 columns. Admission is arithmetic and bounded to 288 slots before mounting. The grid shrinks within its container; dense column counts use a local horizontal viewport to keep labels legible. TimeFormat selects 12/24-hour cycles and seconds. Roving arrows, Home/End and page commands follow the declared logical grid without rebuilding slots."
            ),
            section!(
                time_bounds,
                "time-grid-bounds",
                "Bounds, disabled slots and controlled selection",
                "TimeBounds are inclusive same-day limits. disabled_times supplies at most 128 explicit unavailable times. Keyboard navigation skips unavailable slots. value accepts Cage/Bond/Vary<Option<LocalTime>>; on_value_change proposes a selection. This example can refuse proposals without mutating its source. Off-step values are displayed as no selected cell; nearest available focus is a separate policy."
            ),
            section!(
                time_states,
                "time-grid-states",
                "Read only, disabled and RTL",
                "readonly preserves roving focus while preventing selection changes; enabled(false) disables all slots. direction controls visual order and horizontal navigation, with logical row/column metadata retained. There is one current Tab stop. These finite time values have no time zone, date, overnight interval or scheduling service."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "affix" => vec![
            ("Viewport positioning", "#affix-viewport"),
            ("Reactive offsets", "#affix-offsets"),
            ("Sticky content", "#affix-sticky"),
        ],
        "background-image" => vec![
            ("Image fit", "#background-fit"),
            ("Sources and fallback", "#background-fallback"),
        ],
        "time-grid" => vec![
            ("Intervals and columns", "#time-grid-intervals"),
            ("Bounds and selection", "#time-grid-bounds"),
            ("States and direction", "#time-grid-states"),
        ],
        _ => Vec::new(),
    }
}
