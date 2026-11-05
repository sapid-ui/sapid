use glory::prelude::*;
use std::rc::Rc;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::rich_text_editor().theme(theme).name("Article editor")
        .default_document(Rc::new(sapid::RichDocument::new(vec![
            sapid::RichBlock { kind: sapid::RichBlockKind::Heading(2), runs: vec![sapid::RichRun {
                text: "A place for your ideas".into(), marks: Default::default(),
            }]},
            sapid::RichBlock { kind: sapid::RichBlockKind::Paragraph, runs: vec![sapid::RichRun {
                text: "Select some text to apply bold, italic or code. Use the block controls to change a paragraph, then try undo and redo.".into(),
                marks: Default::default(),
            }]},
        ]).expect("finite article")))
}
