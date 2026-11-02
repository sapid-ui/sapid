//! Native scrolling viewports with bounded, ordered application intents.
use crate::appearance::{Common, configure, merge, observe};
use crate::composite::{self, Callback};
use crate::state::{DelayFactory, OwnerDelay};
use glory::render::{
    Action, Event, EventKind, FocusMode, NodeId, Orientation, Query, QueryValue, ScrollAxes,
    ScrollCommand, ScrollMetrics, ScrollMovement, Semantics,
};
use glory::style::{Edges, Length, Overflow, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Bond, Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
    time::Duration,
};

pub const MAX_PENDING_SCROLL_INTENTS: usize = 128;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scrollbars {
    #[default]
    Auto,
    Always,
    Hover,
    Scroll,
    Never,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollbarOffset {
    #[default]
    None,
    Both,
    Horizontal,
    Vertical,
    Present,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverscrollBehavior {
    #[default]
    Auto,
    Contain,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct PresentationSettings {
    bars: Scrollbars,
    size: f64,
    offset: ScrollbarOffset,
    overscroll: OverscrollBehavior,
    hide_delay: Duration,
}
type ScrollDelay = Rc<dyn Fn(Duration) -> glory::core::LocalFuture>;
struct Presentation {
    settings: Cage<PresentationSettings>,
    visible: Cage<bool>,
    hovered: Cage<bool>,
    pending: Rc<OwnerDelay>,
    delay: Option<ScrollDelay>,
}
impl Presentation {
    fn activity(self: &Rc<Self>, scope: &Scope, hover: Option<bool>) -> Result {
        if let Some(hover) = hover {
            self.hovered.try_revise(|value| *value = hover)?;
        }
        let settings = self.settings.try_get()?;
        if !matches!(settings.bars, Scrollbars::Hover | Scrollbars::Scroll) {
            return Ok(());
        }
        if settings.bars == Scrollbars::Hover && self.hovered.try_get()? {
            self.pending.cancel();
            self.visible.try_revise(|value| *value = true)?;
            return Ok(());
        }
        if settings.bars == Scrollbars::Scroll && hover.is_some() {
            return Ok(());
        }
        self.visible.try_revise(|value| *value = true)?;
        let duration = settings.hide_delay;
        let factory: Option<DelayFactory> = self.delay.as_ref().map(|delay| {
            let delay = delay.clone();
            Rc::new(move || delay(duration)) as DelayFactory
        });
        let visible = self.visible;
        self.pending.run(scope, factory.as_ref(), move || {
            visible.try_revise(|value| *value = false)?;
            Ok(())
        })
    }
}
#[derive(Clone, Copy, Debug)]
enum Intent {
    Command(ScrollCommand),
    To(f64, f64),
    By(f64, f64),
    Fraction(f64, f64),
}
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn position(
    metrics: ScrollMetrics,
    intents: &mut VecDeque<Intent>,
    step: f64,
    axes: ScrollAxes,
) -> Result<(f64, f64)> {
    let mut x = metrics.x;
    let mut y = metrics.y;
    let max_x = (metrics.content_width - metrics.viewport_width).max(0.0);
    let max_y = (metrics.content_height - metrics.viewport_height).max(0.0);
    // The shared ScrollTo contract currently has a nonnegative assignment domain.
    // Raw metrics preserve RTL/elastic negatives rather than inventing a range.
    if x < 0.0 || y < 0.0 {
        return Err(glory::render::Error::Unsupported(
            "scroll commands on negative RTL/elastic offsets",
        )
        .into());
    }
    while let Some(intent) = intents.pop_front() {
        match intent {
            Intent::To(next_x, next_y) => {
                if next_x != 0.0 && !axes.allows(Orientation::Horizontal)
                    || next_y != 0.0 && !axes.allows(Orientation::Vertical)
                {
                    return Err(invalid("pending scroll destination uses an excluded axis"));
                }
                x = next_x.clamp(0.0, max_x);
                y = next_y.clamp(0.0, max_y);
            }
            Intent::By(dx, dy) => {
                if dx != 0.0 && !axes.allows(Orientation::Horizontal)
                    || dy != 0.0 && !axes.allows(Orientation::Vertical)
                {
                    return Err(invalid("pending scroll delta uses an excluded axis"));
                }
                x = (x + dx).clamp(0.0, max_x);
                y = (y + dy).clamp(0.0, max_y);
            }
            Intent::Fraction(next_x, next_y) => {
                if next_x != 0.0 && !axes.allows(Orientation::Horizontal)
                    || next_y != 0.0 && !axes.allows(Orientation::Vertical)
                {
                    return Err(invalid("pending scroll fraction uses an excluded axis"));
                }
                x = next_x * max_x;
                y = next_y * max_y;
            }
            Intent::Command(command) => {
                if !axes.allows(command.axis) {
                    return Err(invalid("pending scroll command uses an excluded axis"));
                }
                let (value, max, page) = match command.axis {
                    Orientation::Horizontal => (&mut x, max_x, metrics.viewport_width),
                    Orientation::Vertical => (&mut y, max_y, metrics.viewport_height),
                };
                *value = match command.movement {
                    ScrollMovement::StepNegative => (*value - step).max(0.0),
                    ScrollMovement::StepPositive => (*value + step).min(max),
                    ScrollMovement::PageNegative => (*value - page).max(0.0),
                    ScrollMovement::PagePositive => (*value + page).min(max),
                    ScrollMovement::Start => 0.0,
                    ScrollMovement::End => max,
                }
                .clamp(0.0, max);
            }
        }
    }
    Ok((x, y))
}
struct Controller {
    weak: WeakScope,
    owner: glory::core::InstanceId,
    live: Cell<bool>,
    viewport: Cage<Option<NodeId>>,
    metrics: Cage<Option<ScrollMetrics>>,
    axes: Vary<ScrollAxes>,
    step: f64,
    queue: RefCell<VecDeque<Intent>>,
    requesting: Cell<bool>,
    observed: Callback<ScrollMetrics>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn request(self: &Rc<Self>) -> Result {
        self.ensure()?;
        if self.requesting.replace(true) {
            return Ok(());
        }
        let result = (|| {
            let scope = self.weak.upgrade().ok_or(glory::ui::Error::Gone)?;
            let node = self.viewport.try_get()?.ok_or(glory::ui::Error::Gone)?;
            let controller = self.clone();
            scope.request(Query::ScrollMetrics(node), move |result| {
                controller.complete(result)
            })?;
            Ok(())
        })();
        if result.is_err() {
            self.requesting.set(false);
            let queue = self.queue.replace(VecDeque::new());
            drop(queue);
        }
        result
    }
    fn enqueue(self: &Rc<Self>, intent: Intent) -> Result {
        self.ensure()?;
        {
            let mut queue = self.queue.borrow_mut();
            if queue.len() >= MAX_PENDING_SCROLL_INTENTS {
                return Err(invalid("pending scroll intent budget exceeded"));
            }
            queue.push_back(intent);
        }
        self.request()
    }
    fn complete(self: &Rc<Self>, result: std::result::Result<QueryValue, glory::render::Error>) {
        if self.ensure().is_err() {
            return;
        }
        self.requesting.set(false);
        let Some(scope) = self.weak.upgrade() else {
            return;
        };
        let mut intents = std::mem::take(&mut *self.queue.borrow_mut());
        let result = (|| -> Result {
            let QueryValue::ScrollMetrics(metrics) = result? else {
                return Err(invalid("scroll metrics result mismatch"));
            };
            metrics.validate()?;
            let assignment = (|| -> Result {
                if intents.is_empty() {
                    return Ok(());
                }
                let (x, y) = position(metrics, &mut intents, self.step, self.axes.try_get()?)?;
                let node = self.viewport.try_get()?.ok_or(glory::ui::Error::Gone)?;
                scope.action(Action::ScrollTo { node, x, y })
            })();
            self.metrics.try_revise(|value| *value = Some(metrics))?;
            self.observed.emit(self.owner, metrics);
            assignment
        })();
        if let Err(error) = result {
            scope.report_error(error);
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.observed.clear();
        let queue = self.queue.replace(VecDeque::new());
        drop(queue);
    }
}
#[derive(Clone)]
pub struct ScrollHandle {
    inner: Rc<Controller>,
    metrics: Bond<Option<ScrollMetrics>>,
}
impl ScrollHandle {
    /// Resolve a fractional destination against actual extents at the request barrier.
    pub fn scroll_to_fraction(&self, x: f64, y: f64) -> Result {
        self.inner.ensure()?;
        if !x.is_finite()
            || !y.is_finite()
            || !(0.0..=1.0).contains(&x)
            || !(0.0..=1.0).contains(&y)
        {
            return Err(invalid("scroll fractions must be finite within 0..=1"));
        }
        let axes = self.inner.axes.try_get()?;
        if x != 0.0 && !axes.allows(Orientation::Horizontal)
            || y != 0.0 && !axes.allows(Orientation::Vertical)
        {
            return Err(invalid("scroll fraction uses an excluded axis"));
        }
        self.inner.enqueue(Intent::Fraction(x, y))
    }
    pub fn metrics(&self) -> Bond<Option<ScrollMetrics>> {
        self.metrics
    }
    pub fn refresh(&self) -> Result {
        self.inner.request()
    }
    pub fn command(&self, command: ScrollCommand) -> Result {
        self.inner.ensure()?;
        if !self.inner.axes.try_get()?.allows(command.axis) {
            return Err(invalid("scroll command uses an excluded axis"));
        }
        self.inner.enqueue(Intent::Command(command))
    }
    pub fn scroll_to(&self, x: f64, y: f64) -> Result {
        self.inner.ensure()?;
        if !x.is_finite() || !y.is_finite() || x < 0.0 || y < 0.0 {
            return Err(invalid("scroll position must be finite and nonnegative"));
        }
        if x != 0.0 && !self.inner.axes.try_get()?.allows(Orientation::Horizontal)
            || y != 0.0 && !self.inner.axes.try_get()?.allows(Orientation::Vertical)
        {
            return Err(invalid("scroll position uses an excluded axis"));
        }
        self.inner.enqueue(Intent::To(x, y))
    }
    pub fn scroll_by(&self, x: f64, y: f64) -> Result {
        self.inner.ensure()?;
        if !x.is_finite() || !y.is_finite() {
            return Err(invalid("scroll delta must be finite"));
        }
        if x != 0.0 && !self.inner.axes.try_get()?.allows(Orientation::Horizontal)
            || y != 0.0 && !self.inner.axes.try_get()?.allows(Orientation::Vertical)
        {
            return Err(invalid("scroll delta uses an excluded axis"));
        }
        self.inner.enqueue(Intent::By(x, y))
    }
}
#[derive(Clone)]
pub struct ScrollContext {
    pub scroll: ScrollHandle,
    pub theme: crate::Theme,
    pub size: crate::Size,
    pub unstyled: bool,
}
type Factory = Box<dyn FnMut(ScrollContext) -> Box<dyn Widget>>;
type ScrollChange = Box<dyn FnMut((f64, f64))>;
pub struct ScrollArea {
    common: Common,
    key: String,
    axes: Vary<ScrollAxes>,
    scrollbars: Vary<Scrollbars>,
    scrollbar_size: Vary<f64>,
    offset_scrollbars: Vary<ScrollbarOffset>,
    overscroll: Vary<OverscrollBehavior>,
    hide_delay: Vary<Duration>,
    delay: Option<ScrollDelay>,
    keyboard: bool,
    step: f64,
    content: Option<Factory>,
    content_ref: Option<Cage<Option<NodeId>>>,
    observed: Option<Box<dyn FnMut(ScrollMetrics)>>,
    scroll_change: Option<ScrollChange>,
    ready: Option<Box<dyn FnMut(ScrollHandle)>>,
}
pub fn scroll_area(key: impl Into<String>) -> ScrollArea {
    ScrollArea {
        common: Common::default(),
        key: key.into(),
        axes: ScrollAxes::Vertical.into(),
        scrollbars: Scrollbars::Auto.into(),
        scrollbar_size: 12.0.into(),
        offset_scrollbars: ScrollbarOffset::None.into(),
        overscroll: OverscrollBehavior::Auto.into(),
        hide_delay: Duration::from_millis(500).into(),
        delay: None,
        keyboard: true,
        step: 40.0,
        content: None,
        content_ref: None,
        observed: None,
        scroll_change: None,
        ready: None,
    }
}
macro_rules! methods {
    () => {
        pub fn theme(mut self, value: crate::Theme) -> Self {
            self.common.theme = value;
            self
        }
        pub fn size(mut self, value: crate::Size) -> Self {
            self.common.size = value;
            self
        }
        pub fn unstyled(mut self) -> Self {
            self.common.unstyled = true;
            self
        }
        pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
            self.common.class = value.into();
            self
        }
        pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
            self.common.style = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
            self.common.name = Some(value.into());
            self
        }
        pub fn node_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
            self.common.node_ref = Some(value);
            self
        }
        pub fn on(mut self, kind: EventKind, callback: impl FnMut(Event) + 'static) -> Self {
            self.common
                .handlers
                .entry(kind)
                .or_default()
                .push(Box::new(callback));
            self
        }
    };
}
impl ScrollArea {
    methods!();
    pub fn axes(mut self, value: impl Into<Vary<ScrollAxes>>) -> Self {
        self.axes = value.into();
        self
    }
    pub fn scrollbars(mut self, value: impl Into<Vary<Scrollbars>>) -> Self {
        self.scrollbars = value.into();
        self
    }
    pub fn scrollbar_size(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.scrollbar_size = value.into();
        self
    }
    pub fn offset_scrollbars(mut self, value: impl Into<Vary<ScrollbarOffset>>) -> Self {
        self.offset_scrollbars = value.into();
        self
    }
    pub fn overscroll_behavior(mut self, value: impl Into<Vary<OverscrollBehavior>>) -> Self {
        self.overscroll = value.into();
        self
    }
    pub fn scroll_hide_delay(mut self, value: impl Into<Vary<Duration>>) -> Self {
        self.hide_delay = value.into();
        self
    }
    /// Explicit host timer; no browser, clock or global executor is installed by the widget.
    pub fn delay<F: std::future::Future<Output = ()> + 'static>(
        mut self,
        factory: impl Fn(Duration) -> F + 'static,
    ) -> Self {
        self.delay = Some(Rc::new(move |duration| Box::pin(factory(duration))));
        self
    }
    pub fn keyboard(mut self, value: bool) -> Self {
        self.keyboard = value;
        self
    }
    pub fn step(mut self, value: f64) -> Self {
        self.step = value;
        self
    }
    pub fn content<W: Widget>(
        mut self,
        mut factory: impl FnMut(ScrollContext) -> W + 'static,
    ) -> Self {
        self.content = Some(Box::new(move |context| Box::new(factory(context))));
        self
    }
    pub fn content_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.content_ref = Some(value);
        self
    }
    pub fn on_metrics_change(mut self, callback: impl FnMut(ScrollMetrics) + 'static) -> Self {
        self.observed = Some(Box::new(callback));
        self
    }
    pub fn on_scroll_change(mut self, callback: impl FnMut((f64, f64)) + 'static) -> Self {
        self.scroll_change = Some(Box::new(callback));
        self
    }
    pub fn on_ready(mut self, callback: impl FnMut(ScrollHandle) + 'static) -> Self {
        self.ready = Some(Box::new(callback));
        self
    }
    fn controller(&mut self, scope: &Scope) -> Result<ScrollHandle> {
        composite::id("scroll", &self.key, "viewport", "viewport")?;
        if !self.step.is_finite() || self.step <= 0.0 || self.step > 1_000_000.0 {
            return Err(invalid("scroll step must be finite and in 0..=1000000"));
        }
        let metrics = scope.cage(None);
        let inner = Rc::new(Controller {
            weak: scope.downgrade(),
            owner: scope.id(),
            live: Cell::new(true),
            viewport: self.common.node_ref.unwrap_or_else(|| scope.cage(None)),
            metrics,
            axes: self.axes.clone(),
            step: self.step,
            queue: RefCell::new(VecDeque::new()),
            requesting: Cell::new(false),
            observed: Callback::new(self.observed.take()),
        });
        let cleanup = inner.clone();
        scope.on_cleanup(move || cleanup.retire())?;
        Ok(ScrollHandle {
            inner,
            metrics: scope.bond(move || metrics.get()),
        })
    }
    fn mount_view(&mut self, scope: &Scope, handle: ScrollHandle) -> Result {
        let owner = scope.id();
        let reference = handle.inner.viewport;
        let bars = self.scrollbars.clone();
        let size = self.scrollbar_size.clone();
        let offset = self.offset_scrollbars.clone();
        let overscroll = self.overscroll.clone();
        let hide_delay = self.hide_delay.clone();
        let timed = self.delay.is_some();
        let raw: Vary<PresentationSettings> = scope
            .bond(move || PresentationSettings {
                bars: bars.get(),
                size: size.get(),
                offset: offset.get(),
                overscroll: overscroll.get(),
                hide_delay: hide_delay.get(),
            })
            .into();
        let settings = composite::validated(scope, &raw, move |value| {
            if !value.size.is_finite()
                || !(1.0..=32.0).contains(&value.size)
                || value.size.fract() != 0.0
                || value.hide_delay > Duration::from_secs(10)
            {
                return Err(invalid(
                    "scrollbar size must be an integer in 1..=32 and hide delay at most 10 seconds",
                ));
            }
            if !timed
                && !value.hide_delay.is_zero()
                && matches!(value.bars, Scrollbars::Hover | Scrollbars::Scroll)
            {
                return Err(invalid(
                    "hover/scroll scrollbar delays require an explicit host timer",
                ));
            }
            Ok(())
        })?;
        let presentation = Rc::new(Presentation {
            settings,
            visible: scope.cage(false),
            hovered: scope.cage(false),
            pending: OwnerDelay::new(scope)?,
            delay: self.delay.take(),
        });
        let visible = presentation.visible;
        let hovered = presentation.hovered;
        let original = self.common.class.clone();
        self.common.class = scope.bond(move || {
            let value = settings.get();
            let behavior = match value.bars { Scrollbars::Auto => "auto", Scrollbars::Always => "always", Scrollbars::Hover => "hover", Scrollbars::Scroll => "scroll", Scrollbars::Never => "never" };
            let overscroll = match value.overscroll { OverscrollBehavior::Auto => "auto", OverscrollBehavior::Contain => "contain", OverscrollBehavior::None => "none" };
            format!("{} sapid-scroll-{behavior} sapid-scroll-size-{} sapid-overscroll-{overscroll}{}", original.get(), value.size as u8, if visible.get() || value.bars == Scrollbars::Hover && hovered.get() { " is-scrolling" } else { "" })
        }).into();
        let axes = self.axes.clone();
        let mut structural = Style::new()
            .width(Length::Percent(100.0))
            .height(Length::Px(240.0));
        if !self.common.unstyled {
            structural.color = Some(self.common.theme.foreground);
            structural.background = Some(self.common.theme.surface);
        }
        let custom = self.common.style.clone();
        let mut viewport = configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-scroll-area",
            Style::new()
        )
        .style(scope.bond(move || {
            let mut style = structural.clone();
            let overflow = if settings.get().bars == Scrollbars::Always {
                Overflow::Scroll
            } else {
                Overflow::Auto
            };
            style.overflow_x = Some(if axes.get().allows(Orientation::Horizontal) {
                overflow
            } else {
                Overflow::Hidden
            });
            style.overflow_y = Some(if axes.get().allows(Orientation::Vertical) {
                overflow
            } else {
                Overflow::Hidden
            });
            merge(style, custom.get())
        }))
        .focus(if self.keyboard {
            FocusMode::TabStop
        } else {
            FocusMode::Programmatic
        })
        .node_ref(reference)
        .semantics({
            let id = composite::id("scroll", &self.key, "viewport", "viewport")?;
            let axes = self.axes.clone();
            let keyboard = self.keyboard;
            scope.bond(move || {
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    scroll_keyboard: keyboard.then(|| axes.get()),
                    ..Default::default()
                })
            })
        });
        if self.scrollbars.is_reactive()
            || self.scrollbars.try_get()? == Scrollbars::Hover
            || self.common.handlers.contains_key(&EventKind::PointerWithin)
        {
            let pointer_presentation = presentation.clone();
            let weak = scope.downgrade();
            let mut pointer_observers = self
                .common
                .handlers
                .remove(&EventKind::PointerWithin)
                .unwrap_or_default();
            viewport = viewport.on(EventKind::PointerWithin, move |event| {
                if let Some(scope) = weak.upgrade() {
                    if let Event::PointerWithin { phase, .. } = event
                        && let Err(error) = pointer_presentation
                            .activity(&scope, Some(phase != glory::render::PointerPhase::Leave))
                    {
                        scope.report_error(error);
                    }
                    observe(&mut pointer_observers, owner, event);
                }
            });
        }
        if self.keyboard {
            let handle = handle.clone();
            let weak = scope.downgrade();
            let mut generic = self
                .common
                .handlers
                .remove(&EventKind::ScrollKeyboard)
                .unwrap_or_default();
            viewport = viewport.on(EventKind::ScrollKeyboard, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                if let Event::ScrollKeyboard { command, .. } = event
                    && let Err(error) = handle.command(command)
                {
                    scope.report_error(error);
                    return;
                }
                observe(&mut generic, owner, event);
            });
        }
        let callback = Callback::new(self.scroll_change.take());
        let cleanup = callback.clone();
        scope.on_cleanup(move || cleanup.clear())?;
        for kind in [EventKind::Scroll, EventKind::Resize] {
            let handle = handle.clone();
            let weak = scope.downgrade();
            let callback = callback.clone();
            let mut generic = self.common.handlers.remove(&kind).unwrap_or_default();
            let presentation = presentation.clone();
            viewport = viewport.on(kind, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                if let Event::Scroll { x, y, .. } = event {
                    if let Err(error) = presentation.activity(&scope, None) {
                        scope.report_error(error);
                        return;
                    }
                    callback.emit(owner, (x, y));
                    if !owner.is_alive() {
                        return;
                    }
                }
                if let Err(error) = handle.refresh() {
                    scope.report_error(error);
                    return;
                }
                observe(&mut generic, owner, event);
            });
        }
        for (kind, mut callbacks) in std::mem::take(&mut self.common.handlers) {
            viewport = viewport.on(kind, move |event| observe(&mut callbacks, owner, event));
        }
        viewport.show_in(scope)?;
        let parent = scope.child_in(reference.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        let handle_resize = handle.clone();
        let metrics = handle.metrics();
        let axes = self.axes.clone();
        let weak = scope.downgrade();
        let mut content = glory::ui::view()
            .class("sapid-scroll-content")
            .style(scope.bond(move || {
                let value = settings.get();
                let present = metrics.get();
                let horizontal = axes.get().allows(Orientation::Horizontal)
                    && match value.offset {
                        ScrollbarOffset::Both | ScrollbarOffset::Horizontal => true,
                        ScrollbarOffset::Present => {
                            present.is_some_and(|m| m.content_width > m.viewport_width)
                        }
                        _ => false,
                    };
                let vertical = axes.get().allows(Orientation::Vertical)
                    && match value.offset {
                        ScrollbarOffset::Both | ScrollbarOffset::Vertical => true,
                        ScrollbarOffset::Present => {
                            present.is_some_and(|m| m.content_height > m.viewport_height)
                        }
                        _ => false,
                    };
                Style::new().padding(Edges {
                    top: 0.0,
                    left: 0.0,
                    right: if vertical { value.size } else { 0.0 },
                    bottom: if horizontal { value.size } else { 0.0 },
                })
            }))
            .on(EventKind::Resize, move |_| {
                if let Err(error) = handle_resize.refresh()
                    && let Some(scope) = weak.upgrade()
                {
                    scope.report_error(error);
                }
            });
        if let Some(reference) = self.content_ref {
            content = content.node_ref(reference);
        }
        if let Some(mut factory) = self.content.take() {
            let context = ScrollContext {
                scroll: handle.clone(),
                theme: self.common.theme.clone(),
                size: self.common.size,
                unstyled: self.common.unstyled,
            };
            content = content.fill(glory::ui::widget(move |scope| {
                factory(context.clone()).show_in(scope)
            }));
        }
        content.show_in(&parent)?;
        scope.retain(parent)?;
        if let Some(mut callback) = self.ready.take() {
            callback(handle);
        }
        Ok(())
    }
}
impl Widget for ScrollArea {
    fn build(&mut self, scope: &Scope) -> Result {
        let handle = self.controller(scope)?;
        self.mount_view(scope, handle)
    }
}

