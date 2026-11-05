use glory::prelude::*;
use glory::render::{FileConfig, FileFilter};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::file_input()
        .theme(theme.clone())
        .name("Small text attachments")
        .label("Choose small text files")
        .description(move || {
            sapid::text()
                .theme(theme.clone())
                .content("Up to two text files, at most 512 bytes each and 1024 bytes in total.")
        })
        .configuration(FileConfig {
            multiple: true,
            max_files: 2,
            max_file_bytes: 512,
            max_total_bytes: 1024,
            filters: vec![FileFilter::Mime("text/plain".into())],
            ..Default::default()
        })
        .clearable(true)
}
