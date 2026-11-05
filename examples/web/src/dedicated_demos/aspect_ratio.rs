use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let portrait = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Portrait frame")
                    .checked(portrait)
                    .on_checked_change(move |v| portrait.revise(|old| *old = v)),
                sapid::aspect_ratio(portrait.map(|v| if *v { 0.8 } else { 16. / 9. }))
                    .theme(theme.clone())
                    .class("docs-aspect-frame")
                    .style(Style::new().max_width(Limit::Px(400.)))
                    .fill(
                        sapid::image(format!(
                            "{}/mountain-evening.svg",
                            option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                        ))
                        .theme(theme.clone())
                        .alt("Mountains, a sunset and a lakeside cabin")
                        .width(Length::Percent(100.))
                        .height(Length::Percent(100.))
                        .fit(glory::style::ObjectFit::Cover),
                    ),
            ))
            .show_in(scope)
    })
}
