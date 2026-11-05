use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).gap(12.).fill(
        [sapid::AnchorSide::Top, sapid::AnchorSide::Bottom]
            .into_iter()
            .map(|side| {
                let body = theme.clone();
                sapid::hover_card(format!("profile-{side:?}"))
                    .theme(theme.clone())
                    .label(format!("{side:?} profile card"))
                    .name(format!("{side:?} profile details"))
                    .placement(glory::render::LayerPlacement::Anchor(
                        sapid::AnchorPlacement {
                            side,
                            arrow: Some(Default::default()),
                            ..Default::default()
                        },
                    ))
                    .close_delay_with(|| delay(160))
                    .content(move || {
                        sapid::stack().theme(body.clone()).gap(10.).fill((
                            sapid::text()
                                .theme(body.clone())
                                .content("Ada — design engineer"),
                            sapid::anchor()
                                .theme(body.clone())
                                .label("Read the profile card guide")
                                .href("https://mantine.dev/core/hover-card/"),
                        ))
                    })
            })
            .collect::<Vec<_>>(),
    )
}

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
