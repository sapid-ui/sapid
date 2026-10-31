use glory::prelude::*;
use sapid::Theme;

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    match id {
        "P3-11" => Box::new(widget(move |scope| {
            let bordered = scope.cage(true);
            let shadow = scope.cage(Some("small".to_owned()));
            let show_header = scope.cage(true);
            let show_footer = scope.cage(true);
            let revision = scope.bond(move || {
                vec![format!(
                    "{}-{}-{}-{}",
                    bordered.get(),
                    shadow.get().unwrap_or_default(),
                    show_header.get(),
                    show_footer.get()
                )]
            });
            let preview_theme = theme.clone();
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |_| {
                    let shadow = match shadow.get().as_deref() {
                        Some("none") => sapid::ShadowSize::None,
                        Some("medium") => sapid::ShadowSize::Medium,
                        Some("large") => sapid::ShadowSize::Large,
                        _ => sapid::ShadowSize::Small,
                    };
                    let mut card = sapid::card()
                        .theme(preview_theme.clone())
                        .title("Project settings")
                        .description("A card groups related content and actions.")
                        .bordered(bordered.get())
                        .shadow(shadow)
                        .fill(
                            sapid::text()
                                .theme(preview_theme.clone())
                                .content("Manage your workspace preferences."),
                        );
                    if show_header.get() {
                        card = card.header(
                            sapid::badge()
                                .theme(preview_theme.clone())
                                .content("Active"),
                        );
                    }
                    if show_footer.get() {
                        card = card.footer(
                            sapid::button()
                                .theme(preview_theme.clone())
                                .size(sapid::Size::Small)
                                .label("Save changes"),
                        );
                    }
                    card
                });
            let controls = vec![
                super::controls::checkbox("Bordered", bordered, &theme),
                super::controls::select(
                    "shadow",
                    "Shadow",
                    shadow,
                    &[
                        ("none", "None"),
                        ("small", "Small"),
                        ("medium", "Medium"),
                        ("large", "Large"),
                    ],
                    &theme,
                ),
                super::controls::checkbox("Header", show_header, &theme),
                super::controls::checkbox("Footer", show_footer, &theme),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        })),
        "P3-17" => Box::new(
            sapid::empty_state()
                .theme(theme.clone())
                .illustration(sapid::badge().theme(theme.clone()).content("Inbox"))
                .title("No messages yet")
                .description("Start a conversation to see it here.")
                .actions(
                    sapid::button()
                        .theme(theme.clone())
                        .label("Compose message"),
                ),
        ),
        "P3-20" => Box::new(
            sapid::stack().theme(theme.clone()).gap(12.0).fill((
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(1234.5)
                    .min_fraction_digits(2)
                    .max_fraction_digits(2),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(1234.5)
                    .locale(sapid::NumberLocale::DeDe)
                    .format(sapid::NumberFormat::Currency(sapid::Currency::Eur)),
                sapid::number_formatter()
                    .theme(theme.clone())
                    .value(0.125)
                    .format(sapid::NumberFormat::Percent)
                    .max_fraction_digits(1),
            )),
        ),
        _ => unreachable!("unsupported display demo"),
    }
}
