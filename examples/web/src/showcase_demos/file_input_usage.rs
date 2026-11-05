use glory::prelude::*;
use glory::render::FileConfig;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::file_input()
        .theme(theme.clone())
        .name("Attachments")
        .label("Choose files")
        .description(move || sapid::text().theme(theme.clone()).content("Attachments"))
        .configuration(FileConfig {
            multiple: true,
            max_files: 4,
            ..Default::default()
        })
        .clearable(true)
        .show_attachments(false)
}
