use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Load preview region")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::loading_overlay(loading)
                    .theme(theme.clone())
                    .label("Loading preview")
                    .style(
                        Style::new()
                            .height(Length::Px(160.))
                            .width(Length::Percent(100.)),
                    )
                    .fill(
                        sapid::text()
                            .theme(theme.clone())
                            .content("The spinner is centered inside this explicit 160px preview."),
                    ),
            ))
            .show_in(scope)
    })
}
