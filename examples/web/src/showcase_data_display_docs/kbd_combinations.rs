use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::group().theme(theme.clone()).gap(6.).fill((
            sapid::kbd()
                .theme(theme.clone())
                .key(sapid::KbdKey::Control),
            sapid::text().theme(theme.clone()).content("+"),
            sapid::kbd().theme(theme.clone()).key(sapid::KbdKey::Shift),
            sapid::text().theme(theme.clone()).content("+"),
            sapid::kbd().theme(theme.clone()).content("P"),
        )),
        sapid::text()
            .theme(theme.clone())
            .content("Command palette hint · No shortcut is installed by this example"),
    ))
}
