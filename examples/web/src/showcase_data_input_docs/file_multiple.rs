use glory::prelude::*;
use glory::render::FileConfig;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::file_input()
        .theme(theme.clone())
        .name("Supporting documents")
        .label("Choose supporting documents")
        .description(move || {
            sapid::text()
                .theme(theme.clone())
                .content("Choose up to three files, including a file with a long descriptive name.")
        })
        .configuration(FileConfig {
            multiple: true,
            max_files: 3,
            ..Default::default()
        })
        .clearable(true)
}
