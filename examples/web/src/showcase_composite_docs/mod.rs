use super::showcase_examples::DetailSection;
mod accordion_content;
mod accordion_controlled;
mod accordion_dynamic;
mod accordion_variants;
mod tabs_content;
mod tabs_controlled;
mod tabs_dynamic;
mod tabs_orientation;
mod tabs_variants;
mod timeline_markers;
mod timeline_progress;
mod timeline_semantics;

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
        "accordion" => vec![
            section!(
                accordion_variants,
                "accordion-variants",
                "Variants and disclosure icons",
                "Default uses full-width headings and separators; Contained, Filled and Separated configure item surfaces. with_chevron(false) omits the passive disclosure graphic. A custom trigger_content factory preserves the native button and its label. Both static and keyed item APIs use the same presentation."
            ),
            section!(
                accordion_controlled,
                "accordion-controlled",
                "Multiple sections and controlled state",
                "multiple(true) permits independent open sections. open/on_open_change proposes the full accepted key list; local defaults apply only without a controlled source. enabled(false) disables a section, while collapsible(false) keeps the last section open. Arrow keys, Home and End navigate enabled headings; loop_focus and direction control the finite policy."
            ),
            section!(
                accordion_content,
                "accordion-content",
                "Retain or retire panel content",
                "ContentPolicy::KeepMounted hides a panel while preserving its editor and Owner. Unmount, the default, retires child state when closed. Type into both drafts, close and reopen the sections to compare. Closing content with actual focus restores the heading trigger."
            ),
            section!(
                accordion_dynamic,
                "accordion-dynamic",
                "Keyed sections",
                "items(source, factory) reconciles stable keys with at most 256 live items. Factories run for new keys; retained keys keep state and physical identity. This example adds an account section without rebuilding the existing questions. heading_level and regions configure semantic headings and optional panel regions."
            ),
        ],
        "tabs" => vec![
            section!(
                tabs_variants,
                "tabs-variants",
                "Default, outline and pills",
                "TabsVariant configures the shared static/keyed skin. Default has a baseline and selected indicator; Outline uses bordered triggers; Pills uses a filled selection. grow(true) distributes horizontal triggers across the available width. Each trigger remains a real tab with selected state and roving focus."
            ),
            section!(
                tabs_orientation,
                "tabs-orientation",
                "Vertical and RTL navigation",
                "orientation changes both physical layout and keyboard navigation. Vertical tabs place panels beside the tab list and use Up/Down. direction(Rtl) reverses horizontal arrows. The link between each tab and its panel stays stable."
            ),
            section!(
                tabs_controlled,
                "tabs-controlled",
                "Controlled value and manual activation",
                "value/on_value_change proposes selection without writing to a controlled source. In Manual activation, arrows move focus and Enter/Space select. Automatic activation selects on focus. Home/End and loop_focus use enabled items; disabled triggers cannot activate. Program changes are silent."
            ),
            section!(
                tabs_content,
                "tabs-content",
                "Keep mounted and unmount",
                "KeepMounted is the default and preserves panel drafts. Unmount retires each inactive panel's child Owner. Select another tab and return to compare the two editors. Focus is restored to the selected available trigger when a panel containing actual focus is hidden."
            ),
            section!(
                tabs_dynamic,
                "tabs-dynamic",
                "Dynamic keyed tabs",
                "tabs(source, factory) reconciles at most 256 current keys. Reordering keeps retained tab and editor identities; removing the active local item selects a surviving enabled item. Controlled state follows the explicit missing-key contract. No visited-tab cache is retained."
            ),
        ],
        "timeline" => vec![
            section!(
                timeline_progress,
                "timeline-progress",
                "Reactive progress",
                "progress accepts Pending, Active(key) or Complete. Earlier items and their connecting lines become completed, while the active item is current. External updates preserve the list and its child state. Unknown active keys report a configuration error while retaining the accepted presentation."
            ),
            section!(
                timeline_markers,
                "timeline-markers",
                "Custom markers, bullet size and line width",
                "marker mounts passive or ordinary Widget content in a centered node. bullet_size accepts 8–64 CSS pixels, line_width accepts 0–8, and both reject non-finite values before mounting. Default nodes and connectors require no custom stylesheet; the final item has no trailing connector."
            ),
            section!(
                timeline_semantics,
                "timeline-semantics",
                "Headings, descriptions and state labels",
                "The timeline retains ordered-list and list-item semantics while drawing its own nodes. heading_level sets title semantics, description and timestamp add supporting content, and state_labels localizes completed/current/upcoming text. Displayed progress never implies an interactive selection."
            ),
        ],
        _ => Vec::new(),
    }
}

pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "accordion" => vec![
            ("Variants", "#accordion-variants"),
            ("Controlled state", "#accordion-controlled"),
            ("Content lifetime", "#accordion-content"),
            ("Keyed sections", "#accordion-dynamic"),
        ],
        "tabs" => vec![
            ("Variants", "#tabs-variants"),
            ("Orientation", "#tabs-orientation"),
            ("Controlled selection", "#tabs-controlled"),
            ("Content lifetime", "#tabs-content"),
            ("Keyed tabs", "#tabs-dynamic"),
        ],
        "timeline" => vec![
            ("Progress", "#timeline-progress"),
            ("Markers", "#timeline-markers"),
            ("Semantics", "#timeline-semantics"),
        ],
        _ => Vec::new(),
    }
}
