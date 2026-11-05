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
        let draft = scope.cage("Project notes".to_string());
        let saved = scope.cage(None::<String>);
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::text_field()
                    .theme(theme.clone())
                    .name("Draft title")
                    .value(draft)
                    .on_value_change(move |v| draft.revise(|o| *o = v)),
                {
                    let handle = handle.clone();
                    let token = token.clone();
                    sapid::button()
                        .theme(theme.clone())
                        .label("Save draft")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    let title = draft.get();
                                    saved.revise(|old| *old = Some(title.clone()));
                                    handle.show(sapid::notification_request(
                                        "saved",
                                        "Draft saved",
                                        format!("{title} is saved in this example."),
                                    ))?;
                                    status.revise(|o| *o = "Draft saved".into());
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                sapid::toaster("notify-save")
                    .theme(theme.clone())
                    .capacity(8)
                    .visible_limit(3)
                    .on_ready(move |h| *ready.borrow_mut() = Some(h)),
                sapid::text().theme(theme.clone()).content(status),
                sapid::text()
                    .theme(theme.clone())
                    .content(saved.map(|value| {
                        value.as_ref().map_or_else(
                            || "No draft saved yet".to_owned(),
                            |title| format!("Saved draft: {title}"),
                        )
                    })),
            ))
            .show_in(scope)
    })
}
