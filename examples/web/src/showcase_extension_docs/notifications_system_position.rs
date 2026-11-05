use glory::prelude::*;
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = Rc::new(RefCell::new(None::<NotificationsHandle>));
        let ready = handle.clone();
        let status = scope.cage("Ready".to_string());
        let token = Rc::new(RefCell::new(None::<NotificationToken>));
        let _ = &token;

        sapid::stack()
            .theme(theme.clone())
            .fill((
                {
                    let handle = handle.clone();
                    let token = token.clone();
                    sapid::button()
                        .theme(theme.clone())
                        .label("Show corner notification")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    handle.show(sapid::notification_request(
                                        "corner",
                                        "Saved at the corner",
                                        "Dismiss to clear this viewport message.",
                                    ))?;
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                sapid::affix("notification-position")
                    .theme(theme.clone())
                    .style(
                        Style::new()
                            .width(Length::Px(300.))
                            .max_width(glory::style::Limit::Percent(90.)),
                    )
                    .fill(
                        sapid::toaster("notify-position")
                            .theme(theme.clone())
                            .capacity(8)
                            .visible_limit(3)
                            .on_ready(move |h| *ready.borrow_mut() = Some(h)),
                    ),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
