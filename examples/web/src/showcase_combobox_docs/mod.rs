mod autocomplete_blur;
mod autocomplete_controlled;
mod autocomplete_data;
mod autocomplete_first;
mod autocomplete_limit;
mod autocomplete_loading;
mod cascade_controlled;
mod cascade_lazy;
mod cascade_mobile;
mod cascade_path;
mod cascade_states;
mod combo_content;
mod combo_controlled;
mod combo_filter;
mod combo_groups;
mod multi_select_controlled;
mod multi_select_data;
mod multi_select_loading;
mod multi_select_states;
mod multi_select_wrapping;
mod pill_long;
mod pill_removal;
mod pill_sizes;
mod pill_states;
mod pills_controlled;
mod pills_loading;
mod pills_policies;
mod pills_states;
mod pills_wrapping;
mod tags_blur;
mod tags_composition;
mod tags_controlled;
mod tags_limits_clear;
mod tags_loading;
mod tags_parsing;
mod tree_cascade;
mod tree_controlled;
mod tree_geometry;
mod tree_limits;
mod tree_multiple;
mod tree_render;
mod tree_search;
mod tree_single;
mod tree_states;
use super::showcase_examples::DetailSection;
mod popover_controlled;
mod popover_empty;
mod popover_groups;
mod popover_target;
pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match id {
        "combobox-popover" => vec![
            DetailSection {
                id: "popover-target",
                title: "A real button target and option list",
                note: "This example uses Select.trigger_content for passive content inside the same intrinsic button. It retains listbox relationships, actual keyboard selection and accepted state; it is not an ordinary dialog Popover. A custom trigger requires an explicit name or Field association. Nested controls and layout Views inside the button are rejected by the shared content rules.",
                example: Box::new(popover_target::demo(theme.clone())),
                source: include_str!("popover_target.rs"),
            },
            DetailSection {
                id: "popover-groups",
                title: "Groups, disabled options and bounded geometry",
                note: "Stable keys are independent of labels. The shared listbox skips disabled options and keeps focus on the target while arrows and Home/End change the active option. The listbox is anchored to the actual target with a bounded height; option adornments do not alter its accessible name.",
                example: Box::new(popover_groups::demo(theme.clone())),
                source: include_str!("popover_groups.rs"),
            },
            DetailSection {
                id: "popover-controlled",
                title: "Controlled opening and refused selection",
                note: "Open and selected sources are independent read-only inputs. Callbacks only propose changes. This application accepts opening and can refuse selected values. Escape closes the current listbox; clicking an option requests selection without moving focus into its passive content.",
                example: Box::new(popover_controlled::demo(theme.clone())),
                source: include_str!("popover_controlled.rs"),
            },
            DetailSection {
                id: "popover-empty",
                title: "Empty, disabled and read-only choices",
                note: "An empty declaration renders a passive bounded empty message. Disabled prevents opening; read-only preserves an accepted display without allowing selection. Use the editable Combobox or Autocomplete pages for target-editor search. This Select composition does not implement a separate popup search field, multiple values or arbitrary polymorphic target elements.",
                example: Box::new(popover_empty::demo(theme.clone())),
                source: include_str!("popover_empty.rs"),
            },
        ],
        "autocomplete" => vec![
            DetailSection {
                id: "autocomplete-controlled",
                title: "Controlled free text",
                note: "Autocomplete owns an ordinary text editor. value contains free text, independently of the selected suggestion key. A controlled value is a read-only accepted source; this application can refuse proposals. on_commit and on_option_select are separate events. Programmatic replacement is silent.",
                example: Box::new(autocomplete_controlled::demo(theme.clone())),
                source: include_str!("autocomplete_controlled.rs"),
            },
            DetailSection {
                id: "autocomplete-loading",
                title: "Loading indicators on either side",
                note: "The compact decorative loader has a fixed 16-pixel inset and busy metadata. Loading is presentation, not an implicit request or a lock on editing. The application controls the source and any resource lifetime. Both indicators retain the same real editor.",
                example: Box::new(autocomplete_loading::demo(theme.clone())),
                source: include_str!("autocomplete_loading.rs"),
            },
            DetailSection {
                id: "autocomplete-first",
                title: "First match and keyboard commitment",
                note: "select_first_option_on_change highlights the first current suggestion. Arrow keys change activity; Enter commits the active suggestion. Composition preserves preedit and prevents choice commands from consuming an unfinished IME candidate. No extra editor or focusable option is introduced.",
                example: Box::new(autocomplete_first::demo(theme.clone())),
                source: include_str!("autocomplete_first.rs"),
            },
            DetailSection {
                id: "autocomplete-blur",
                title: "Select on real blur",
                note: "auto_select_on_blur proposes the first matching suggestion only on an actual blur, outside composition and while available. Enter and free-text commitment remain distinct. The application may still refuse a controlled proposal.",
                example: Box::new(autocomplete_blur::demo(theme.clone())),
                source: include_str!("autocomplete_blur.rs"),
            },
            DetailSection {
                id: "autocomplete-data",
                title: "Groups, descriptions, clearing and unavailable states",
                note: "Groups and disabled options use the same bounded declaration. option_content supplies passive leading/description/trailing text; accessible names and matching still use the original label. Clear returns focus to the same editor. Read-only and disabled sources are checked again when events run.",
                example: Box::new(autocomplete_data::demo(theme.clone())),
                source: include_str!("autocomplete_data.rs"),
            },
            DetailSection {
                id: "autocomplete-limit",
                title: "Filtering, empty results and a visible limit",
                note: "Contains, Prefix and None are finite filter policies. This prefix example displays at most five matches among 100 declarations and a passive empty message. The declaration budget is 1024; at most 256 rows may have bounded embellishments. This is not virtualization or a custom JavaScript filter callback.",
                example: Box::new(autocomplete_limit::demo(theme.clone())),
                source: include_str!("autocomplete_limit.rs"),
            },
        ],
        "combobox" => vec![
            DetailSection {
                id: "combo-controlled",
                title: "Target, search, selected value and options",
                note: "The editable Input is the target and keeps actual focus. Its search String and selected Option<String> key are independent accepted sources; callbacks propose changes. The shared controller owns the listbox, active descendant, keyboard selection, revisions and composition. This finite Rust widget does not expose a second React-style store/hooks API.",
                example: Box::new(combo_controlled::demo(theme.clone())),
                source: include_str!("combo_controlled.rs"),
            },
            DetailSection {
                id: "combo-groups",
                title: "Groups, unavailable options and empty results",
                note: "Option keys remain stable independently of labels. Groups preserve source order and disabled items are excluded from navigation. No match displays a passive empty message while the same target editor stays mounted.",
                example: Box::new(combo_groups::demo(theme.clone())),
                source: include_str!("combo_groups.rs"),
            },
            DetailSection {
                id: "combo-content",
                title: "Passive option composition",
                note: "option_content adds bounded leading, description and trailing text inside each intrinsic option. The original label remains the accessible name and matching source. Nested interactive content and arbitrary host DOM callbacks are not part of this contract. Up to 256 embellished rows fit alongside 1024 declarations.",
                example: Box::new(combo_content::demo(theme.clone())),
                source: include_str!("combo_content.rs"),
            },
            DetailSection {
                id: "combo-filter",
                title: "Filtering, limits and loading sources",
                note: "Contains, Prefix and None choose finite matching policies. limit bounds visible matches, while loading only exposes application-owned progress. The target editor retains UTF-16 selection and IME state. For a passive custom button target, use the separate ComboboxPopover composition.",
                example: Box::new(combo_filter::demo(theme.clone())),
                source: include_str!("combo_filter.rs"),
            },
        ],
        "multi-select" => vec![
            DetailSection {
                id: "multi-select-controlled",
                title: "Controlled values and refused proposals",
                note: "value is the accepted Vec of stable keys; search remains an independent editor value. Selection, removal and clear propose a replacement. Refusal leaves accepted tokens unchanged. External updates are silent. An inset disclosure opens the existing listbox and returns focus to the same editor without duplicating its opening proposal.",
                example: Box::new(multi_select_controlled::demo(theme.clone())),
                source: include_str!("multi_select_controlled.rs"),
            },
            DetailSection {
                id: "multi-select-loading",
                title: "Compact loading and action regions",
                note: "A 16-pixel decorative indicator fits either side of the last editor row. Clear uses a 24-pixel named action; the disclosure uses the same right-side slot when clearing is unavailable. loading exposes busy but does not start a request or lock editing. dropdown_label localizes the disclosure action.",
                example: Box::new(multi_select_loading::demo(theme.clone())),
                source: include_str!("multi_select_loading.rs"),
            },
            DetailSection {
                id: "multi-select-data",
                title: "Groups, selection limits and empty results",
                note: "max_values bounds accepted selection; limit bounds visible matches. hide_picked_options removes accepted keys from matching without changing declarations. Disabled options are skipped. Option descriptions are bounded passive text and do not alter label matching or names. The empty message reflects current results.",
                example: Box::new(multi_select_data::demo(theme.clone())),
                source: include_str!("multi_select_data.rs"),
            },
            DetailSection {
                id: "multi-select-wrapping",
                title: "Wrapping tokens and the retained editor",
                note: "Tokens and the genuine editor share one bordered wrapping surface. Short tokens leave room for typing on the same line; long selections increase intrinsic height and shrink again when cleared. Long labels clip locally while removal actions keep complete accessible names. Source replacement preserves the real editor, caret and composition protocol.",
                example: Box::new(multi_select_wrapping::demo(theme.clone())),
                source: include_str!("multi_select_wrapping.rs"),
            },
            DetailSection {
                id: "multi-select-states",
                title: "Unavailable, invalid and five sizes",
                note: "Disabled and read-only retain accepted values while preventing changes. Required/invalid metadata belongs to the visible editor and successful-value proxy contract. The five sizes preserve compact token removal, clear/loading centering and independent font metrics. Native CPU/model checks do not establish OS input or assistive technology acceptance.",
                example: Box::new(multi_select_states::demo(theme.clone())),
                source: include_str!("multi_select_states.rs"),
            },
        ],
        "pill" => vec![
            DetailSection {
                id: "pill-sizes",
                title: "Five sizes and named removal",
                note: "A Pill is passive text with an optional separately named removal button. Five size tokens set compact label and removal dimensions. A passive Pill has no click or keyboard action; the named removal button uses native activation.",
                example: Box::new(pill_sizes::demo(theme.clone())),
                source: include_str!("pill_sizes.rs"),
            },
            DetailSection {
                id: "pill-removal",
                title: "Controlled visibility and removal proposals",
                note: "Without a visible source removal hides the Pill locally. With a controlled source on_remove only proposes dismissal, so this application can refuse it. Hidden content retains its identity. Applications own focus restoration when dismissing an independently placed Pill.",
                example: Box::new(pill_removal::demo(theme.clone())),
                source: include_str!("pill_removal.rs"),
            },
            DetailSection {
                id: "pill-states",
                title: "Passive, disabled and custom presentation",
                note: "enabled(false) disables the removal action and preserves accepted content. unstyled removes theme presentation while explicit styles still apply. A Pill is not a pressed toggle, checkbox or link.",
                example: Box::new(pill_states::demo(theme.clone())),
                source: include_str!("pill_states.rs"),
            },
            DetailSection {
                id: "pill-long",
                title: "Long labels and an actual editable composition",
                note: "Long labels stay inside their finite container and the remove button keeps its own hit area. Use PillsInput for free values plus a real editor: it owns UTF-16 selection, composition and atomic tag commands. A collection of passive Pill widgets does not manufacture that editing contract.",
                example: Box::new(pill_long::demo(theme.clone())),
                source: include_str!("pill_long.rs"),
            },
        ],
        "pills-input" => vec![
            DetailSection {
                id: "pills-controlled",
                title: "Atomic controlled topics",
                note: "Accepted values and draft search form one TagsInputState. on_change proposes both atomically; the application can refuse the whole replacement. The input remains a genuine editor with UTF-16 selection and composition. PillsInput changes accepted token presentation, not the editing protocol.",
                example: Box::new(pills_controlled::demo(theme.clone())),
                source: include_str!("pills_controlled.rs"),
            },
            DetailSection {
                id: "pills-loading",
                title: "Loading and inset actions",
                note: "The decorative 16-pixel indicator and the 24-pixel clear action are centered on the final editor row at every size. Loading only exposes application-owned progress. Clearing and individual removal return focus through the live input controller, including after a token or clear button retires.",
                example: Box::new(pills_loading::demo(theme.clone())),
                source: include_str!("pills_loading.rs"),
            },
            DetailSection {
                id: "pills-policies",
                title: "Separators and duplicate policy",
                note: "Enter, EnterComma and EnterCommaSemicolon are finite commit policies. The accepted values contain free text, not option keys. Duplicate Reject, Ignore and Allow govern complete proposals; disabled, read-only and composition are rechecked before commands. No suggestion list or combobox role is invented.",
                example: Box::new(pills_policies::demo(theme.clone())),
                source: include_str!("pills_policies.rs"),
            },
            DetailSection {
                id: "pills-wrapping",
                title: "Wrapping, caret and conditional placeholder",
                note: "Tokens and the retained editor share one wrapping border. The editor follows tokens on the current line or the next available line. This example deliberately hides its hint while topics exist and restores it when empty; placeholder remains independent editor metadata and does not become a value or label. Replacing topics preserves the draft and editor identity.",
                example: Box::new(pills_wrapping::demo(theme.clone())),
                source: include_str!("pills_wrapping.rs"),
            },
            DetailSection {
                id: "pills-states",
                title: "Disabled, read-only and five sizes",
                note: "Unavailable controls retain accepted topics and suppress commands. Read-only permits focus and text selection without accepting changes. Five sizes share typed token geometry, local long-label clipping and centered removal. There is no hidden second editable control.",
                example: Box::new(pills_states::demo(theme.clone())),
                source: include_str!("pills_states.rs"),
            },
        ],
        "tags-input" => vec![
            DetailSection {
                id: "tags-controlled",
                title: "Atomic controlled keywords",
                note: "values and search are one accepted TagsInputState. Refusing on_change preserves both rather than accepting a partial paste or removal. Program updates are silent and the real editor remains mounted. The output below is accepted business content, not an event/revision trace.",
                example: Box::new(tags_controlled::demo(theme.clone())),
                source: include_str!("tags_controlled.rs"),
            },
            DetailSection {
                id: "tags-loading",
                title: "Loading and retained editing",
                note: "Loading is a decorative bounded indicator and busy metadata. It does not start a request or prohibit editing. Both sides share the final editor row, with compact clearing and focus restoration through the live controller.",
                example: Box::new(tags_loading::demo(theme.clone())),
                source: include_str!("tags_loading.rs"),
            },
            DetailSection {
                id: "tags-limits-clear",
                title: "Whole-proposal limits and clearing",
                note: "max_tags limits the whole proposed list. Individual tags are bounded to 256 UTF-8 bytes and all accepted tag text to 64 KiB; paste transport has its own 16 KiB bound. A later invalid token refuses the entire paste without partial publication. Clear proposes empty values and search and restores focus to the same editor.",
                example: Box::new(tags_limits_clear::demo(theme.clone())),
                source: include_str!("tags_limits_clear.rs"),
            },
            DetailSection {
                id: "tags-blur",
                title: "Commit on actual blur",
                note: "accept_value_on_blur is optional. Actual blur proposes the current nonempty draft outside composition and while available; empty or invalid candidates do not manufacture tags. Applications retain controlled acceptance and error policy.",
                example: Box::new(tags_blur::demo(theme.clone())),
                source: include_str!("tags_blur.rs"),
            },
            DetailSection {
                id: "tags-parsing",
                title: "Duplicate occurrences and separators",
                note: "Reject refuses an existing value, Ignore skips duplicates within the atomic proposal, and Allow keeps distinct occurrences with independently named removal actions. Enter/comma/semicolon delimiter policies preserve ordinary caret editing and clipboard replacement. Whitespace normalization is part of the finite tag model.",
                example: Box::new(tags_parsing::demo(theme.clone())),
                source: include_str!("tags_parsing.rs"),
            },
            DetailSection {
                id: "tags-composition",
                title: "Selection, paste, empty deletion and unavailable states",
                note: "The ordinary input owns caret, native paste and IME. Empty uncomposed Backspace removes one accepted trailing tag; ordinary text deletion remains in the editor. Composition cannot trigger tokenization, removal or clearing. Native provider/CPU tests and browser automation are separate from actual OS IME, mobile devices and assistive technology acceptance.",
                example: Box::new(tags_composition::demo(theme.clone())),
                source: include_str!("tags_composition.rs"),
            },
        ],
        "tree-select" => vec![
            DetailSection {
                id: "tree-single",
                title: "Single selection and full paths",
                note: "TreeNode declares explicit stable keys, parent keys and finite kinds. Accepted TreeChoices contain complete TreePath ancestry, independently from labels and current expansion. The popup uses actual tree items. Clearing returns focus to the same current trigger and clear_label localizes the inset action.",
                example: Box::new(tree_single::demo(theme.clone())),
                source: include_str!("tree_single.rs"),
            },
            DetailSection {
                id: "tree-multiple",
                title: "Multiple leaf selection",
                note: "multiple(true) accepts several full paths. Leaf-only policy does not silently select a parent. The same tree controller owns keyboard navigation, active items, disabled skipping and selected state; values are not flattened to ambiguous leaf labels.",
                example: Box::new(tree_multiple::demo(theme.clone())),
                source: include_str!("tree_multiple.rs"),
            },
            DetailSection {
                id: "tree-cascade",
                title: "Parent-child checkbox selection",
                note: "DescendantLeaves requires multiple selection. Parent toggles affect current enabled loaded leaves and mixed state is derived from them. Disabled leaves and future children of lazy nodes are excluded. Loading later children does not retroactively fabricate accepted selections.",
                example: Box::new(tree_cascade::demo(theme.clone())),
                source: include_str!("tree_cascade.rs"),
            },
            DetailSection {
                id: "tree-search",
                title: "Editable search and empty results",
                note: "searchable uses a real target editor and preserves its identity while filtering. Ancestors of matches remain visible. Search and selected paths are independent accepted sources; closing clears the search through its proposal callback. nothing_found_message names the empty result, not an invented option.",
                example: Box::new(tree_search::demo(theme.clone())),
                source: include_str!("tree_search.rs"),
            },
            DetailSection {
                id: "tree-controlled",
                title: "Controlled value, expansion and opening",
                note: "value, expanded and open are independent controlled sources. Callbacks propose changes and applications can refuse selected paths while accepting expansion and opening. Reconciliation restores accepted state after the host tick; program changes remain silent. The popup focus policy and return target stay with the shared Layer/Tree contracts.",
                example: Box::new(tree_controlled::demo(theme.clone())),
                source: include_str!("tree_controlled.rs"),
            },
            DetailSection {
                id: "tree-limits",
                title: "Selection limits and hidden picked leaves",
                note: "max_values bounds the next complete choice, and hide_picked filters accepted leaves from current options. Data and paths are admitted together; invalid reparenting or stale ancestry preserves the previous accepted presentation and reports an error. This does not create a visited-node or selection-history registry.",
                example: Box::new(tree_limits::demo(theme.clone())),
                source: include_str!("tree_limits.rs"),
            },
            DetailSection {
                id: "tree-render",
                title: "Custom node and accepted-value content",
                note: "render_node and render_value receive reactive handles to current TreeNode data. The examples mount passive text widgets and preserve original item identity, names and data matching. Trigger content must respect the shared button content rules. Callers keep business data and resource ownership.",
                example: Box::new(tree_render::demo(theme.clone())),
                source: include_str!("tree_render.rs"),
            },
            DetailSection {
                id: "tree-geometry",
                title: "Popup geometry and connecting lines",
                note: "popup_width and max_dropdown_height are explicit finite dimensions. popup_placement uses the shared collision policy; with_lines controls presentation. Intrinsic Tree keyboard navigation explicitly reveals current items in the scroll area. Geometry/model checks do not establish OS input or screen-reader acceptance.",
                example: Box::new(tree_geometry::demo(theme.clone())),
                source: include_str!("tree_geometry.rs"),
            },
            DetailSection {
                id: "tree-states",
                title: "Disabled, read-only and associated errors",
                note: "Field supplies visible labels, required/error metadata and real associations. Disabled and read-only preserve accepted paths while suppressing interaction. External source changes are independent from user availability. Clear checks the current controller and restores focus only while its trigger remains available.",
                example: Box::new(tree_states::demo(theme.clone())),
                source: include_str!("tree_states.rs"),
            },
        ],
        "cascader" => vec![
            DetailSection {
                id: "cascade-path",
                title: "Hierarchical expansion and selected paths",
                note: "The column presentation expands one complete ancestor chain. Selecting a leaf proposes its full TreePath, while labels remain independent data. Values and column expansion are controlled separately. clear_label names the inset clearing action; clearing restores the same actual trigger.",
                example: Box::new(cascade_path::demo(theme.clone())),
                source: include_str!("cascade_path.rs"),
            },
            DetailSection {
                id: "cascade-controlled",
                title: "Controlled refusal and parent selection",
                note: "change_on_select(true) also proposes parent paths and keeps branch browsing open; selecting a leaf completes the choice. A controlled source may refuse the proposal. Expansion still follows the accepted column chain and no value source is mutated by the component.",
                example: Box::new(cascade_controlled::demo(theme.clone())),
                source: include_str!("cascade_controlled.rs"),
            },
            DetailSection {
                id: "cascade-mobile",
                title: "Compact layout, RTL and searchable alternatives",
                note: "with_columns(false) chooses a single-column tree at mount time, useful for narrow layouts. It is not an implicit viewport observer or a dynamic layout switch. Column mode remains locally scrollable and keyboard navigation reveals current rows. This example also shows the separate editable TreeSelect search composition; Cascader itself remains a select-only target.",
                example: Box::new(cascade_mobile::demo(theme.clone())),
                source: include_str!("cascade_mobile.rs"),
            },
            DetailSection {
                id: "cascade-lazy",
                title: "Explicitly supplied lazy children",
                note: "Lazy nodes request on_load_children. The application publishes validated Branch/child declarations; this example supplies local sample data exactly once by checking the current node kind. A real Resource, transport, cancellation and permission policy remain application-owned. No fetch or filesystem access is implied.",
                example: Box::new(cascade_lazy::demo(theme.clone())),
                source: include_str!("cascade_lazy.rs"),
            },
            DetailSection {
                id: "cascade-states",
                title: "Unavailable categories and associated errors",
                note: "Disabled and read-only preserve full selected paths and suppress changes. Field owns visible labels and error associations. Stable keys and current ancestry are validated, including when data changes. Protocol/provider/native geometry evidence stays separate from actual mobile devices, OS dialogs and assistive technology.",
                example: Box::new(cascade_states::demo(theme.clone())),
                source: include_str!("cascade_states.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "combobox-popover" => vec![
            ("A real button target and option list", "#popover-target"),
            (
                "Groups, disabled options and bounded geometry",
                "#popover-groups",
            ),
            (
                "Controlled opening and refused selection",
                "#popover-controlled",
            ),
            ("Empty, disabled and read-only choices", "#popover-empty"),
        ],
        "autocomplete" => vec![
            ("Controlled free text", "#autocomplete-controlled"),
            ("Loading indicators on either side", "#autocomplete-loading"),
            ("First match and keyboard commitment", "#autocomplete-first"),
            ("Select on real blur", "#autocomplete-blur"),
            (
                "Groups, descriptions, clearing and unavailable states",
                "#autocomplete-data",
            ),
            (
                "Filtering, empty results and a visible limit",
                "#autocomplete-limit",
            ),
        ],
        "combobox" => vec![
            (
                "Target, search, selected value and options",
                "#combo-controlled",
            ),
            (
                "Groups, unavailable options and empty results",
                "#combo-groups",
            ),
            ("Passive option composition", "#combo-content"),
            ("Filtering, limits and loading sources", "#combo-filter"),
        ],
        "multi-select" => vec![
            (
                "Controlled values and refused proposals",
                "#multi-select-controlled",
            ),
            (
                "Compact loading and action regions",
                "#multi-select-loading",
            ),
            (
                "Groups, selection limits and empty results",
                "#multi-select-data",
            ),
            (
                "Wrapping tokens and the retained editor",
                "#multi-select-wrapping",
            ),
            (
                "Unavailable, invalid and five sizes",
                "#multi-select-states",
            ),
        ],
        "pill" => vec![
            ("Five sizes and named removal", "#pill-sizes"),
            (
                "Controlled visibility and removal proposals",
                "#pill-removal",
            ),
            ("Passive, disabled and custom presentation", "#pill-states"),
            (
                "Long labels and an actual editable composition",
                "#pill-long",
            ),
        ],
        "pills-input" => vec![
            ("Atomic controlled topics", "#pills-controlled"),
            ("Loading and inset actions", "#pills-loading"),
            ("Separators and duplicate policy", "#pills-policies"),
            (
                "Wrapping, caret and conditional placeholder",
                "#pills-wrapping",
            ),
            ("Disabled, read-only and five sizes", "#pills-states"),
        ],
        "tags-input" => vec![
            ("Atomic controlled keywords", "#tags-controlled"),
            ("Loading and retained editing", "#tags-loading"),
            ("Whole-proposal limits and clearing", "#tags-limits-clear"),
            ("Commit on actual blur", "#tags-blur"),
            ("Duplicate occurrences and separators", "#tags-parsing"),
            (
                "Selection, paste, empty deletion and unavailable states",
                "#tags-composition",
            ),
        ],
        "tree-select" => vec![
            ("Single selection and full paths", "#tree-single"),
            ("Multiple leaf selection", "#tree-multiple"),
            ("Parent-child checkbox selection", "#tree-cascade"),
            ("Editable search and empty results", "#tree-search"),
            (
                "Controlled value, expansion and opening",
                "#tree-controlled",
            ),
            ("Selection limits and hidden picked leaves", "#tree-limits"),
            ("Custom node and accepted-value content", "#tree-render"),
            ("Popup geometry and connecting lines", "#tree-geometry"),
            ("Disabled, read-only and associated errors", "#tree-states"),
        ],
        "cascader" => vec![
            ("Hierarchical expansion and selected paths", "#cascade-path"),
            (
                "Controlled refusal and parent selection",
                "#cascade-controlled",
            ),
            (
                "Compact layout, RTL and searchable alternatives",
                "#cascade-mobile",
            ),
            ("Explicitly supplied lazy children", "#cascade-lazy"),
            (
                "Unavailable categories and associated errors",
                "#cascade-states",
            ),
        ],
        _ => Vec::new(),
    }
}
