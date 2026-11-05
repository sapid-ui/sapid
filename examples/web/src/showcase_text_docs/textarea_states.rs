use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("Retained multiline draft\nSecond line".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::field("textarea-controlled")
                    .theme(theme.clone())
                    .label("Controlled notes")
                    .error("Include a concrete next step.")
                    .control(move |field| {
                        sapid::textarea()
                            .field(field)
                            .rows(3)
                            .value(value)
                            .on_value_change(move |v| value.revise(|old| *old = v))
                    }),
                sapid::field("textarea-readonly")
                    .theme(theme.clone())
                    .label("Read-only notes")
                    .readonly(true)
                    .control(|field| {
                        sapid::textarea()
                            .field(field)
                            .rows(2)
                            .default_value("Saved notes\nRemain selectable")
                    }),
                sapid::field("textarea-disabled")
                    .theme(theme.clone())
                    .label("Disabled notes")
                    .enabled(false)
                    .control(|field| {
                        sapid::textarea()
                            .field(field)
                            .rows(2)
                            .default_value("Retained notes")
                    }),
            ))
            .show_in(scope)
    })
}
