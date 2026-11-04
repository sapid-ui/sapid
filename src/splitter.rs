//! Two stable panes and a finite, focusable separator.
use crate::appearance::{Common, configure, merge};
use crate::composite::{self, Callback, Observers};
use crate::state::{Control, Value};
use glory::render::{
    Action, DragPhase, Event, EventKind, FocusMode, NodeId, Orientation, Query, QueryToken,
    QueryValue, RangeCommand, RangeValue, Rect, Role, SemanticId, Semantics,
};
use glory::style::{
    Align, Display, Edges, FlexDirection, Justify, Length, Overflow, PointerEvents, Position, Style,
};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Snapshot {
    value: f64,
    min: f64,
    max: f64,
    enabled: bool,
}
impl Snapshot {
    fn validate(&self) -> Result {
        RangeValue {
            min: self.min,
            max: self.max,
            value: self.value,
        }
        .validate()?;
        if self.min < 0.0 || self.max > 100.0 {
            return Err(invalid("splitter percentages must stay within 0..=100"));
        }
        Ok(())
    }
    fn range(self) -> RangeValue {
        RangeValue {
            min: self.min,
            max: self.max,
            value: self.value,
        }
    }
}
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
struct Drag {
    axis: Orientation,
    ticket: u64,
    start: f64,
    value: f64,
    extent: Option<f64>,
    sample: Option<(DragPhase, f64, f64)>,
    moved: bool,
}
impl Drag {
    fn coordinate(&self, x: f64, y: f64) -> f64 {
        if self.axis == Orientation::Horizontal {
            x
        } else {
            y
        }
    }
}
struct Controller {
    weak: WeakScope,
    owner: glory::core::InstanceId,
    live: Cell<bool>,
    control: Control<f64>,
    local: bool,
    accepted: Cage<Snapshot>,
    axis: Vary<Orientation>,
    step: f64,
    thickness: f64,
    root: Cage<Option<NodeId>>,
    handle: Cage<Option<NodeId>>,
    change: Callback<f64>,
    commit: Callback<f64>,
    range_events: Observers,
    drag_events: Observers,
    drag: RefCell<Option<Drag>>,
    pending: Cell<Option<QueryToken>>,
    next: Cell<u64>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn propose(&self, value: f64) -> Result {
        self.ensure()?;
        let state = self.accepted.try_get()?;
        if !state.enabled {
            return Ok(());
        }
        let value = value.clamp(state.min, state.max);
        self.control.accept(&value)?;
        self.change.emit(self.owner, value);
        Ok(())
    }
    fn key(&self, command: RangeCommand) -> Result {
        let state = self.accepted.try_get()?;
        if !state.enabled {
            return Ok(());
        }
        command.validate(state.range())?;
        let current = if self.local {
            self.control.source.try_get()?.clamp(state.min, state.max)
        } else {
            state.value
        };
        let next = match command {
            RangeCommand::PageDecrease => (current - self.step * 10.0).max(state.min),
            RangeCommand::PageIncrease => (current + self.step * 10.0).min(state.max),
            RangeCommand::Decrease => (current - self.step).max(state.min),
            RangeCommand::Increase => (current + self.step).min(state.max),
            RangeCommand::Minimum => state.min,
            RangeCommand::Maximum => state.max,
            RangeCommand::Set(value) => value,
        };
        if next != current {
            self.propose(next)?;
            self.commit.emit(self.owner, next);
        }
        Ok(())
    }
    fn cancel(&self) {
        self.drag.borrow_mut().take();
        if let Some(token) = self.pending.take()
            && let Some(scope) = self.weak.upgrade()
            && let Err(error) = scope.cancel_request(token)
            && self.live.get()
            && self.owner.is_alive()
        {
            scope.report_error(error);
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.cancel();
        self.change.clear();
        self.commit.clear();
        self.range_events.clear();
        self.drag_events.clear();
    }
    fn apply_drag(&self) -> Result {
        let state = self.accepted.try_get()?;
        let axis = self.axis.try_get()?;
        let (value, phase, x, y) = {
            let mut slot = self.drag.borrow_mut();
            let Some(drag) = slot.as_mut() else {
                return Ok(());
            };
            if !state.enabled || drag.axis != axis {
                drop(slot);
                self.cancel();
                return Ok(());
            }
            let Some(extent) = drag.extent else {
                return Ok(());
            };
            let Some((phase, x, y)) = drag.sample.take() else {
                return Ok(());
            };
            let value = drag.moved.then(|| {
                // Clamp the divider, keeping the original pointer grab offset
                // so it lines up again when the pointer reenters the range.
                let delta = drag.coordinate(x, y) - drag.start;
                (drag.value + delta / extent * 100.0).clamp(state.min, state.max)
            });
            if phase == DragPhase::Drop {
                slot.take();
            }
            (value, phase, x, y)
        };
        if let Some(value) = value {
            self.propose(value)?;
            if phase == DragPhase::Drop {
                self.commit.emit(self.owner, value);
            }
        }
        let from = self.handle.try_get()?.ok_or(glory::ui::Error::Gone)?;
        self.drag_events
            .emit(self.owner, Event::Drag { from, phase, x, y });
        Ok(())
    }

    fn accept_bounds(&self, ticket: u64, rect: Rect) -> Result {
        {
            let mut slot = self.drag.borrow_mut();
            let Some(drag) = slot.as_mut().filter(|drag| drag.ticket == ticket) else {
                return Ok(());
            };
            let extent = (if drag.axis == Orientation::Horizontal {
                rect.width
            } else {
                rect.height
            }) - self.thickness;
            if !extent.is_finite() || extent <= 0.0 {
                return Err(invalid("splitter needs positive available pane extent"));
            }
            drag.extent = Some(extent);
        }
        self.apply_drag()
    }

    fn pointer(self: &Rc<Self>, event: Event) -> Result {
        let Event::Drag { phase, x, y, .. } = event else {
            return Ok(());
        };
        self.ensure()?;
        if phase == DragPhase::Cancel {
            self.cancel();
            self.drag_events.emit(self.owner, event);
            return Ok(());
        }
        let state = self.accepted.try_get()?;
        if !state.enabled {
            self.cancel();
            return Ok(());
        }
        if phase == DragPhase::Start {
            self.cancel();
            let ticket = self
                .next
                .get()
                .checked_add(1)
                .ok_or_else(|| invalid("splitter drag ticket exhausted"))?;
            self.next.set(ticket);
            let axis = self.axis.try_get()?;
            *self.drag.borrow_mut() = Some(Drag {
                axis,
                ticket,
                start: if axis == Orientation::Horizontal {
                    x
                } else {
                    y
                },
                value: state.value,
                extent: None,
                sample: None,
                moved: false,
            });
            let scope = self.weak.upgrade().ok_or(glory::ui::Error::Gone)?;
            let root = self.root.try_get()?.ok_or(glory::ui::Error::Gone)?;
            if let Some(rect) = scope.bounds_now(root)? {
                self.accept_bounds(ticket, rect)?;
                self.drag_events.emit(self.owner, event);
                return Ok(());
            }
            let controller = self.clone();
            let token = scope.request(Query::Bounds(root), move |result| {
                if controller.ensure().is_err() {
                    return;
                }
                if controller.drag.borrow().as_ref().map(|drag| drag.ticket) != Some(ticket) {
                    return;
                }
                controller.pending.set(None);
                let Some(scope) = controller.weak.upgrade() else {
                    return;
                };
                let result = (|| -> Result {
                    let QueryValue::Bounds(rect) = result? else {
                        return Err(invalid("splitter bounds response mismatch"));
                    };
                    controller.accept_bounds(ticket, rect)
                })();
                if let Err(error) = result {
                    controller.cancel();
                    scope.report_error(error);
                }
            })?;
            self.pending.set(Some(token));
            self.drag_events.emit(self.owner, event);
        } else {
            {
                let mut slot = self.drag.borrow_mut();
                let Some(drag) = slot.as_mut() else {
                    return Ok(());
                };
                drag.moved |= drag.coordinate(x, y) != drag.start;
                drag.sample = Some((phase, x, y));
            }
            // The browser and native shell already coalesce consecutive moves.
            // Publish in this tick once the initial measurement is available.
            self.apply_drag()?;
        }
        Ok(())
    }
}

type Factory = Box<dyn FnMut() -> Box<dyn Widget>>;
pub struct Splitter {
    common: Common,
    key: String,
    value: Value<f64>,
    min: Vary<f64>,
    max: Vary<f64>,
    step: f64,
    axis: Vary<Orientation>,
    thickness: f64,
    line_size: Vary<f64>,
    with_grip: Vary<bool>,
    grip_size: (f64, f64),
    first: Option<Factory>,
    second: Option<Factory>,
    first_ref: Option<Cage<Option<NodeId>>>,
    second_ref: Option<Cage<Option<NodeId>>>,
    handle_ref: Option<Cage<Option<NodeId>>>,
    change: Option<Box<dyn FnMut(f64)>>,
    commit: Option<Box<dyn FnMut(f64)>>,
}
pub fn splitter(key: impl Into<String>) -> Splitter {
    Splitter {
        common: Common::default(),
        key: key.into(),
        value: Value {
            source: None,
            default: 50.0,
        },
        min: 10.0.into(),
        max: 90.0.into(),
        step: 1.0,
        axis: Orientation::Horizontal.into(),
        thickness: 6.0,
        line_size: 6.0.into(),
        with_grip: true.into(),
        grip_size: (12.0, 2.0),
        first: None,
        second: None,
        first_ref: None,
        second_ref: None,
        handle_ref: None,
        change: None,
        commit: None,
    }
}
impl Splitter {
    crate::appearance::common_methods!();
    pub fn value(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: f64) -> Self {
        self.value.default = value;
        self
    }
    pub fn min(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.min = value.into();
        self
    }
    pub fn max(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.max = value.into();
        self
    }
    pub fn step(mut self, value: f64) -> Self {
        self.step = value;
        self
    }
    pub fn axis(mut self, value: impl Into<Vary<Orientation>>) -> Self {
        self.axis = value.into();
        self
    }
    /// Set the width of the draggable area in CSS pixels.
    pub fn thickness(mut self, value: f64) -> Self {
        self.thickness = value;
        self
    }
    /// Set the visible line thickness in CSS pixels, up to the draggable area.
    /// Zero hides the line.
    pub fn line_size(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.line_size = value.into();
        self
    }
    /// Show or hide the centered grip.
    pub fn with_grip(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.with_grip = value.into();
        self
    }
    /// Set the grip's length along the divider and width across it in CSS pixels.
    pub fn grip_size(mut self, length: f64, width: f64) -> Self {
        self.grip_size = (length, width);
        self
    }
    pub fn first<W: Widget>(mut self, mut factory: impl FnMut() -> W + 'static) -> Self {
        self.first = Some(Box::new(move || Box::new(factory())));
        self
    }
    pub fn second<W: Widget>(mut self, mut factory: impl FnMut() -> W + 'static) -> Self {
        self.second = Some(Box::new(move || Box::new(factory())));
        self
    }
    pub fn first_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.first_ref = Some(value);
        self
    }
    pub fn second_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.second_ref = Some(value);
        self
    }
    pub fn handle_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.handle_ref = Some(value);
        self
    }
    pub fn on_value_change(mut self, callback: impl FnMut(f64) + 'static) -> Self {
        self.change = Some(Box::new(callback));
        self
    }
    pub fn on_value_commit(mut self, callback: impl FnMut(f64) + 'static) -> Self {
        self.commit = Some(Box::new(callback));
        self
    }
}
impl Widget for Splitter {
    fn build(&mut self, scope: &Scope) -> Result {
        let first_id = composite::id("splitter", &self.key, "pane", "first")?;
        let second_id = composite::id("splitter", &self.key, "pane", "second")?;
        let handle_id = composite::id("splitter", &self.key, "handle", "handle")?;
        if !self.step.is_finite()
            || self.step <= 0.0
            || self.step > 100.0
            || !self.thickness.is_finite()
            || self.thickness <= 0.0
            || self.thickness > 128.0
        {
            return Err(invalid(
                "splitter needs step in (0,100] and thickness in (0,128]",
            ));
        }
        if self.common.focus != FocusMode::Auto {
            return Err(invalid("splitter owns its handle tab stop"));
        }
        if [self.grip_size.0, self.grip_size.1]
            .into_iter()
            .any(|size| !(0.0 < size && size <= 128.0))
        {
            return Err(invalid("splitter grip dimensions must be in (0,128]"));
        }
        let line_size = composite::validated(scope, &self.line_size, |size| {
            if (0.0..=128.0).contains(size) {
                Ok(())
            } else {
                Err(invalid("splitter line size must be in [0,128]"))
            }
        })?;
        let local = self.value.source.is_none();
        let axis = self.axis.clone();
        let handle_name = self
            .common
            .name
            .clone()
            .unwrap_or_else(|| "Resize panes".into());
        let control = std::mem::take(&mut self.value).mount(scope);
        let source = control.source.clone();
        let min = self.min.clone();
        let max = self.max.clone();
        let enabled = self.common.enabled.clone();
        let raw: Vary<Snapshot> = scope
            .bond(move || Snapshot {
                value: source.get(),
                min: min.get(),
                max: max.get(),
                enabled: enabled.get(),
            })
            .into();
        let accepted = composite::validated(scope, &raw, Snapshot::validate)?;
        let root = scope.cage(None);
        let handle = self.handle_ref.unwrap_or_else(|| scope.cage(None));
        let controller = Rc::new(Controller {
            weak: scope.downgrade(),
            owner: scope.id(),
            live: Cell::new(true),
            control,
            local,
            accepted,
            axis: self.axis.clone(),
            step: self.step,
            thickness: self.thickness,
            root,
            handle,
            change: Callback::new(self.change.take()),
            commit: Callback::new(self.commit.take()),
            range_events: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Range)
                    .unwrap_or_default(),
            ),
            drag_events: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Drag)
                    .unwrap_or_default(),
            ),
            drag: RefCell::new(None),
            pending: Cell::new(None),
            next: Cell::new(0),
        });
        let cleanup = controller.clone();
        scope.on_cleanup(move || cleanup.retire())?;
        let direction = self.axis.clone();
        let observer = controller.clone();
        let mut previous = direction.get();
        scope.effect(move || {
            let next = direction.get();
            if next != previous {
                observer.cancel();
                previous = next;
            }
        });
        let outer_ref = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let custom = self.common.style.clone();
        let mut outer = configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-splitter",
            Style::new()
                .width(Length::Percent(100.0))
                .height(Length::Px(300.0))
        )
        .style(scope.bond(move || {
            merge(
                Style::new()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(300.0)),
                custom.get(),
            )
        }))
        .node_ref(outer_ref);
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            outer = outer.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        outer.show_in(scope)?;
        let parent = scope.child_in(outer_ref.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        let direction = self.axis.clone();
        glory::ui::view()
            .class("sapid-splitter-frame")
            .node_ref(root)
            .style(scope.bond(move || {
                Style::new()
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .display(Display::Flex)
                    .flex_direction(if direction.get() == Orientation::Horizontal {
                        FlexDirection::Row
                    } else {
                        FlexDirection::Column
                    })
                    .gap(0.0)
                    .padding(Edges::all(0.0))
                    .border_width(Edges::all(0.0))
                    .margin(Edges::all(0.0))
            }))
            .show_in(&parent)?;
        let frame = scope.child_in(root.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        let first_focused = scope.cage(false);
        let second_focused = scope.cage(false);
        for (first, id, reference, factory, focused) in [
            (
                true,
                first_id.clone(),
                self.first_ref,
                self.first.take(),
                first_focused,
            ),
            (
                false,
                second_id,
                self.second_ref,
                self.second.take(),
                second_focused,
            ),
        ] {
            if !first {
                self.divider(
                    &frame,
                    controller.clone(),
                    first_id.clone(),
                    handle_id.clone(),
                    handle_name.clone(),
                    line_size,
                )?;
            }
            let mut pane = glory::ui::view()
                .name(if first { "First pane" } else { "Second pane" })
                .class(if first {
                    "sapid-splitter-first"
                } else {
                    "sapid-splitter-second"
                })
                .semantics(scope.bond(move || {
                    composite::metadata(Semantics {
                        id: Some(id.clone()),
                        hidden: if first {
                            accepted.get().value == 0.0
                        } else {
                            accepted.get().value == 100.0
                        },
                        ..Default::default()
                    })
                }))
                .style(scope.bond({
                    let axis = axis.clone();
                    move || Style {
                        grow: Some(if first {
                            accepted.get().value
                        } else {
                            100.0 - accepted.get().value
                        }),
                        shrink: Some(1.0),
                        ..Style::new()
                            .width(if axis.get() == Orientation::Horizontal {
                                Length::Px(0.0)
                            } else {
                                Length::Percent(100.0)
                            })
                            .height(if axis.get() == Orientation::Vertical {
                                Length::Px(0.0)
                            } else {
                                Length::Percent(100.0)
                            })
                            .min_width(Length::Px(0.0))
                            .min_height(Length::Px(0.0))
                            .margin(Edges::all(0.0))
                            .overflow_x(Overflow::Hidden)
                            .overflow_y(Overflow::Hidden)
                    }
                }))
                .on(EventKind::FocusWithin, move |event| {
                    if let Event::FocusWithin { focused: value, .. } = event {
                        focused.revise(|old| *old = value);
                    }
                });
            if let Some(reference) = reference {
                pane = pane.node_ref(reference);
            }
            if let Some(mut factory) = factory {
                pane = pane.fill(glory::ui::widget(move |scope| factory().show_in(scope)));
            }
            pane.show_in(&frame)?;
        }
        let weak = scope.downgrade();
        let adapt = controller.clone();
        scope.effect(move || {
            let state = accepted.get();
            if !state.enabled {
                adapt.cancel();
            }
            if state.enabled
                && (state.value == 0.0 && first_focused.get()
                    || state.value == 100.0 && second_focused.get())
                && let Some(scope) = weak.upgrade()
                && let Some(handle) = handle.get_untracked()
            {
                first_focused.revise(|v| *v = false);
                second_focused.revise(|v| *v = false);
                if let Err(error) = scope.action(Action::Focus(handle)) {
                    scope.report_error(error);
                }
            }
        });
        scope.retain(parent)?;
        scope.retain(frame)?;
        Ok(())
    }
}
impl Splitter {
    fn divider(
        &self,
        scope: &Scope,
        controller: Rc<Controller>,
        first: SemanticId,
        id: SemanticId,
        name: Vary<String>,
        line_size: Cage<f64>,
    ) -> Result {
        let axis = self.axis.clone();
        let line_axis = axis.clone();
        let grip_axis = axis.clone();
        let with_grip = self.with_grip.clone();
        let (grip_length, grip_width) = self.grip_size;
        let thickness = self.thickness;
        let grip = (!matches!(&with_grip, Vary::Value(false))).then(|| {
            let color = self.common.theme.muted;
            glory::ui::view()
                .class("sapid-splitter-grip")
                .style(scope.bond(move || {
                    let length = Length::Px(grip_length);
                    let width = Length::Px(grip_width.min(thickness));
                    Style {
                        shrink: Some(0.0),
                        align: Some(Align::Center),
                        justify: Some(Justify::Center),
                        ..divider_size(grip_axis.get(), width, length)
                            .display(if with_grip.get() {
                                Display::Flex
                            } else {
                                Display::None
                            })
                            .flex_direction(if grip_axis.get() == Orientation::Horizontal {
                                FlexDirection::Column
                            } else {
                                FlexDirection::Row
                            })
                            .gap(2.0)
                            .pointer_events(PointerEvents::None)
                    }
                }))
                .fill(
                    (0..3)
                        .map(|_| {
                            glory::ui::view().style(Style {
                                background: Some(color),
                                shrink: Some(0.0),
                                ..Style::new()
                                    .width(Length::Px(2.0))
                                    .height(Length::Px(2.0))
                                    .radius(1.0)
                            })
                        })
                        .collect::<Vec<_>>(),
                )
        });
        let line = (!self.common.unstyled).then(|| {
            let color = self.common.theme.border;
            glory::ui::view()
                .class("sapid-splitter-line")
                .style(scope.bond(move || {
                    Style {
                        background: Some(color),
                        align: Some(Align::Center),
                        justify: Some(Justify::Center),
                        ..divider_size(
                            line_axis.get(),
                            Length::Px(line_size.get().min(thickness)),
                            Length::Percent(100.0),
                        )
                        .display(Display::Flex)
                        .pointer_events(PointerEvents::None)
                    }
                }))
                .fill(grip)
        });
        glory::ui::view()
            .class("sapid-splitter-divider")
            .style(scope.bond(move || {
                Style {
                    shrink: Some(0.0),
                    align: Some(Align::Center),
                    justify: Some(Justify::Center),
                    ..divider_size(axis.get(), Length::Px(thickness), Length::Percent(100.0))
                        .position(Position::Relative)
                        .display(Display::Flex)
                }
            }))
            .fill((
                line,
                glory::ui::widget(move |scope| {
                    build_handle(
                        scope,
                        controller.clone(),
                        first.clone(),
                        id.clone(),
                        name.clone(),
                    )
                }),
            ))
            .show_in(scope)
    }
}

