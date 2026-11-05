use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show viewport action")
                    .checked(visible)
                    .on_checked_change(move |next| visible.revise(|old| *old = next)),
                sapid::affix("viewport-example")
                    .theme(theme.clone())
                    .visible(visible)
                    .style(Style::new().max_width(Limit::Percent(90.)))
                    .fill(
                        sapid::button()
                            .theme(theme.clone())
                            .label("Dismiss floating action")
                            .on_press(move || visible.revise(|old| *old = false)),
                    ),
            ))
            .show_in(scope)
    })
}
