use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let count = scope.cage(0);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::action_icon("Refresh results")
                    .theme(theme.clone())
                    .loading(loading)
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Check.drawing(theme.accent_text))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .on_press(move || count.revise(|v| *v += 1)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Loading action")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|v| format!("Refresh requests: {v}"))),
                sapid::action_icon("Unavailable action")
                    .theme(theme.clone())
                    .enabled(false)
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Close.drawing(theme.disabled_text))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    ),
            ))
            .show_in(scope)
    })
}
