use glory::prelude::*;
use glory::render::memory::Memory;
use glory::render::{
    Action, Capabilities, Error, Input, Mutation, Query, QueryToken, QueryValue, Receipt, Render,
    ScrollAxes, ScrollMetrics,
};
use sapid::{ScrollHandle, scroll_area};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Default)]
struct Wire {
    requests: Vec<(QueryToken, u64)>,
    inputs: Vec<Input>,
    actions: Vec<Action>,
}
struct Host {
    memory: Memory,
    wire: Rc<RefCell<Wire>>,
    applied: u64,
}
impl Render for Host {
    fn style_profile(&self) -> glory::style::Profile {
        glory::style::Profile::RECORD
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            scroll: true,
            measure: true,
            ..Default::default()
        }
    }
    fn apply(&mut self, mutations: &[Mutation]) -> std::result::Result<Receipt, Error> {
        let receipt = self.memory.apply(mutations)?;
        self.applied = receipt.applied;
        Ok(receipt)
    }
    fn request(&mut self, query: Query, token: QueryToken) -> std::result::Result<(), Error> {
        query.validate_target(&self.memory.node(query.node())?.descriptor)?;
        assert!(matches!(query, Query::ScrollMetrics(_)));
        self.wire.borrow_mut().requests.push((token, self.applied));
        Ok(())
    }
    fn action(&mut self, action: Action) -> std::result::Result<(), Error> {
        self.wire.borrow_mut().actions.push(action);
        Ok(())
    }
    fn drain(&mut self, inputs: &mut Vec<Input>) {
        inputs.append(&mut self.wire.borrow_mut().inputs);
    }
}
fn fixture() -> (App, Rc<RefCell<Wire>>, ScrollHandle) {
    let wire = Rc::new(RefCell::new(Wire::default()));
    let saved = wire.clone();
    let app = App::new(move |runtime| Host {
        memory: Memory::new(runtime),
        wire: saved,
        applied: 0,
    });
    let output = Rc::new(RefCell::new(None));
    let saved = output.clone();
    app.mount(
        scroll_area("test")
            .axes(ScrollAxes::Both)
            .on_ready(move |handle| *saved.borrow_mut() = Some(handle)),
    )
    .unwrap();
    let handle = output.borrow_mut().take().unwrap();
    (app, wire, handle)
}
fn reply(wire: &RefCell<Wire>, metrics: ScrollMetrics) {
    let (token, applied) = wire.borrow().requests[0];
    wire.borrow_mut().inputs.push(Input::Response {
        token,
        applied,
        result: Ok(QueryValue::ScrollMetrics(metrics)),
    });
}
#[test]
fn one_metrics_barrier_resolves_a_bounded_fifo_without_losing_edge_clamping_order() {
    let (app, wire, handle) = fixture();
    handle.scroll_by(0.0, -100.0).unwrap();
    handle.scroll_by(0.0, 50.0).unwrap();
    handle.refresh().unwrap();
    assert_eq!(wire.borrow().requests.len(), 1);
    reply(
        &wire,
        ScrollMetrics {
            viewport_width: 200.0,
            viewport_height: 100.0,
            content_width: 200.0,
            content_height: 500.0,
            ..Default::default()
        },
    );
    app.pump().unwrap();
    assert_eq!(wire.borrow().actions.len(), 1);
    assert!(matches!(
        wire.borrow().actions[0],
        Action::ScrollTo {
            x: 0.0,
            y: 50.0,
            ..
        }
    ));
    assert_eq!(
        handle.metrics().get().unwrap().y,
        0.0,
        "only actual query observations are published"
    );
}
#[test]
fn pending_commands_are_bounded_and_invalid_commands_do_not_create_queries() {
    let (app, wire, handle) = fixture();
    assert!(handle.scroll_to(f64::NAN, 0.0).is_err());
    assert!(handle.scroll_by(0.0, f64::INFINITY).is_err());
    assert!(wire.borrow().requests.is_empty());
    for _ in 0..sapid::MAX_PENDING_SCROLL_INTENTS {
        handle.scroll_by(0.0, 1.0).unwrap();
    }
    for _ in 0..1000 {
        assert!(handle.scroll_by(0.0, 1.0).is_err());
    }
    assert_eq!(wire.borrow().requests.len(), 1);
    reply(
        &wire,
        ScrollMetrics {
            viewport_height: 100.0,
            content_height: 1000.0,
            ..Default::default()
        },
    );
    app.pump().unwrap();
    assert!(matches!(
        wire.borrow().actions[0],
        Action::ScrollTo { y: 128.0, .. }
    ));
}
#[test]
fn retained_handle_retires_query_callback_captures_and_refs_with_owner() {
    struct Probe(Rc<Cell<usize>>);
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let app = App::memory();
    let dropped = Rc::new(Cell::new(0));
    let probe = Probe(dropped.clone());
    let reference = app.scope().cage(None);
    let output = Rc::new(RefCell::new(None));
    let saved = output.clone();
    app.mount(
        scroll_area("retire")
            .node_ref(reference)
            .on_metrics_change(move |_| {
                let _ = &probe;
            })
            .on_ready(move |handle| *saved.borrow_mut() = Some(handle)),
    )
    .unwrap();
    let handle = output.borrow_mut().take().unwrap();
    assert!(reference.get().is_some());
    app.clear().unwrap();
    assert_eq!(dropped.get(), 1);
    assert!(reference.get().is_none());
    assert!(handle.refresh().is_err());
    assert!(handle.scroll_by(0.0, 2.0).is_err());
}
#[test]
fn ssr_scroll_area_has_real_native_overflow_and_independent_keyboard_without_custom_dom() {
    let html = glory::ssr::render(
        scroll_area("ssr")
            .name("Documents")
            .scrollbars(sapid::Scrollbars::Always)
            .content(|_| glory::ui::text().content("Long content")),
    )
    .unwrap();
    assert!(html.contains("data-gly-scroll-keyboard=\"vertical\""));
    assert!(html.contains("overflow-y:scroll"));
    assert!(html.contains("tabindex=\"0\""));
    assert!(html.contains("Long content"));
}

