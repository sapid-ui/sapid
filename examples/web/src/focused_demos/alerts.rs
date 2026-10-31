use glory::prelude::*;
use sapid::{AlertSeverity, Theme};

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
            let severity = scope.cage(Some("success".to_owned()));
            let show_title = scope.cage(true);
            let closable = scope.cage(true);
            let revision = scope.bond(move || {
                vec![format!(
                    "{}-{}-{}",
                    severity.get().unwrap_or_default(),
                    show_title.get(),
                    closable.get()
                )]
            });
            let preview_theme = theme.clone();
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |_| {
                    let severity = match severity.get().as_deref() {
                        Some("info") => AlertSeverity::Info,
                        Some("warning") => AlertSeverity::Warning,
                        Some("danger") => AlertSeverity::Danger,
                        _ => AlertSeverity::Success,
                    };
                    sapid::alert("playground-alert")
                        .theme(preview_theme.clone())
                        .severity(severity)
                        .title(if show_title.get() {
                            "Changes saved"
                        } else {
                            ""
                        })
                        .content("Your workspace is up to date.")
                        .closable(closable.get())
                        .close_label("Dismiss message")
                });
            let controls = vec![
                super::controls::select(
                    "severity",
                    "Severity",
                    severity,
                    &[
                        ("info", "Info"),
                        ("success", "Success"),
                        ("warning", "Warning"),
                        ("danger", "Danger"),
                    ],
                    &theme,
                ),
                super::controls::checkbox("Show title", show_title, &theme),
                super::controls::checkbox("Closable", closable, &theme),
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
