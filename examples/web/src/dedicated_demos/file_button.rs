use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let files = scope.cage(Vec::<glory::render::FileInfo>::new());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::file_button()
                    .theme(theme.clone())
                    .name("Upload profile image")
                    .label("Upload image")
                    .configuration(glory::render::FileConfig {
                        filters: vec![glory::render::FileFilter::Mime("image/png".into()), glory::render::FileFilter::Mime("image/jpeg".into())],
                        ..Default::default()
                    })
                    .files_ref(files),
                sapid::text().theme(theme.clone()).content(files.map(|v| {
                    v.first()
                        .map(|f| format!("Selected: {}", f.name))
                        .unwrap_or_else(|| "No image selected".into())
                })),
            ))
            .show_in(scope)
    })
}