pub struct Scroller {
    area: ScrollArea,
    axis: Orientation,
}
pub fn scroller(key: impl Into<String>) -> Scroller {
    Scroller {
        area: scroll_area(key).axes(ScrollAxes::Horizontal),
        axis: Orientation::Horizontal,
    }
}
impl Scroller {
    pub fn area(mut self, configure: impl FnOnce(ScrollArea) -> ScrollArea) -> Self {
        self.area = configure(self.area);
        self.area.axes = match self.axis {
            Orientation::Horizontal => ScrollAxes::Horizontal,
            Orientation::Vertical => ScrollAxes::Vertical,
        }
        .into();
        self
    }
    pub fn axis(mut self, value: Orientation) -> Self {
        self.axis = value;
        self.area.axes = match value {
            Orientation::Horizontal => ScrollAxes::Horizontal,
            Orientation::Vertical => ScrollAxes::Vertical,
        }
        .into();
        self
    }
    pub fn content<W: Widget>(mut self, factory: impl FnMut(ScrollContext) -> W + 'static) -> Self {
        self.area = self.area.content(factory);
        self
    }
}
impl Widget for Scroller {
    fn build(&mut self, scope: &Scope) -> Result {
        let handle = self.area.controller(scope)?;
        let metrics = handle.metrics();
        let axis = self.axis;
        for (label, movement, forward) in [
            ("Previous scroll page", ScrollMovement::PageNegative, false),
            ("Next scroll page", ScrollMovement::PagePositive, true),
        ] {
            let request = handle.clone();
            let weak = scope.downgrade();
            let enabled = scope.bond(move || {
                metrics.get().is_some_and(|metrics| {
                    let (offset, viewport, content) = match axis {
                        Orientation::Horizontal => {
                            (metrics.x, metrics.viewport_width, metrics.content_width)
                        }
                        Orientation::Vertical => {
                            (metrics.y, metrics.viewport_height, metrics.content_height)
                        }
                    };
                    offset >= 0.0
                        && if forward {
                            offset + viewport < content - 0.5
                        } else {
                            offset > 0.5
                        }
                })
            });
            crate::button()
                .theme(self.area.common.theme.clone())
                .size(self.area.common.size)
                .label(label)
                .enabled(enabled)
                .on_press(move || {
                    if let Err(error) = request.command(ScrollCommand { axis, movement })
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error);
                    }
                })
                .show_in(scope)?;
        }
        self.area.mount_view(scope, handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered_clamping_is_not_an_unbounded_delta_sum() {
        let metrics = ScrollMetrics {
            y: 0.0,
            viewport_height: 100.0,
            content_height: 500.0,
            ..Default::default()
        };
        let mut queue = VecDeque::from([Intent::By(0.0, -100.0), Intent::By(0.0, 50.0)]);
        assert_eq!(
            position(metrics, &mut queue, 40.0, ScrollAxes::Both).unwrap(),
            (0.0, 50.0)
        );
        let mut queue = VecDeque::from([
            Intent::To(0.0, 900.0),
            Intent::By(0.0, -40.0),
            Intent::Command(ScrollCommand {
                axis: Orientation::Vertical,
                movement: ScrollMovement::PageNegative,
            }),
        ]);
        assert_eq!(
            position(metrics, &mut queue, 40.0, ScrollAxes::Both).unwrap(),
            (0.0, 260.0)
        );
        assert!(queue.is_empty());
    }
}
