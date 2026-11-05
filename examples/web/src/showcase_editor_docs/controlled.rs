use glory::prelude::*;
use std::rc::Rc;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let document = scope.cage(Rc::new(sapid::RichDocument::default()));
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(10.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept editor changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::rich_text_editor()
                    .theme(theme.clone())
                    .name("Controlled document")
                    .placeholder("Write a short note")
                    .document(document)
                    .on_document_change(move |next| {
                        if accept.get() {
                            document.revise(|old| *old = next);
                        }
                    }),
                sapid::text().theme(theme.clone()).content(
                    document.map(|document| format!("Accepted text: {}", document.plain_text())),
                ),
            ))
            .show_in(scope)
    })
}
