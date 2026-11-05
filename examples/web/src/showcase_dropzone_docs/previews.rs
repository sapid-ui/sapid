use glory::prelude::*;
use glory::render::{FileConfig, FileFilter, FilePreviewLimits};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let description = theme.clone();
    sapid::dropzone()
        .theme(theme)
        .label("Choose PNG or JPEG images")
        .configuration(FileConfig {
            multiple: true,
            max_files: 3,
            max_file_bytes: 2 * 1024 * 1024,
            max_total_bytes: 4 * 1024 * 1024,
            filters: vec![
                FileFilter::Mime("image/png".into()),
                FileFilter::Mime("image/jpeg".into()),
            ],
            preview: Some(FilePreviewLimits::default()),
            ..Default::default()
        })
        .previews(true)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .min_height(Length::Px(120.))
                .border_style(glory::style::BorderStyle::Solid),
        )
        .description(move || {
            sapid::text().theme(description.clone()).content(
                "Choose images, then use Preview to inspect them. Remove an image to discard it.",
            )
        })
}
