//! Widgets mounted directly by the Rust documentation site, one demo at a time.
use glory::prelude::*;

pub fn demo(id: &str, theme: sapid::Theme) -> Box<dyn Widget> {
    let selected = crate::showcase_catalog::find(id).expect("catalogued widget");
    Box::new(view().class("site-example").fill((selected.build)(theme)))
}

pub struct DetailSection {
    pub id: &'static str,
    pub title: &'static str,
    pub note: &'static str,
    pub example: Box<dyn Widget>,
    pub source: &'static str,
}

fn time_input_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    super::showcase_dates_docs::sections("time-input", theme)
}
fn time_picker_sections(theme: sapid::Theme) -> Vec<DetailSection> {
    super::showcase_dates_docs::sections("time-picker", theme)
}
pub fn detail_sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match id {
        "code-highlight"
        | "json-viewer"
        | "notifications-system"
        | "spotlight"
        | "lightbox"
        | "navigation-progress"
        | "modals-manager" => super::showcase_extension_docs::sections(id, theme),
        "schedule"
        | "day-view"
        | "week-view"
        | "month-view"
        | "year-view"
        | "mobile-month-view"
        | "agenda-view"
        | "resources-day-view"
        | "resources-week-view"
        | "resources-month-view"
        | "resources-schedule"
        | "recurring-events"
        | "events-data" => super::showcase_schedule_docs::sections(id, theme),
        "area-chart" | "bar-chart" | "line-chart" | "composite-chart" | "candlestick-chart"
        | "donut-chart" | "pie-chart" | "funnel-chart" | "radar-chart" | "scatter-chart"
        | "bubble-chart" | "radial-bar-chart" | "sparkline" | "bars-list" | "bullet-chart"
        | "gauge-chart" | "waffle-chart" | "treemap" | "sunburst-chart" | "sankey-chart"
        | "matrix-chart" => super::showcase_chart_docs::sections(id, theme),
        "date-input"
        | "month-picker"
        | "year-picker"
        | "time-value"
        | "calendar"
        | "inline-date-time-picker" => super::showcase_dates_docs::sections(id, theme),
        "box" | "collapse" | "divider" | "focus-trap" | "marquee" | "paper" | "portal"
        | "scroll-area" | "toolbar" | "visually-hidden" => {
            super::showcase_misc_docs::sections(id, theme)
        }
        "action-icon" | "button" | "close-button" | "copy-button" | "file-button" | "toggle"
        | "unstyled-button" => super::showcase_button_docs::sections(id, theme),
        "autocomplete" | "combobox" | "combobox-popover" | "multi-select" | "pill"
        | "pills-input" | "tags-input" | "tree-select" | "cascader" => {
            super::showcase_combobox_docs::sections(id, theme)
        }
        "input" | "text-input" | "textarea" => super::showcase_text_docs::sections(id, theme),
        "json-input" | "number-input" | "mask-input" | "native-select" | "file-input" => {
            super::showcase_data_input_docs::sections(id, theme)
        }
        "pin-input" => super::showcase_pin_docs::sections(theme),
        "scroller" => super::showcase_scroller_docs::sections(theme),
        "rich-text-editor" => super::showcase_editor_docs::sections(theme),
        "affix" | "background-image" | "time-grid" => {
            super::showcase_geometry_docs::sections(id, theme)
        }
        "accordion" | "tabs" | "timeline" => super::showcase_composite_docs::sections(id, theme),
        "anchor" | "burger" | "nav-link" | "pagination" | "breadcrumbs" => {
            super::showcase_navigation_docs::sections(id, theme)
        }
        "menu" => super::showcase_menu_docs::sections(theme),
        "mini-calendar" => super::showcase_mini_calendar_docs::sections(theme),
        "heatmap" => super::showcase_heatmap_docs::sections(theme),
        "transition" => super::showcase_transition_docs::sections(theme),
        "password-input" => super::showcase_password_docs::sections(theme),
        "menubar" => super::showcase_menubar::sections(theme),
        "aspect-ratio" | "center" | "container" | "flex" | "grid" | "group" | "simple-grid"
        | "space" | "stack" | "splitter" => super::showcase_layout_docs::sections(id, theme),
        "app-shell" => super::showcase_shell::sections(theme),
        "checkbox" | "radio" | "chip" | "fieldset" | "rating" | "segmented-control" | "switch" => {
            super::showcase_choice_docs::sections(id, theme)
        }
        "color-input" => super::showcase_color_input_docs::sections(theme),
        "alpha-slider" | "hue-slider" | "angle-slider" | "color-picker" => {
            super::showcase_color_docs::sections(id, theme)
        }
        "modal" | "drawer" | "popover" | "dialog" | "loading-overlay" | "overlay" | "tooltip"
        | "hover-card" | "action-bar" | "floating-indicator" | "tour" => {
            super::showcase_overlay_docs::sections(id, theme)
        }
        "slider" => super::showcase_slider_docs::sections(theme),
        "range-slider" => super::showcase_range_slider_docs::sections(theme),
        "avatar" | "badge" | "card" | "color-swatch" | "data-list" | "image" | "indicator"
        | "kbd" | "number-formatter" | "overflow-list" | "rolling-number" | "spoiler"
        | "theme-icon" => super::showcase_data_display_docs::sections(id, theme),
        "alert"
        | "empty-state"
        | "loader"
        | "notification"
        | "skeleton"
        | "progress"
        | "ring-progress"
        | "semi-circle-progress" => super::showcase_feedback_docs::sections(id, theme),
        "select" => super::showcase_select_docs::sections(theme),
        "floating-window" => super::showcase_window_docs::sections(theme),
        "carousel" => super::showcase_carousel_docs::sections(theme),
        "dropzone" => super::showcase_dropzone_docs::sections(theme),
        "blockquote" | "code" | "highlight" | "list" | "mark" | "table" | "text" | "title"
        | "typography" => super::showcase_typography_docs::sections(id, theme),
        "time-input" => time_input_sections(theme),
        "date-picker" => {
            let mut sections = super::showcase_date_picker_docs::sections(theme.clone());
            sections.extend(super::showcase_dates_docs::sections(id, theme));
            sections
        }
        "date-picker-input" => super::showcase_date_picker_docs::sections(theme),
        "month-picker-input" | "year-picker-input" => {
            super::showcase_period_input_docs::sections(id, theme)
        }
        "time-picker" => time_picker_sections(theme),
        "date-time-picker" => super::showcase_date_time_picker_docs::sections(theme),
        "stepper" | "table-of-contents" | "tree" => {
            super::showcase_navigation_extra_docs::sections(id, theme)
        }
        _ => Vec::new(),
    }
}

