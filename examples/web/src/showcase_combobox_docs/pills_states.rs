use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::field("pills-disabled-field")
            .theme(theme.clone())
            .label("Unavailable topics")
            .control(|field| {
                sapid::pills_input("pills-disabled")
                    .field(field)
                    .default_value(vec!["Design".into()])
                    .enabled(false)
            }),
        sapid::field("pills-readonly-field")
            .theme(theme.clone())
            .label("Read-only topics")
            .control(|field| {
                sapid::pills_input("pills-readonly")
                    .field(field)
                    .default_value(vec!["Design".into()])
                    .readonly(true)
            }),
        sapid::stack().theme(theme.clone()).gap(12.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .enumerate()
            .map(|(i, size)| {
                sapid::field(format!("pills-size-field-{i}"))
                    .theme(theme.clone())
                    .size(size)
                    .label(format!("Topics size {}", i + 1))
                    .control(move |field| {
                        sapid::pills_input(format!("pills-size-{i}"))
                            .field(field)
                            .default_value(vec!["Design".into()])
                            .clearable(true)
                    })
            })
            .collect::<Vec<_>>(),
        ),
    ))
}
