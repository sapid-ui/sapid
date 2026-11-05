use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let long = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Include the longer delivery note")
                    .checked(long)
                    .on_checked_change(move |v| long.revise(|s| *s = v)),
                sapid::collapsible("intrinsic-delivery")
                    .theme(theme.clone())
                    .label("Delivery note")
                    .default_open(true)
                    .height_transition(350)
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::text().theme(theme.clone()).content(long.map(|v| {
                                if *v {
                                    TEXT_1.to_owned()
                                } else {
                                    "Leave the package with reception.".to_owned()
                                }
                            }))
                        }
                    }),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "Leave the package with the reception desk near the main entrance. Please ask the receptionist to notify the design team when it arrives. Deliveries are accepted from nine in the morning until five in the afternoon on weekdays.";
