use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::image(media("alpine-lake-photo.png"))
            .theme(theme.clone())
            .alt("Full alpine lake panorama")
            .width(Length::Percent(100.))
            .style(Style::new().radius(12.)),
        sapid::image(media("team-member-photo.png"))
            .theme(theme.clone())
            .alt("Portrait of fictional designer Ava Patel")
            .width(Length::Px(120.))
            .height(Length::Px(120.))
            .fit(glory::style::ObjectFit::Cover)
            .style(Style::new().radius(60.)),
    ))
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
