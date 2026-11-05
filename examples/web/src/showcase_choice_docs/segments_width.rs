use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let full = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use equal full-width segments")
                    .checked(full)
                    .on_checked_change(move |v| full.revise(|old| *old = v)),
                sapid::segmented_control("period-width")
                    .theme(theme.clone())
                    .label("Period width")
                    .class("docs-segments-width")
                    .full_width(full)
                    .default_value("day")
                    .segments([
                        sapid::segment("day").label("Day"),
                        sapid::segment("fortnight").label("Two weeks"),
                        sapid::segment("month").label("Calendar month"),
                    ]),
            ))
            .show_in(scope)
    })
}
