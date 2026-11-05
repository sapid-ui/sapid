use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let show = scope.cage(false);
        let body = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show full viewport shade")
                    .checked(show)
                    .on_checked_change(move |v| show.revise(|old| *old = v)),
                when(show, move || {
                    sapid::overlay()
                        .theme(body.clone())
                        .color(Color(0, 0, 0, 255))
                        .opacity(0.25)
                        .style(
                            Style::new()
                                .position(Position::Fixed)
                                .top(Length::Px(0.))
                                .right(Length::Px(0.))
                                .bottom(Length::Px(0.))
                                .left(Length::Px(0.))
                                .pointer_events(glory::style::PointerEvents::None),
                        )
                }),
            ))
            .show_in(scope)
    })
}
