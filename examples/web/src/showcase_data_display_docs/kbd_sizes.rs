use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::group().theme(theme.clone()).gap(10.).fill(
            [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ]
            .into_iter()
            .map(|size| {
                sapid::kbd()
                    .theme(theme.clone())
                    .size(size)
                    .key(sapid::KbdKey::Enter)
            })
            .collect::<Vec<_>>(),
        ),
        sapid::field("kbd-typing")
            .theme(theme.clone())
            .label("Try normal typing")
            .control(|field| {
                sapid::text_field()
                    .field(field)
                    .placeholder("Keycaps do not capture your keys")
            }),
    ))
}
