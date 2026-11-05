use super::showcase_examples::DetailSection;
mod code_highlight_files;
mod code_highlight_languages;
mod code_highlight_lines;
mod code_highlight_long_lines;
mod code_highlight_theme;
mod json_viewer_copy;
mod json_viewer_expansion;
mod json_viewer_paths;
mod json_viewer_updates;
mod lightbox_controlled;
mod lightbox_metadata;
mod lightbox_navigation;
mod lightbox_zoom;
mod modals_manager_closing;
mod modals_manager_focus;
mod modals_manager_metadata;
mod modals_manager_queue;
mod navigation_progress_cancel;
mod navigation_progress_concurrency;
mod navigation_progress_finish;
mod navigation_progress_replacement;
mod notifications_system_loading;
mod notifications_system_position;
mod notifications_system_stack;
mod notifications_system_timeouts;
mod notifications_system_updates;
mod spotlight_controlled;
mod spotlight_groups;
mod spotlight_search;
mod spotlight_shortcut;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "code-highlight" => vec![
            DetailSection {
                id: "extensions-code-highlight-languages",
                title: "Rust, JSON and plain text",
                note: "These are bounded lexical highlighters. A language change preserves the exact source; highlighting neither compiles Rust nor validates JSON.",
                example: Box::new(code_highlight_languages::demo(theme.clone())),
                source: include_str!("code_highlight_languages.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-theme",
                title: "Theme and unstyled ink",
                note: "The active page theme supplies syntax colors; unstyled removes ink while retaining selectable whitespace and scrolling.",
                example: Box::new(code_highlight_theme::demo(theme.clone())),
                source: include_str!("code_highlight_theme.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-lines",
                title: "Line numbers and highlighted lines",
                note: "Numbers are a separate inert gutter. start_line changes displayed numbers; highlighted_lines uses relative one-based lines.",
                example: Box::new(code_highlight_lines::demo(theme.clone())),
                source: include_str!("code_highlight_lines.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-long_lines",
                title: "Long lines stay inside the code viewport",
                note: "A long literal scrolls locally. The page and neighboring content keep their width; the complete source remains selectable.",
                example: Box::new(code_highlight_long_lines::demo(theme.clone())),
                source: include_str!("code_highlight_long_lines.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-files",
                title: "Multiple files and copy feedback",
                note: "The application owns the current file; changing it replaces only the accepted source. Copy feedback is beside its action and never claims success before the host result.",
                example: Box::new(code_highlight_files::demo(theme.clone())),
                source: include_str!("code_highlight_files.rs"),
            },
        ],
        "json-viewer" => vec![
            DetailSection {
                id: "extensions-json-viewer-expansion",
                title: "Controlled expansion and refusal",
                note: "Expanded JSON pointers are read-only input. A callback proposes the next paths and the application may refuse them.",
                example: Box::new(json_viewer_expansion::demo(theme.clone())),
                source: include_str!("json_viewer_expansion.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-paths",
                title: "JSON pointers, escaping and keyboard navigation",
                note: "A slash is ~1 and a tilde is ~0 in JSON pointers. Tree mode uses real TreeItem keyboard navigation; the displayed values remain ordinary text.",
                example: Box::new(json_viewer_paths::demo(theme.clone())),
                source: include_str!("json_viewer_paths.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-copy",
                title: "Copy the complete JSON source",
                note: "Collapsed branches do not truncate copying. Feedback is a real host result; the browser acceptance injects an isolated clipboard provider.",
                example: Box::new(json_viewer_copy::demo(theme.clone())),
                source: include_str!("json_viewer_copy.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-updates",
                title: "Search and accepted data updates",
                note: "Publish a complete valid snapshot. Searching highlights current values; updating release preserves the current paths and mounted editor.",
                example: Box::new(json_viewer_updates::demo(theme.clone())),
                source: include_str!("json_viewer_updates.rs"),
            },
        ],
        "notifications-system" => vec![
            DetailSection {
                id: "extensions-notifications-system-stack",
                title: "Stacking and bounded queues",
                note: "The local queue admits eight messages and displays two. Dismissing a visible row promotes the next request; repeated keys replace the current lifetime.",
                example: Box::new(notifications_system_stack::demo(theme.clone())),
                source: include_str!("notifications_system_stack.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-updates",
                title: "Update, dismiss and application actions",
                note: "Updating a live token preserves its notification row. Dismiss retires the request; the Undo action executes an application command after removing its row.",
                example: Box::new(notifications_system_updates::demo(theme.clone())),
                source: include_str!("notifications_system_updates.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-position",
                title: "Viewport placement belongs to the application",
                note: "Affix positions this one local Toaster at the lower right. Ordinary layout remains the default; there is no global notifications singleton.",
                example: Box::new(notifications_system_position::demo(theme.clone())),
                source: include_str!("notifications_system_position.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-timeouts",
                title: "Timeout, pause and resume",
                note: "This example explicitly supplies a monotonic browser clock and cancellable timer. Hover, focus or Pause preserve remaining time; queued deadlines begin only after promotion. Cold SSR/native previews install a pending clock.",
                example: Box::new(notifications_system_timeouts::demo(theme.clone())),
                source: include_str!("notifications_system_timeouts.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-loading",
                title: "Inline loading is independent of the queue",
                note: "An inline Notification can show a busy operation and block dismissal until completion. Toaster owns requests and deadlines; its update command updates title/message, rather than claiming a loading API it does not expose.",
                example: Box::new(notifications_system_loading::demo(theme.clone())),
                source: include_str!("notifications_system_loading.rs"),
            },
        ],
        "spotlight" => vec![
            DetailSection {
                id: "extensions-spotlight-search",
                title: "Search, keywords and empty results",
                note: "Search labels, descriptions and keywords. Try fresh for Create note, then an unmatched query for the actual empty state. Arrow keys and Enter keep input focus until execution.",
                example: Box::new(spotlight_search::demo(theme.clone())),
                source: include_str!("spotlight_search.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-groups",
                title: "Grouped actions and disabled commands",
                note: "Workspace and Preferences are real result groups. Disabled Archive workspace is visible but cannot execute; descriptions explain each command.",
                example: Box::new(spotlight_groups::demo(theme.clone())),
                source: include_str!("spotlight_groups.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-shortcut",
                title: "Explicitly scoped Ctrl/Cmd+K",
                note: "Focus the Keyboard shortcut target and use Ctrl/Cmd+K. Registration belongs to this live physical ancestor; ordinary editors and composition keep their shortcut guards. No global listener is installed.",
                example: Box::new(spotlight_shortcut::demo(theme.clone())),
                source: include_str!("spotlight_shortcut.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-controlled",
                title: "Controlled opening and search refusal",
                note: "Open and search are separate typed proposals. Initially opening is refused; accepting changes permits the same retained palette to open and search.",
                example: Box::new(spotlight_controlled::demo(theme.clone())),
                source: include_str!("spotlight_controlled.rs"),
            },
        ],
        "lightbox" => vec![
            DetailSection {
                id: "extensions-lightbox-navigation",
                title: "Image navigation and boundaries",
                note: "Open either thumbnail, then use Previous/Next or focus Image navigation and press Arrow/Home/End. Boundaries disable the matching controls; image alternatives and captions remain separate.",
                example: Box::new(lightbox_navigation::demo(theme.clone())),
                source: include_str!("lightbox_navigation.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-zoom",
                title: "Zoom, fit and local scrolling",
                note: "Zoom increases the current image from 100% to 400% inside its viewport. Fit resets the current zoom; this is finite stepped zoom, without a pinch-gesture promise.",
                example: Box::new(lightbox_zoom::demo(theme.clone())),
                source: include_str!("lightbox_zoom.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-metadata",
                title: "Stable keys, alternatives and captions",
                note: "Change the lake caption before opening. The same image key selects the same photo; the current accepted caption appears in the gallery. The two local photos are fictional generated samples.",
                example: Box::new(lightbox_metadata::demo(theme.clone())),
                source: include_str!("lightbox_metadata.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-controlled",
                title: "Controlled view and close refusal",
                note: "The application may refuse image/zoom changes. This example refuses the first close proposal once, then accepts the next close so the modal always has an exit. Focus restoration belongs to the shared Layer host.",
                example: Box::new(lightbox_controlled::demo(theme.clone())),
                source: include_str!("lightbox_controlled.rs"),
            },
        ],
        "navigation-progress" => vec![
            DetailSection {
                id: "extensions-navigation-progress-replacement",
                title: "Replacing a route invalidates its old token",
                note: "Start twice, then try to publish the previous route. The old token cannot change the retained current bar. Applications must also cancel their previous future.",
                example: Box::new(navigation_progress_replacement::demo(theme.clone())),
                source: include_str!("navigation_progress_replacement.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-cancel",
                title: "Cancellation and a new operation",
                note: "Cancel hides the current bar and removes its numeric value. A later Start creates a new lifetime; no arbitrary network future is implicitly cancelled.",
                example: Box::new(navigation_progress_cancel::demo(theme.clone())),
                source: include_str!("navigation_progress_cancel.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-finish",
                title: "Determinate progress and completion",
                note: "Reports accept finite percentages. Finish stores 100% and immediately hides the retained bar; there is no automatic increment or completion delay.",
                example: Box::new(navigation_progress_finish::demo(theme.clone())),
                source: include_str!("navigation_progress_finish.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-concurrency",
                title: "One controller, one current operation",
                note: "Reports and Settings share this local controller: the latest start wins. Use independent controllers for unrelated concurrent work rather than aggregating their percentages implicitly.",
                example: Box::new(navigation_progress_concurrency::demo(theme.clone())),
                source: include_str!("navigation_progress_concurrency.rs"),
            },
        ],
        "modals-manager" => vec![
            DetailSection {
                id: "extensions-modals-manager-queue",
                title: "Sequential editor and confirmation",
                note: "Only the first request constructs its content. Continue closes the editor and reveals the queued confirmation; Cancel is the safe initial focus.",
                example: Box::new(modals_manager_queue::demo(theme.clone())),
                source: include_str!("modals_manager_queue.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-metadata",
                title: "Update metadata while preserving the editor",
                note: "Type a draft, then update the dialog title from inside the modal. Metadata changes retain the content Owner, input identity, focus and selection.",
                example: Box::new(modals_manager_metadata::demo(theme.clone())),
                source: include_str!("modals_manager_metadata.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-closing",
                title: "Close queued or current requests",
                note: "Remove waiting confirmation before closing the editor, or close the whole current sequence. Tokens are local to this manager; completed requests cannot be completed twice.",
                example: Box::new(modals_manager_closing::demo(theme.clone())),
                source: include_str!("modals_manager_closing.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-focus",
                title: "Focus and final restoration",
                note: "Open the form from its button, edit it, then save or press Escape. The shared modal Layer contains focus while open and restores the live original opener after the final request.",
                example: Box::new(modals_manager_focus::demo(theme.clone())),
                source: include_str!("modals_manager_focus.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "code-highlight" => vec![
            (
                "Rust, JSON and plain text",
                "#extensions-code-highlight-languages",
            ),
            ("Theme and unstyled ink", "#extensions-code-highlight-theme"),
            (
                "Line numbers and highlighted lines",
                "#extensions-code-highlight-lines",
            ),
            (
                "Long lines stay inside the code viewport",
                "#extensions-code-highlight-long_lines",
            ),
            (
                "Multiple files and copy feedback",
                "#extensions-code-highlight-files",
            ),
        ],
        "json-viewer" => vec![
            (
                "Controlled expansion and refusal",
                "#extensions-json-viewer-expansion",
            ),
            (
                "JSON pointers, escaping and keyboard navigation",
                "#extensions-json-viewer-paths",
            ),
            (
                "Copy the complete JSON source",
                "#extensions-json-viewer-copy",
            ),
            (
                "Search and accepted data updates",
                "#extensions-json-viewer-updates",
            ),
        ],
        "notifications-system" => vec![
            (
                "Stacking and bounded queues",
                "#extensions-notifications-system-stack",
            ),
            (
                "Update, dismiss and application actions",
                "#extensions-notifications-system-updates",
            ),
            (
                "Viewport placement belongs to the application",
                "#extensions-notifications-system-position",
            ),
            (
                "Timeout, pause and resume",
                "#extensions-notifications-system-timeouts",
            ),
            (
                "Inline loading is independent of the queue",
                "#extensions-notifications-system-loading",
            ),
        ],
        "spotlight" => vec![
            (
                "Search, keywords and empty results",
                "#extensions-spotlight-search",
            ),
            (
                "Grouped actions and disabled commands",
                "#extensions-spotlight-groups",
            ),
            (
                "Explicitly scoped Ctrl/Cmd+K",
                "#extensions-spotlight-shortcut",
            ),
            (
                "Controlled opening and search refusal",
                "#extensions-spotlight-controlled",
            ),
        ],
        "lightbox" => vec![
            (
                "Image navigation and boundaries",
                "#extensions-lightbox-navigation",
            ),
            ("Zoom, fit and local scrolling", "#extensions-lightbox-zoom"),
            (
                "Stable keys, alternatives and captions",
                "#extensions-lightbox-metadata",
            ),
            (
                "Controlled view and close refusal",
                "#extensions-lightbox-controlled",
            ),
        ],
        "navigation-progress" => vec![
            (
                "Replacing a route invalidates its old token",
                "#extensions-navigation-progress-replacement",
            ),
            (
                "Cancellation and a new operation",
                "#extensions-navigation-progress-cancel",
            ),
            (
                "Determinate progress and completion",
                "#extensions-navigation-progress-finish",
            ),
            (
                "One controller, one current operation",
                "#extensions-navigation-progress-concurrency",
            ),
        ],
        "modals-manager" => vec![
            (
                "Sequential editor and confirmation",
                "#extensions-modals-manager-queue",
            ),
            (
                "Update metadata while preserving the editor",
                "#extensions-modals-manager-metadata",
            ),
            (
                "Close queued or current requests",
                "#extensions-modals-manager-closing",
            ),
            (
                "Focus and final restoration",
                "#extensions-modals-manager-focus",
            ),
        ],
        _ => Vec::new(),
    }
}
