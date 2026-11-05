use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sizes = [
        (sapid::Size::ExtraSmall, "Extra small"),
        (sapid::Size::Small, "Small"),
        (sapid::Size::Medium, "Medium"),
        (sapid::Size::Large, "Large"),
        (sapid::Size::ExtraLarge, "Extra large"),
    ];
    sapid::stack().theme(theme.clone()).gap(14.).fill(
        sizes
            .into_iter()
            .enumerate()
            .map(|(i, (size, label))| {
                sapid::field(format!("number-size-{i}"))
                    .theme(theme.clone())
                    .size(size)
                    .label(label)
                    .control(|field| {
                        sapid::number_input()
                            .field(field)
                            .default_value(sapid::Decimal::new(1, 0).unwrap())
                    })
            })
            .collect::<Vec<_>>(),
    )
}
