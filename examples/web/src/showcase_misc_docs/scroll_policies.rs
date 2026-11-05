use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [
            sapid::Scrollbars::Auto,
            sapid::Scrollbars::Always,
            sapid::Scrollbars::Never,
            sapid::Scrollbars::Hover,
            sapid::Scrollbars::Scroll,
        ]
        .into_iter()
        .enumerate()
        .map(|(i, bars)| {
            let content = theme.clone();
            sapid::scroll_area(format!("policy-reading-{i}"))
                .theme(theme.clone())
                .scrollbars(bars)
                .scrollbar_size(8.)
                .scroll_hide_delay(std::time::Duration::from_millis(300))
                .delay(|duration| delay(duration.as_millis() as i32))
                .name(format!("{bars:?} reading region"))
                .style(
                    Style::new()
                        .height(Length::Px(110.))
                        .max_width(glory::style::Limit::Percent(100.)),
                )
                .content(move |_| {
                    sapid::text()
                        .theme(content.clone())
                        .content(TEXT_1)
                        .style(Style::new().min_height(Length::Px(260.)))
                })
        })
        .collect::<Vec<_>>(),
    )
}

const TEXT_1: &str = "Clear project notes make it easier to return to a task. Read the brief, review the latest decisions, keep the useful details together and share the next step with the team. This longer note demonstrates an ordinary local reading region rather than a replaced document.";

// Explicit application timer. Native applications can supply their own local
// clock Future; this standalone source uses immediate native completion.
fn delay(millis: i32) -> glory::core::LocalFuture {
    #[cfg(target_arch = "wasm32")]
    {
        Box::pin(timer::Delay::new(millis))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = millis;
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
