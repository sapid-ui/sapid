use glory::prelude::*;
use sapid::*;
use std::rc::Rc;
pub fn demo(theme: Theme) -> impl Widget {
    rich_text_editor()
        .theme(theme)
        .name("Configured editor")
        .size(Size::Small)
        .toolbar_actions([
            EditorToolbarAction::Bold,
            EditorToolbarAction::Italic,
            EditorToolbarAction::Heading(1),
            EditorToolbarAction::Heading(3),
            EditorToolbarAction::Paragraph,
            EditorToolbarAction::Quote,
            EditorToolbarAction::Undo,
            EditorToolbarAction::Redo,
        ])
        .default_document(Rc::new(
            RichDocument::new(vec![RichBlock {
                kind: RichBlockKind::Paragraph,
                runs: vec![RichRun {
                    text: "Select this text and compare the active formatting controls.".into(),
                    marks: Default::default(),
                }],
            }])
            .expect("finite example"),
        ))
}