fn divider_size(axis: Orientation, width: Length, length: Length) -> Style {
    let (width, height) = if axis == Orientation::Horizontal {
        (width, length)
    } else {
        (length, width)
    };
    Style::new().width(width).height(height)
}

fn build_handle(
    scope: &Scope,
    controller: Rc<Controller>,
    first: SemanticId,
    id: SemanticId,
    name: Vary<String>,
) -> Result {
    let accepted = controller.accepted;
    let axis = controller.axis.clone();
    let range = controller.clone();
    let drag = controller.clone();
    let weak = scope.downgrade();
    let drag_weak = weak.clone();
    glory::ui::view()
        .role(Role::Separator)
        .name(name)
        .focus(scope.bond(move || {
            if accepted.get().enabled {
                FocusMode::TabStop
            } else {
                FocusMode::Programmatic
            }
        }))
        .class("sapid-splitter-handle")
        .node_ref(controller.handle)
        .semantics(scope.bond(move || {
            let state = accepted.get();
            composite::metadata(Semantics {
                id: Some(id.clone()),
                splitter: Some(state.range()),
                separator_axis: Some(if axis.get() == Orientation::Horizontal {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                }),
                controls: Some(first.clone()),
                disabled: !state.enabled,
                ..Default::default()
            })
        }))
        .style(
            Style::new()
                .position(Position::Absolute)
                .top(Length::Px(0.0))
                .left(Length::Px(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .margin(Edges::all(0.0)),
        )
        .on(EventKind::Range, move |event| {
            if let Event::Range { command, .. } = event
                && let Err(error) = range.key(command)
            {
                if let Some(scope) = weak.upgrade() {
                    scope.report_error(error);
                }
                return;
            }
            range.range_events.emit(range.owner, event);
        })
        .on(EventKind::Drag, move |event| {
            if let Err(error) = drag.pointer(event)
                && let Some(scope) = drag_weak.upgrade()
            {
                drag.cancel();
                scope.report_error(error);
            }
        })
        .show_in(scope)
}
