use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            sapid::TabsVariant::Default,
            sapid::TabsVariant::Outline,
            sapid::TabsVariant::Pills,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, variant)| {
            sapid::tabs(format!("settings-variant-{index}"))
                .theme(theme.clone())
                .name(format!("{variant:?} tabs"))
                .variant(variant)
                .grow(true)
                .tab(
                    sapid::tab("profile")
                        .label("Profile")
                        .content(|| text().content("Your public profile.")),
                )
                .tab(
                    sapid::tab("security")
                        .label("Security")
                        .content(|| text().content("Manage your sign-in methods.")),
                )
                .tab(
                    sapid::tab("billing")
                        .label("Billing")
                        .content(|| text().content("Plan and invoices.")),
                )
        })
        .collect::<Vec<_>>(),
    )
}
