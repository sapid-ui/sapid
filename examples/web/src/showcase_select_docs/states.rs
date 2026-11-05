use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some("rust".to_owned()));
        let accept = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept select changes")
                    .checked(accept)
                    .on_checked_change(move |next| accept.revise(|old| *old = next)),
                sapid::field("select-refusal")
                    .theme(theme.clone())
                    .label("Accepted language")
                    .control(move |field| {
                        sapid::select("select-refusal")
                            .field(field)
                            .value(value)
                            .clearable(true)
                            .options([
                                sapid::select_option("rust", "Rust"),
                                sapid::select_option("go", "Go"),
                            ])
                            .on_value_change(move |next| {
                                if accept.get() {
                                    value.revise(|old| *old = next);
                                }
                            })
                    }),
                sapid::field("select-readonly")
                    .theme(theme.clone())
                    .label("Read only selection")
                    .readonly(true)
                    .control(|field| {
                        sapid::select("select-readonly")
                            .field(field)
                            .default_value(Some("rust".into()))
                            .option(sapid::select_option("rust", "Rust"))
                    }),
                sapid::field("select-disabled")
                    .theme(theme.clone())
                    .label("Disabled selection")
                    .enabled(false)
                    .control(|field| {
                        sapid::select("select-disabled")
                            .field(field)
                            .option(sapid::select_option("go", "Go"))
                    }),
                sapid::field("select-error")
                    .theme(theme.clone())
                    .label("Required selection")
                    .required(true)
                    .error("Choose an available language")
                    .control(|field| {
                        sapid::select("select-error")
                            .field(field)
                            .option(sapid::select_option("rust", "Rust"))
                    }),
            ))
            .show_in(scope)
    })
}
