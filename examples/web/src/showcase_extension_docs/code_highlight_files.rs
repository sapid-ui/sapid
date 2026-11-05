use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let config = scope.cage(false);
        let copied = scope.cage(CopyState::Idle);
        let source=scope.bond(move||if config.get(){"{\n  \"name\": \"Sapid\",\n  \"ready\": true,\n  \"members\": [\"Ari\", \"Lin\"]\n}\n".to_owned()}else{"pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n".to_owned()});
        let language = scope.bond(move || {
            if config.get() {
                CodeLanguage::Json
            } else {
                CodeLanguage::Rust
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::button()
                        .theme(theme.clone())
                        .label("main.rs")
                        .pressed(scope.bond(move || !config.get()))
                        .on_press(move || config.revise(|o| *o = false)),
                    sapid::button()
                        .theme(theme.clone())
                        .label("config.json")
                        .pressed(config)
                        .on_press(move || config.revise(|o| *o = true)),
                )),
                sapid::group().theme(theme.clone()).fill((
                    sapid::copy_button()
                        .theme(theme.clone())
                        .source(source)
                        .label("Copy current file")
                        .state_ref(copied),
                    sapid::text()
                        .theme(theme.clone())
                        .content(copied.map(|s| format!("Copy: {s:?}"))),
                )),
                sapid::code_highlight()
                    .theme(theme.clone())
                    .name("Current file source")
                    .source(source)
                    .language(language)
                    .line_numbers(true),
            ))
            .show_in(scope)
    })
}
