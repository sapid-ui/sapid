use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::affix_container("preview-affix")
        .theme(theme.clone())
        .name("Local positioning preview")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .height(Length::Px(190.))
                .border_width(Edges::all(1.))
                .border_style(BorderStyle::Solid)
                .border_color(theme.border)
                .radius(8.),
        )
        .content(move |context| {
            glory::ui::view().fill((
                glory::ui::view()
                    .style(Style::new().padding(Edges::all(16.)))
                    .fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("This action is positioned within the preview container."),
                    ),
                sapid::affix("local-action")
                    .within(context)
                    .theme(theme.clone())
                    .fill(sapid::button().theme(theme.clone()).label("Save draft")),
            ))
        })
}
