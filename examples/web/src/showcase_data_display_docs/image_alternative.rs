use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(14.).fill((
        sapid::image(media("team-member-photo.png"))
            .theme(theme.clone())
            .decorative()
            .width(Length::Px(64.))
            .height(Length::Px(64.))
            .fit(glory::style::ObjectFit::Cover)
            .style(Style::new().radius(32.)),
        sapid::text()
            .theme(theme.clone())
            .content("Ava Patel · Product designer"),
    ))
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