#[test]
fn fractional_destination_uses_current_extents_and_shares_the_ordered_barrier() {
    let (app, wire, handle) = fixture();
    assert!(handle.scroll_to_fraction(0.0, f64::NAN).is_err());
    assert!(handle.scroll_to_fraction(1.01, 0.0).is_err());
    assert!(wire.borrow().requests.is_empty());
    handle.scroll_to_fraction(0.0, 0.5).unwrap();
    handle.scroll_by(0.0, 20.0).unwrap();
    assert_eq!(wire.borrow().requests.len(), 1);
    reply(
        &wire,
        ScrollMetrics {
            viewport_height: 100.0,
            content_height: 700.0,
            ..Default::default()
        },
    );
    app.pump().unwrap();
    assert!(matches!(
        wire.borrow().actions[0],
        Action::ScrollTo {
            x: 0.0,
            y: 320.0,
            ..
        }
    ));
    assert_eq!(handle.metrics().get().unwrap().y, 0.0);
}

#[test]
fn presentation_source_updates_do_not_reenter_the_app_flush_to_measure() {
    let wire = Rc::new(RefCell::new(Wire::default()));
    let saved = wire.clone();
    let app = App::new(move |runtime| Host {
        memory: Memory::new(runtime),
        wire: saved,
        applied: 0,
    });
    let size = app.scope().cage(12.0);
    let bars = app.scope().cage(sapid::Scrollbars::Auto);
    let output = Rc::new(RefCell::new(None));
    let captured = output.clone();
    app.mount(
        scroll_area("presentation")
            .scrollbar_size(size)
            .scrollbars(bars)
            .on_ready(move |handle| *captured.borrow_mut() = Some(handle)),
    )
    .unwrap();
    size.revise(|value| *value = 24.0);
    bars.revise(|value| *value = sapid::Scrollbars::Always);
    app.flush().unwrap();
    assert!(
        wire.borrow().requests.is_empty(),
        "presentation must wait for host resize/scroll observations or an explicit request"
    );
    let handle = output.borrow_mut().take().unwrap();
    handle.refresh().unwrap();
    assert_eq!(wire.borrow().requests.len(), 1);
    reply(
        &wire,
        ScrollMetrics {
            viewport_height: 200.0,
            content_height: 700.0,
            ..Default::default()
        },
    );
    app.pump().unwrap();
    assert_eq!(handle.metrics().get().unwrap().viewport_height, 200.0);
}

