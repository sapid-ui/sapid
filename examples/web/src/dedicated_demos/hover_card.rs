use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(false);
        let body_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::hover_card("ada-profile")
                    .theme(theme.clone())
                    .label("About Ada")
                    .name("Ada profile")
                    .close_delay_with(close_delay)
                    .content(move || {
                        sapid::stack().theme(body_theme.clone()).gap(10.).fill((
                            sapid::text()
                                .theme(body_theme.clone())
                                .content("Ada builds thoughtful developer tools."),
                            sapid::text()
                                .theme(body_theme.clone())
                                .content("Design engineer · London"),
                            sapid::button()
                                .theme(body_theme.clone())
                                .label("Select Ada's profile")
                                .on_press(move || selected.revise(|v| *v = true)),
                        ))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|v| {
                        if *v {
                            "Selected profile: Ada".to_owned()
                        } else {
                            "Hover or focus About Ada to view the profile.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}

// This application supplies browser timing explicitly. Native hosts can supply
// their own local timer; this standalone example uses immediate native close.
fn close_delay() -> glory::core::LocalFuture {
    #[cfg(target_arch = "wasm32")]
    {
        Box::pin(timer::Delay::new(160))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::pin(std::future::ready(()))
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
