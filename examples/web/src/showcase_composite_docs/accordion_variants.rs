use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let variants = [
        sapid::AccordionVariant::Default,
        sapid::AccordionVariant::Contained,
        sapid::AccordionVariant::Filled,
        sapid::AccordionVariant::Separated,
    ];
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        variants
            .into_iter()
            .enumerate()
            .map(|(index, variant)| {
                sapid::accordion(format!("faq-variant-{index}"))
                    .theme(theme.clone())
                    .variant(variant)
                    .item(
                        sapid::accordion_item("details")
                            .label(format!("{variant:?} appearance"))
                            .content(|| {
                                text()
                                    .content("A full-width disclosure with its own content panel.")
                            }),
                    )
                    .item(
                        sapid::accordion_item("support")
                            .label("Support")
                            .content(|| text().content("Contact your workspace administrator.")),
                    )
            })
            .collect::<Vec<_>>(),
    )
}
