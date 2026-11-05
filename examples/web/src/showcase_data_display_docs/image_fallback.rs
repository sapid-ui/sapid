use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let source = scope.cage(media("alpine-lake-photo.png"));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::image(source)
                    .theme(theme.clone())
                    .alt("Alpine village beside the lake")
                    .fallback_src(media("alpine-lake-photo.png"))
                    .width(Length::Percent(100.))
                    .height(Length::Px(180.))
                    .fit(glory::style::ObjectFit::Cover),
                sapid::button()
                    .theme(theme.clone())
                    .label("Load unavailable photo")
                    .on_press(move || source.revise(|s| *s = "data:image/png;base64,AA==".into())),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore original photo")
                    .on_press(move || source.revise(|s| *s = media("alpine-lake-photo.png"))),
            ))
            .show_in(scope)
    })
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
