use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let feedback = scope.cage("Choose a text file up to 1024 bytes".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::file_button()
                    .theme(theme.clone())
                    .name("Attach a small text file")
                    .label("Attach text file")
                    .style(Style {
                        background: Some(theme.accent_soft),
                        color: Some(theme.accent_text),
                        ..Style::new()
                            .radius(20.)
                            .padding(glory::style::Edges::all(12.))
                    })
                    .on_result(move |outcome| {
                        feedback.revise(|value| {
                            *value = match outcome {
                                glory::render::FileOutcome::Selected(_) => {
                                    "Text file selected".to_owned()
                                }
                                glory::render::FileOutcome::Cancelled => {
                                    "Selection cancelled".to_owned()
                                }
                                glory::render::FileOutcome::Rejected(
                                    glory::render::FileRejection::Bytes,
                                ) => "File is larger than the 1024-byte limit".to_owned(),
                                _ => "This file could not be selected".to_owned(),
                            }
                        })
                    })
                    .configuration(glory::render::FileConfig {
                        filters: vec![glory::render::FileFilter::Extension(".txt".into())],
                        max_file_bytes: 1024,
                        max_total_bytes: 1024,
                        ..Default::default()
                    }),
                sapid::text().theme(theme.clone()).content(feedback),
                sapid::file_button()
                    .theme(theme.clone())
                    .label("Unavailable picker")
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
