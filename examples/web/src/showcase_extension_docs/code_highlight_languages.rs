use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let json = scope.cage(false);
        let source=scope.bond(move||if json.get(){"{\n  \"name\": \"Sapid\",\n  \"ready\": true,\n  \"members\": [\"Ari\", \"Lin\"]\n}\n".to_owned()}else{"pub fn welcome(name: &str) -> String {\n    format!(\"Welcome, {name}!\")\n}\n".to_owned()});
        let language = scope.bond(move || {
            if json.get() {
                CodeLanguage::Json
            } else {
                CodeLanguage::Rust
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::switch()
                    .theme(theme.clone())
                    .label("Show JSON source")
                    .checked(json)
                    .on_checked_change(move |v| json.revise(|o| *o = v)),
                sapid::code_highlight()
                    .theme(theme.clone())
                    .name("Language example")
                    .source(source)
                    .language(language),
                sapid::code_highlight()
                    .theme(theme.clone())
                    .name("Plain text example")
                    .language(CodeLanguage::Plain)
                    .source("<script> stays ordinary text 😀"),
            ))
            .show_in(scope)
    })
}
