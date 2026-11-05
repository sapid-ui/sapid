use glory::prelude::*;
use glory::style::ObjectFit;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [("Cover", ObjectFit::Cover), ("Contain", ObjectFit::Contain)]
            .into_iter()
            .map(|(label, fit)| {
                let source = format!(
                    "{}/mountain-evening.svg",
                    option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                );
                sapid::background_image(source)
                    .theme(theme.clone())
                    .fit(fit)
                    .width(Length::Percent(100.))
                    .height(Length::Px(150.))
                    .fill(
                        glory::ui::view()
                            .style(Style {
                                background: Some(Color(0, 12, 24, 130)),
                                ..Style::new()
                                    .height(Length::Px(150.))
                                    .padding(Edges::all(16.))
                            })
                            .fill(
                                text()
                                    .style(Style {
                                        color: Some(Color(255, 255, 255, 255)),
                                        ..Style::new()
                                    })
                                    .content(label),
                            ),
                    )
            })
            .collect::<Vec<_>>(),
    )
}
