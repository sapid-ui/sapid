use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let revision = scope.cage(0u32);
        let source = revision.map(|v| format!("Invitation version {v}"));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::copy_button()
                    .theme(theme.clone())
                    .source(source)
                    .label("Copy versioned invitation"),
                sapid::button()
                    .theme(theme.clone())
                    .label("Replace invitation and reset feedback")
                    .on_press(move || revision.revise(|v| *v += 1)),
                sapid::text()
                    .theme(theme.clone())
                    .content(revision.map(|v| format!("Invitation version {v}"))),
            ))
            .show_in(scope)
    })
}
