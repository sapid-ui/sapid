use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let animated = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate unknown progress")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|old| *old = v)),
                sapid::group().theme(theme.clone()).gap(24.).fill((
                    sapid::progress(40.)
                        .theme(theme.clone())
                        .name("Loading results")
                        .width(Length::Px(220.))
                        .indeterminate(true)
                        .animated(animated),
                    sapid::ring_progress(40.)
                        .theme(theme.clone())
                        .name("Unknown upload")
                        .size(96.)
                        .thickness(8.)
                        .indeterminate(true)
                        .animated(animated),
                    sapid::semi_circle_progress(40.)
                        .theme(theme.clone())
                        .name("Unknown processing")
                        .size(120.)
                        .thickness(10.)
                        .indeterminate(true)
                        .animated(animated),
                    sapid::ring_progress(72.)
                        .theme(theme.clone())
                        .name("Upload")
                        .value_text("72 percent uploaded")
                        .size(112.)
                        .thickness(10.),
                )),
            ))
            .show_in(scope)
    })
}
