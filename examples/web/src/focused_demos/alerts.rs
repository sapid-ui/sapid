use glory::prelude::*;
use sapid::Theme;

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    match id {
        "P5-01" => Box::new(
            sapid::alert_dialog("focused-delete")
                .theme(theme.clone())
                .label("Delete draft")
                .title("Delete this draft?")
                .description("This action cannot be undone.")
                .confirm_label("Delete draft")
                .confirm_variant(sapid::ButtonVariant::Danger)
                .cancel_label("Keep draft"),
        ),
        "P5-05" => Box::new(widget(move |scope| {
            let variant = scope.cage(Some("light".to_owned()));
            let color = scope.cage("Blue".to_owned());
            let radius = scope.cage(4.0);
            let closable = scope.cage(false);
            let title = scope.cage("Alert title".to_owned());
            let content = scope.cage("Lorem ipsum dolor sit, amet consectetur adipisicing elit. AtIcon officiis, quae tempore necessitatibus placeat saepe.".to_owned());
            let visible = scope.cage(true);
            let revision = scope.bond(move || vec![(variant.get(), color.get(), closable.get(), title.with(|value| value.trim().is_empty()))]);
            let preview_theme = theme.clone();
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |_| {
                    let theme = super::controls::accent(preview_theme.clone(), &color.get());
                    let foreground = if variant.get().as_deref() == Some("filled") { theme.on_accent } else { theme.accent };
                    let mut alert = sapid::alert("playground-alert")
                        .theme(theme.clone())
                        .variant(match variant.get().as_deref() {
                            Some("filled") => sapid::AlertVariant::Filled,
                            Some("outline") => sapid::AlertVariant::Outline,
                            Some("transparent") => sapid::AlertVariant::Transparent,
                            _ => sapid::AlertVariant::Light,
                        })
                        .color(theme.accent)
                        .style(radius.map(|radius| Style::new().radius(*radius).width(Length::Percent(100.0))))
                        .icon(sapid::theme_icon(sapid::Icon::Info).unstyled().color(foreground).style(Style::new().width(Length::Px(20.0)).height(Length::Px(20.0))))
                        .content(content.map(|value| if value.trim().is_empty() { " \u{200b}".to_owned() } else { value.clone() }))
                        .visible(visible)
                        .on_close(move || visible.revise(|value| *value = false))
                        .closable(closable.get())
                        .close_label("Dismiss alert");
                    if !title.with(|value| value.trim().is_empty()) {
                        // The previous alert may still observe this source while
                        // Each retires it for the title-less configuration.
                        alert = alert.title(title.map(|value| if value.trim().is_empty() { "Alert title".to_owned() } else { value.clone() }));
                    }
                    alert
                });
            let controls = vec![
                super::controls::select("alert-variant", "Variant", variant, &[("light", "Light"), ("filled", "Filled"), ("outline", "Outline"), ("transparent", "Transparent")], &theme),
                super::controls::color_choices(color, &theme),
                super::controls::radius_slider(radius, &theme),
                super::controls::switch("With close button", closable, &theme),
                super::controls::text_field("Title", title, &theme),
                super::controls::text_field("Children", content, &theme),
                Box::new(when(visible.map(|visible| !visible), { let theme = theme.clone(); move || sapid::button().theme(theme.clone()).label("Show alert").on_press(move || visible.revise(|value| *value = true)) })),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        })),
        "P5-10" => Box::new(
            sapid::stack().theme(theme.clone()).gap(14.0).fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::skeleton()
                        .theme(theme.clone())
                        .circle(42.0)
                        .loading(true),
                    sapid::stack().theme(theme.clone()).gap(8.0).fill((
                        sapid::skeleton()
                            .theme(theme.clone())
                            .width(Length::Px(180.0))
                            .height(18.0)
                            .loading(true),
                        sapid::skeleton()
                            .theme(theme.clone())
                            .width(Length::Px(120.0))
                            .height(14.0)
                            .loading(true),
                    )),
                )),
                sapid::skeleton()
                    .theme(theme.clone())
                    .width(Length::Percent(100.0))
                    .height(14.0)
                    .loading(true),
                sapid::skeleton()
                    .theme(theme.clone())
                    .width(Length::Percent(80.0))
                    .height(14.0)
                    .loading(true),
            )),
        ),
        "P5-12" => Box::new(
            sapid::floating_dialog("focused-background-note")
                .theme(theme.clone())
                .label("Show floating notice")
                .title("Background sync")
                .description("Keep editing while this notice is open.")
                .close_label("Dismiss floating notice")
                .content(move || {
                    sapid::text()
                        .theme(theme.clone())
                        .content("Your changes are synchronized in the background.")
                }),
        ),
        _ => unreachable!("unsupported alerts demo"),
    }
}
