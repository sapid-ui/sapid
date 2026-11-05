use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let paused = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::marquee()
                    .theme(theme.clone())
                    .text(TEXT_1)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(glory::style::Limit::Px(360.)),
                    )
                    .animated(false),
                sapid::marquee()
                    .theme(theme.clone())
                    .text(TEXT_2)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(glory::style::Limit::Px(360.)),
                    )
                    .paused(paused),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Keep comparison message paused")
                    .checked(paused)
                    .on_checked_change(move |v| paused.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str =
    "A static alternative keeps this message available without continuous movement.";
const TEXT_2: &str = "This message starts paused · Resume when you are ready to read more";
