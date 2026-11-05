use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(
        [
            ("compact", "Four adjacent cells", 4, 0., 8.),
            ("spaced", "Six spaced cells", 6, 8., 12.),
            ("padded", "More cell padding", 6, 8., 16.),
        ]
        .into_iter()
        .map(|(key, label, length, gap, padding)| {
            let input_theme = theme.clone();
            sapid::field(format!("pin-{key}"))
                .theme(theme.clone())
                .label(label)
                .control(move |field| {
                    sapid::input_otp()
                        .theme(input_theme.clone())
                        .field(field)
                        .length(length)
                        .gap(gap)
                        .cell_padding(padding)
                })
        })
        .collect::<Vec<_>>(),
    )
}
