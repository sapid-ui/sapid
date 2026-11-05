mod actions_keyboard;
mod actions_position;
mod actions_selection;
mod drawer_controlled;
mod drawer_directions;
mod drawer_nested;
mod drawer_scroll;
mod drawer_sizes;
mod floating_geometry;
mod floating_layout;
mod floating_lifetime;
mod hover_controlled;
mod hover_delays;
mod hover_placements;
mod hover_targets;
mod loading_geometry;
mod loading_presentation;
mod loading_retained;
mod modal_centered;
mod modal_controlled;
mod modal_full_screen;
mod modal_nested;
mod modal_scroll;
mod modal_sizes;
mod notice_content;
mod notice_controlled;
mod notice_position;
mod overlay_colors;
mod overlay_pointer;
mod overlay_viewport;
mod popover_content;
mod popover_controlled;
mod popover_nested;
mod popover_placements;
mod popover_widths;
mod tooltip_delays;
mod tooltip_disabled;
mod tooltip_multiline;
mod tooltip_placements;
mod tooltip_targets;
mod tour_controlled;
mod tour_missing;
mod tour_reveal;
use super::showcase_examples::DetailSection;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "modal" => vec![
            DetailSection {
                id: "modal-centered",
                title: "Centered modal",
                note: "The shared host centers the finite panel, manages focus and returns to the invoking trigger. Close is a localized compact vector action.",
                example: Box::new(modal_centered::demo(theme.clone())),
                source: include_str!("modal_centered.rs"),
            },
            DetailSection {
                id: "modal-full-screen",
                title: "Full-screen modal",
                note: "Full screen uses the same modal focus and dismissal protocol. The host viewport constrains layout; it does not replace the current document.",
                example: Box::new(modal_full_screen::demo(theme.clone())),
                source: include_str!("modal_full_screen.rs"),
            },
            DetailSection {
                id: "modal-sizes",
                title: "Explicit panel width and longer headings",
                note: "Panel dimensions use explicit Style width, constrained by the host viewport. size selects control metrics, so it is not a panel-width alias.",
                example: Box::new(modal_sizes::demo(theme.clone())),
                source: include_str!("modal_sizes.rs"),
            },
            DetailSection {
                id: "modal-scroll",
                title: "Long content with a local scrolling body",
                note: "A fixed-height body scrolls inside the constrained panel. The heading and close action remain outside this local scrolling region.",
                example: Box::new(modal_scroll::demo(theme.clone())),
                source: include_str!("modal_scroll.rs"),
            },
            DetailSection {
                id: "modal-controlled",
                title: "Controlled dismissal and a retained draft",
                note: "open is read-only. The application may refuse close requests; KeepMounted preserves the real draft through accepted closes without creating another editor.",
                example: Box::new(modal_controlled::demo(theme.clone())),
                source: include_str!("modal_controlled.rs"),
            },
            DetailSection {
                id: "modal-nested",
                title: "Nested dialogs and explicit initial focus",
                note: "The shared host owns nested focus containment, Escape order and return focus. The inner dialog explicitly focuses its named safe action.",
                example: Box::new(modal_nested::demo(theme.clone())),
                source: include_str!("modal_nested.rs"),
            },
        ],
        "drawer" => vec![
            DetailSection {
                id: "drawer-directions",
                title: "Four physical viewport edges",
                note: "DrawerSide chooses Left, Right, Top or Bottom. Side drawers use explicit width; top and bottom examples use full width and a bounded height. Native edge layers retain their explicit unsupported boundary.",
                example: Box::new(drawer_directions::demo(theme.clone())),
                source: include_str!("drawer_directions.rs"),
            },
            DetailSection {
                id: "drawer-sizes",
                title: "Panel width and a custom close name",
                note: "Explicit panel width is separate from the theme size of controls. Long titles wrap beside the close action and the shared host constrains the panel on narrow screens.",
                example: Box::new(drawer_sizes::demo(theme.clone())),
                source: include_str!("drawer_sizes.rs"),
            },
            DetailSection {
                id: "drawer-scroll",
                title: "Scrolling content",
                note: "The drawer body has its own scroll box. Long content does not require an oversized page or hide the localized close action.",
                example: Box::new(drawer_scroll::demo(theme.clone())),
                source: include_str!("drawer_scroll.rs"),
            },
            DetailSection {
                id: "drawer-controlled",
                title: "Controlled opening and retained content",
                note: "The application accepts open and dismissal proposals explicitly. A refused close preserves the panel, focus constraints and its retained draft.",
                example: Box::new(drawer_controlled::demo(theme.clone())),
                source: include_str!("drawer_controlled.rs"),
            },
            DetailSection {
                id: "drawer-nested",
                title: "Nested modal focus",
                note: "An ordinary modal may open inside a drawer. The shared layer host handles the topmost Escape request, focus scope and restoration to the inner trigger.",
                example: Box::new(drawer_nested::demo(theme.clone())),
                source: include_str!("drawer_nested.rs"),
            },
        ],
        "popover" => vec![
            DetailSection {
                id: "popover-placements",
                title: "Sides, alignment and collision-aware arrows",
                note: "Anchor placement, gap and arrows use the shared host geometry. Requested sides may flip to keep the panel visible; the arrow follows the resolved side.",
                example: Box::new(popover_placements::demo(theme.clone())),
                source: include_str!("popover_placements.rs"),
            },
            DetailSection {
                id: "popover-widths",
                title: "Intrinsic, numeric and target-matched width",
                note: "Auto uses content width, Px uses a fixed CSS-pixel width, and Target follows the actual trigger border box. Host collision constraints still apply.",
                example: Box::new(popover_widths::demo(theme.clone())),
                source: include_str!("popover_widths.rs"),
            },
            DetailSection {
                id: "popover-controlled",
                title: "Controlled state and dismissal policies",
                note: "Opening and dismissal are proposals. Outside dismissal is disabled here; the close action or Escape requests closing, and the application can refuse that request.",
                example: Box::new(popover_controlled::demo(theme.clone())),
                source: include_str!("popover_controlled.rs"),
            },
            DetailSection {
                id: "popover-content",
                title: "Retained editors and long content",
                note: "KeepMounted retains the actual editor through close/reopen. A local scroll box bounds long content independently of the trigger width.",
                example: Box::new(popover_content::demo(theme.clone())),
                source: include_str!("popover_content.rs"),
            },
            DetailSection {
                id: "popover-nested",
                title: "Interactive content and nested layers",
                note: "Popover content may contain ordinary controls and another shared modal. Keyboard focus and Escape remain host-owned rather than custom browser listeners.",
                example: Box::new(popover_nested::demo(theme.clone())),
                source: include_str!("popover_nested.rs"),
            },
        ],
        "dialog" => vec![
            DetailSection {
                id: "notice-position",
                title: "Finite viewport placement",
                note: "position takes a bounded ViewportPlacement in CSS pixels, including viewport padding and constraint policy. Opening remains nonmodal and preserves current focus.",
                example: Box::new(notice_position::demo(theme.clone())),
                source: include_str!("notice_position.rs"),
            },
            DetailSection {
                id: "notice-controlled",
                title: "Controlled close with background editing",
                note: "Nonmodal notices preserve focus and do not trap Tab. Outside dismissal defaults off; this application may also refuse the close action or Escape proposal.",
                example: Box::new(notice_controlled::demo(theme.clone())),
                source: include_str!("notice_controlled.rs"),
            },
            DetailSection {
                id: "notice-content",
                title: "Local drafts and content lifetime",
                note: "KeepMounted keeps its child editor and owner. Unmount recreates a local draft on the next open. Neither policy changes the notice into a modal window.",
                example: Box::new(notice_content::demo(theme.clone())),
                source: include_str!("notice_content.rs"),
            },
        ],
        "loading-overlay" => vec![
            DetailSection {
                id: "loading-presentation",
                title: "Loader kind, size, color and motion",
                note: "The shade opacity applies only to the mask. Loader kind, theme size, explicit color and animated opt-out configure retained host Graphics without changing busy/inert behavior.",
                example: Box::new(loading_presentation::demo(theme.clone())),
                source: include_str!("loading_presentation.rs"),
            },
            DetailSection {
                id: "loading-retained",
                title: "Retained content, focus and keyboard blocking",
                note: "The source coordinates busy, content inert and mask visibility. Ending loading restores availability without focusing the old editor; it does not cancel tasks or disable successful form values.",
                example: Box::new(loading_retained::demo(theme.clone())),
                source: include_str!("loading_retained.rs"),
            },
            DetailSection {
                id: "loading-geometry",
                title: "Containing box and local geometry",
                note: "The mask contributes no height. The retained content or explicit root dimensions define its containing box. Static positioning is rejected; this example stays inside its local preview.",
                example: Box::new(loading_geometry::demo(theme.clone())),
                source: include_str!("loading_geometry.rs"),
            },
        ],
        "overlay" => vec![
            DetailSection {
                id: "overlay-colors",
                title: "Color alpha and shade opacity",
                note: "color alpha and opacity multiply with byte rounding. The underlying content keeps its own opacity; this is decorative shading with no busy or modal semantics.",
                example: Box::new(overlay_colors::demo(theme.clone())),
                source: include_str!("overlay_colors.rs"),
            },
            DetailSection {
                id: "overlay-pointer",
                title: "Pointer-transparent presentation",
                note: "A decorative mask does not block pointer, keyboard or programmatic focus. Use LoadingOverlay when the application intends to make a region inert.",
                example: Box::new(overlay_pointer::demo(theme.clone())),
                source: include_str!("overlay_pointer.rs"),
            },
            DetailSection {
                id: "overlay-viewport",
                title: "Explicit fixed viewport composition",
                note: "A fixed wrapper uses the host viewport instead of the local preview. The example starts closed and remains pointer-transparent; its own shade includes no dismissal/focus controller. Backdrop blur and CSS filters are outside this finite typed API.",
                example: Box::new(overlay_viewport::demo(theme.clone())),
                source: include_str!("overlay_viewport.rs"),
            },
        ],
        "tooltip" => vec![
            DetailSection {
                id: "tooltip-placements",
                title: "Position, arrows and viewport collisions",
                note: "A finite anchor placement chooses side, alignment and arrow. The host may flip the requested side and constrains the tooltip to its current viewport. Tooltip text stays passive and never owns focus.",
                example: Box::new(tooltip_placements::demo(theme.clone())),
                source: include_str!("tooltip_placements.rs"),
            },
            DetailSection {
                id: "tooltip-multiline",
                title: "Longer text and explicit line breaks",
                note: "Tooltip accepts finite plain text, not interactive content. An explicit width and pre-wrap whitespace retain line breaks while allowing longer lines to wrap.",
                example: Box::new(tooltip_multiline::demo(theme.clone())),
                source: include_str!("tooltip_multiline.rs"),
            },
            DetailSection {
                id: "tooltip-delays",
                title: "Explicit open and close delay",
                note: "This example explicitly supplies cancellable browser Futures: 200ms open and 120ms close. Native completion here is immediate; a native application supplies its own clock service. Reversal and Owner retirement cancel the current wait.",
                example: Box::new(tooltip_delays::demo(theme.clone())),
                source: include_str!("tooltip_delays.rs"),
            },
            DetailSection {
                id: "tooltip-disabled",
                title: "Disabled controls and an available help action",
                note: "Disabled tooltip targets do not provide hover/focus help. Keep the explanation in visible content or a separate enabled help action; tooltip must not be a control’s only accessible name.",
                example: Box::new(tooltip_disabled::demo(theme.clone())),
                source: include_str!("tooltip_disabled.rs"),
            },
            DetailSection {
                id: "tooltip-targets",
                title: "Actual input targets and visible touch guidance",
                note: "FloatingTarget adapts a real input and preserves its editor and accessible name. Focus opens the description. Important guidance remains visible for touch users; no implicit long-press policy is installed.",
                example: Box::new(tooltip_targets::demo(theme.clone())),
                source: include_str!("tooltip_targets.rs"),
            },
        ],
        "hover-card" => vec![
            DetailSection {
                id: "hover-placements",
                title: "Positions and interactive card content",
                note: "The requested anchor side and optional arrow use host collision handling. A 160ms application-supplied browser close delay lets the pointer cross the gap into the card. Native applications supply their own timer policy.",
                example: Box::new(hover_placements::demo(theme.clone())),
                source: include_str!("hover_placements.rs"),
            },
            DetailSection {
                id: "hover-delays",
                title: "Delayed opening and pointer gap protection",
                note: "This example explicitly uses 120ms open and 200ms close browser waits. Returning to trigger or panel cancels closing. The same controller coalesces focus movement between interactive child controls.",
                example: Box::new(hover_delays::demo(theme.clone())),
                source: include_str!("hover_delays.rs"),
            },
            DetailSection {
                id: "hover-controlled",
                title: "Controlled state and an explicit touch entry",
                note: "The visible state is application-owned. This separate action provides an explicit touch entry; the application may refuse automatic dismissal while retaining the original card content.",
                example: Box::new(hover_controlled::demo(theme.clone())),
                source: include_str!("hover_controlled.rs"),
            },
            DetailSection {
                id: "hover-targets",
                title: "Adapted trigger controls and retained actions",
                note: "FloatingTarget preserves the actual button’s content and activation handler. Opening does not steal focus; keyboard users can Tab into the interactive card. Browser close timing is explicit application composition.",
                example: Box::new(hover_targets::demo(theme.clone())),
                source: include_str!("hover_targets.rs"),
            },
        ],
        "action-bar" => vec![
            DetailSection {
                id: "actions-selection",
                title: "Selection count and unavailable actions",
                note: "Selection is business data, not local state inside the bar. The bar appears when count is positive; actions and Clear propose application changes. Unavailable actions remain explicitly disabled.",
                example: Box::new(actions_selection::demo(theme.clone())),
                source: include_str!("actions_selection.rs"),
            },
            DetailSection {
                id: "actions-position",
                title: "Explicit fixed placement",
                note: "The action bar owns selection actions; the application owns its placement. This bounded fixed example starts closed and wraps its actions at narrow widths instead of compressing labels into vertical text.",
                example: Box::new(actions_position::demo(theme.clone())),
                source: include_str!("actions_position.rs"),
            },
            DetailSection {
                id: "actions-keyboard",
                title: "Grouped keyboard actions and RTL",
                note: "A bar reuses Toolbar’s single roving entry, cross-group arrows and Home/End. Direction changes horizontal arrow policy. Disabled actions are skipped; navigation does not activate an action.",
                example: Box::new(actions_keyboard::demo(theme.clone())),
                source: include_str!("actions_keyboard.rs"),
            },
        ],
        "floating-indicator" => vec![
            DetailSection {
                id: "floating-geometry",
                title: "Live dimensions and explicit remeasurement",
                note: "Resize interests track changing target sizes. measurement_revision explicitly requests a position refresh after a layout change that keeps dimensions unchanged; only the active mounted target is queried.",
                example: Box::new(floating_geometry::demo(theme.clone())),
                source: include_str!("floating_geometry.rs"),
            },
            DetailSection {
                id: "floating-layout",
                title: "Vertical layout and physical coordinates",
                note: "The marker follows actual measured boxes rather than assuming equal tab widths. Switching row/column layout requests a measurement revision. RTL layouts use the same physical box coordinates; application direction changes that preserve size require an explicit refresh.",
                example: Box::new(floating_layout::demo(theme.clone())),
                source: include_str!("floating_layout.rs"),
            },
            DetailSection {
                id: "floating-lifetime",
                title: "Missing targets and motion control",
                note: "Removing the current target hides the decoration; restoring that key registers its new live identity. animated(false), zero duration and host reduced motion keep a static marker without per-frame source writes.",
                example: Box::new(floating_lifetime::demo(theme.clone())),
                source: include_str!("floating_lifetime.rs"),
            },
        ],
        "tour" => vec![
            DetailSection {
                id: "tour-controlled",
                title: "Controlled navigation and dismissal",
                note: "Current step and open are read-only inputs. This application can refuse Next or closing without advancing the accepted step or replacing the guide panel.",
                example: Box::new(tour_controlled::demo(theme.clone())),
                source: include_str!("tour_controlled.rs"),
            },
            DetailSection {
                id: "tour-missing",
                title: "Target disappearance and recovery",
                note: "The guide only queries its current live target. Missing or hidden targets suppress the panel and report one loss episode; restoring the current target recovers without rewriting the open or current source.",
                example: Box::new(tour_missing::demo(theme.clone())),
                source: include_str!("tour_missing.rs"),
            },
            DetailSection {
                id: "tour-reveal",
                title: "Current target reveal and stable editing",
                note: "A guide explicitly reveals its current mounted target before measuring its live box. The second target is inside a local scroll viewport; navigation preserves the real draft editor and the shared host owns focus return.",
                example: Box::new(tour_reveal::demo(theme.clone())),
                source: include_str!("tour_reveal.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "modal" => vec![
            ("Centered modal", "#modal-centered"),
            ("Full-screen modal", "#modal-full-screen"),
            ("Explicit panel width and longer headings", "#modal-sizes"),
            ("Long content with a local scrolling body", "#modal-scroll"),
            (
                "Controlled dismissal and a retained draft",
                "#modal-controlled",
            ),
            ("Nested dialogs and explicit initial focus", "#modal-nested"),
        ],
        "drawer" => vec![
            ("Four physical viewport edges", "#drawer-directions"),
            ("Panel width and a custom close name", "#drawer-sizes"),
            ("Scrolling content", "#drawer-scroll"),
            (
                "Controlled opening and retained content",
                "#drawer-controlled",
            ),
            ("Nested modal focus", "#drawer-nested"),
        ],
        "popover" => vec![
            (
                "Sides, alignment and collision-aware arrows",
                "#popover-placements",
            ),
            (
                "Intrinsic, numeric and target-matched width",
                "#popover-widths",
            ),
            (
                "Controlled state and dismissal policies",
                "#popover-controlled",
            ),
            ("Retained editors and long content", "#popover-content"),
            ("Interactive content and nested layers", "#popover-nested"),
        ],
        "dialog" => vec![
            ("Finite viewport placement", "#notice-position"),
            (
                "Controlled close with background editing",
                "#notice-controlled",
            ),
            ("Local drafts and content lifetime", "#notice-content"),
        ],
        "loading-overlay" => vec![
            (
                "Loader kind, size, color and motion",
                "#loading-presentation",
            ),
            (
                "Retained content, focus and keyboard blocking",
                "#loading-retained",
            ),
            ("Containing box and local geometry", "#loading-geometry"),
        ],
        "overlay" => vec![
            ("Color alpha and shade opacity", "#overlay-colors"),
            ("Pointer-transparent presentation", "#overlay-pointer"),
            ("Explicit fixed viewport composition", "#overlay-viewport"),
        ],
        "tooltip" => vec![
            (
                "Position, arrows and viewport collisions",
                "#tooltip-placements",
            ),
            ("Longer text and explicit line breaks", "#tooltip-multiline"),
            ("Explicit open and close delay", "#tooltip-delays"),
            (
                "Disabled controls and an available help action",
                "#tooltip-disabled",
            ),
            (
                "Actual input targets and visible touch guidance",
                "#tooltip-targets",
            ),
        ],
        "hover-card" => vec![
            (
                "Positions and interactive card content",
                "#hover-placements",
            ),
            (
                "Delayed opening and pointer gap protection",
                "#hover-delays",
            ),
            (
                "Controlled state and an explicit touch entry",
                "#hover-controlled",
            ),
            (
                "Adapted trigger controls and retained actions",
                "#hover-targets",
            ),
        ],
        "action-bar" => vec![
            (
                "Selection count and unavailable actions",
                "#actions-selection",
            ),
            ("Explicit fixed placement", "#actions-position"),
            ("Grouped keyboard actions and RTL", "#actions-keyboard"),
        ],
        "floating-indicator" => vec![
            (
                "Live dimensions and explicit remeasurement",
                "#floating-geometry",
            ),
            (
                "Vertical layout and physical coordinates",
                "#floating-layout",
            ),
            ("Missing targets and motion control", "#floating-lifetime"),
        ],
        "tour" => vec![
            ("Controlled navigation and dismissal", "#tour-controlled"),
            ("Target disappearance and recovery", "#tour-missing"),
            ("Current target reveal and stable editing", "#tour-reveal"),
        ],
        _ => Vec::new(),
    }
}
