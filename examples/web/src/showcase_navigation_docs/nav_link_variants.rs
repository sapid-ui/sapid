use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(8.).fill(
        [
            ("light", sapid::NavLinkVariant::Light),
            ("filled", sapid::NavLinkVariant::Filled),
            ("subtle", sapid::NavLinkVariant::Subtle),
        ]
        .into_iter()
        .map(|(key, variant)| {
            sapid::nav_link(format!("variant-{key}"))
                .theme(theme.clone())
                .label(format!("Active {key}"))
                .href(format!("#{key}"))
                .highlighted(true)
                .variant(variant)
                .description("Additional information")
                .left_section(text().content("▤"))
                .right_section(sapid::badge().theme(theme.clone()).content("3"))
        })
        .collect::<Vec<_>>(),
    )
}
