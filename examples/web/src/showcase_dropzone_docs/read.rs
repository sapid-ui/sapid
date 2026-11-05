use glory::prelude::*;
use glory::render::{FileConfig, FileFilter, FileRead};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let attachment_theme = theme.clone();
    sapid::dropzone()
        .theme(theme)
        .label("Choose a small text file")
        .configuration(FileConfig {
            max_file_bytes: 4096,
            max_total_bytes: 4096,
            filters: vec![FileFilter::Mime("text/plain".into())],
            ..Default::default()
        })
        .attachment(move |info, controller| {
            let theme = attachment_theme.clone();
            widget(move |scope| {
                let output = scope.cage(
                    if info.size == 0 {
                        "This file is empty"
                    } else {
                        "Text is read only after you choose Read text"
                    }
                    .to_owned(),
                );
                let weak = scope.downgrade();
                let reader = controller.clone();
                let remove = controller.clone();
                let file = info.token;
                let length = info.size.min(4096) as u32;
                sapid::stack()
                    .theme(theme.clone())
                    .gap(8.)
                    .fill((
                        sapid::text()
                            .theme(theme.clone())
                            .content(info.name.clone()),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Read text")
                            .enabled(length > 0)
                            .on_press(move || {
                                let Some(scope) = weak.upgrade() else { return };
                                let result = reader.read_in(
                                    &scope,
                                    FileRead {
                                        file,
                                        offset: 0,
                                        length,
                                    },
                                    move |result| {
                                        output.revise(|old| {
                                            *old = match result {
                                                Ok(bytes) => match String::from_utf8(bytes) {
                                                    Ok(text) if !text.contains('\0') => text,
                                                    _ => "This file is not UTF-8 text".into(),
                                                },
                                                Err(_) => {
                                                    "The selected file could not be read".into()
                                                }
                                            }
                                        });
                                    },
                                );
                                if result.is_err() {
                                    output.revise(|old| {
                                        *old = "Read is unavailable for this selection".into()
                                    });
                                }
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Remove text file")
                            .on_press(move || {
                                let _ = remove.remove(file);
                            }),
                        sapid::text()
                            .theme(theme.clone())
                            .content(output)
                            .style(Style::new().white_space(glory::style::WhiteSpace::PreWrap)),
                    ))
                    .show_in(scope)
            })
        })
}
