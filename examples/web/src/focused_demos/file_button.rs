use glory::prelude::*;
use glory::render::FileConfig;
use sapid::Theme;

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    assert_eq!(id, "P10-01");
    Box::new(widget(move |scope| {
        let selected = scope.cage(Vec::<glory::render::FileInfo>::new());
        let size = scope.cage(Some("md".to_owned()));
        let multiple = scope.cage(false);
        let disabled = scope.cage(false);
        let enabled = scope.bond(move || !disabled.get());
        let revision = scope.bond(move || {
            vec![format!(
                "{}-{}",
                size.get().unwrap_or_default(),
                multiple.get()
            )]
        });
        let summary = scope.bond(move || {
            let files = selected.get();
            if files.is_empty() {
                "No files selected".to_owned()
            } else {
                files
                    .iter()
                    .map(|file| file.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        });
        let preview_theme = theme.clone();
        let preview = each(revision)
            .key(|revision| revision.clone())
            .children(move |entry| {
                let size = super::controls::size_value(&entry.value.get());
                let multiple = multiple.get();
                sapid::stack().theme(preview_theme.clone()).gap(16.0).fill((
                    sapid::file_button()
                        .theme(preview_theme.clone())
                        .size(size)
                        .label(if multiple {
                            "Choose files"
                        } else {
                            "Choose file"
                        })
                        .configuration(FileConfig {
                            multiple,
                            max_files: if multiple { 3 } else { 1 },
                            ..Default::default()
                        })
                        .enabled(enabled)
                        .files_ref(selected),
                    sapid::text().theme(preview_theme.clone()).content(summary),
                ))
            });
        let controls = vec![
            super::controls::size(size, &theme),
            super::controls::checkbox("Multiple", multiple, &theme),
            super::controls::checkbox("Disabled", disabled, &theme),
        ];
        super::controls::playground(Box::new(preview), controls).show_in(scope)
    }))
}
