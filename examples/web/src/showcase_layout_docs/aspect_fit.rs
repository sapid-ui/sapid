use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .gap(16.)
        .align(Align::Start)
        .fill(Vec::from(
            [
                glory::style::ObjectFit::Cover,
                glory::style::ObjectFit::Contain,
            ]
            .map(|fit| {
                sapid::stack()
                    .theme(theme.clone())
                    .gap(8.)
                    .style(Style {
                        grow: Some(1.),
                        ..Style::new()
                            .width(Length::Px(180.))
                            .min_width(Length::Px(0.))
                    })
                    .fill((
                        sapid::text().theme(theme.clone()).content(
                            if fit == glory::style::ObjectFit::Cover {
                                "Cover crops the scene"
                            } else {
                                "Contain preserves the whole scene"
                            },
                        ),
                        sapid::aspect_ratio(1.)
                            .theme(theme.clone())
                            .style(Style {
                                background: Some(theme.input),
                                ..Style::new()
                            })
                            .fill(
                                sapid::image(format!(
                                    "{}/mountain-evening.svg",
                                    option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                                ))
                                .theme(theme.clone())
                                .alt(if fit == glory::style::ObjectFit::Cover {
                                    "Cropped mountains and cabin"
                                } else {
                                    "Complete mountains and cabin"
                                })
                                .width(Length::Percent(100.))
                                .height(Length::Percent(100.))
                                .fit(fit),
                            ),
                    ))
            }),
        ))
}
