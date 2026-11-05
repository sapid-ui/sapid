use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let files = scope.cage(Vec::<glory::render::FileInfo>::new());
        let controller = scope.cage(None::<sapid::FileController>);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::file_button()
                    .theme(theme.clone())
                    .name("Choose project documents")
                    .label("Choose up to three documents")
                    .configuration(glory::render::FileConfig {
                        multiple: true,
                        max_files: 3,
                        ..Default::default()
                    })
                    .files_ref(files)
                    .controller_ref(controller),
                sapid::text()
                    .theme(theme.clone())
                    .content(files.map(|v| format!("Selected documents: {}", v.len()))),
                sapid::button()
                    .theme(theme.clone())
                    .label("Reset selected documents")
                    .on_press(move || {
                        if let Some(controller) = controller.get() {
                            let _ = controller.clear();
                        }
                    }),
            ))
            .show_in(scope)
    })
}
