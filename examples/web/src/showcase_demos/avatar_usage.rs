use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(18.).fill((
        sapid::avatar("Ava Patel")
            .theme(theme.clone())
            .size(sapid::Size::ExtraLarge)
            .fallback("AP")
            .source(media("team-member-photo.png")),
        sapid::stack().theme(theme.clone()).gap(4.).fill((
            sapid::text().theme(theme.clone()).content("Ava Patel"),
            sapid::text()
                .theme(theme.clone())
                .content("Product designer · Mountain studio"),
        )),
    ))
}

fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
