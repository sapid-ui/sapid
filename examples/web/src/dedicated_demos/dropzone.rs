use glory::prelude::*;
use glory::render::{FileConfig, FileFilter};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let description = theme.clone();
    sapid::dropzone()
        .theme(theme.clone())
        .label("Drop files here or choose files")
        .configuration(FileConfig {
            multiple: true,
            max_files: 3,
            max_file_bytes: 1024 * 1024,
            max_total_bytes: 2 * 1024 * 1024,
            filters: vec![
                FileFilter::Mime("text/plain".into()),
                FileFilter::Mime("image/png".into()),
                FileFilter::Mime("image/jpeg".into()),
            ],
            ..Default::default()
        })
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .min_height(Length::Px(140.))
                .padding(Edges::all(24.))
                .border_style(glory::style::BorderStyle::Solid),
        )
        .description(move || {
            sapid::text().theme(description.clone()).content(
                "Attach text, PNG or JPEG files. Up to 3 files, 1 MiB each and 2 MiB total.",
            )
        })
}
