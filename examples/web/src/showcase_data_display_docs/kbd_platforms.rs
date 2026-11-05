use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [
            sapid::KbdPlatform::Windows,
            sapid::KbdPlatform::Mac,
            sapid::KbdPlatform::Linux,
        ]
        .into_iter()
        .map(|platform| {
            sapid::group().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(format!("{platform:?}")),
                sapid::kbd()
                    .theme(theme.clone())
                    .platform(platform)
                    .key(sapid::KbdKey::Meta),
                sapid::text().theme(theme.clone()).content("+"),
                sapid::kbd().theme(theme.clone()).content("K"),
            ))
        })
        .collect::<Vec<_>>(),
    )
}
