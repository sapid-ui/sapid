use glory::prelude::*;
use glory::render::{Direction, Orientation};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let rtl = theme.clone();
    let vertical = theme.clone();
    let dots = theme.clone();
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::text()
            .theme(theme.clone())
            .content("Right-to-left, small controls"),
        sapid::carousel()
            .theme(theme.clone())
            .name("RTL cards")
            .count(3usize)
            .direction(Direction::Rtl)
            .looping(true)
            .control_size(24.)
            .render_slide(move |index| {
                sapid::text()
                    .theme(rtl.clone())
                    .content(format!("RTL card {}", index + 1))
                    .style(Style {
                        background: Some(rtl.accent_soft),
                        ..Style::new()
                            .height(Length::Px(120.))
                            .padding(Edges::all(44.))
                    })
            }),
        sapid::text()
            .theme(theme.clone())
            .content("Vertical controls and swipe direction"),
        sapid::carousel()
            .theme(theme.clone())
            .name("Vertical cards")
            .count(3usize)
            .orientation(Orientation::Vertical)
            .with_indicators(true)
            .render_slide(move |index| {
                sapid::text()
                    .theme(vertical.clone())
                    .content(format!("Vertical card {}", index + 1))
                    .style(Style {
                        background: Some(vertical.accent_soft),
                        ..Style::new()
                            .height(Length::Px(160.))
                            .padding(Edges::all(56.))
                    })
            }),
        sapid::text()
            .theme(theme.clone())
            .content("Indicators with arrow controls omitted"),
        sapid::carousel()
            .theme(theme.clone())
            .name("Indicator navigation")
            .count(3usize)
            .with_controls(false)
            .with_indicators(true)
            .render_slide(move |index| {
                sapid::text()
                    .theme(dots.clone())
                    .content(format!("Panel {}", index + 1))
                    .style(Style {
                        background: Some(dots.accent_soft),
                        ..Style::new()
                            .height(Length::Px(100.))
                            .padding(Edges::all(28.))
                    })
            }),
    ))
}
