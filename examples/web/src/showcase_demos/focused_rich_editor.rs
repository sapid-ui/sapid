use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("P10-08", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use sapid::prelude::*;
    use std::{boxed::Box, rc::Rc};

    fn document(value: &str) -> Rc<RichDocument> {
        Rc::new(
            RichDocument::new(vec![RichBlock {
                kind: RichBlockKind::Paragraph,
                runs: vec![RichRun {
                    text: value.into(),
                    marks: RichMarks::default(),
                }],
            }])
            .expect("showcase rich document"),
        )
    }

    fn heading(theme: &Theme, id: &'static str, title: &'static str) -> impl Widget {
        glory::ui::view()
            .semantics(Some(Box::new(glory::render::Semantics {
                id: Some(glory::render::SemanticId::new(id).expect("showcase section id")),
                ..Default::default()
            })))
            .fill(sapid::title(3).theme(theme.clone()).content(title))
    }

    pub fn demo_for(identity: &str, theme: Theme) -> impl Widget {
        assert_eq!(identity, "P10-08");
        widget(move |scope| {
            let controlled = scope.cage(document("This value is owned by the application."));
            let controlled_text = controlled.map(|document| document.plain_text());
            let children: Vec<Box<dyn Widget>> = vec![
                Box::new(heading(&theme, "rich-editor-usage", "Usage and formatting")),
                Box::new(
                    rich_text_editor()
                        .theme(theme.clone())
                        .name("Article editor")
                        .default_document(document(
                            "Select text and use the finite formatting toolbar.",
                        )),
                ),
                Box::new(heading(&theme, "rich-editor-subtle", "Subtle variant")),
                Box::new(
                    rich_text_editor()
                        .theme(theme.clone())
                        .name("Subtle editor")
                        .subtle(true)
                        .default_document(document("A quieter toolbar and border treatment.")),
                ),
                Box::new(heading(
                    &theme,
                    "rich-editor-controlled",
                    "Controlled document",
                )),
                Box::new(
                    rich_text_editor()
                        .theme(theme.clone())
                        .name("Controlled editor")
                        .document(controlled)
                        .on_document_change(move |next| {
                            controlled.revise(|current| *current = next)
                        }),
                ),
                Box::new(
                    sapid::text()
                        .theme(theme.clone())
                        .class("site-rich-editor-value")
                        .content(controlled_text),
                ),
                Box::new(heading(&theme, "rich-editor-placeholder", "Placeholder")),
                Box::new(
                    rich_text_editor()
                        .theme(theme.clone())
                        .name("Editor with placeholder")
                        .placeholder("Start writing your note..."),
                ),
                Box::new(heading(
                    &theme,
                    "rich-editor-content",
                    "Content without toolbar",
                )),
                Box::new(
                    rich_text_editor()
                        .theme(theme.clone())
                        .name("Editor without toolbar")
                        .toolbar(false)
                        .default_document(document(
                            "The editing surface can be used without the built-in toolbar.",
                        )),
                ),
                Box::new(heading(
                    &theme,
                    "rich-editor-states",
                    "Read only and disabled",
                )),
                Box::new(
                    sapid::group().theme(theme.clone()).fill((
                        rich_text_editor()
                            .theme(theme.clone())
                            .name("Read only rich document")
                            .toolbar(false)
                            .readonly(true)
                            .default_document(document("This document is read only.")),
                        rich_text_editor()
                            .theme(theme.clone())
                            .name("Disabled rich document")
                            .toolbar(false)
                            .enabled(false)
                            .default_document(document("This document is disabled.")),
                    )),
                ),
            ];
            sapid::stack()
                .theme(theme.clone())
                .gap(24.)
                .fill(children)
                .show_in(scope)
        })
    }
}
