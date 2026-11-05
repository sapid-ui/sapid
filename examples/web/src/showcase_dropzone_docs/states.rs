use glory::prelude::*;
use glory::render::{FileConfig, FileFilter};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let enabled = scope.cage(true);
        let multiple = scope.cage(false);
        let configuration = scope.bond(move || FileConfig {
            multiple: multiple.get(),
            max_files: if multiple.get() { 3 } else { 1 },
            max_file_bytes: 1024 * 1024,
            max_total_bytes: 2 * 1024 * 1024,
            filters: vec![FileFilter::Mime("text/plain".into())],
            ..Default::default()
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable file selection")
                    .checked(enabled)
                    .on_checked_change(move |next| enabled.revise(|old| *old = next)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow multiple text files")
                    .checked(multiple)
                    .on_checked_change(move |next| multiple.revise(|old| *old = next)),
                sapid::dropzone()
                    .theme(theme.clone())
                    .label("Configured text files")
                    .configuration(configuration)
                    .enabled(enabled)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .min_height(Length::Px(100.))
                            .border_style(glory::style::BorderStyle::Solid),
                    ),
            ))
            .show_in(scope)
    })
}
