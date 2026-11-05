use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let slow = scope.cage(true);
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
                    .period_millis(slow.map(|v| if *v { 18000u32 } else { 6000u32 })),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use a slower message cycle")
                    .checked(slow)
                    .on_checked_change(move |v| slow.revise(|s| *s = v)),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "Project updates · Design reviews · Research notes · Team conversations";
