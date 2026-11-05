use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::rich_text_editor()
            .theme(theme.clone())
            .name("Subtle note")
            .subtle(true)
            .placeholder("Start a note..."),
        sapid::rich_text_editor()
            .theme(theme.clone())
            .name("Read only document")
            .readonly(true)
            .toolbar(false)
            .default_document(std::rc::Rc::new(
                sapid::RichDocument::new(vec![sapid::RichBlock {
                    kind: sapid::RichBlockKind::Quote,
                    runs: vec![sapid::RichRun {
                        text: "A read only document still allows text selection.".into(),
                        marks: Default::default(),
                    }],
                }])
                .expect("read only document"),
            )),
        sapid::rich_text_editor()
            .theme(theme)
            .name("Disabled document")
            .enabled(false)
            .placeholder("Editing unavailable"),
    ))
}
