use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(6.).fill((
        sapid::text().theme(theme.clone()).content("Open search"),
        sapid::kbd()
            .theme(theme.clone())
            .key(sapid::KbdKey::Control),
        sapid::text().theme(theme.clone()).content("+"),
        sapid::kbd().theme(theme.clone()).content("K"),
    ))
}
