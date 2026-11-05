use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let modes = [
        (sapid::JsonFormat::PrettyTwo, "Two-space indentation"),
        (sapid::JsonFormat::PrettyFour, "Four-space indentation"),
        (sapid::JsonFormat::Compact, "Compact JSON"),
    ];
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        modes
            .into_iter()
            .enumerate()
            .map(|(i, (format, label))| {
                sapid::field(format!("json-format-{i}"))
                    .theme(theme.clone())
                    .label(label)
                    .control(move |field| {
                        sapid::json_input(format!("json-format-editor-{i}"))
                            .field(field)
                            .rows(4)
                            .default_value("{\"items\":[1,2],\"enabled\":true}")
                            .format(format)
                    })
            })
            .collect::<Vec<_>>(),
    )
}
