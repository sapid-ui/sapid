use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            ("Auto", sapid::PopoverWidth::Auto),
            ("240px", sapid::PopoverWidth::Px(240.)),
            ("Target", sapid::PopoverWidth::Target),
        ]
        .into_iter()
        .map(|(label, width)| {
            sapid::popover(format!("width-{label}"))
                .theme(theme.clone())
                .label(format!("Open {label} width popover"))
                .title(format!("{label} width"))
                .width(width)
                .description("This content demonstrates the selected width policy.")
        })
        .collect::<Vec<_>>(),
    )
}
