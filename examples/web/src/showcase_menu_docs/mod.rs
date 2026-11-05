use super::showcase_examples::DetailSection;
mod choices;
mod content;
mod controlled;
mod geometry;
mod keyboard;
mod search;
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Content slots", "#menu-content"),
        ("Controlled", "#menu-controlled"),
        ("Checkbox and radio", "#menu-choices"),
        ("Search", "#menu-search"),
        ("Submenu geometry", "#menu-geometry"),
        ("Keyboard and state", "#menu-keyboard"),
    ]
}
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "menu-content",
            title: "Group captions and passive content slots",
            note: "menu_label adds an independent caption without a keyboard stop. Action items and submenu triggers accept passive Text/Graphic/Group content in left_section and right_section; the label remains their accessible name and typeahead text. Nested controls are rejected. Shortcut annotations are display text; the application owns any global shortcut registration. Intrinsic checkbox/radio items retain their own labels and check indicators.",
            example: Box::new(content::demo(theme.clone())),
            source: include_str!("content.rs"),
        },
        DetailSection {
            id: "menu-controlled",
            title: "Controlled opening, refusal and selection policy",
            note: "open is a read-only source. on_open_change proposes updates; refusing a close keeps the menu open. Action items close by default. close_on_select(false) keeps a command menu open after an action. Escape and outside pointer dismissal use the same proposal; Tab leaves the menu tree even when closing is refused.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "menu-choices",
            title: "Checkbox and radio items",
            note: "Checkbox and radio items use actual intrinsic check controls. Their sources remain read-only and callbacks can accept or refuse a proposed value. They keep the menu open by default, and radio grouping owns one keyed value. Enter and Space retain the shared host activation behavior.",
            example: Box::new(choices::demo(theme.clone())),
            source: include_str!("choices.rs"),
        },
        DetailSection {
            id: "menu-search",
            title: "Search, external filtering and empty results",
            note: "MenuSearch retains the real search editor while Up/Down change a highlighted item and Enter invokes its existing action/check/radio/submenu callback. The application derives visible for each declared item, including parent visibility for matching descendants. Hidden rows keep their Owners and cannot activate. A query is bounded to 4 KiB UTF-8. clear_on_close(true) proposes an empty query after logical close; this example lets you preserve it instead. Empty feedback is an ordinary conditional menu caption.",
            example: Box::new(search::demo(theme.clone())),
            source: include_str!("search.rs"),
        },
        DetailSection {
            id: "menu-geometry",
            title: "Dropdown width, placement and submenus",
            note: "width controls the panel separately from the trigger and accepts finite 80–1000px values. position uses the shared anchored placement and viewport collision policy. Submenus have independent width/position and directional chevrons; right_section replaces the default chevron. A menu stays within its declared finite tree: no arbitrary portaled controls or browser-specific safe-area polygon is implied.",
            example: Box::new(geometry::demo(theme.clone())),
            source: include_str!("geometry.rs"),
        },
        DetailSection {
            id: "menu-keyboard",
            title: "Keyboard, RTL, disabled and unstyled states",
            note: "Without search, arrows/Home/End move focus and explicit-clock typeahead matches item labels. Tab leaves all menu trees. RTL reverses submenu entry/exit directions. Disabled items are skipped; a disabled root cannot open. Unstyled removes theme paint while explicit width/style remains application-owned. Browser and Apace have actual menu protocols; Wry explicitly rejects these semantics until its host implements them.",
            example: Box::new(keyboard::demo(theme)),
            source: include_str!("keyboard.rs"),
        },
    ]
}