#[test]
fn scrollbar_settings_change_in_place_reject_invalid_updates_and_keep_content() {
    use glory::style::{Edges, Length, Overflow};
    use sapid::{ScrollbarOffset, Scrollbars};
    let app = App::memory();
    let size = app.scope().cage(12.0);
    let bars = app.scope().cage(Scrollbars::Auto);
    let axes = app.scope().cage(ScrollAxes::Both);
    let viewport = app.scope().cage(None);
    let content = app.scope().cage(None);
    let calls = Rc::new(Cell::new(0));
    let created = calls.clone();
    app.mount(
        scroll_area("settings")
            .node_ref(viewport)
            .content_ref(content)
            .scrollbar_size(size)
            .scrollbars(bars)
            .axes(axes)
            .offset_scrollbars(ScrollbarOffset::Both)
            .content(move |_| {
                created.set(created.get() + 1);
                glory::ui::input().name("Kept draft").value("Retained")
            }),
    )
    .unwrap();
    let root = viewport.get().unwrap();
    let body = content.get().unwrap();
    let editor = app.tree().node(body).unwrap().children[0];
    size.revise(|size| *size = f64::NAN);
    assert!(app.flush().is_err());
    assert!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .classes
            .iter()
            .any(|class| class == "sapid-scroll-size-12")
    );
    size.revise(|size| *size = 18.0);
    bars.revise(|bars| *bars = Scrollbars::Never);
    axes.revise(|axes| *axes = ScrollAxes::Horizontal);
    app.flush().unwrap();
    assert_eq!((viewport.get(), content.get()), (Some(root), Some(body)));
    assert_eq!(app.tree().node(body).unwrap().children[0], editor);
    assert_eq!(calls.get(), 1);
    let tree = app.tree();
    let style = &tree.node(root).unwrap().descriptor.view().style;
    assert_eq!(style.overflow_x, Some(Overflow::Auto));
    assert_eq!(style.overflow_y, Some(Overflow::Hidden));
    assert_eq!(
        tree.node(body).unwrap().descriptor.view().style.padding,
        Some(Edges {
            top: 0.0,
            left: 0.0,
            bottom: 18.0,
            right: 0.0
        })
    );
    assert_eq!(style.width, Some(Length::Percent(100.0)));
    drop(tree);
    app.clear().unwrap();
    assert_eq!((viewport.get(), content.get()), (None, None));
}

#[test]
fn hover_hide_uses_one_cancellable_owner_task_and_cannot_revive_after_retirement() {
    use glory::render::{Event, PointerPhase};
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
        time::Duration,
    };
    struct Wait {
        done: Rc<Cell<bool>>,
        waker: Rc<RefCell<Option<Waker>>>,
        dropped: Rc<Cell<usize>>,
    }
    impl Future for Wait {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.done.get() {
                Poll::Ready(())
            } else {
                *self.waker.borrow_mut() = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
    impl Drop for Wait {
        fn drop(&mut self) {
            self.dropped.set(self.dropped.get() + 1);
        }
    }
    let executor = Rc::new(glory::core::executor::Executor::new(4));
    let app = App::with_host(executor.clone(), Memory::new);
    let done = Rc::new(Cell::new(false));
    let waker = Rc::new(RefCell::new(None));
    let dropped = Rc::new(Cell::new(0));
    let (timer_done, timer_waker, timer_dropped) = (done.clone(), waker.clone(), dropped.clone());
    app.mount(
        scroll_area("hover")
            .scrollbars(sapid::Scrollbars::Hover)
            .scroll_hide_delay(Duration::from_millis(500))
            .delay(move |duration| {
                assert_eq!(duration, Duration::from_millis(500));
                Wait {
                    done: timer_done.clone(),
                    waker: timer_waker.clone(),
                    dropped: timer_dropped.clone(),
                }
            }),
    )
    .unwrap();
    let root = app.tree().roots()[0];
    let pointer = |entered| {
        app.dispatch(
            root,
            Event::Pointer {
                from: root,
                related: None,
                phase: if entered {
                    PointerPhase::Enter
                } else {
                    PointerPhase::Leave
                },
            },
        )
        .unwrap();
    };
    let visible = || {
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .classes
            .iter()
            .any(|class| class == "is-scrolling")
    };
    pointer(true);
    assert!(visible());
    assert!(executor.is_empty());
    pointer(false);
    assert_eq!(executor.len(), 1);
    executor.poll_one();
    let late = waker.borrow().clone().unwrap();
    pointer(true);
    assert!(visible());
    assert!(executor.is_empty());
    assert_eq!(dropped.get(), 1);
    late.wake();
    executor.poll_one();
    assert!(visible());
    pointer(false);
    executor.poll_one();
    done.set(true);
    waker.borrow().as_ref().unwrap().wake_by_ref();
    executor.poll_one();
    app.flush().unwrap();
    assert!(!visible());
    done.set(false);
    pointer(true);
    pointer(false);
    executor.poll_one();
    let retired = waker.borrow().clone().unwrap();
    app.clear().unwrap();
    assert!(executor.is_empty());
    retired.wake();
    executor.poll_one();
    assert!(app.tree().is_empty());
}
