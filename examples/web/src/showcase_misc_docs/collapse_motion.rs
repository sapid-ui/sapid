use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let animated = scope.cage(true);
        let completed = scope.cage("Open the section to view delivery information.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Animate delivery section")
                    .checked(animated)
                    .on_checked_change(move |v| animated.revise(|s| *s = v)),
                sapid::collapsible("delivery-motion")
                    .theme(theme.clone())
                    .label("Show delivery section")
                    .height_transition(400)
                    .animated(animated)
                    .on_transition_end(move |open| {
                        completed.revise(|s| {
                            *s = if open {
                                "Section opened.".to_owned()
                            } else {
                                "Section closed.".to_owned()
                            }
                        })
                    })
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::text().theme(theme.clone()).content(
                                "Your order will be delivered to the studio reception desk.",
                            )
                        }
                    }),
                sapid::text().theme(theme.clone()).content(completed),
            ))
            .show_in(scope)
    })
}
