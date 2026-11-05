use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let show = scope.cage(false);
        let body = theme.clone();
        let fixed = when(show, move || {
            let actions = sapid::action_bar("fixed-report-actions")
                .theme(body.clone())
                .selected_count(3usize)
                .group(
                    sapid::toolbar_group("fixed-actions")
                        .button(sapid::button().theme(body.clone()).label("Archive reports"))
                        .button(sapid::button().theme(body.clone()).label("Export reports")),
                )
                .on_clear_selection(move || show.revise(|v| *v = false));
            view()
                .class("fixed-selection-example")
                .style(
                    Style::new()
                        .position(Position::Fixed)
                        .left(Length::Px(12.))
                        .bottom(Length::Px(12.))
                        .width(Length::Px(280.))
                        .max_width(Limit::Percent(90.)),
                )
                .fill(actions)
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show fixed selection actions")
                    .checked(show)
                    .on_checked_change(move |v| show.revise(|old| *old = v)),
                fixed,
            ))
            .show_in(scope)
    })
}
