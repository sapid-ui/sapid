use glory::prelude::*;
use glory::render::{Direction, Orientation};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::tabs("vertical-settings")
            .theme(theme.clone())
            .name("Vertical settings")
            .orientation(Orientation::Vertical)
            .tab(
                sapid::tab("profile")
                    .label("Profile")
                    .content(|| text().content("Edit your profile details.")),
            )
            .tab(
                sapid::tab("security")
                    .label("Security")
                    .content(|| text().content("Review your security settings.")),
            ),
        sapid::tabs("rtl-settings")
            .theme(theme)
            .name("RTL settings")
            .direction(Direction::Rtl)
            .tab(
                sapid::tab("profile")
                    .label("Profile")
                    .content(|| text().content("Horizontal arrows follow RTL navigation.")),
            )
            .tab(
                sapid::tab("security")
                    .label("Security")
                    .content(|| text().content("Security settings.")),
            ),
    ))
}
