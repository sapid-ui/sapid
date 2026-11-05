use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(16.).fill((
        sapid::avatar("Ava Patel")
            .theme(theme.clone())
            .fallback("AP")
            .source(media("team-member-photo.png")),
        sapid::avatar("Jamie Chen")
            .theme(theme.clone())
            .fallback("JC"),
        sapid::avatar("Unassigned teammate")
            .theme(theme.clone())
            .fallback_icon(sapid::Icon::Info),
    ))
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
