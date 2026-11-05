use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            sapid::SeparatorLabelPosition::Start,
            sapid::SeparatorLabelPosition::Center,
            sapid::SeparatorLabelPosition::End,
        ]
        .into_iter()
        .map(|position| {
            sapid::separator()
                .theme(theme.clone())
                .label(format!("{position:?} label"))
                .label_position(position)
        })
        .collect::<Vec<_>>(),
    )
}
