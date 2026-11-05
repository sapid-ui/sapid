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
                sapid::field(format!("native-size-{i}"))
                    .theme(theme.clone())
                    .size(size)
                    .label(label)
                    .control(move |field| {
                        sapid::native_select(format!("native-size-control-{i}"))
                            .field(field)
                            .option(sapid::select_option("starter", "Starter"))
                            .option(sapid::select_option("pro", "Pro"))
                    })
            })
            .collect::<Vec<_>>(),
    )
}
