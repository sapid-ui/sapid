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
                        .label("Show save result")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    let result = handle.show(
                                        sapid::notification_request(
                                            "save-result",
                                            "Draft saved",
                                            "Version 1",
                                        )
                                        .action("Undo save", move || {
                                            status.revise(|o| *o = "Save undone".into())
                                        }),
                                    )?;
                                    *token.borrow_mut() = Some(result);
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
                        .label("Update save result")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    if let Some(token) = token.borrow().as_ref() {
                                        handle.update(token, "Draft synced", "Version 2")?;
                                    };
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
                        .label("Dismiss save result")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    if let Some(token) = token.borrow().as_ref() {
                                        handle.dismiss(token)?;
                                    };
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                sapid::toaster("notify-updates")
                    .theme(theme.clone())
                    .capacity(8)
                    .visible_limit(3)
                    .on_ready(move |h| *ready.borrow_mut() = Some(h)),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
