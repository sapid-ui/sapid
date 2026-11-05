mod step_content;
mod step_controls;
mod step_statuses;
mod step_vertical;
mod toc_nested;
mod toc_registration;
mod toc_scroll;
mod tree_checked;
mod tree_controlled;
mod tree_drag_drop;
mod tree_expansion;
mod tree_filter;
mod tree_lazy;
mod tree_render_label;
mod tree_strict;
use super::showcase_examples::DetailSection;
pub fn tree_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "tree-controlled",
            title: "Controlled expansion and selection",
            note: "Expansion and selection are application-owned values. Keyboard, pointer and typeahead proposals use the same controlled callbacks.",
            example: Box::new(tree_controlled::demo(theme.clone())),
            source: include_str!("tree_controlled.rs"),
        },
        DetailSection {
            id: "tree-expansion",
            title: "Expand and collapse all",
            note: "expanded_tree_branches derives the finite set of current branch keys; no component-side visited-node registry is retained.",
            example: Box::new(tree_expansion::demo(theme.clone())),
            source: include_str!("tree_expansion.rs"),
        },
        DetailSection {
            id: "tree-checked",
            title: "Cascading checked state",
            note: "DescendantLeaves toggles current enabled leaves and derives checked or mixed parent state in one bounded postorder pass.",
            example: Box::new(tree_checked::demo(theme.clone())),
            source: include_str!("tree_checked.rs"),
        },
        DetailSection {
            id: "tree-strict",
            title: "Independent checked state",
            note: "AnyNode provides strict independent toggles: checking a branch does not implicitly change its descendants.",
            example: Box::new(tree_strict::demo(theme.clone())),
            source: include_str!("tree_strict.rs"),
        },
        DetailSection {
            id: "tree-lazy",
            title: "Async loading contract",
            note: "Opening a lazy branch emits one explicit load request. The application publishes the loaded declaration without hidden I/O in the widget.",
            example: Box::new(tree_lazy::demo(theme.clone())),
            source: include_str!("tree_lazy.rs"),
        },
        DetailSection {
            id: "tree-filter",
            title: "External search and filtering",
            note: "filter_tree_nodes is a bounded pure utility that retains every matching node's ancestor chain; the search field remains ordinary application state.",
            example: Box::new(tree_filter::demo(theme.clone())),
            source: include_str!("tree_filter.rs"),
        },
        DetailSection {
            id: "tree-drag-drop",
            title: "Drag and drop",
            note: "Pointer drops propose Before, Inside or After. allow_drop can reject a target, and move_tree_node publishes a validated finite declaration.",
            example: Box::new(tree_drag_drop::demo(theme.clone())),
            source: include_str!("tree_drag_drop.rs"),
        },
        DetailSection {
            id: "tree-render-label",
            title: "Custom labels and connecting lines",
            note: "render_label mounts typed Widget content with stable keyed Owners. File icons and real branch guide elements remain separate finite presentation facets.",
            example: Box::new(tree_render_label::demo(theme.clone())),
            source: include_str!("tree_render_label.rs"),
        },
    ]
}

pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "tree" => tree_sections(theme),
        "stepper" => vec![
            DetailSection {
                id: "step-controlled",
                title: "Controlled navigation, refusal and skipping",
                note: "active is a zero-based accepted value. Exactly the step count represents completion. Header selection is opt-in, skipping is separate, and disabled state is rechecked before proposals or generic observers. This application can refuse a requested step; content, current marker and connectors follow the accepted source.",
                example: Box::new(step_controls::demo(theme.clone())),
                source: include_str!("step_controls.rs"),
            },
            DetailSection {
                id: "step-statuses",
                title: "Completed, current, error and optional states",
                note: "Completed status normally derives from the accepted active step; an explicit status can override it. Error does not validate a form or block a separate Next button. Optional is descriptive. Five theme sizes keep square circular markers, centered content and matching connector geometry.",
                example: Box::new(step_statuses::demo(theme.clone())),
                source: include_str!("step_statuses.rs"),
            },
            DetailSection {
                id: "step-vertical",
                title: "Vertical layout and unavailable steps",
                note: "The vertical track retains the same workflow semantics and joins real marker positions. Header buttons have complete accessible names; the last step is independently disabled. Horizontal tracks remain locally scrollable at narrow widths, and actual focused headers request reveal through the shared host action.",
                example: Box::new(step_vertical::demo(theme.clone())),
                source: include_str!("step_vertical.rs"),
            },
            DetailSection {
                id: "step-content",
                title: "Content lifetime, editing and completion",
                note: "KeepMounted preserves the draft widget and its real editor when another step is active. Unmount retires inactive child owners and creates fresh local state on return. The completed stage has its own content. Applications own validation, routing and submission; Next is an ordinary application action.",
                example: Box::new(step_content::demo(theme.clone())),
                source: include_str!("step_content.rs"),
            },
        ],
        "table-of-contents" => vec![
            DetailSection {
                id: "toc-registration",
                title: "Explicit heading registration and live updates",
                note: "toc_context owns one bounded scrolling document. toc_heading registers its key, level, label and order; ordinary external DOM headings are not discovered. Rename, reorder, remove and restore demonstrate live registration while the notes editor retains its state. These application actions do not add a second heading registry.",
                example: Box::new(toc_registration::demo(theme.clone())),
                source: include_str!("toc_registration.rs"),
            },
            DetailSection {
                id: "toc-scroll",
                title: "Local scrolling, heading offset and reveal",
                note: "The context observes this scrolling document, not the entire browser window. Following a real heading link or calling reveal scrolls the owned document through shared host actions. offset reserves space above the heading; keyboard focus and surrounding page position stay independent. refresh is available when an application changes layout outside normal notifications.",
                example: Box::new(toc_scroll::demo(theme.clone())),
                source: include_str!("toc_scroll.rs"),
            },
            DetailSection {
                id: "toc-nested",
                title: "Nested levels and an application-owned current marker",
                note: "Heading levels determine relative indentation, and order determines document order. This example explicitly supplies current as a read-only visual source. The marker button does not scroll; link activation still reveals a heading. Omit current to use the context scroll-derived marker. The Rust API uses explicit registration rather than arbitrary CSS selectors or DOM callbacks.",
                example: Box::new(toc_nested::demo(theme.clone())),
                source: include_str!("toc_nested.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "stepper" => vec![
            (
                "Controlled navigation, refusal and skipping",
                "#step-controlled",
            ),
            (
                "Completed, current, error and optional states",
                "#step-statuses",
            ),
            ("Vertical layout and unavailable steps", "#step-vertical"),
            ("Content lifetime, editing and completion", "#step-content"),
        ],
        "table-of-contents" => vec![
            (
                "Explicit heading registration and live updates",
                "#toc-registration",
            ),
            ("Local scrolling, heading offset and reveal", "#toc-scroll"),
            (
                "Nested levels and an application-owned current marker",
                "#toc-nested",
            ),
        ],
        _ => Vec::new(),
    }
}
