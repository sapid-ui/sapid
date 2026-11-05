use glory::prelude::*;
use std::rc::Rc;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let controller = scope.cage(None::<sapid::EditorController>);
        sapid::stack()
            .theme(theme.clone())
            .gap(10.)
            .fill((
                sapid::group().theme(theme.clone()).fill(
                    [
                        ("Link selection", Some("https://example.com/")),
                        ("Remove link", None),
                    ]
                    .into_iter()
                    .map(|(label, url)| {
                        let weak = scope.downgrade();
                        sapid::button()
                            .theme(theme.clone())
                            .label(label)
                            .on_press(move || {
                                if let Some(controller) = controller.get()
                                    && let Err(error) = controller
                                        .format(sapid::RichFormat::Link(url.map(str::to_owned)))
                                    && let Some(scope) = weak.upgrade()
                                {
                                    scope.report_error(error);
                                }
                            })
                    })
                    .collect::<Vec<_>>(),
                ),
                sapid::rich_text_editor()
                    .theme(theme.clone())
                    .name("Link example")
                    .controller_ref(controller)
                    .default_document(Rc::new(
                        sapid::RichDocument::new(vec![sapid::RichBlock {
                            kind: sapid::RichBlockKind::Paragraph,
                            runs: vec![sapid::RichRun {
                                text: "Select these words to add a safe link.".into(),
                                marks: Default::default(),
                            }],
                        }])
                        .expect("finite link example"),
                    )),
            ))
            .show_in(scope)
    })
}
