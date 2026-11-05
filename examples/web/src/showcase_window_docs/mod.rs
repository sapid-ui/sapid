use super::showcase_examples::DetailSection;
mod controlled;
mod handles;
mod lifetime;
mod modal;
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Controlled geometry", "#window-controlled"),
        ("Content lifetime", "#window-lifetime"),
        ("Modal confirmation", "#window-modal"),
        ("Handles and state", "#window-handles"),
    ]
}
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "window-controlled",
            title: "Controlled bounds and refusal",
            note: "WindowBounds owns position and dimensions; WindowLimits bounds resizing. Pointer gestures and the compact keyboard actions propose through the same controller. Disable acceptance to inspect refused moves/resizes. Root style remains for decoration, and cannot override geometry.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "window-lifetime",
            title: "Content lifetime",
            note: "KeepMounted retains the real editor and local draft while closed. Unmount retires its content Owner and creates a fresh editor on reopening. These examples share one opened key so the comparison never starts with overlapping panels.",
            example: Box::new(lifetime::demo(theme.clone())),
            source: include_str!("lifetime.rs"),
        },
        DetailSection {
            id: "window-modal",
            title: "Nonmodal notes and modal confirmation",
            note: "FloatingWindow is a nonmodal viewport Dialog. It permits interaction with the surrounding page. A composed modal Dialog supplies blocking confirmation, focus containment and Escape/return behavior through the shared host layer.",
            example: Box::new(modal::demo(theme.clone())),
            source: include_str!("modal.rs"),
        },
        DetailSection {
            id: "window-handles",
            title: "Current handles, disabling and disposal",
            note: "on_ready supplies one Owner-bound weak handle. Raise and commands operate on the current window; a stale handle cannot revive disposed UI. enabled(false) makes the editor and window controls unavailable. Remounting provides a new current handle.",
            example: Box::new(handles::demo(theme)),
            source: include_str!("handles.rs"),
        },
    ]
}
