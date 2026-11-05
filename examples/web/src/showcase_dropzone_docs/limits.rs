use glory::prelude::*;
use glory::render::{FileConfig, FileFilter, FileOutcome, FileRejection};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let message = scope.cage("Choose or drop text and PNG files".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::dropzone()
                    .theme(theme.clone())
                    .label("Choose text or PNG files")
                    .configuration(FileConfig {
                        multiple: true,
                        max_files: 2,
                        max_file_bytes: 512 * 1024,
                        max_total_bytes: 1024 * 1024,
                        filters: vec![
                            FileFilter::Mime("text/plain".into()),
                            FileFilter::Mime("image/png".into()),
                        ],
                        ..Default::default()
                    })
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .min_height(Length::Px(120.))
                            .border_style(glory::style::BorderStyle::Solid),
                    )
                    .on_result(move |result| {
                        message.revise(|old| {
                            *old = match result {
                                FileOutcome::Selected(files) => {
                                    format!("Accepted {} files", files.len())
                                }
                                FileOutcome::Cancelled => "Selection cancelled".into(),
                                FileOutcome::Rejected(reason) => match reason {
                                    FileRejection::Count => "Choose at most two files",
                                    FileRejection::Type => "Choose text or PNG files",
                                    FileRejection::Bytes => {
                                        "Keep each file within 512 KiB and the total within 1 MiB"
                                    }
                                    _ => "This selection could not be completed",
                                }
                                .into(),
                            }
                        })
                    }),
                sapid::text().theme(theme.clone()).content(message),
            ))
            .show_in(scope)
    })
}
