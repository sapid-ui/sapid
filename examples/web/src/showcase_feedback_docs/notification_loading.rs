use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::notification("sync-notice")
                    .theme(theme.clone())
                    .title(loading.map(|v| {
                        if *v {
                            "Synchronizing files".to_owned()
                        } else {
                            "Files synchronized".to_owned()
                        }
                    }))
                    .content("Loading is supplied by this example; it performs no network request.")
                    .loading(loading)
                    .close_label("Dismiss sync notification")
                    .icon(
                        glory::ui::graphic()
                            .class("sync-result-icon")
                            .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                            .style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))),
                    ),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Synchronizing files")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
