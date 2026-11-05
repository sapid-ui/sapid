use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ready = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::badge()
                    .theme(theme.clone())
                    .left_section(sapid::Icon::Check)
                    .right_section("v2")
                    .content(ready.map(|r| {
                        if *r {
                            "Ready to publish".to_owned()
                        } else {
                            "Draft in review".to_owned()
                        }
                    })),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Ready to publish")
                    .checked(ready)
                    .on_checked_change(move |v| ready.revise(|r| *r = v)),
            ))
            .show_in(scope)
    })
}
