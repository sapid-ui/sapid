use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0u32);
        let show = scope.cage(true);
        sapid::stack()
            .align(Align::Start)
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::indicator()
                    .theme(theme.clone())
                    .position(sapid::IndicatorPosition::MiddleCenter)
                    .diameter(26.)
                    .label("!")
                    .visible(show)
                    .fill(
                        sapid::button()
                            .theme(theme.clone())
                            .label("Confirm notification")
                            .on_press(move || count.revise(|n| *n += 1)),
                    ),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show overlapping marker")
                    .checked(show)
                    .on_checked_change(move |v| show.revise(|s| *s = v)),
                sapid::text()
                    .theme(theme.clone())
                    .content(count.map(|n| format!("Confirmed {n} times"))),
            ))
            .show_in(scope)
    })
}
