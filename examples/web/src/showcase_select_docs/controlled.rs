use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some("rust".to_owned()));
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::field("showcase-select-controlled")
                    .theme(theme.clone())
                    .label("Controlled selection")
                    .control(move |field| {
                        sapid::select("showcase-select-controlled")
                            .field(field)
                            .value(value)
                            .clearable(true)
                            .options([
                                sapid::select_option("rust", "Rust"),
                                sapid::select_option("go", "Go"),
                                sapid::select_option("zig", "Zig"),
                            ])
                            .on_value_change(move |next| value.revise(|old| *old = next))
                    }),
                text().class("showcase-select-controlled-value").content(
                    value.map(|value| format!("Selected: {}", value.as_deref().unwrap_or("none"))),
                ),
            ))
            .show_in(scope)
    })
}
