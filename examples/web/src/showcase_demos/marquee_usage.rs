use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let paused = scope.cage(false);
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::marquee()
                    .theme(theme.clone())
                    .text(TEXT_1)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(glory::style::Limit::Px(360.)),
                    )
                    .paused(paused)
                    .animated(animated)
                    .period_millis(12000u32),
                sapid::group().theme(theme.clone()).gap(12.).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Pause studio message")
                        .checked(paused)
                        .on_checked_change(move |v| paused.revise(|s| *s = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Animate studio message")
                        .checked(animated)
                        .on_checked_change(move |v| animated.revise(|s| *s = v)),
                )),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str =
    "Design with focus · Share clear ideas · Keep useful notes · Make room for your next project";
