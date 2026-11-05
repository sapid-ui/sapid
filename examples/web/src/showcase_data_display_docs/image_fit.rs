use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill(
        [
            glory::style::ObjectFit::Cover,
            glory::style::ObjectFit::Contain,
        ]
        .into_iter()
        .map(|fit| {
            sapid::stack().theme(theme.clone()).gap(6.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .content(format!("{fit:?}")),
                sapid::image(media("alpine-lake-photo.png"))
                    .theme(theme.clone())
                    .alt(format!("Alpine lake, {fit:?} fit"))
                    .fit(fit)
                    .width(Length::Percent(100.))
                    .height(Length::Px(180.))
                    .style(Style {
                        background: Some(theme.surface),
                        radius: Some(8.),
                        ..Style::new()
                    }),
            ))
        })
        .collect::<Vec<_>>(),
    )
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
