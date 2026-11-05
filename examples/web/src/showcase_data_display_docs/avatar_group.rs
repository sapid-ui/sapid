use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::avatar_group("design-team")
        .theme(theme.clone())
        .max_visible(3)
        .overlap(9.)
        .name("Design team")
        .avatars([
            (
                "ava".to_owned(),
                sapid::avatar("Ava Patel")
                    .fallback("AP")
                    .source(media("team-member-photo.png")),
            ),
            (
                "jamie".to_owned(),
                sapid::avatar("Jamie Chen").fallback("JC"),
            ),
            ("sam".to_owned(), sapid::avatar("Sam Rivera").fallback("SR")),
            (
                "robin".to_owned(),
                sapid::avatar("Robin Park").fallback("RP"),
            ),
            ("lee".to_owned(), sapid::avatar("Lee Morgan").fallback("LM")),
        ])
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
