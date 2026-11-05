use super::showcase_examples::DetailSection;
mod collapsed;
mod configuration;
mod regions;
mod responsive;
mod scrolling;
pub(crate) mod viewport;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "shell-regions",
            title: "Header, main and footer",
            note: "The header spans the frame, the sidebar belongs to the body, and main fills the remaining width. Footer spans the bottom. A shell without a sidebar is also supported. Give the frame a height for an application viewport. This embedded example is scoped to its container; it does not fix its sections over the documentation page.",
            example: Box::new(regions::demo(theme.clone())),
            source: include_str!("regions.rs"),
        },
        DetailSection {
            id: "shell-responsive",
            title: "Responsive navigation",
            note: "The breakpoint uses the shell's measured container width. Below 560px, navigation moves into the shared modal drawer and the menu icon opens it. Resize this page to try it. The same navigation tree and draft stay mounted. Escape or an outside click closes the drawer and restores the trigger.",
            example: Box::new(responsive::demo(theme.clone())),
            source: include_str!("responsive.rs"),
        },
        DetailSection {
            id: "shell-collapsed",
            title: "Collapsed desktop navigation",
            note: "The controlled collapsed source starts with a compact rail. The menu icon proposes expansion; turn off acceptance to retain the rail. width and rail_width configure desktop sizes. Without a source, default_collapsed(true) initializes local state. Mobile opening uses the separate mobile_open source and on_mobile_open_change callback. External writes remain silent and never rebuild the navigation content.",
            example: Box::new(collapsed::demo(theme.clone())),
            source: include_str!("collapsed.rs"),
        },
        DetailSection {
            id: "shell-scrolling",
            title: "Independent scrolling and sections",
            note: "main_scrollable(true) gives main its own keyboard, wheel and touch scroll area. A bounded frame height keeps header/footer stationary. shell_section().scrollable(true) allocates the remaining navigation space; static sections above and below stay visible. Sidebar.scrollable(true) scrolls the whole navigation instead. Give scrollable sections descriptive names. These use the shared host scroll protocol and preserve their children.",
            example: Box::new(scrolling::demo(theme.clone())),
            source: include_str!("scrolling.rs"),
        },
        DetailSection {
            id: "shell-configuration",
            title: "Region dimensions, borders and visibility",
            note: "header_style, footer_style, main_style, body_style and aside_style accept reactive typed styles. Explicit header height replaces its default minimum. with_border configures all region borders, with an optional Sidebar override. header_visible, footer_visible and aside_visible hide their regions without rebuilding content; flex layout automatically releases the occupied space. aside_width configures the optional right-hand panel. Toggle the controls and keep a note while regions change.",
            example: Box::new(configuration::demo(theme.clone())),
            source: include_str!("configuration.rs"),
        },
        DetailSection {
            id: "shell-viewport",
            title: "Full-window application frame",
            note: "viewport() fixes the outer frame to the CSS viewport and enables independent main scrolling. Header, footer and navigation remain outside that scroll area. The live example opens on its own page so it cannot cover this documentation. Embedded shells use the container layout by default. Region style overrides are explicit; absolute/fixed custom regions require corresponding body_style spacing to reserve their offsets. Mobile navigation uses the shared modal drawer; native adapters that lack this layer reject it before moving navigation.",
            example: Box::new(viewport::link(theme)),
            source: include_str!("viewport.rs"),
        },
    ]
}
