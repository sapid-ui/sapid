mod alert_colors;
mod alert_content;
mod alert_controlled;
mod alert_variants;
mod empty_compact;
mod empty_search;
mod empty_slots;
mod loader_custom;
mod loader_inline;
mod loader_sizes;
mod loader_types;
mod notification_controlled;
mod notification_loading;
mod notification_presentation;
mod progress_geometry;
mod progress_ring_angles;
mod progress_ring_caps_label;
mod progress_ring_gaps;
mod progress_sections;
mod progress_semicircle_direction;
mod progress_semicircle_labels;
mod progress_states;
mod progress_vertical;
mod skeleton_animation;
mod skeleton_content;
mod skeleton_shapes;
use super::showcase_examples::DetailSection;
pub fn progress_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "progress-geometry",
            title: "Line, ring and semicircle geometry",
            note: "Each view keeps one progressbar semantic while size and thickness change only its finite presentation geometry.",
            example: Box::new(progress_geometry::demo(theme.clone())),
            source: include_str!("progress_geometry.rs"),
        },
        DetailSection {
            id: "progress-sections",
            title: "Compound sections",
            note: "Line and ring indicators accept up to 16 ordered positive sections. Their sum is validated against the exposed progress value. Horizontal section_label text clips within its segment; value_text provides a readable category breakdown.",
            example: Box::new(progress_sections::demo(theme.clone())),
            source: include_str!("progress_sections.rs"),
        },
        DetailSection {
            id: "progress-vertical",
            title: "Vertical orientation",
            note: "Linear indicators use the same bounded progress semantic in either orientation. Determinate and compound fills grow from the bottom; indeterminate presentation stays on the same main axis.",
            example: Box::new(progress_vertical::demo(theme.clone())),
            source: include_str!("progress_vertical.rs"),
        },
        DetailSection {
            id: "progress-semicircle-direction",
            title: "Semicircle orientation and fill direction",
            note: "Up and down geometry is independent from left-to-right and right-to-left value advancement. Filled and empty segments accept separate finite colors.",
            example: Box::new(progress_semicircle_direction::demo(theme.clone())),
            source: include_str!("progress_semicircle_direction.rs"),
        },
        DetailSection {
            id: "progress-semicircle-labels",
            title: "Semicircle label position",
            note: "A bounded text label can sit at the arc baseline or in the center while the root retains its single progressbar semantic.",
            example: Box::new(progress_semicircle_labels::demo(theme.clone())),
            source: include_str!("progress_semicircle_labels.rs"),
        },
        DetailSection {
            id: "progress-ring-gaps",
            title: "Ring section gaps",
            note: "A finite degree gap reduces each visible section without changing the ordered progress total or section offsets.",
            example: Box::new(progress_ring_gaps::demo(theme.clone())),
            source: include_str!("progress_ring_gaps.rs"),
        },
        DetailSection {
            id: "progress-ring-angles",
            title: "Ring start angle and background",
            note: "Start angles follow the same degree convention as the reference component: 0° right, 90° bottom, 180° left and 270° top. Root color styles the unfilled track independently.",
            example: Box::new(progress_ring_angles::demo(theme.clone())),
            source: include_str!("progress_ring_angles.rs"),
        },
        DetailSection {
            id: "progress-ring-caps-label",
            title: "Ring caps and center label",
            note: "Rounded caps apply only to the visible outer section ends; the center label remains passive content inside the single progressbar.",
            example: Box::new(progress_ring_caps_label::demo(theme.clone())),
            source: include_str!("progress_ring_caps_label.rs"),
        },
        DetailSection {
            id: "progress-states",
            title: "Indeterminate and accessible values",
            note: "Unknown progress omits the current value and uses host motion: ring rotation or line/semicircle opacity pulse. animated(false) and reduced motion keep a static shape. Determinate indicators expose bounded value text without becoming interactive.",
            example: Box::new(progress_states::demo(theme.clone())),
            source: include_str!("progress_states.rs"),
        },
    ]
}

pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "progress" | "ring-progress" | "semi-circle-progress" => progress_sections(theme),
        "alert" => vec![
            DetailSection {
                id: "alert-variants",
                title: "Four message variants",
                note: "Light, Filled, Outline and Transparent change the presentation of the same finite message. Title and description relationships remain stable; the close control is a named 24-pixel action with a passive 16-pixel vector.",
                example: Box::new(alert_variants::demo(theme.clone())),
                source: include_str!("alert_variants.rs"),
            },
            DetailSection {
                id: "alert-colors",
                title: "Severity and explicit colors",
                note: "Info, Success, Warning and Danger use the theme palette. Filled messages select a readable foreground for title, body and close glyph, including bright custom colors. Announcement policy is independent from severity.",
                example: Box::new(alert_colors::demo(theme.clone())),
                source: include_str!("alert_colors.rs"),
            },
            DetailSection {
                id: "alert-controlled",
                title: "Controlled dismissal and localized action names",
                note: "visible is a read-only accepted source. on_close proposes dismissal and the application can refuse it. A reactive close_label updates the same native button; programmatic restoration does not invoke dismissal.",
                example: Box::new(alert_controlled::demo(theme.clone())),
                source: include_str!("alert_controlled.rs"),
            },
            DetailSection {
                id: "alert-content",
                title: "Untitled and long messages",
                note: "An untitled message keeps its description relationship and independently named action. Longer content wraps in the flexible body while icon and close geometry stay bounded. These examples turn announcements off for static documentation content.",
                example: Box::new(alert_content::demo(theme.clone())),
                source: include_str!("alert_content.rs"),
            },
        ],
        "empty-state" => vec![
            DetailSection {
                id: "empty-search",
                title: "Empty search results and a real reset action",
                note: "The application filters a finite local project list. Reset clears the accepted query and reveals the matching-result count. EmptyState supplies illustration, title, explanation and actions without installing a live announcement or owning search state.",
                example: Box::new(empty_search::demo(theme.clone())),
                source: include_str!("empty_search.rs"),
            },
            DetailSection {
                id: "empty-compact",
                title: "A compact empty region",
                note: "A smaller theme size, heading level and explicit padding suit a constrained region. The illustration and explanation retain their hierarchy. Unsupplied recipe slots contribute no blank wrappers or artificial gaps.",
                example: Box::new(empty_compact::demo(theme.clone())),
                source: include_str!("empty_compact.rs"),
            },
            DetailSection {
                id: "empty-slots",
                title: "Body content and primary or secondary actions",
                note: "illustration, description, fill and actions are ordinary Widget slots. This example reveals actual help content and provides a secondary link. Actions wrap in the available width; applications own their effects.",
                example: Box::new(empty_slots::demo(theme.clone())),
                source: include_str!("empty_slots.rs"),
            },
        ],
        "notification" => vec![
            DetailSection {
                id: "notification-presentation",
                title: "Icons, borders, titles and longer content",
                note: "Standalone notifications use the same finite message presentation as Alert. Icons are passive typed content; an outlined variant, an untitled message and a persistent nonclosable notice are independent configurations. Long text wraps beside the compact action.",
                example: Box::new(notification_presentation::demo(theme.clone())),
                source: include_str!("notification_presentation.rs"),
            },
            DetailSection {
                id: "notification-loading",
                title: "Loading with a retained result icon",
                note: "loading exposes busy, presents a compact decorative indicator and prevents dismissal while work is pending. The original icon and message remain mounted. The application supplies loading; no request, timeout or queue is started by this widget.",
                example: Box::new(notification_loading::demo(theme.clone())),
                source: include_str!("notification_loading.rs"),
            },
            DetailSection {
                id: "notification-controlled",
                title: "Controlled dismissal and action content",
                note: "A controlled visible source can refuse close requests. The action callback changes real detail content and its accepted label. Queue admission, timers and replacement tokens belong to the separate Notifications system page, not to this standalone message.",
                example: Box::new(notification_controlled::demo(theme.clone())),
                source: include_str!("notification_controlled.rs"),
            },
        ],
        "loader" => vec![
            DetailSection {
                id: "loader-types",
                title: "Oval, bars and dots",
                note: "Oval rotates a finite arc. Bars and Dots use three passive shapes with phased host-owned opacity pulses. Each indicator remains one status when named; decorative removes its own announcement when a surrounding region already provides one.",
                example: Box::new(loader_types::demo(theme.clone())),
                source: include_str!("loader_types.rs"),
            },
            DetailSection {
                id: "loader-sizes",
                title: "Theme sizes, numeric diameter and color",
                note: "Five theme sizes produce 12, 16, 24, 32 and 40 pixel indicators. Explicit diameter is bounded to 8–128 pixels; color is a finite RGBA value. Thickness configures the oval stroke. The loader conveys activity rather than a numeric percentage.",
                example: Box::new(loader_sizes::demo(theme.clone())),
                source: include_str!("loader_sizes.rs"),
            },
            DetailSection {
                id: "loader-inline",
                title: "Inline alignment and one meaningful status",
                note: "A 16-pixel decorative indicator aligns with adjacent text inside a named status. The application controls busy and visible state. The same indicator is retained when hidden. Reduced motion keeps a visible static shape and cannot be overridden by animated(true).",
                example: Box::new(loader_inline::demo(theme.clone())),
                source: include_str!("loader_inline.rs"),
            },
            DetailSection {
                id: "loader-custom",
                title: "Custom finite Widget composition",
                note: "A custom indicator can be an ordinary Widget with finite GraphicProps and one status owner. It uses shared host animation and reduced-motion handling. This API does not provide an arbitrary SVG-string loader registry or application frame loop.",
                example: Box::new(loader_custom::demo(theme.clone())),
                source: include_str!("loader_custom.rs"),
            },
        ],
        "skeleton" => vec![
            DetailSection {
                id: "skeleton-shapes",
                title: "Circles, rounded blocks and text rows",
                note: "Circle, height, width and radius describe finite placeholders. A profile row combines a circular image placeholder with text lines; another block uses square corners. Percent widths are resolved by the actual containing layout.",
                example: Box::new(skeleton_shapes::demo(theme.clone())),
                source: include_str!("skeleton_shapes.rs"),
            },
            DetailSection {
                id: "skeleton-content",
                title: "Retained content and intrinsic height",
                note: "fill keeps the genuine editor mounted. Loading makes its content inert and visually hidden while preserving layout, accepted text and identity; finishing restores interaction. Without an explicit height, the wrapper uses its content height. It does not cancel child tasks or disable successful form values.",
                example: Box::new(skeleton_content::demo(theme.clone())),
                source: include_str!("skeleton_content.rs"),
            },
            DetailSection {
                id: "skeleton-animation",
                title: "Animation controls and reduced motion",
                note: "animated(false) opts out of the host-owned opacity pulse. A reduced-motion preference presents the fully opaque shape. Toggling presentation does not create a timer or publish per-frame business values. A standalone placeholder hides when loading is false.",
                example: Box::new(skeleton_animation::demo(theme.clone())),
                source: include_str!("skeleton_animation.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "alert" => vec![
            ("Four message variants", "#alert-variants"),
            ("Severity and explicit colors", "#alert-colors"),
            (
                "Controlled dismissal and localized action names",
                "#alert-controlled",
            ),
            ("Untitled and long messages", "#alert-content"),
        ],
        "empty-state" => vec![
            (
                "Empty search results and a real reset action",
                "#empty-search",
            ),
            ("A compact empty region", "#empty-compact"),
            (
                "Body content and primary or secondary actions",
                "#empty-slots",
            ),
        ],
        "notification" => vec![
            (
                "Icons, borders, titles and longer content",
                "#notification-presentation",
            ),
            (
                "Loading with a retained result icon",
                "#notification-loading",
            ),
            (
                "Controlled dismissal and action content",
                "#notification-controlled",
            ),
        ],
        "loader" => vec![
            ("Oval, bars and dots", "#loader-types"),
            ("Theme sizes, numeric diameter and color", "#loader-sizes"),
            (
                "Inline alignment and one meaningful status",
                "#loader-inline",
            ),
            ("Custom finite Widget composition", "#loader-custom"),
        ],
        "skeleton" => vec![
            ("Circles, rounded blocks and text rows", "#skeleton-shapes"),
            ("Retained content and intrinsic height", "#skeleton-content"),
            (
                "Animation controls and reduced motion",
                "#skeleton-animation",
            ),
        ],
        _ => Vec::new(),
    }
}
