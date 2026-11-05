use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let sizes = [
        ("xs", sapid::Size::ExtraSmall),
        ("sm", sapid::Size::Small),
        ("md", sapid::Size::Medium),
        ("lg", sapid::Size::Large),
        ("xl", sapid::Size::ExtraLarge),
    ];
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        sizes
            .into_iter()
            .map(|(label, size)| {
                sapid::field(format!("password-size-{label}"))
                    .theme(theme.clone())
                    .size(size)
                    .label(format!("Password size {label}"))
                    .control(|field| sapid::password_input().field(field).placeholder("Password"))
            })
            .collect::<Vec<_>>(),
    )
}
