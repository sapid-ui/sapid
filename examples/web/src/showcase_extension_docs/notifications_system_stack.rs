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
                        .label("Queue three updates")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    for (key, title) in [
                                        ("one", "Design saved"),
                                        ("two", "Notes saved"),
                                        ("three", "Report saved"),
                                    ] {
                                        handle.show(sapid::notification_request(
                                            key,
                                            title,
                                            "A separate queued message.",
                                        ))?;
                                    }
                                    status.revise(|o| *o = "Three updates queued".into());
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                {
                    let handle = handle.clone();
                    let token = token.clone();
                    sapid::button()
                        .theme(theme.clone())
                        .label("Clear queued updates")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    handle.clear()?;
                                    status.revise(|o| *o = "Queue cleared".into());
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                sapid::toaster("notify-stack")
                    .theme(theme.clone())
                    .capacity(8)
                    .visible_limit(2)
                    .on_ready(move |h| *ready.borrow_mut() = Some(h)),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
