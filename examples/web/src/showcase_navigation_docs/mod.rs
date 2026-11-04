use super::showcase_examples::DetailSection;
mod anchor_content;
mod anchor_requests;
mod breadcrumb_collapsed;
mod breadcrumb_separator;
mod burger_options;
mod nav_link_nested;
mod nav_link_variants;
mod pagination_controls;
mod pagination_unknown;
mod pagination_window;

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
        "pagination" => vec![
            section!(
                pagination_controls,
                "pagination-controls",
                "Directional controls and disabled state",
                "with_controls configures previous/next; with_edges configures first/last. Both default to true. Visible arrows and page numbers stay compact, while every native button has a complete accessible name. Known boundaries and enabled(false) disable actual activation."
            ),
            section!(
                pagination_window,
                "pagination-window",
                "Siblings, boundaries and controlled pages",
                "siblings and boundaries each accept 0–8. A finite window inserts ellipses without scanning the total. page and on_page_change propose controlled changes; the example accepts proposals and updates its content. Rejected proposals preserve the displayed page. Invalid page/total combinations report an error rather than silently clamping."
            ),
            section!(
                pagination_unknown,
                "pagination-unknown",
                "Unknown totals",
                "total(None) shows the current page without inventing a total. has_next explicitly enables the next request, and Last remains disabled. External page changes are silent; the component does not fetch or prefetch data."
            ),
        ],
        "breadcrumbs" => vec![
            section!(
                breadcrumb_separator,
                "breadcrumbs-separator",
                "Separator and spacing",
                "separator accepts static or reactive text; the default is /. gap controls the space around separators. Items can be links or plain text; current supplies the current key, defaulting to the final item. An empty current key clears that claim."
            ),
            section!(
                breadcrumb_collapsed,
                "breadcrumbs-collapsed",
                "Long paths and controlled expansion",
                "max_items folds the middle of a long path while retaining a current middle item. expanded and on_expanded_change control the disclosure; defaults apply only to local state. Hidden items retain identity, and collapsing an actually focused hidden item restores the disclosure trigger."
            ),
        ],
        "nav-link" => vec![
            section!(
                nav_link_variants,
                "nav-link-sections",
                "Descriptions, sections and active variants",
                "description and passive left_section/right_section content belong to the same real link. The label remains its accessible name. current claims the current page; highlighted affects appearance only. Light, Filled and Subtle configure active presentation. Interactive descendants are rejected by the shared host contract."
            ),
            section!(
                nav_link_nested,
                "nav-link-nested",
                "Nested links and controlled expansion",
                "Navigation and disclosure are separate controls: following the parent does not toggle its children. expanded/on_expanded_change control opening; this example accepts proposals. KeepMounted retains child state; the default Unmount retires it on close. Disclosure restores focus if closing hides the focused child."
            ),
        ],
        "anchor" => vec![
            section!(
                anchor_requests,
                "anchor-requests",
                "Application-owned navigation requests",
                "Without on_navigate, a real link keeps normal host navigation. With the callback, the application receives the captured accepted href/open request before generic observers and decides how to navigate. Relative, http, https, mailto and tel targets are supported; executable/custom schemes are rejected."
            ),
            section!(
                anchor_content,
                "anchor-content",
                "Passive content and accessible names",
                "fill adds passive display content inside the real link. label remains its accessible name unless name explicitly overrides it. Content cannot introduce nested interactive controls. current uses finite page/step/location metadata and does not claim selection."
            ),
        ],
        "burger" => vec![section!(
            burger_options,
            "burger-options",
            "Sizes, local state and disabled controls",
            "open/on_open_change is controlled; default_open supplies local state. The icon animates between menu and close shapes while preserving the native button. enabled(false) blocks activation. Size changes both icon and hit area. The widget controls only disclosure state; the application owns the navigation panel."
        )],
        _ => Vec::new(),
    }
}

pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "pagination" => vec![
            ("Controls", "#pagination-controls"),
            ("Page window", "#pagination-window"),
            ("Unknown totals", "#pagination-unknown"),
        ],
        "breadcrumbs" => vec![
            ("Separator", "#breadcrumbs-separator"),
            ("Long paths", "#breadcrumbs-collapsed"),
        ],
        "nav-link" => vec![
            ("Sections and variants", "#nav-link-sections"),
            ("Nested links", "#nav-link-nested"),
        ],
        "anchor" => vec![
            ("Navigation requests", "#anchor-requests"),
            ("Link content", "#anchor-content"),
        ],
        "burger" => vec![("Sizes and state", "#burger-options")],
        _ => Vec::new(),
    }
}
