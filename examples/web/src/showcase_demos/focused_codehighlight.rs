use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("CodeHighlight", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{FileConfig, Orientation};
    use glory::style::{FlexWrap, ObjectFit};
    use glory::ui::text;
    use sapid::prelude::*;
    use sapid::{
        Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button,
        file_input, group, image, indicator, progress, ring_progress, select_group, select_option,
        semi_circle_progress, theme_icon,
    };
    use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

    const RUST_EXAMPLE: &str = "pub fn greet(name: &str) -> String {\n    // Unicode stays plain code: 😀 <script>\n    format!(\"Hello, {name}!\")\n}\n";
    const JSON_EXAMPLE: &str =
        "{\n  \"name\": \"Sapid\",\n  \"ready\": true,\n  \"values\": [1, 2, 3]\n}\n";

    fn code_highlight_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let source = scope.cage(RUST_EXAMPLE.to_owned());
            let language = scope.cage(CodeLanguage::Rust);
            let expanded = scope.cage(false);
            let clipboard = scope.cage(CopyState::Idle);
            let visible_source = scope.bond(move || {
                let source = source.get();
                if expanded.get() {
                    source
                } else {
                    source.lines().take(3).collect::<Vec<_>>().join("\n")
                }
            });
            let label = expanded.map(|expanded| {
                if *expanded {
                    "Collapse code".to_owned()
                } else {
                    "Expand code".to_owned()
                }
            });
            sapid::stack()
                .theme(theme.clone())
                .gap(12.0)
                .fill((
                    sapid::code_highlight()
                        .theme(theme.clone())
                        .class("site-code-highlight-demo")
                        .name("Rust and JSON code")
                        .source(visible_source)
                        .language(language)
                        .line_numbers(true)
                        .highlighted_lines([2])
                        .copy_button(
                            sapid::copy_button()
                                .theme(theme.clone())
                                .label("Copy code")
                                .copied_label("Copied")
                                .state_ref(clipboard),
                        ),
                    sapid::group().theme(theme.clone()).fill((
                        sapid::button()
                            .theme(theme.clone())
                            .label("Rust example")
                            .on_press(move || {
                                source.revise(|value| *value = RUST_EXAMPLE.into());
                                language.revise(|value| *value = CodeLanguage::Rust);
                                expanded.revise(|value| *value = false);
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .label("JSON example")
                            .on_press(move || {
                                source.revise(|value| *value = JSON_EXAMPLE.into());
                                language.revise(|value| *value = CodeLanguage::Json);
                                expanded.revise(|value| *value = true);
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .label(label)
                            .on_press(move || expanded.revise(|value| *value = !*value)),
                    )),
                    sapid::text()
                        .theme(theme.clone())
                        .variant(TextVariant::Small)
                        .content(clipboard.map(|state| format!("Clipboard status: {state:?}"))),
                ))
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "CodeHighlight" => Box::new(code_highlight_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
