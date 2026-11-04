use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let opened = scope.cage(vec!["delivery".to_owned()]);
        sapid::accordion("controlled-delivery")
            .theme(theme.clone())
            .multiple(true)
            .open(opened)
            .on_open_change(move |next| opened.revise(|old| *old = next))
            .item(
                sapid::accordion_item("delivery")
                    .label("Delivery options")
                    .content(|| text().content("Choose standard or express delivery at checkout.")),
            )
            .item(
                sapid::accordion_item("payment")
                    .label("Payment methods")
                    .content(|| {
                        text().content("Your organization manages its accepted payment methods.")
                    }),
            )
            .item(
                sapid::accordion_item("unavailable")
                    .label("Archived information")
                    .enabled(false)
                    .content(|| text().content("This section is currently unavailable.")),
            )
            .show_in(scope)
    })
}
