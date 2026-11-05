use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let visible = scope.cage(true);
        let details = scope.cage(false);
        let detail_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::notification("project-saved")
                    .theme(theme.clone())
                    .title(loading.map(|v| {
                        if *v {
                            "Saving project".to_owned()
                        } else {
                            "Project saved".to_owned()
                        }
                    }))
                    .content("The latest project settings are available to your team.")
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                            .style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))),
                    )
                    .loading(loading)
                    .visible(visible)
                    .close_label("Dismiss project notification")
                    .on_close(move || visible.revise(|v| *v = false))
                    .action(details.map(|v|if *v {"Hide details".to_owned()} else {"Show details".to_owned()}), move || details.revise(|v| *v = !*v)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Project is saving")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::button()
                    .theme(theme.clone())
                    .label("Restore project notification")
                    .enabled(visible.map(|v| !*v))
                    .on_press(move || visible.revise(|v| *v = true)),
                when(details,move||sapid::text().theme(detail_theme.clone()).content("Saved settings include the workspace name, member preferences and storage limits.")),
            ))
            .show_in(scope)
    })
}
