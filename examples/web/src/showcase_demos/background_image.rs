use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let source = format!(
        "{}/mountain-evening.svg",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    );
    sapid::background_image(source)
        .theme(theme.clone())
        .name("Mountain evening background")
        .width(Length::Percent(100.))
        .height(Length::Px(220.))
        .style(Style::new().radius(12.))
        .fill(
            glory::ui::view()
                .style(Style {
                    background: Some(Color(0, 12, 24, 130)),
                    ..Style::new()
                        .height(Length::Px(220.))
                        .padding(Edges::all(24.))
                })
                .fill(
                    sapid::stack().theme(theme.clone()).gap(12.).fill((
                        sapid::title(3)
                            .theme(theme.clone())
                            .style(Style {
                                color: Some(Color(255, 255, 255, 255)),
                                ..Style::new()
                            })
                            .content("Make room for a new view"),
                        sapid::text()
                            .theme(theme)
                            .style(Style {
                                color: Some(Color(255, 255, 255, 255)),
                                ..Style::new()
                            })
                            .content("A responsive cover image with readable foreground content."),
                    )),
                ),
        )
}
