use super::showcase_examples::DetailSection;
mod controlled;
mod links;
mod states;
mod toolbar;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "editor-toolbar",
            title: "Toolbar configuration and selection state",
            note: "toolbar_actions configures up to 16 unique actions: Bold, Italic, Code, Undo, Redo, Paragraph, Heading(1–6) and Quote. Formatting controls expose pressed state from the accepted selection and document. Inline marks require a nonempty selection; a collapsed selection can change block kind. Arrow/Home/End move through the toolbar. Size and theme apply to its real buttons.",
            example: Box::new(toolbar::demo(theme.clone())),
            source: include_str!("toolbar.rs"),
        },
        DetailSection {
            id: "editor-controlled",
            title: "Controlled documents and bounded history",
            note: "document accepts Cage/Bond/Vary<Rc<RichDocument>>. on_document_change proposes the full finite value; this example accepts it. Refusing a proposal restores the accepted value without a history entry. Undo/redo records accepted edits only, with 32 snapshots and a shared 2MiB payload limit. External program changes are silent and clear history; the editor node remains stable.",
            example: Box::new(controlled::demo(theme.clone())),
            source: include_str!("controlled.rs"),
        },
        DetailSection {
            id: "editor-links",
            title: "Safe links and the editor controller",
            note: "controller_ref exposes the Owner-bound editor controller. Select text and apply or remove a link. format queries the current physical UTF-16 selection; format_at accepts an explicit validated range. Only http, https and mailto links are accepted. The document is structured data and does not accept raw HTML or arbitrary Tiptap extensions. Clipboard paste is plain text; images, tables and persistent typing-format marks are outside this finite model.",
            example: Box::new(links::demo(theme.clone())),
            source: include_str!("links.rs"),
        },
        DetailSection {
            id: "editor-states",
            title: "Placeholder, subtle and read only content",
            note: "placeholder is passive guidance while empty; subtle changes presentation. toolbar(false) removes the toolbar. readonly preserves selection while rejecting edits, and enabled(false) disables both the editor and its controls. Web and SSR support the finite document; native rich editing is explicitly unsupported. These examples do not establish OS IME or screen-reader acceptance.",
            example: Box::new(states::demo(theme)),
            source: include_str!("states.rs"),
        },
    ]
}
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Toolbar and selection", "#editor-toolbar"),
        ("Controlled document", "#editor-controlled"),
        ("Links and controller", "#editor-links"),
        ("Editor states", "#editor-states"),
    ]
}
