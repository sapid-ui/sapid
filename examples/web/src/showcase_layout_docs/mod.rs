use super::showcase_examples::DetailSection;
mod aspect_bounds;
mod aspect_fit;
mod center_axes;
mod container_widths;
mod flex_directions;
mod flex_responsive;
mod grid_responsive;
mod grid_spans;
mod group_growth;
mod group_wrap;
mod simple_columns;
mod simple_responsive;
mod space_axes;
mod splitter_controlled;
mod splitter_fixed;
mod splitter_nested;
mod stack_distribution;

pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident,$id:literal,$title:literal,$note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    match id {
        "aspect-ratio" => vec![
            section!(
                aspect_fit,
                "ratio-fit",
                "Image fitting and cropping",
                "The ratio reserves the frame; the child chooses how to fit. Cover fills the square and crops the wide local illustration. Contain preserves the full scene with empty space around it. Both images have explicit 100% width/height and meaningful alternative text. This works for arbitrary content; an image's intrinsic size does not define the frame."
            ),
            section!(
                aspect_bounds,
                "ratio-bounds",
                "Flex constraints and frame dimensions",
                "Set an explicit width for a non-shrinking thumbnail in a flex row. The ratio controls height when the other dimension is automatic; explicitly providing both width and height takes precedence over a preferred ratio. Long sibling content needs min_width(0). Ratios must be finite in 0.000001..=1000000; invalid reactive proposals keep the last accepted layout."
            ),
        ],
        "center" => vec![section!(
            center_axes,
            "center-axes",
            "Axes, inline content and height",
            "Center aligns content on both axes. Vertical centering is visible only when the container has spare height. A typed align(Start) override keeps horizontal centering and moves content to the top. inline(true) uses an inline flex frame whose width follows its content; it does not center that frame within its parent. Use the surrounding layout to position the frame itself."
        )],
        "container" => vec![section!(
            container_widths,
            "container-widths",
            "Maximum widths, fluid layout and nesting",
            "max_width caps the inner content region while the outer container keeps 16px of default side padding. Compare 240px, 420px and fluid widths as the page narrows. fluid removes the content cap. Nested containers add their padding; explicitly set padding to zero on the inner container when that extra inset is unwanted. A custom outer Style max_width is a separate constraint from the content cap."
        )],
        "flex" => vec![
            section!(
                flex_directions,
                "flex-directions",
                "Direction, wrapping and spacing",
                "Direction, flex_wrap and gap are independent typed styles. Toggle row/column, wrapping and spacing without recreating children. The items deliberately retain their width; the no-wrap example scrolls locally when they no longer fit. Align controls the cross axis and justify controls the main axis; those axes change when direction changes."
            ),
            section!(
                flex_responsive,
                "flex-responsive",
                "Measured responsive layout and alignment",
                "responsive_root uses an explicit SSR width and then the container's queued measurements. Its styles Bond changes this same flex row to a column below 576px. The editor and its draft remain mounted. Rust uses finite typed styles and explicit ResponsiveValues rather than JavaScript prop objects or implicit global viewport queries. Set min_width(0) on shrinkable content; provide a container height when demonstrating vertical justification."
            ),
        ],
        "grid" => vec![
            section!(
                grid_spans,
                "grid-spans",
                "Spans, row spans and column offsets",
                "Grid uses real CSS tracks. column and row are one-based start positions; span and row_span reserve tracks. Starting this final cell at column 3 leaves two columns before it. The long article spans two rows while summary/actions occupy the right. Ordinary Grid validates unique keys, placements and overlap within its finite 64-by-64 budget; it adds no interactive grid role."
            ),
            section!(
                grid_responsive,
                "grid-responsive",
                "Responsive spans with stable content",
                "This example keeps twelve tracks and changes each cell between a six-track and full-row span. Explicit typed grid_column and grid_row styles take responsibility for automatic placement instead of the builder's initial first-fit positions. responsive_root derives the span from the measured container. Resize the page while editing a note: the same cells and editors stay mounted. Custom styles remain subject to each rendering adapter's checked support."
            ),
        ],
        "group" => vec![
            section!(
                group_wrap,
                "group-wrap",
                "Wrapping and local overflow",
                "Group defaults to a wrapping horizontal flex row. This switch changes flex_wrap on the existing group. Fixed action widths use shrink(0); when wrapping is off, the surrounding box supplies local horizontal scrolling. No action is squeezed into a vertical column of letters."
            ),
            section!(
                group_growth,
                "group-growth",
                "Equal growth and long content",
                "Equal growth is explicit on each child: grow(1), a zero width basis and min_width(0) share available width. The second row lets its long text shrink and wrap while the Open action keeps its intrinsic width. Group owns the row, gap and alignment; it does not silently rewrite child sizing or clone children to assign growth. Use a smaller gap or wrapping when a narrow container cannot fit the requested minimums."
            ),
        ],
        "simple-grid" => vec![
            section!(
                simple_columns,
                "simple-columns",
                "Column count, gaps and unequal content",
                "SimpleGrid gives every item one equal fractional track. These two- and three-column examples use different gaps and unequal text lengths so both width and wrapping are visible. Content can make rows taller; the columns remain equal. Use Grid for spanning or explicit row/column placement. Counts must fit 1..=64 and the finite item/placement budget."
            ),
            section!(
                simple_responsive,
                "simple-responsive",
                "Measured columns without remounting",
                "The measured container chooses one, two or three equal columns and matching spacing through typed GridTracks. Each item explicitly opts into automatic row/column placement; otherwise the builder's initial positions would remain fixed. The width comes from responsive_root, not a browser global. Notes retain their editor identity while reflowing. The explicit initial width supplies deterministic SSR and adoption before real measurement."
            ),
        ],
        "space" => vec![section!(
            space_axes,
            "space-axes",
            "Horizontal/vertical spacing and parent gaps",
            "A width adds horizontal space only in a horizontal layout; a height adds vertical space in a column. The fixed horizontal spacer uses shrink(0). Both demonstrations set parent gap to zero so the measured separation comes from Space. Prefer a parent's gap for uniform spacing between all children; Space is useful for one deliberate separation. It is passive layout with no focus or selection behavior."
        )],
        "stack" => vec![section!(
            stack_distribution,
            "stack-distribution",
            "Alignment, distribution and long content",
            "Stack is a column: align controls horizontal child alignment and justify distributes children vertically. Toggle between stretch/start and start/space-between while retaining the same children. A fixed 240px height makes the extra vertical space visible; an auto-height stack would have little space to distribute. gap supplies minimum separation independently. Long content wraps within the available width; its container still owns sizing."
        )],
        "splitter" => vec![
            section!(
                splitter_controlled,
                "splitter-controlled",
                "Controlled sizes, limits and keyboard input",
                "value is a read-only percentage source; on_value_change proposes the first pane's share. Declining proposals retains the current layout. min/max constrain both pointer and keyboard operations; step controls arrow increments, Home/End reach bounds. Reset is an external silent write. on_value_commit reports a finished interaction separately. Drag the handle or focus its named separator and use the keyboard while preserving the note editor."
            ),
            section!(
                splitter_nested,
                "splitter-nested",
                "Nested splitters and multiple panels",
                "Compose two independent two-pane splitters to create a three-panel workspace. The inner separator is vertical and receives an explicit 100% height from its bounded parent. Each handle has its own accessible name, current range and keyboard axis. This is composition of stable panes; it does not require a second multi-panel controller."
            ),
            section!(
                splitter_fixed,
                "splitter-fixed",
                "Fixed navigation and narrow flexible content",
                "A fixed-width, non-shrinking navigation box sits outside the splitter. Its maximum is 35% so it leaves usable space on narrow screens. The splitter grows into the remaining width with min_width(0). Pointer deltas resolve against its actual current extent, not the browser window; percentage min/max still apply. Native drag/keyboard model checks are distinct from OS pointer-device acceptance."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "aspect-ratio" => vec![
            ("Image fitting and cropping", "#ratio-fit"),
            ("Flex constraints", "#ratio-bounds"),
        ],
        "center" => vec![("Axes and inline content", "#center-axes")],
        "container" => vec![("Widths, fluid and nesting", "#container-widths")],
        "flex" => vec![
            ("Direction and wrapping", "#flex-directions"),
            ("Responsive layout", "#flex-responsive"),
        ],
        "grid" => vec![
            ("Spans and offsets", "#grid-spans"),
            ("Responsive spans", "#grid-responsive"),
        ],
        "group" => vec![
            ("Wrapping", "#group-wrap"),
            ("Growth and long content", "#group-growth"),
        ],
        "simple-grid" => vec![
            ("Columns and gaps", "#simple-columns"),
            ("Responsive columns", "#simple-responsive"),
        ],
        "space" => vec![("Axes and parent gaps", "#space-axes")],
        "stack" => vec![("Alignment and distribution", "#stack-distribution")],
        "splitter" => vec![
            ("Controlled sizes and keyboard", "#splitter-controlled"),
            ("Multiple panels", "#splitter-nested"),
            ("Fixed navigation", "#splitter-fixed"),
        ],
        _ => Vec::new(),
    }
}
