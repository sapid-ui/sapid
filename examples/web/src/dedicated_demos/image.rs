use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::image(media("alpine-lake-photo.png"))
        .theme(theme.clone())
        .alt("An alpine village beside a turquoise lake and high mountains")
        .fit(glory::style::ObjectFit::Cover)
        .width(Length::Percent(100.))
        .height(Length::Px(230.))
        .style(Style::new().radius(10.))
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
