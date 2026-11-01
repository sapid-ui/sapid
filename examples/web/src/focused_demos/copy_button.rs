use glory::prelude::*;
use sapid::Theme;

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    assert_eq!(id, "P10-03");
    Box::new(widget(move |scope| {
        let source = scope.cage(Some("rust".to_owned()));
        let size = scope.cage(Some("md".to_owned()));
        let disabled = scope.cage(false);
        let enabled = scope.bond(move || !disabled.get());
        let revision = scope.bond(move || vec![size.get().unwrap_or_default()]);
        let preview_theme = theme.clone();
        let preview = each(revision)
            .key(|revision| revision.clone())
            .children(move |entry| {
                let size = super::controls::size_value(&entry.value.get());
                let source = source.map(|choice| match choice.as_deref() {
                    Some("unicode") => "Sapid supports exact text: 😀 <script>".to_owned(),
                    _ => "fn main() { println!(\"Hello, Sapid!\"); }".to_owned(),
                });
                sapid::stack().theme(preview_theme.clone()).gap(16.0).fill((
                    sapid::text().theme(preview_theme.clone()).content(source),
                    sapid::copy_button()
                        .theme(preview_theme.clone())
                        .size(size)
                        .source(source)
                        .label("Copy code")
                        .copied_label("Copied")
                        .failed_label("Copy failed")
                        .enabled(enabled),
                ))
            });
        let controls = vec![
            super::controls::select(
                "copy-source",
                "Content",
                source,
                &[("rust", "Rust code"), ("unicode", "Unicode text")],
                &theme,
            ),
            super::controls::size(size, &theme),
            super::controls::checkbox("Disabled", disabled, &theme),
        ];
        super::controls::playground(Box::new(preview), controls).show_in(scope)
    }))
}
