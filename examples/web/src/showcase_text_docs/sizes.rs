use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let examples = [
        (sapid::Size::ExtraSmall, "Extra small"),
        (sapid::Size::Small, "Small"),
        (sapid::Size::Medium, "Medium"),
        (sapid::Size::Large, "Large"),
        (sapid::Size::ExtraLarge, "Extra large"),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (size, label))| {
        sapid::field(format!("input-size-{i}"))
            .theme(theme.clone())
            .size(size)
            .label(label)
            .control(|field| sapid::text_field().field(field).placeholder("Enter text"))
    })
    .collect::<Vec<_>>();
    sapid::stack().theme(theme).gap(12.).fill(examples)
}
