use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::affix_container("sticky-preview")
        .theme(theme.clone())
        .name("Sticky scrolling example")
        .style(Style {
            background: Some(theme.accent_soft),
            ..Style::new().height(Length::Px(180.))
        })
        .content(move |context| {
            glory::ui::view()
                .style(
                    Style::new()
                        .min_height(Length::Px(620.))
                        .padding(Edges::all(12.)),
                )
                .fill((
                    glory::ui::view()
                        .style(Style::new().height(Length::Px(48.)))
                        .fill(text().content("Scroll this container")),
                    sapid::affix("sticky-heading")
                        .sticky(context)
                        .theme(theme.clone())
                        .style(Style {
                            background: Some(theme.surface),
                            ..Style::new().padding(Edges::all(12.))
                        })
                        .fill(text().content("A heading that stays in view")),
                    glory::ui::view()
                        .style(Style::new().padding(Edges::all(16.)))
                        .fill(text().content("The content continues below the sticky heading.")),
                ))
        })
}
