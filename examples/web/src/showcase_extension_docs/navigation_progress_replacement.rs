use glory::prelude::*;
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
#[derive(Default)]
struct Current {
    handle: Option<NavigationProgressHandle>,
    token: Option<NavigationProgressToken>,
    stale: Option<NavigationProgressToken>,
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = Rc::new(RefCell::new(Current::default()));
        let ready = current.clone();
        let status = scope.cage("Ready".to_string());
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::navigation_progress()
                    .theme(theme.clone())
                    .name("replacement progress")
                    .thickness(6.)
                    .on_ready(move |h| ready.borrow_mut().handle = Some(h)),
                sapid::group()
                    .theme(theme.clone())
                    .wrap(glory::style::FlexWrap::Wrap)
                    .fill((
                        {
                            let current = current.clone();
                            let weak = scope.downgrade();
                            sapid::button()
                                .theme(theme.clone())
                                .label("Start or replace route")
                                .on_press(move || {
                                    let handle = current.borrow().handle.clone();
                                    if let Some(handle) = handle {
                                        let result = (|| -> glory::ui::Result {
                                            let previous = current.borrow_mut().token.take();
                                            let next = handle.start()?;
                                            current.borrow_mut().stale = previous;
                                            current.borrow_mut().token = Some(next);
                                            status.revise(|o| *o = "Started current route".into());
                                            Ok(())
                                        })();
                                        if let Err(error) = result
                                            && let Some(owner) = weak.upgrade()
                                        {
                                            owner.report_error(error);
                                        }
                                    }
                                })
                        },
                        {
                            let current = current.clone();
                            let weak = scope.downgrade();
                            sapid::button()
                                .theme(theme.clone())
                                .label("Publish previous route")
                                .on_press(move || {
                                    let handle = current.borrow().handle.clone();
                                    if let Some(handle) = handle {
                                        let result = (|| -> glory::ui::Result {
                                            let token = current.borrow().stale.clone();
                                            if let Some(token) = token {
                                                let accepted = handle.set(&token, 90.)?;
                                                status.revise(|o| {
                                                    *o = if accepted {
                                                        "Accepted current route"
                                                    } else {
                                                        "Obsolete route ignored"
                                                    }
                                                    .into()
                                                });
                                            }
                                            Ok(())
                                        })();
                                        if let Err(error) = result
                                            && let Some(owner) = weak.upgrade()
                                        {
                                            owner.report_error(error);
                                        }
                                    }
                                })
                        },
                        {
                            let current = current.clone();
                            let weak = scope.downgrade();
                            sapid::button()
                                .theme(theme.clone())
                                .label("Report current progress")
                                .on_press(move || {
                                    let handle = current.borrow().handle.clone();
                                    if let Some(handle) = handle {
                                        let result = (|| -> glory::ui::Result {
                                            let token = current.borrow().token.clone();
                                            if let Some(token) = token {
                                                let accepted = handle.set(&token, 60.)?;
                                                status.revise(|o| {
                                                    *o = if accepted {
                                                        "Accepted current route"
                                                    } else {
                                                        "Obsolete route ignored"
                                                    }
                                                    .into()
                                                });
                                            }
                                            Ok(())
                                        })();
                                        if let Err(error) = result
                                            && let Some(owner) = weak.upgrade()
                                        {
                                            owner.report_error(error);
                                        }
                                    }
                                })
                        },
                    )),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
