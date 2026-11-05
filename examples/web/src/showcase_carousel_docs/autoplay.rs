use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let run = scope.cage(false);
        let inner = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Run autoplay")
                    .checked(run)
                    .on_checked_change(move |next| run.revise(|old| *old = next)),
                sapid::carousel()
                    .theme(theme.clone())
                    .name("Automatic cards")
                    .count(3usize)
                    .looping(true)
                    .with_indicators(true)
                    .paused(run.map(|run| !*run))
                    .autoplay(clock)
                    .render_slide(move |index| {
                        sapid::text()
                            .theme(inner.clone())
                            .content(format!("Automatic card {}", index + 1))
                            .style(Style {
                                background: Some(inner.accent_soft),
                                ..Style::new().height(Length::Px(140.)).padding(Edges {
                                    top: 48.,
                                    bottom: 32.,
                                    left: 52.,
                                    right: 52.,
                                })
                            })
                    }),
            ))
            .show_in(scope)
    })
}
fn clock() -> glory::core::LocalFuture {
    #[cfg(target_arch = "wasm32")]
    return std::boxed::Box::pin(browser::Delay::new(2000));
    #[cfg(not(target_arch = "wasm32"))]
    std::boxed::Box::pin(std::future::pending())
}

#[cfg(target_arch = "wasm32")]
mod browser {
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
    pub(super) struct Delay {
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
