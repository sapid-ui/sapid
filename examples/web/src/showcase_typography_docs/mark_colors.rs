use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Start)
        .gap(12.)
        .fill((
            sapid::mark()
                .theme(theme.clone())
                .content("Theme highlight"),
            sapid::mark()
                .theme(theme.clone())
                .style(Style {
                    background: Some(Color(255, 229, 143, 255)),
                    color: Some(Color(70, 45, 0, 255)),
                    ..Style::new()
                })
                .content("Amber review note"),
            sapid::mark()
                .theme(theme.clone())
                .style(Style {
                    background: Some(Color(205, 238, 222, 255)),
                    color: Some(Color(12, 65, 40, 255)),
                    ..Style::new()
                })
                .content("Green approval note"),
        ))
}
