use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("{\"accepted\":true}".to_owned());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept JSON proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::field("json-controlled")
                    .theme(theme.clone())
                    .label("Controlled document")
                    .control(move |field| {
                        sapid::json_input("json-controlled-editor")
                            .field(field)
                            .rows(4)
                            .value(value)
                            .format_on_blur(true)
                            .on_value_change(move |next| {
                                if accept.get() {
                                    value.revise(|old| *old = next)
                                }
                            })
                    }),
                sapid::field("json-readonly")
                    .theme(theme.clone())
                    .label("Read-only JSON")
                    .readonly(true)
                    .control(|field| {
                        sapid::json_input("json-readonly-editor")
                            .field(field)
                            .rows(3)
                            .default_value("{\"saved\":true}")
                    }),
                sapid::field("json-disabled")
                    .theme(theme.clone())
                    .label("Disabled JSON")
                    .enabled(false)
                    .control(|field| {
                        sapid::json_input("json-disabled-editor")
                            .field(field)
                            .rows(3)
                            .default_value("{}")
                    }),
            ))
            .show_in(scope)
    })
}
