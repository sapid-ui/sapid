use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let cards = [
        ("Design", "Give your next idea a clear direction."),
        ("Build", "Turn a small prototype into something useful."),
        ("Share", "Review the result with your team."),
    ];
    let card_theme = theme.clone();
    sapid::carousel()
        .theme(theme.clone())
        .name("Project highlights")
        .count(cards.len())
        .looping(true)
        .with_indicators(true)
        .style(Style::new().width(Length::Percent(100.)))
        .render_slide(move |index| {
            let (title, description) = cards[index];
            sapid::stack()
                .theme(card_theme.clone())
                .gap(12.)
                .style(Style {
                    background: Some(card_theme.accent_soft),
                    ..Style::new().height(Length::Px(220.)).padding(Edges {
                        top: 32.,
                        bottom: 32.,
                        left: 52.,
                        right: 52.,
                    })
                })
                .fill((
                    sapid::text()
                        .theme(card_theme.clone())
                        .content(format!("0{} / 03", index + 1))
                        .style(Style {
                            color: Some(card_theme.muted),
                            ..Style::new()
                        }),
                    sapid::text()
                        .theme(card_theme.clone())
                        .content(title)
                        .style(Style {
                            font_size: Some(28.),
                            line_height: Some(34.),
                            font_weight: Some(650.),
                            color: Some(card_theme.accent_text),
                            ..Style::new()
                        }),
                    sapid::text().theme(card_theme.clone()).content(description),
                ))
        })
}
