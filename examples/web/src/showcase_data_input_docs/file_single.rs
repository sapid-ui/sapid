use glory::prelude::*;
use glory::render::{FileConfig, FileFilter};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::file_input()
        .theme(theme.clone())
        .name("Project document")
        .label("Choose a text document")
        .description(move || {
            sapid::text()
                .theme(theme.clone())
                .content("Project document")
        })
        .configuration(FileConfig {
            filters: vec![FileFilter::Mime("text/plain".into())],
            max_file_bytes: 1024 * 1024,
            ..Default::default()
        })
        .clearable(true)
        .show_attachments(false)
}
