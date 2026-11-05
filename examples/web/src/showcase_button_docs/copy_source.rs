use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let source = scope.cage("First invitation".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("copy-message")
                    .theme(theme.clone())
                    .label("Text to copy")
                    .control(move |field| {
                        sapid::text_field()
                            .field(field)
                            .value(source)
                            .on_value_change(move |v| source.revise(|old| *old = v))
                    }),
                sapid::copy_button()
                    .theme(theme.clone())
                    .source(source)
                    .label("Copy current message"),
                sapid::copy_button()
                    .theme(theme.clone())
                    .source("Unavailable")
                    .label("Unavailable copy")
                    .enabled(false),
            ))
            .show_in(scope)
    })
}
