use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let source = scope.cage(media("team-member-photo.png"));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::avatar("Ava Patel")
                    .theme(theme.clone())
                    .fallback_icon(sapid::Icon::Info)
                    .source(source),
                sapid::button()
                    .theme(theme.clone())
                    .label("Use unavailable avatar")
                    .on_press(move || source.revise(|s| *s = "data:image/png;base64,AA==".into())),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore avatar photo")
                    .on_press(move || source.revise(|s| *s = media("team-member-photo.png"))),
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
