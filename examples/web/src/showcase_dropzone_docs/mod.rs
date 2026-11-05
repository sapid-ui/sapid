use super::showcase_examples::DetailSection;
mod limits;
mod previews;
mod read;
mod states;
pub fn navigation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Limits and rejection", "#dropzone-limits"),
        ("Image previews", "#dropzone-previews"),
        ("Bounded text read", "#dropzone-read"),
        ("State and reconfiguration", "#dropzone-states"),
    ]
}
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
    vec![
        DetailSection {
            id: "dropzone-limits",
            title: "File limits and rejection",
            note: "FileConfig bounds count, each file and the whole selection, with explicit MIME filters. Chooser and actual drop use the same atomic policy. A rejected selection preserves the previous accepted attachments; cancellation is separate from rejection. Drag does not read file bodies before a completed drop.",
            example: Box::new(limits::demo(theme.clone())),
            source: include_str!("limits.rs"),
        },
        DetailSection {
            id: "dropzone-previews",
            title: "Explicit image previews",
            note: "previews(true) adds explicit actions for selected PNG/JPEG files. The configured host checks image headers and body budgets before publishing a preview. Removing, clearing or reconfiguring a selection retires its preview grant. This does not promise SVG, arbitrary codecs or automatic uploads.",
            example: Box::new(previews::demo(theme.clone())),
            source: include_str!("previews.rs"),
        },
        DetailSection {
            id: "dropzone-read",
            title: "Bounded text read and attachment content",
            note: "attachment receives display metadata, a generational file token and the current controller. This example accepts one text file up to 4 KiB and reads it only on demand. read_in binds the reply to the attachment Owner; removing it cancels delivery. Display names are never reopened as paths, and the app owns any later upload.",
            example: Box::new(read::demo(theme.clone())),
            source: include_str!("read.rs"),
        },
        DetailSection {
            id: "dropzone-states",
            title: "Disabling and reactive configuration",
            note: "enabled(false) blocks selection and drop. Changing configuration resets the current selection and grants, including pending read/preview delivery. The application host supplies explicit file services; no native filesystem or network transport is installed by a widget. Focus, chosen-file references and Owner cleanup remain with the shared host.",
            example: Box::new(states::demo(theme)),
            source: include_str!("states.rs"),
        },
    ]
}
