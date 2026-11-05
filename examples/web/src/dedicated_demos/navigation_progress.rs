use glory::prelude::*;
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct Current {
    handle: Option<NavigationProgressHandle>,
    task: Option<glory::core::TaskToken>,
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = Rc::new(RefCell::new(Current::default()));
        let ready = current.clone();
        let retired = current.clone();
        scope.on_cleanup(move || {
            let pending = retired.borrow_mut().task.take();
            if let Some(task) = pending {
                task.cancel();
            }
        })?;
        let page = scope.cage("Project overview".to_string());
        let status = scope.cage("Ready".to_string());
        let mut links = Vec::new();
        for destination in ["Project overview", "Project reports"] {
            let current = current.clone();
            let weak = scope.downgrade();
            links.push(
                sapid::button()
                    .theme(theme.clone())
                    .label(format!("Open {destination}"))
                    .on_press(move || {
                        let previous = current.borrow_mut().task.take();
                        if let Some(task) = previous {
                            task.cancel();
                        }
                        let handle = current.borrow().handle.clone();
                        let Some(handle) = handle else { return };
                        let Some(owner) = weak.upgrade() else { return };
                        let result = (|| -> glory::ui::Result {
                            let token = handle.start()?;
                            handle.set(&token, 25.)?;
                            status.revise(|o| *o = format!("Loading {destination}…"));
                            let task = owner.spawn(async move {
                                delay(450).await;
                                if handle.set(&token, 70.).unwrap_or(false) {
                                    delay(450).await;
                                    if handle.finish(&token).unwrap_or(false) {
                                        page.revise(|o| *o = destination.into());
                                        status.revise(|o| *o = "Navigation complete".into());
                                    }
                                }
                            })?;
                            current.borrow_mut().task = Some(task);
                            Ok(())
                        })();
                        if let Err(error) = result {
                            owner.report_error(error)
                        }
                    }),
            );
        }
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::navigation_progress()
                    .theme(theme.clone())
                    .name("Project navigation")
                    .thickness(4.)
                    .on_ready(move |h| ready.borrow_mut().handle = Some(h)),
                sapid::group()
                    .theme(theme.clone())
                    .wrap(glory::style::FlexWrap::Wrap)
                    .fill(links),
                sapid::title(3)
                    .theme(theme.clone())
                    .weight(400.)
                    .content(page),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
fn delay(millis: i32) -> glory::core::LocalFuture {
    #[cfg(target_arch = "wasm32")]
    {
        std::boxed::Box::pin(timer::Delay::new(millis))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = millis;
        std::boxed::Box::pin(std::future::ready(()))
    }
}
#[cfg(target_arch = "wasm32")]
mod timer {
    use std::{
        cell::{Cell, RefCell},
        future::Future,
        pin::Pin,
        rc::Rc,
        task::{Context, Poll, Waker},
    };
    use wasm_bindgen::{JsCast, prelude::*};
    struct TimerState {
        done: Cell<bool>,
        waker: RefCell<Option<Waker>>,
    }
    pub struct Delay {
        millis: i32,
        state: Rc<TimerState>,
        timer: Option<i32>,
        callback: Option<Closure<dyn FnMut()>>,
    }
    impl Delay {
        pub fn new(millis: i32) -> Self {
            Self {
                millis,
                state: Rc::new(TimerState {
                    done: Cell::new(false),
                    waker: RefCell::new(None),
                }),
                timer: None,
                callback: None,
            }
        }
    }
    impl Future for Delay {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.state.done.get() {
                return Poll::Ready(());
            }
            let previous = self.state.waker.borrow_mut().replace(cx.waker().clone());
            drop(previous);
            if self.timer.is_none() {
                let state = self.state.clone();
                let callback = Closure::new(move || {
                    state.done.set(true);
                    let waker = state.waker.borrow_mut().take();
                    if let Some(waker) = waker {
                        waker.wake();
                    }
                });
                self.timer = Some(
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            callback.as_ref().unchecked_ref(),
                            self.millis,
                        )
                        .unwrap(),
                );
                self.callback = Some(callback);
            }
            Poll::Pending
        }
    }
    impl Drop for Delay {
        fn drop(&mut self) {
            if let Some(timer) = self.timer.take()
                && let Some(window) = web_sys::window()
            {
                window.clear_timeout_with_handle(timer);
            }
        }
    }
}
