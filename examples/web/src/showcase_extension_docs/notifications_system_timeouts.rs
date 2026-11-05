use glory::prelude::*;
use sapid::prelude::*;
use std::time::Duration;
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
                        .label("Show timed message")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    handle.show(
                                        sapid::notification_request(
                                            "timed",
                                            "Temporary message",
                                            "Closes after two unpaused seconds.",
                                        )
                                        .timeout(Duration::from_secs(2)),
                                    )?;
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
                        .label("Pause timed messages")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    handle.pause(true)?;
                                    status.revise(|o| *o = "Timers paused".into());
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
                        .label("Resume timed messages")
                        .on_press(move || {
                            let _ = &token;
                            if let Some(handle) = handle.borrow().clone() {
                                let result = (|| -> glory::ui::Result {
                                    handle.pause(false)?;
                                    status.revise(|o| *o = "Timers resumed".into());
                                    Ok(())
                                })();
                                if let Err(error) = result {
                                    status.revise(|o| *o = format!("Request rejected: {error}"));
                                }
                            }
                        })
                },
                sapid::toaster("notify-timeouts")
                    .theme(theme.clone())
                    .capacity(8)
                    .visible_limit(3)
                    .clock(clock())
                    .pause_on_hover(true)
                    .on_ready(move |h| *ready.borrow_mut() = Some(h)),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
fn clock() -> NotificationClock {
    #[cfg(target_arch = "wasm32")]
    {
        let performance = web_sys::window().unwrap().performance().unwrap();
        NotificationClock::new(
            move || Duration::from_secs_f64(performance.now() / 1000.),
            move |duration| delay(duration.as_millis().clamp(1, i32::MAX as u128) as i32),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        NotificationClock::new(|| Duration::ZERO, |_| std::future::pending::<()>())
    }
}
#[cfg(target_arch = "wasm32")]
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