pub fn detail_navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "code-highlight"
        | "json-viewer"
        | "notifications-system"
        | "spotlight"
        | "lightbox"
        | "navigation-progress"
        | "modals-manager" => super::showcase_extension_docs::navigation(id),
        "schedule"
        | "day-view"
        | "week-view"
        | "month-view"
        | "year-view"
        | "mobile-month-view"
        | "agenda-view"
        | "resources-day-view"
        | "resources-week-view"
        | "resources-month-view"
        | "resources-schedule"
        | "recurring-events"
        | "events-data" => super::showcase_schedule_docs::navigation(id),
        "area-chart" | "bar-chart" | "line-chart" | "composite-chart" | "candlestick-chart"
        | "donut-chart" | "pie-chart" | "funnel-chart" | "radar-chart" | "scatter-chart"
        | "bubble-chart" | "radial-bar-chart" | "sparkline" | "bars-list" | "bullet-chart"
        | "gauge-chart" | "waffle-chart" | "treemap" | "sunburst-chart" | "sankey-chart"
        | "matrix-chart" => super::showcase_chart_docs::contents(id),
        "date-picker" => {
            let mut navigation = detail_navigation("date-picker-input");
            navigation.extend(super::showcase_dates_docs::navigation(id));
            navigation
        }
        "date-input"
        | "month-picker"
        | "year-picker"
        | "time-value"
        | "calendar"
        | "inline-date-time-picker" => super::showcase_dates_docs::navigation(id),

        "time-input" | "time-picker" => super::showcase_dates_docs::navigation(id),
        "box" | "collapse" | "divider" | "focus-trap" | "marquee" | "paper" | "portal"
        | "scroll-area" | "toolbar" | "visually-hidden" => {
            super::showcase_misc_docs::navigation(id)
        }
        "blockquote" | "code" | "highlight" | "list" | "mark" | "table" | "text" | "title"
        | "typography" => super::showcase_typography_docs::navigation(id),
        "avatar" | "badge" | "card" | "color-swatch" | "data-list" | "image" | "indicator"
        | "kbd" | "number-formatter" | "overflow-list" | "rolling-number" | "spoiler"
        | "theme-icon" => super::showcase_data_display_docs::navigation(id),
        "alert" | "empty-state" | "loader" | "notification" | "skeleton" => {
            super::showcase_feedback_docs::navigation(id)
        }
        "stepper" | "table-of-contents" => super::showcase_navigation_extra_docs::navigation(id),
        "action-icon" | "button" | "close-button" | "copy-button" | "file-button" | "toggle"
        | "unstyled-button" => super::showcase_button_docs::navigation(id),
        "autocomplete" | "combobox" | "combobox-popover" | "multi-select" | "pill"
        | "pills-input" | "tags-input" | "tree-select" | "cascader" => {
            super::showcase_combobox_docs::navigation(id)
        }
        "input" | "text-input" | "textarea" => super::showcase_text_docs::navigation(id),
        "json-input" | "number-input" | "mask-input" | "native-select" | "file-input" => {
            super::showcase_data_input_docs::navigation(id)
        }
        "floating-window" => super::showcase_window_docs::navigation(),
        "carousel" => super::showcase_carousel_docs::navigation(),
        "dropzone" => super::showcase_dropzone_docs::navigation(),
        "month-picker-input" | "year-picker-input" => {
            super::showcase_period_input_docs::navigation(id)
        }
        "pin-input" => super::showcase_pin_docs::navigation(),
        "scroller" => super::showcase_scroller_docs::navigation(),
        "rich-text-editor" => super::showcase_editor_docs::navigation(),
        "affix" | "background-image" | "time-grid" => super::showcase_geometry_docs::navigation(id),
        "accordion" | "tabs" | "timeline" => super::showcase_composite_docs::navigation(id),
        "anchor" | "burger" | "nav-link" | "pagination" | "breadcrumbs" => {
            super::showcase_navigation_docs::navigation(id)
        }
        "menu" => super::showcase_menu_docs::navigation(),
        "mini-calendar" => super::showcase_mini_calendar_docs::navigation(),
        "heatmap" => super::showcase_heatmap_docs::navigation(),
        "transition" => super::showcase_transition_docs::navigation(),
        "password-input" => super::showcase_password_docs::navigation(),
        "menubar" => super::showcase_menubar::navigation(),
        "aspect-ratio" | "center" | "container" | "flex" | "grid" | "group" | "simple-grid"
        | "space" | "stack" | "splitter" => super::showcase_layout_docs::navigation(id),
        "app-shell" => vec![
            ("Layout regions", "#shell-regions"),
            ("Responsive navigation", "#shell-responsive"),
            ("Collapsed navigation", "#shell-collapsed"),
            ("Scrolling and sections", "#shell-scrolling"),
            ("Region configuration", "#shell-configuration"),
            ("Full-window frame", "#shell-viewport"),
        ],
        "date-picker-input" => vec![
            ("Actions and forms", "#date-picker-actions"),
            ("Multiple and range", "#date-picker-modes"),
            ("Number of columns", "#date-picker-columns"),
            ("Input columns", "#date-picker-input-columns"),
            ("Value format", "#date-picker-value-format"),
            ("Levels", "#date-picker-levels"),
            ("Week numbers", "#date-picker-calendar-rows"),
        ],
        "color-input" => vec![
            ("Controlled swatches only", "#swatches-only"),
            ("Controlled", "#controlled"),
            ("Loading state", "#loading"),
            ("Formats", "#formats"),
            ("Preserve invalid input", "#preserve-invalid-input"),
            ("Committed changes", "#change-end"),
            ("Disable free input", "#disable-free-input"),
            ("With swatches", "#with-swatches"),
            ("Close on swatch", "#close-dropdown-on-swatch-click"),
            ("Hide dropdown", "#hide-dropdown"),
            ("Eye dropper", "#eye-dropper"),
            ("Eye dropper icon", "#eye-dropper-icon"),
            ("Left/right sections", "#left-right-sections"),
            ("Error", "#error"),
            ("Success", "#success"),
            ("Disabled", "#disabled"),
            ("Read only", "#read-only"),
        ],
        "alpha-slider" | "hue-slider" | "angle-slider" | "color-picker" => {
            super::showcase_color_docs::navigation(id)
        }
        "checkbox" | "radio" | "chip" | "fieldset" | "rating" | "segmented-control" | "switch" => {
            super::showcase_choice_docs::navigation(id)
        }
        "modal" | "drawer" | "popover" | "dialog" | "loading-overlay" | "overlay" | "tooltip"
        | "hover-card" | "action-bar" | "floating-indicator" | "tour" => {
            super::showcase_overlay_docs::navigation(id)
        }
        "slider" => vec![
            ("States and live steps", "#slider-states"),
            ("Controlled value", "#controlled"),
            ("Committed changes", "#change-end"),
            ("Bounds and step", "#bounds-and-step"),
            ("Marks", "#marks"),
        ],
        "range-slider" => vec![
            ("States and live steps", "#range-states"),
            ("Controlled range", "#controlled"),
            ("Bounds and gap", "#range-gap"),
            ("Marks", "#range-marks"),
            ("Maximum and inverted", "#range-advanced"),
            ("Domain", "#range-domain"),
            ("Control labels", "#range-labels"),
            ("Collision behavior", "#range-collision"),
        ],
        "progress" | "ring-progress" | "semi-circle-progress" => vec![
            ("Geometry", "#progress-geometry"),
            ("Compound sections", "#progress-sections"),
            ("Vertical orientation", "#progress-vertical"),
            ("Semicircle direction", "#progress-semicircle-direction"),
            ("Semicircle labels", "#progress-semicircle-labels"),
            ("Ring gaps", "#progress-ring-gaps"),
            ("Ring angles", "#progress-ring-angles"),
            ("Ring caps and label", "#progress-ring-caps-label"),
            ("States and values", "#progress-states"),
        ],
        "select" => super::showcase_select_docs::navigation(),
        "date-time-picker" => vec![
            ("Actions and forms", "#date-time-picker-actions"),
            ("Usage", "#date-time-picker-usage"),
            ("Seconds", "#date-time-picker-seconds"),
            ("Presets", "#date-time-picker-presets"),
            ("TimePicker configuration", "#date-time-picker-time-options"),
            ("Controlled and clearable", "#date-time-picker-controlled"),
            ("Inline picker", "#date-time-picker-inline"),
            ("Calendar controls", "#date-time-picker-calendar-controls"),
            ("Date and time range", "#date-time-picker-range"),
            ("Inline range", "#date-time-picker-inline-range"),
            ("States", "#date-time-picker-states"),
        ],
        "tree" => vec![
            ("Controlled state", "#tree-controlled"),
            ("Expand all", "#tree-expansion"),
            ("Cascading checks", "#tree-checked"),
            ("Independent checks", "#tree-strict"),
            ("Lazy loading", "#tree-lazy"),
            ("Search", "#tree-filter"),
            ("Drag and drop", "#tree-drag-drop"),
            ("Custom labels", "#tree-render-label"),
        ],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glory::render::{Event, Props};

    #[test]
    fn extension_documentation_mounts_each_complete_source_and_retires() {
        for (slug, count) in [
            ("code-highlight", 5),
            ("json-viewer", 4),
            ("notifications-system", 5),
            ("spotlight", 4),
            ("lightbox", 4),
            ("navigation-progress", 4),
            ("modals-manager", 4),
        ] {
            let sections = detail_sections(slug, sapid::Theme::blue_light());
            assert_eq!(sections.len(), count);
            assert_eq!(detail_navigation(slug).len(), count);
            for section in sections {
                assert!(section.source.contains("pub fn demo("));
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|error| panic!("{}: {error}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
            }
        }
    }
    #[test]
    fn schedule_documentation_mounts_each_complete_source_and_retires_its_graph() {
        for (slug, count) in [
            ("schedule", 5),
            ("day-view", 5),
            ("week-view", 5),
            ("month-view", 5),
            ("year-view", 5),
            ("mobile-month-view", 5),
            ("agenda-view", 5),
            ("resources-day-view", 5),
            ("resources-week-view", 5),
            ("resources-month-view", 5),
            ("resources-schedule", 5),
            ("recurring-events", 4),
            ("events-data", 3),
        ] {
            let sections = detail_sections(slug, sapid::Theme::blue_light());
            assert_eq!(sections.len(), count);
            assert_eq!(detail_navigation(slug).len(), count);
            for section in sections {
                assert!(section.source.contains("pub fn demo("));
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|error| panic!("{}: {error}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
            }
        }
    }
    #[test]
    fn chart_documentation_mounts_each_full_source_example_and_retires_its_owned_graph() {
        for (slug, count) in [
            ("area-chart", 6),
            ("bar-chart", 6),
            ("line-chart", 5),
            ("composite-chart", 5),
            ("candlestick-chart", 4),
            ("donut-chart", 4),
            ("pie-chart", 4),
            ("funnel-chart", 4),
            ("radar-chart", 4),
            ("scatter-chart", 4),
            ("bubble-chart", 4),
            ("radial-bar-chart", 4),
            ("sparkline", 4),
            ("bars-list", 4),
            ("bullet-chart", 4),
            ("gauge-chart", 4),
            ("waffle-chart", 4),
            ("treemap", 4),
            ("sunburst-chart", 4),
            ("sankey-chart", 4),
            ("matrix-chart", 4),
        ] {
            let sections = detail_sections(slug, sapid::Theme::blue_light());
            assert_eq!(sections.len(), count);
            assert_eq!(detail_navigation(slug).len(), count);
            for section in sections {
                assert!(section.source.contains("pub fn demo("));
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
            }
        }
    }
    #[test]
    fn shell_and_menubar_documentation_sections_mount_real_widgets_and_retire() {
        for (slug, count) in [
            ("app-shell", 6),
            ("aspect-ratio", 2),
            ("center", 1),
            ("container", 1),
            ("flex", 2),
            ("grid", 2),
            ("group", 2),
            ("simple-grid", 2),
            ("space", 1),
            ("stack", 1),
            ("splitter", 3),
            ("menubar", 9),
            ("menu", 6),
            ("mini-calendar", 4),
            ("heatmap", 4),
            ("transition", 5),
            ("password-input", 5),
            ("alpha-slider", 2),
            ("hue-slider", 2),
            ("angle-slider", 2),
            ("checkbox", 3),
            ("radio", 3),
            ("chip", 2),
            ("fieldset", 2),
            ("rating", 3),
            ("segmented-control", 3),
            ("switch", 4),
            ("input", 4),
            ("text-input", 4),
            ("textarea", 3),
            ("json-input", 4),
            ("number-input", 5),
            ("mask-input", 4),
            ("native-select", 4),
            ("file-input", 4),
            ("slider", 5),
            ("range-slider", 8),
            ("anchor", 2),
            ("burger", 1),
            ("nav-link", 2),
            ("breadcrumbs", 2),
            ("pagination", 3),
            ("accordion", 4),
            ("tabs", 5),
            ("timeline", 3),
            ("pin-input", 3),
            ("scroller", 3),
            ("rich-text-editor", 4),
            ("affix", 3),
            ("background-image", 2),
            ("time-grid", 3),
            ("select", 6),
            ("month-picker-input", 4),
            ("year-picker-input", 4),
            ("floating-window", 4),
            ("carousel", 4),
            ("dropzone", 4),
        ] {
            let sections = detail_sections(slug, sapid::Theme::blue_light());
            assert_eq!(sections.len(), count);
            assert_eq!(detail_navigation(slug).len(), count);
            for section in sections {
                assert!(section.source.contains("pub fn demo("));
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|error| panic!("{}: {error}", section.id));
                assert!(!app.tree().is_empty(), "{}", section.id);
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0, "{}", section.id);
            }
        }
    }

    #[test]
    fn clean_examples_build_real_widgets_and_retire_their_owned_state() {
        for id in [
            "button",
            "text-input",
            "checkbox",
            "collapse",
            "accordion",
            "tabs",
            "modal",
            "drawer",
            "combobox-popover",
        ] {
            let app = App::memory();
            app.mount(demo(id, sapid::Theme::blue_light())).unwrap();
            assert!(!app.tree().is_empty(), "empty demo: {id}");
            if id == "button" {
                let mut pending = app.tree().roots().to_vec();
                let mut button = None;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    if matches!(node.descriptor.view().props, Props::Button(p) if p.label == "Filled button")
                    {
                        button = Some(id);
                    }
                    pending.extend(node.children.iter().copied());
                }
                app.dispatch(button.unwrap(), Event::Activate).unwrap();
                let mut pending = app.tree().roots().to_vec();
                let mut updated = false;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    updated |= matches!(node.descriptor.view().props, Props::Text(p) if p.content == "Button presses: 1");
                    pending.extend(node.children.iter().copied());
                }
                assert!(
                    updated,
                    "the displayed counter must respond to native activation"
                );
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired demo: {id}");
            assert_eq!(app.runtime_stats().tasks, 0, "retired tasks: {id}");
        }
    }

    #[test]
    fn color_input_documentation_builds_every_interactive_detail_section() {
        let sections = detail_sections("color-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "swatches-only",
                "controlled",
                "loading",
                "formats",
                "preserve-invalid-input",
                "change-end",
                "disable-free-input",
                "with-swatches",
                "close-dropdown-on-swatch-click",
                "hide-dropdown",
                "eye-dropper",
                "eye-dropper-icon",
                "left-right-sections",
                "error",
                "success",
                "disabled",
                "read-only",
            ]
        );
        for section in sections {
            assert!(section.source.contains("pub fn demo("), "{}", section.id);
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount ColorInput section {}: {error}", section.id)
            });
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            if section.id == "formats" {
                let mut pending = app.tree().roots().to_vec();
                let mut inputs = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    inputs += usize::from(matches!(node.descriptor.view().props, Props::Input(_)));
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(inputs, 6, "every documented color format must render");
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn color_picker_documentation_builds_swatches_and_precision_sections() {
        let sections = detail_sections("color-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "picker-swatches",
                "picker-swatches-only",
                "picker-precision",
                "picker-form-value"
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!(
                    "failed to mount ColorPicker section {}: {error}",
                    section.id
                )
            });
            let mut pending = app.tree().roots().to_vec();
            let mut swatches = 0;
            let mut planes = 0;
            while let Some(id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(id).unwrap();
                swatches += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-color-picker-swatch"),
                );
                planes += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-color-plane"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "picker-swatches-only" {
                assert_eq!(swatches, 7);
                assert_eq!(planes, 0);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn modal_documentation_builds_centered_and_full_screen_layers() {
        let sections = detail_sections("modal", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "modal-centered",
                "modal-full-screen",
                "modal-sizes",
                "modal-scroll",
                "modal-controlled",
                "modal-nested"
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut full_screen = 0;
            while let Some(id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(id).unwrap();
                full_screen += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-dialog-full-screen"),
                );
                pending.extend(node.children.iter().copied());
            }
            assert_eq!(full_screen, usize::from(section.id == "modal-full-screen"));
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn slider_documentation_builds_feature_sections_and_real_marks() {
        let sections = detail_sections("slider", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "slider-states",
                "controlled",
                "change-end",
                "bounds-and-step",
                "marks"
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            if section.id == "marks" {
                let mut pending = app.tree().roots().to_vec();
                let mut marks = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    marks += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-slider-mark"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(marks, 8);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn range_slider_documentation_builds_atomic_range_and_marks_sections() {
        let sections = detail_sections("range-slider", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "range-states",
                "controlled",
                "range-gap",
                "range-marks",
                "range-advanced",
                "range-domain",
                "range-labels",
                "range-collision",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            if matches!(
                section.id,
                "range-marks" | "range-advanced" | "range-domain"
            ) {
                let mut pending = app.tree().roots().to_vec();
                let mut marks = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    marks += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-range-slider-mark"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(
                    marks,
                    match section.id {
                        "range-marks" => 5,
                        "range-advanced" => 3,
                        "range-domain" => 2,
                        _ => unreachable!(),
                    }
                );
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn progress_documentation_builds_geometry_sections_and_accessibility_states() {
        for id in ["progress", "ring-progress", "semi-circle-progress"] {
            let sections = detail_sections(id, sapid::Theme::blue_light());
            assert_eq!(
                sections
                    .iter()
                    .map(|section| section.id)
                    .collect::<Vec<_>>(),
                [
                    "progress-geometry",
                    "progress-sections",
                    "progress-vertical",
                    "progress-semicircle-direction",
                    "progress-semicircle-labels",
                    "progress-ring-gaps",
                    "progress-ring-angles",
                    "progress-ring-caps-label",
                    "progress-states",
                ]
            );
            for section in sections {
                let app = App::memory();
                app.mount(section.example).unwrap();
                if section.id == "progress-sections" {
                    let mut pending = app.tree().roots().to_vec();
                    let mut parts = 0;
                    while let Some(node_id) = pending.pop() {
                        let tree = app.tree();
                        let node = tree.node(node_id).unwrap();
                        parts += usize::from(
                            node.descriptor
                                .view()
                                .classes
                                .iter()
                                .any(|class| class == "sapid-progress-section"),
                        );
                        pending.extend(node.children.iter().copied());
                    }
                    assert_eq!(parts, 6);
                }
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
            }
        }
    }

    #[test]
    fn autocomplete_documentation_builds_all_behavior_sections_and_limits_results() {
        let sections = detail_sections("autocomplete", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "autocomplete-controlled",
                "autocomplete-loading",
                "autocomplete-first",
                "autocomplete-blur",
                "autocomplete-data",
                "autocomplete-limit",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut visible_options = 0;
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                visible_options += usize::from(
                    node.descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .is_some_and(|value| value.option.is_some() && !value.hidden),
                );
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-autocomplete-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-autocomplete-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "autocomplete-limit" {
                assert_eq!(visible_options, 5);
            }
            if section.id == "autocomplete-data" {
                assert_eq!(clear_buttons, 1);
            }
            if section.id == "autocomplete-loading" {
                assert_eq!(loading_indicators, 2);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn select_documentation_builds_controlled_loading_content_geometry_and_search_composition() {
        let sections = detail_sections("select", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "select-controlled",
                "select-loading",
                "select-content",
                "select-geometry",
                "select-searchable",
                "select-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount Select section {}: {error}", section.id)
            });
            if section.id == "select-searchable" {
                let mut pending = app.tree().roots().to_vec();
                let mut trigger = None;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    if node
                        .descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .is_some_and(|value| value.combobox.is_some())
                    {
                        trigger = Some(id);
                        break;
                    }
                    pending.extend(node.children.iter().copied());
                }
                let from = trigger.expect("search editor");
                app.dispatch(
                    from,
                    Event::Choice {
                        from,
                        command: glory::render::ChoiceCommand::OpenFirst,
                    },
                )
                .unwrap();
            }
            let mut pending = app.tree().roots().to_vec();
            let mut loading = 0;
            let mut descriptions = 0;
            let mut checks = 0;
            let mut visible_options = 0;
            let mut listbox_heights = Vec::new();
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                let view = node.descriptor.view();
                loading += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-loading"),
                );
                descriptions += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-option-description"),
                );
                checks += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-select-option-check"),
                );
                visible_options += usize::from(
                    view.semantics
                        .as_ref()
                        .is_some_and(|value| value.option.is_some() && !value.hidden),
                );
                if view.role == Some(glory::render::Role::Listbox) {
                    listbox_heights.push(view.style.max_height);
                }
                pending.extend(node.children.iter().copied());
            }
            match section.id {
                "select-loading" => assert_eq!(loading, 1),
                "select-content" => {
                    assert_eq!(descriptions, 2);
                    assert_eq!(checks, 2);
                }
                "select-geometry" => {
                    assert!(listbox_heights.contains(&Some(glory::style::Limit::Px(180.0))))
                }
                "select-searchable" => assert_eq!(visible_options, 2),
                _ => {}
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn tree_documentation_builds_controlled_checked_lazy_filter_drag_and_custom_label_sections() {
        let sections = detail_sections("tree", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "tree-controlled",
                "tree-expansion",
                "tree-checked",
                "tree-strict",
                "tree-lazy",
                "tree-filter",
                "tree-drag-drop",
                "tree-render-label",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount Tree section {}: {error}", section.id)
            });
            let mut pending = app.tree().roots().to_vec();
            let mut tree_items = 0;
            let mut checked_items = 0;
            let mut draggable_items = 0;
            let mut custom_badges = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                let view = node.descriptor.view();
                tree_items += usize::from(view.role == Some(glory::render::Role::TreeItem));
                checked_items += usize::from(
                    view.semantics
                        .as_ref()
                        .is_some_and(|value| value.tree_checked.is_some()),
                );
                draggable_items += usize::from(
                    view.classes
                        .iter()
                        .any(|class| class == "sapid-tree-draggable"),
                );
                custom_badges +=
                    usize::from(view.classes.iter().any(|class| class == "sapid-badge"));
                pending.extend(node.children.iter().copied());
            }
            assert!(tree_items > 0, "empty Tree section: {}", section.id);
            if matches!(section.id, "tree-checked" | "tree-strict") {
                assert_eq!(checked_items, tree_items);
            }
            if section.id == "tree-drag-drop" {
                assert_eq!(draggable_items, tree_items);
            }
            if section.id == "tree-render-label" {
                assert_eq!(custom_badges, tree_items);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn multi_select_documentation_builds_controlled_loading_and_data_sections() {
        let sections = detail_sections("multi-select", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "multi-select-controlled",
                "multi-select-loading",
                "multi-select-data",
                "multi-select-wrapping",
                "multi-select-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-multi-select-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-combobox-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "multi-select-loading" {
                assert_eq!(loading_indicators, 2);
            }
            assert_eq!(
                clear_buttons,
                match section.id {
                    "multi-select-loading" => 2,
                    "multi-select-states" => 5,
                    _ => 1,
                }
            );
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn tags_input_documentation_builds_controlled_loading_clear_blur_and_parsing_sections() {
        let sections = detail_sections("tags-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "tags-controlled",
                "tags-loading",
                "tags-limits-clear",
                "tags-blur",
                "tags-parsing",
                "tags-composition",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut clear_buttons = 0;
            let mut loading_indicators = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                clear_buttons += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-tags-input-clear"),
                );
                loading_indicators += usize::from(
                    node.descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-tags-input-loading"),
                );
                pending.extend(node.children.iter().copied());
            }
            if section.id == "tags-loading" {
                assert_eq!(loading_indicators, 2);
            }
            if section.id == "tags-limits-clear" {
                assert_eq!(clear_buttons, 1);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
        assert_eq!(
            detail_sections("pills-input", sapid::Theme::blue_light())
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "pills-controlled",
                "pills-loading",
                "pills-policies",
                "pills-wrapping",
                "pills-states"
            ]
        );
    }

    #[test]
    fn time_input_documentation_builds_usage_controlled_format_placeholder_and_state_sections() {
        let sections = detail_sections("time-input", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "time-input-usage",
                "time-input-controlled",
                "time-input-precision",
                "time-input-placeholder",
                "time-input-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let mut pending = app.tree().roots().to_vec();
            let mut inputs = 0;
            let mut placeholders = 0;
            while let Some(node_id) = pending.pop() {
                let tree = app.tree();
                let node = tree.node(node_id).unwrap();
                inputs += usize::from(matches!(node.descriptor.view().props, Props::Input(_)));
                placeholders += usize::from(
                    node.descriptor
                        .view()
                        .semantics
                        .as_ref()
                        .and_then(|value| value.placeholder.as_ref())
                        .is_some(),
                );
                pending.extend(node.children.iter().copied());
            }
            assert!(inputs > 0);
            if matches!(section.id, "time-input-usage" | "time-input-placeholder") {
                assert!(placeholders > 0);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn time_picker_documentation_builds_input_grid_preset_popup_and_state_sections() {
        let sections = detail_sections("time-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "time-picker-usage",
                "time-picker-controlled",
                "time-picker-format",
                "time-picker-duration",
                "time-picker-duration-digits",
                "time-picker-paste",
                "time-picker-dropdown",
                "time-picker-steps",
                "time-picker-grid",
                "time-picker-presets",
                "time-picker-grouped-presets",
                "time-picker-controlled-popup",
                "time-picker-labels-focus",
                "time-picker-popup-layout",
                "time-picker-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap_or_else(|error| {
                panic!("failed to mount TimePicker section {}: {error}", section.id)
            });
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            if section.id == "time-picker-controlled" {
                let mut pending = app.tree().roots().to_vec();
                let mut clear = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    clear += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-time-picker-clear"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(clear, 1);
            }
            if matches!(
                section.id,
                "time-picker-presets"
                    | "time-picker-grouped-presets"
                    | "time-picker-controlled-popup"
                    | "time-picker-popup-layout"
            ) {
                let mut pending = app.tree().roots().to_vec();
                let mut trigger = 0;
                while let Some(id) = pending.pop() {
                    let tree = app.tree();
                    let node = tree.node(id).unwrap();
                    trigger += usize::from(
                        node.descriptor
                            .view()
                            .classes
                            .iter()
                            .any(|class| class == "sapid-time-picker-preset-trigger"),
                    );
                    pending.extend(node.children.iter().copied());
                }
                assert_eq!(trigger, 1);
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0, "retired section: {}", section.id);
        }
    }

    #[test]
    fn date_picker_documentation_builds_modes_columns_and_input_columns() {
        let sections = detail_sections("date-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "date-picker-actions",
                "date-picker-modes",
                "date-picker-columns",
                "date-picker-input-columns",
                "date-picker-value-format",
                "date-picker-levels",
                "date-picker-calendar-rows",
                "dates-locale",
                "dates-constraints",
                "dates-controlled",
                "dates-keyboard",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example).unwrap();
            let grid_count = app
                .tree()
                .roots()
                .iter()
                .copied()
                .flat_map(|root| {
                    let mut nodes = vec![root];
                    let mut output = Vec::new();
                    while let Some(id) = nodes.pop() {
                        let tree = app.tree();
                        let node = tree.node(id).unwrap();
                        nodes.extend(node.children.iter().copied());
                        output.push(id);
                    }
                    output
                })
                .filter(|id| {
                    app.tree()
                        .node(*id)
                        .unwrap()
                        .descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "sapid-calendar-grid")
                })
                .count();
            match section.id {
                "date-picker-actions" | "dates-constraints" | "dates-controlled" => {
                    assert_eq!(grid_count, 1)
                }
                "dates-locale" => assert_eq!(grid_count, 2),
                "dates-keyboard" => assert_eq!(grid_count, 3),
                _ => assert!(grid_count >= 2),
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }

    #[test]
    fn date_time_picker_documentation_builds_all_official_behavior_sections() {
        let sections = detail_sections("date-time-picker", sapid::Theme::blue_light());
        assert_eq!(
            sections
                .iter()
                .map(|section| section.id)
                .collect::<Vec<_>>(),
            [
                "date-time-picker-actions",
                "date-time-picker-usage",
                "date-time-picker-seconds",
                "date-time-picker-presets",
                "date-time-picker-time-options",
                "date-time-picker-controlled",
                "date-time-picker-inline",
                "date-time-picker-calendar-controls",
                "date-time-picker-range",
                "date-time-picker-inline-range",
                "date-time-picker-states",
            ]
        );
        for section in sections {
            let app = App::memory();
            app.mount(section.example)
                .unwrap_or_else(|error| panic!("failed to mount {}: {error}", section.id));
            assert!(!app.tree().is_empty(), "empty section: {}", section.id);
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}
