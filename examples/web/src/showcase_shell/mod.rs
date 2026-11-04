use super::showcase_examples::DetailSection;
mod collapsed;
mod regions;
mod responsive;
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
            note: "default_collapsed(true) starts with the compact rail. The menu icon expands it. width and rail_width configure desktop sizes; collapsed and on_collapsed_change provide controlled state. Mobile opening uses the separate mobile_open source and on_mobile_open_change callback.",
            example: Box::new(collapsed::demo(theme)),
            source: include_str!("collapsed.rs"),
        },
    ]
}
