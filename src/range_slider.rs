//! Two independently named sliders on one accepted numeric track.
use crate::{
    appearance::Common,
    composite::{self, Callback, Observers},
    field::{FieldControl, control_metadata},
    state::{Control, Value},
};
use glory::render::{Direction, Orientation, RangeCommand, RangeValue, Rect, SliderValue};
use glory::render::{
    DragPhase, Event, EventKind, FocusMode, NodeId, Query, QueryToken, QueryValue, Role, Semantics,
};
use glory::style::{Edges, Length, Position, Style};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeThumb {
    Lower,
    Upper,
}
impl RangeThumb {
    fn index(self) -> usize {
        match self {
            Self::Lower => 0,
            Self::Upper => 1,
        }
    }
}
/// The two identities and their tab order never exchange. Collision policy is
/// explicit and always publishes one atomic pair.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RangeCollision {
    Clamp,
    #[default]
    Push,
    Hold,
}
#[derive(Clone, Debug, PartialEq)]
struct Snapshot {
    values: [f64; 2],
    min: f64,
    max: f64,
    domain: [f64; 2],
    step: f64,
    page: f64,
    gap: f64,
    max_gap: Option<f64>,
    collision: RangeCollision,
    axis: Orientation,
    direction: Direction,
    enabled: bool,
    readonly: bool,
    names: [String; 2],
    text: [Option<String>; 2],
    scale: f64,
    marks: Rc<[crate::SliderMark]>,
    restrict_to_marks: bool,
}
impl Snapshot {
    fn validate_numeric(&self) -> Result {
        if !self.domain[0].is_finite()
            || !self.domain[1].is_finite()
            || self.domain[0] >= self.domain[1]
            || self.domain.iter().any(|value| value.abs() > 1e12)
            || self.domain[0] > self.min
            || self.domain[1] < self.max
        {
            return Err(invalid(
                "range slider domain must be a finite strict interval containing min and max",
            ));
        }
        RangeValue {
            min: self.min,
            max: self.max,
            value: self.values[0],
        }
        .validate()?;
        RangeValue {
            min: self.min,
            max: self.max,
            value: self.values[1],
        }
        .validate()?;
        if [self.min, self.max].into_iter().any(|v| v.abs() > 1e12)
            || !self.step.is_finite()
            || self.step <= 0.
            || self.step > 1e12
            || !self.page.is_finite()
            || self.page <= 0.
            || self.page > 1e12
            || !self.gap.is_finite()
            || self.gap < 0.
            || self.gap > self.max - self.min
            || self.max_gap.is_some_and(|max_gap| {
                !max_gap.is_finite() || max_gap < self.gap || max_gap > self.max - self.min
            })
            || self.values[1] - self.values[0] < self.gap
            || !self.scale.is_finite()
            || self
                .values
                .iter()
                .any(|value| (value * self.scale).abs() > 1e12)
        {
            return Err(invalid(
                "range slider requires ordered finite values, positive steps, valid gaps and a finite display scale",
            ));
        }
        let mut previous = None;
        for mark in self.marks.iter() {
            if !mark.value.is_finite()
                || mark.value < self.min
                || mark.value > self.max
                || previous.is_some_and(|value| value >= mark.value)
                || mark.label.as_ref().is_some_and(|label| {
                    label.is_empty() || label.len() > 256 || label.contains(['\0', '\r', '\n'])
                })
            {
                return Err(invalid(
                    "range slider marks must be sorted, unique, inside the range and have optional 1..=256 byte single-line labels",
                ));
            }
            previous = Some(mark.value);
        }
        if self.restrict_to_marks
            && (self.marks.is_empty()
                || self
                    .values
                    .iter()
                    .any(|value| !self.marks.iter().any(|mark| mark.value == *value)))
        {
            return Err(invalid(
                "range slider restricted values must equal declared marks",
            ));
        }
        Ok(())
    }
    #[cfg(test)]
    fn validate(&self) -> Result {
        self.validate_numeric()?;
        for name in &self.names {
            if name.len() > 512 || name.trim().is_empty() {
                return Err(invalid("range slider names require 1..=512 bytes"));
            }
        }
        for text in self.text.iter().flatten() {
            if text.len() > 1024 || text.trim().is_empty() {
                return Err(invalid("range slider value text requires 1..=1024 bytes"));
            }
        }
        Ok(())
    }
    fn bounds(&self, thumb: RangeThumb, values: [f64; 2]) -> RangeValue {
        let index = thumb.index();
        if self.collision != RangeCollision::Clamp {
            return RangeValue {
                min: if index == 0 {
                    self.min
                } else {
                    self.min + self.gap
                },
                max: if index == 0 {
                    self.max - self.gap
                } else {
                    self.max
                },
                value: values[index],
            };
        }
        let mut upper_min = values[0] + self.gap;
        if upper_min - values[0] < self.gap {
            upper_min = upper_min.next_up();
        }
        let mut lower_max = values[1] - self.gap;
        if values[1] - lower_max < self.gap {
            lower_max = lower_max.next_down();
        }
        let current_gap = values[1] - values[0];
        let max_lower = self.max_gap.map_or(self.min, |max_gap| {
            if current_gap > max_gap {
                values[0]
            } else {
                (values[1] - max_gap).max(self.min)
            }
        });
        let max_upper = self.max_gap.map_or(self.max, |max_gap| {
            if current_gap > max_gap {
                values[1]
            } else {
                (values[0] + max_gap).min(self.max)
            }
        });
        RangeValue {
            min: if index == 0 { max_lower } else { upper_min },
            max: if index == 0 { lower_max } else { max_upper },
            value: values[index],
        }
    }
    fn metadata(&self, thumb: RangeThumb) -> SliderValue {
        let index = thumb.index();
        SliderValue {
            range: self.bounds(thumb, self.values),
            axis: self.axis,
            direction: self.direction,
            step: self.step,
            page_step: self.page,
            value_text: self.text[index].clone().or_else(|| {
                (self.scale != 1.).then(|| (self.values[index] * self.scale).to_string())
            }),
        }
    }
    fn display_value(&self, index: usize) -> String {
        self.text[index]
            .clone()
            .unwrap_or_else(|| (self.values[index] * self.scale).to_string())
    }
    fn snap(&self, value: f64) -> f64 {
        let value = value.clamp(self.min, self.max);
        if self.restrict_to_marks {
            return self
                .marks
                .iter()
                .min_by(|left, right| {
                    (left.value - value)
                        .abs()
                        .total_cmp(&(right.value - value).abs())
                })
                .map_or(value, |mark| mark.value);
        }
        if value == self.min || value == self.max {
            return value;
        }
        let lower = value - (value - self.min) % self.step;
        let upper = (lower + self.step).min(self.max);
        if value - lower < upper - value {
            lower.max(self.min)
        } else {
            upper
        }
    }
    fn propose(
        &self,
        thumb: RangeThumb,
        values: [f64; 2],
        command: RangeCommand,
    ) -> Result<[f64; 2]> {
        let bounds = self.bounds(thumb, values);
        command.validate(bounds)?;
        let current = values[thumb.index()];
        let proposed = if self.restrict_to_marks {
            let mut candidates = self
                .marks
                .iter()
                .filter(|mark| mark.value >= bounds.min && mark.value <= bounds.max);
            match command {
                RangeCommand::Decrease | RangeCommand::PageDecrease => candidates
                    .rfind(|mark| mark.value < current)
                    .map_or(current, |mark| mark.value),
                RangeCommand::Increase | RangeCommand::PageIncrease => candidates
                    .find(|mark| mark.value > current)
                    .map_or(current, |mark| mark.value),
                RangeCommand::Minimum => candidates.next().map_or(current, |mark| mark.value),
                RangeCommand::Maximum => candidates.next_back().map_or(current, |mark| mark.value),
                RangeCommand::Set(value) => candidates
                    .min_by(|left, right| {
                        (left.value - value)
                            .abs()
                            .total_cmp(&(right.value - value).abs())
                    })
                    .map_or(current, |mark| mark.value),
            }
        } else {
            match command {
                RangeCommand::Decrease => current - self.step,
                RangeCommand::Increase => current + self.step,
                RangeCommand::PageDecrease => current - self.page,
                RangeCommand::PageIncrease => current + self.page,
                RangeCommand::Minimum => bounds.min,
                RangeCommand::Maximum => bounds.max,
                RangeCommand::Set(v) => v,
            }
        };
        let candidate = self.snap(proposed).clamp(bounds.min, bounds.max);
        if self.collision == RangeCollision::Hold {
            let gap = if thumb == RangeThumb::Lower {
                values[1] - candidate
            } else {
                candidate - values[0]
            };
            if gap < self.gap || self.max_gap.is_some_and(|maximum| gap > maximum) {
                return Ok(values);
            }
        }
        let mut next = values;
        next[thumb.index()] = candidate;
        if self.collision == RangeCollision::Push {
            let other = if thumb == RangeThumb::Lower { 1 } else { 0 };
            let gap = next[1] - next[0];
            let target = if gap < self.gap {
                let mut target = if thumb == RangeThumb::Lower {
                    candidate + self.gap
                } else {
                    candidate - self.gap
                };
                if thumb == RangeThumb::Lower && target - candidate < self.gap {
                    target = target.next_up();
                } else if thumb == RangeThumb::Upper && candidate - target < self.gap {
                    target = target.next_down();
                }
                Some(target)
            } else if self.max_gap.is_some_and(|maximum| gap > maximum) {
                let maximum = self.max_gap.unwrap();
                let mut target = if thumb == RangeThumb::Lower {
                    candidate + maximum
                } else {
                    candidate - maximum
                };
                if thumb == RangeThumb::Lower && target - candidate > maximum {
                    target = target.next_down();
                } else if thumb == RangeThumb::Upper && candidate - target > maximum {
                    target = target.next_up();
                }
                Some(target)
            } else {
                None
            };
            if let Some(target) = target {
                next[other] = if self.restrict_to_marks {
                    let candidate_mark = if thumb == RangeThumb::Lower {
                        self.marks.iter().find(|mark| {
                            mark.value >= target
                                && self
                                    .max_gap
                                    .is_none_or(|maximum| mark.value - candidate <= maximum)
                        })
                    } else {
                        self.marks.iter().rev().find(|mark| {
                            mark.value <= target
                                && self
                                    .max_gap
                                    .is_none_or(|maximum| candidate - mark.value <= maximum)
                        })
                    };
                    let Some(mark) = candidate_mark else {
                        return Ok(values);
                    };
                    mark.value
                } else {
                    target
                };
            }
        }
        Ok(next)
    }
    fn point(&self, rect: Rect, x: f64, y: f64) -> Result<f64> {
        let (point, start, extent) = if self.axis == Orientation::Horizontal {
            (x, rect.x, rect.width)
        } else {
            (y, rect.y, rect.height)
        };
        if extent <= 0. {
            return Err(invalid("range slider needs positive measurable bounds"));
        }
        let mut fraction = ((point - start) / extent).clamp(0., 1.);
        if self.axis == Orientation::Vertical || self.direction == Direction::Rtl {
            fraction = 1. - fraction
        };
        Ok(self.snap(self.domain[0] + (self.domain[1] - self.domain[0]) * fraction))
    }
    fn same_domain(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.domain == other.domain
            && self.step == other.step
            && self.page == other.page
            && self.gap == other.gap
            && self.max_gap == other.max_gap
            && self.collision == other.collision
            && self.axis == other.axis
            && self.direction == other.direction
            && self.enabled == other.enabled
            && self.readonly == other.readonly
    }
}
#[cfg(test)]
mod model_tests {
    use super::*;
    fn state() -> Snapshot {
        Snapshot {
            values: [25., 75.],
            min: 0.,
            max: 100.,
            domain: [0., 100.],
            step: 1.,
            page: 10.,
            gap: 10.,
            max_gap: None,
            collision: RangeCollision::Clamp,
            axis: Orientation::Horizontal,
            direction: Direction::Ltr,
            enabled: true,
            readonly: false,
            names: ["Minimum".into(), "Maximum".into()],
            text: [None, None],
            scale: 1.,
            marks: Rc::from([]),
            restrict_to_marks: false,
        }
    }
    #[test]
    fn two_thumb_proposals_clamp_without_swapping_and_allow_singleton_neighbour_bounds() {
        let mut state = state();
        state.validate().unwrap();
        assert_eq!(
            state
                .propose(RangeThumb::Lower, state.values, RangeCommand::Maximum)
                .unwrap(),
            [65., 75.]
        );
        assert_eq!(
            state
                .propose(RangeThumb::Upper, state.values, RangeCommand::Minimum)
                .unwrap(),
            [25., 35.]
        );
        state.values = [0., 10.];
        state.validate().unwrap();
        assert_eq!(
            state.bounds(RangeThumb::Lower, state.values),
            RangeValue {
                min: 0.,
                max: 0.,
                value: 0.
            }
        );
        assert_eq!(
            state
                .propose(RangeThumb::Lower, state.values, RangeCommand::Increase)
                .unwrap(),
            [0., 10.]
        );
    }
    #[test]
    fn off_grid_neighbours_keep_exact_gap_and_invalid_domains_are_rejected() {
        let mut state = state();
        state.values = [25.5, 75.5];
        assert_eq!(
            state
                .propose(RangeThumb::Lower, state.values, RangeCommand::Maximum)
                .unwrap(),
            [65.5, 75.5]
        );
        for values in [
            [80., 20.],
            [f64::NAN, 30.],
            [-1., 30.],
            [10., 101.],
            [20., 25.],
        ] {
            state.values = values;
            assert!(state.validate().is_err());
        }
        state.values = [25., 75.];
        state.domain = [0., 200.];
        state.validate().unwrap();
        assert_eq!(
            state
                .point(
                    Rect {
                        x: 10.,
                        y: 0.,
                        width: 200.,
                        height: 20.
                    },
                    110.,
                    10.
                )
                .unwrap(),
            100.
        );
        state.domain = [10., 90.];
        assert!(state.validate().is_err());
    }
}

#[derive(Clone, Copy)]
struct Drag {
    ticket: u64,
    thumb: RangeThumb,
    start: (f64, f64),
    latest: (f64, f64),
    phase: DragPhase,
    start_pending: bool,
    pending: bool,
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    control: Control<[f64; 2]>,
    local: bool,
    accepted: Cage<Snapshot>,
    root: Cage<Option<NodeId>>,
    thumbs: [Cage<Option<NodeId>>; 2],
    changed: Callback<[f64; 2]>,
    committed: Callback<[f64; 2]>,
    range_events: Observers,
    drag_events: Observers,
    expected: Cell<Option<[f64; 2]>>,
    confirmed: Cell<Option<([f64; 2], Option<u64>)>>,
    drag: RefCell<Option<Drag>>,
    dragging: Cage<Option<RangeThumb>>,
    active: Cell<Option<u64>>,
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
    fn report(&self, error: glory::ui::Error) {
        if let Some(scope) = self.weak.upgrade() {
            scope.report_error(error)
        }
    }
    fn cancel(&self) {
        self.active.set(None);
        self.drag.borrow_mut().take();
        let _ = self.dragging.try_revise(|thumb| *thumb = None);
        if let Some(token) = self.pending.take()
            && let Some(scope) = self.weak.upgrade()
            && let Err(error) = scope.cancel_request(token)
            && self.owner.is_alive()
        {
            scope.report_error(error)
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.cancel();
        self.changed.clear();
        self.committed.clear();
        self.range_events.clear();
        self.drag_events.clear()
    }
    fn stamp(&self) -> Result<([f64; 2], Option<u64>)> {
        let value = self.control.source.try_get()?;
        let version = match self.control.source {
            Vary::Cage(source) => Some(source.try_version()?),
            _ => None,
        };
        Ok((value, version))
    }
    fn current(&self, state: &Snapshot) -> Result<[f64; 2]> {
        if self.local {
            Ok(self.control.source.try_get()?)
        } else if let Some(confirmed) = self.confirmed.get()
            && self.stamp()? == confirmed
        {
            Ok(confirmed.0)
        } else {
            Ok(state.values)
        }
    }
    fn confirm(&self, value: [f64; 2], before: ([f64; 2], Option<u64>)) -> Result {
        if self.ensure().is_ok() && self.expected.get() == Some(value) {
            let after = self.stamp()?;
            if after.0 == value && after != before {
                self.confirmed.set(Some(after))
            }
        }
        Ok(())
    }
    fn propose(&self, value: [f64; 2]) -> Result {
        self.ensure()?;
        let before = self.stamp()?;
        self.expected.set(Some(value));
        self.confirmed.set(None);
        self.control.accept(&value)?;
        self.changed.emit(self.owner, value);
        self.confirm(value, before)
    }
    fn commit(&self, value: [f64; 2]) -> Result {
        let before = self.stamp()?;
        self.committed.emit(self.owner, value);
        self.confirm(value, before)
    }
    fn key(&self, thumb: RangeThumb, command: RangeCommand) -> Result {
        self.ensure()?;
        self.cancel();
        let state = self.accepted.try_get()?;
        let current = self.current(&state)?;
        let bounds = state.bounds(thumb, current);
        if !state.enabled || state.readonly || bounds.min == bounds.max {
            return Ok(());
        }
        let next = state.propose(thumb, current, command)?;
        if next != current {
            self.propose(next)?;
            self.commit(next)?
        }
        Ok(())
    }
    fn publish(
        &self,
        ticket: u64,
        thumb: RangeThumb,
        point: (f64, f64),
        phase: DragPhase,
        rect: Rect,
    ) -> Result {
        if self.active.get() != Some(ticket) || self.ensure().is_err() {
            return Ok(());
        }
        let state = self.accepted.try_get()?;
        let current = self.current(&state)?;
        let bounds = state.bounds(thumb, current);
        if !state.enabled || state.readonly || bounds.min == bounds.max {
            self.cancel();
            return Ok(());
        }
        let value = state
            .point(rect, point.0, point.1)?
            .clamp(bounds.min, bounds.max);
        let next = state.propose(thumb, current, RangeCommand::Set(value))?;
        if next != current {
            self.propose(next)?
        }
        if self.active.get() != Some(ticket) || self.ensure().is_err() {
            return Ok(());
        }
        if phase == DragPhase::Drop {
            self.commit(next)?
        }
        if self.active.get() == Some(ticket)
            && let Some(from) = self.thumbs[thumb.index()].try_get()?
        {
            self.drag_events.emit(
                self.owner,
                Event::Drag {
                    from,
                    phase,
                    x: point.0,
                    y: point.1,
                },
            )
        }
        Ok(())
    }
    fn geometry(&self, ticket: u64, rect: Rect) -> Result {
        let work = {
            let mut slot = self.drag.borrow_mut();
            let Some(drag) = slot.as_mut().filter(|drag| drag.ticket == ticket) else {
                return Ok(());
            };
            if !drag.pending {
                return Ok(());
            }
            let work = *drag;
            drag.pending = false;
            drag.start_pending = false;
            if work.phase == DragPhase::Drop {
                slot.take();
            }
            work
        };
        if work.start_pending {
            self.publish(ticket, work.thumb, work.start, DragPhase::Start, rect)?
        }
        if work.phase != DragPhase::Start {
            self.publish(ticket, work.thumb, work.latest, work.phase, rect)?
        }
        if work.phase == DragPhase::Drop && self.active.get() == Some(ticket) {
            self.active.set(None);
            let _ = self.dragging.try_revise(|thumb| *thumb = None);
        }
        Ok(())
    }
    fn request(self: &Rc<Self>) -> Result {
        if self.pending.get().is_some() {
            return Ok(());
        }
        let Some(ticket) = self.active.get() else {
            return Ok(());
        };
        let scope = self.weak.upgrade().ok_or(glory::ui::Error::Gone)?;
        let node = self.root.try_get()?.ok_or(glory::ui::Error::Gone)?;
        let state = self.clone();
        let token = scope.request(Query::Bounds(node), move |result| {
            if state.ensure().is_err() || state.active.get() != Some(ticket) {
                return;
            }
            state.pending.set(None);
            let result = (|| -> Result {
                let QueryValue::Bounds(rect) = result? else {
                    return Err(invalid("range slider bounds response mismatch"));
                };
                state.geometry(ticket, rect)
            })();
            if let Err(error) = result {
                state.cancel();
                state.report(error)
            }
        })?;
        if self.active.get() == Some(ticket) {
            self.pending.set(Some(token))
        } else {
            scope.cancel_request(token)?;
        }
        Ok(())
    }
    fn pointer(self: &Rc<Self>, thumb: RangeThumb, event: Event) -> Result {
        let Event::Drag { phase, x, y, .. } = event else {
            return Ok(());
        };
        self.ensure()?;
        if phase == DragPhase::Cancel {
            let matches = self
                .drag
                .borrow()
                .as_ref()
                .is_some_and(|drag| drag.thumb == thumb);
            if matches {
                self.cancel();
                self.drag_events.emit(self.owner, event)
            }
            return Ok(());
        }
        let state = self.accepted.try_get()?;
        let bounds = state.bounds(thumb, state.values);
        if !state.enabled || state.readonly || bounds.min == bounds.max {
            self.cancel();
            return Ok(());
        }
        if phase == DragPhase::Start {
            self.cancel();
            let ticket = self
                .next
                .get()
                .checked_add(1)
                .ok_or_else(|| invalid("range slider gesture ticket exhausted"))?;
            self.next.set(ticket);
            self.active.set(Some(ticket));
            self.dragging.try_revise(|active| *active = Some(thumb))?;
            *self.drag.borrow_mut() = Some(Drag {
                ticket,
                thumb,
                start: (x, y),
                latest: (x, y),
                phase,
                start_pending: true,
                pending: true,
            })
        } else {
            let mut slot = self.drag.borrow_mut();
            let Some(drag) = slot.as_mut().filter(|drag| drag.thumb == thumb) else {
                return Ok(());
            };
            drag.latest = (x, y);
            drag.phase = phase;
            drag.pending = true;
        }
        self.request()
    }
}

/// A fixed pair of independently named thumbs. Local/controlled values are a
/// typed pair, so no collection is scanned or cloned to validate its budget.
pub struct RangeSlider {
    key: String,
    common: Common,
    value: Value<[f64; 2]>,
    min: Vary<f64>,
    max: Vary<f64>,
    domain: Option<Vary<[f64; 2]>>,
    step: Vary<f64>,
    page: Option<Vary<f64>>,
    gap: Vary<f64>,
    max_gap: Option<Vary<f64>>,
    names: Vary<[String; 2]>,
    text: Vary<[Option<String>; 2]>,
    axis: Orientation,
    direction: Direction,
    collision: RangeCollision,
    field: Option<FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    inherit_theme: bool,
    inherit_size: bool,
    thumbs: [Option<Cage<Option<NodeId>>>; 2],
    changed: Option<Box<dyn FnMut([f64; 2])>>,
    committed: Option<Box<dyn FnMut([f64; 2])>>,
    marks: Vec<crate::SliderMark>,
    restrict_to_marks: bool,
    inverted: bool,
    show_value_labels: bool,
    value_labels_always_on: bool,
    scale: f64,
}
pub fn range_slider(key: impl Into<String>) -> RangeSlider {
    RangeSlider {
        key: key.into(),
        common: Common::default(),
        value: Value {
            source: None,
            default: [25., 75.],
        },
        min: 0.0.into(),
        max: 100.0.into(),
        domain: None,
        step: 1.0.into(),
        page: None,
        gap: 0.0.into(),
        max_gap: None,
        names: ["Minimum".into(), "Maximum".into()].into(),
        text: [None, None].into(),
        axis: Orientation::Horizontal,
        direction: Direction::Ltr,
        collision: RangeCollision::Push,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        inherit_theme: true,
        inherit_size: true,
        thumbs: [None, None],
        changed: None,
        committed: None,
        marks: Vec::new(),
        restrict_to_marks: false,
        inverted: false,
        show_value_labels: true,
        value_labels_always_on: false,
        scale: 1.,
    }
}
macro_rules! source_method {
    ($name:ident,$field:ident,$ty:ty) => {
        pub fn $name(mut self, value: impl Into<Vary<$ty>>) -> Self {
            self.$field = value.into();
            self
        }
    };
}
impl RangeSlider {
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self.inherit_theme = false;
        self
    }
    pub fn size(mut self, value: crate::Size) -> Self {
        self.common.size = value;
        self.inherit_size = false;
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
    pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(value.into());
        self
    }
    pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
        self.common.style = value.into();
        self
    }
    pub fn enabled(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.common.enabled = value.into();
        self
    }
    pub fn node_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn thumb_ref(mut self, thumb: RangeThumb, value: Cage<Option<NodeId>>) -> Self {
        self.thumbs[thumb.index()] = Some(value);
        self
    }
    pub fn value(mut self, value: impl Into<Vary<[f64; 2]>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: [f64; 2]) -> Self {
        self.value.default = value;
        self
    }
    pub fn page_step(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.page = Some(value.into());
        self
    }
    pub fn domain(mut self, value: impl Into<Vary<[f64; 2]>>) -> Self {
        self.domain = Some(value.into());
        self
    }
    pub fn marks(mut self, values: impl IntoIterator<Item = crate::SliderMark>) -> Self {
        self.marks = values.into_iter().collect();
        self
    }
    pub fn restrict_to_marks(mut self, value: bool) -> Self {
        self.restrict_to_marks = value;
        self
    }
    pub fn maximum_gap(mut self, value: impl Into<Vary<f64>>) -> Self {
        self.max_gap = Some(value.into());
        self
    }
    pub fn inverted(mut self, value: bool) -> Self {
        self.inverted = value;
        self
    }
    pub fn show_value_labels(mut self, value: bool) -> Self {
        self.show_value_labels = value;
        self
    }
    pub fn value_labels_always_on(mut self, value: bool) -> Self {
        self.value_labels_always_on = value;
        self
    }
    pub fn scale(mut self, factor: f64) -> Self {
        self.scale = factor;
        self
    }
    pub fn orientation(mut self, value: Orientation) -> Self {
        self.axis = value;
        self
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn collision(mut self, value: RangeCollision) -> Self {
        self.collision = value;
        self
    }
    pub fn push_on_overlap(mut self, value: bool) -> Self {
        self.collision = if value {
            RangeCollision::Push
        } else {
            RangeCollision::Hold
        };
        self
    }
    pub fn field(mut self, value: FieldControl) -> Self {
        self.field = Some(value);
        self
    }
    pub fn on_value_change(mut self, callback: impl FnMut([f64; 2]) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
    pub fn on_value_commit(mut self, callback: impl FnMut([f64; 2]) + 'static) -> Self {
        self.committed = Some(Box::new(callback));
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
    source_method!(min, min, f64);
    source_method!(max, max, f64);
    source_method!(step, step, f64);
    source_method!(minimum_gap, gap, f64);
    source_method!(labels, names, [String; 2]);
    source_method!(value_text, text, [Option<String>; 2]);
    source_method!(required, required, bool);
    source_method!(readonly, readonly, bool);
    source_method!(invalid, invalid, bool);
}
impl Widget for RangeSlider {
    fn build(&mut self, scope: &Scope) -> Result {
        let root_id = composite::id("range-slider", &self.key, "group", "root")?;
        let mut ids = [
            composite::id("range-slider", &self.key, "thumb", "lower")?,
            composite::id("range-slider", &self.key, "thumb", "upper")?,
        ];
        if let Some(field) = &self.field {
            ids[0] = field.id.clone();
            if self.inherit_theme {
                self.common.theme = field.theme.clone()
            }
            if self.inherit_size {
                self.common.size = field.size
            }
            self.common.unstyled |= field.unstyled;
        }
        let collision = self.collision;
        let local = self.value.source.is_none();
        let control = std::mem::take(&mut self.value).mount(scope);
        let enabled = self.field.as_ref().map_or_else(
            || self.common.enabled.clone(),
            |field| field.enabled(scope, self.common.enabled.clone()),
        );
        let metadata = control_metadata(
            scope,
            self.field.as_ref(),
            false,
            self.required.clone(),
            self.readonly.clone(),
            self.invalid.clone(),
        );
        let source = control.source.clone();
        let min = self.min.clone();
        let max = self.max.clone();
        let domain = self.domain.clone();
        let step = self.step.clone();
        let page = self.page.clone();
        let gap = self.gap.clone();
        let max_gap = self.max_gap.clone();
        let axis = self.axis;
        let direction = self.direction;
        let names = std::mem::replace(&mut self.names, ["Minimum".into(), "Maximum".into()].into());
        let text = std::mem::replace(&mut self.text, [None, None].into());
        let scale = self.scale;
        let field_metadata = metadata.clone();
        if self.marks.len() > 64 {
            return Err(invalid("range slider supports at most 64 marks"));
        }
        let marks: Rc<[crate::SliderMark]> = std::mem::take(&mut self.marks).into();
        let read_marks = marks.clone();
        let restrict_to_marks = self.restrict_to_marks;
        let read = move || -> Result<Snapshot> {
            let step = step.try_get()?;
            let page = page
                .as_ref()
                .map_or(Ok((step * 10.).min(1e12)), Vary::try_get)?;
            let min = min.try_get()?;
            let max = max.try_get()?;
            let mut state = Snapshot {
                values: source.try_get()?,
                min,
                max,
                domain: domain
                    .as_ref()
                    .map(Vary::try_get)
                    .transpose()?
                    .unwrap_or([min, max]),
                step,
                page,
                gap: gap.try_get()?,
                max_gap: max_gap.as_ref().map(Vary::try_get).transpose()?,
                collision,
                axis,
                direction,
                enabled: enabled.try_get()?,
                readonly: field_metadata.try_with(|s| s.as_ref().is_some_and(|s| s.readonly))?,
                names: [String::new(), String::new()],
                text: [None, None],
                scale,
                marks: read_marks.clone(),
                restrict_to_marks,
            };
            state.validate_numeric()?;
            state.names = names.try_with(|names| {
                for name in names {
                    if name.len() > 512 || name.trim().is_empty() {
                        return Err(invalid("range slider names require 1..=512 bytes"));
                    }
                }
                Ok::<_, glory::ui::Error>(names.clone())
            })??;
            state.text = text.try_with(|text| {
                for text in text.iter().flatten() {
                    if text.len() > 1024 || text.trim().is_empty() {
                        return Err(invalid("range slider value text requires 1..=1024 bytes"));
                    }
                }
                Ok::<_, glory::ui::Error>(text.clone())
            })??;
            Ok(state)
        };
        let accepted = scope.cage(read()?);
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let thumbs = [
            self.thumbs[0].unwrap_or_else(|| scope.cage(None)),
            self.thumbs[1].unwrap_or_else(|| scope.cage(None)),
        ];
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            control,
            local,
            accepted,
            root: reference,
            thumbs,
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
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
            expected: Cell::new(None),
            confirmed: Cell::new(None),
            drag: RefCell::new(None),
            dragging: scope.cage(None),
            active: Cell::new(None),
            pending: Cell::new(None),
            next: Cell::new(0),
        });
        let weak = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(state) = weak.upgrade() {
                state.retire()
            }
        })?;
        let structural = state.clone();
        scope.structural(move || {
            let result = (|| -> Result {
                let next = read()?;
                let old = structural.accepted.try_get()?;
                if next != old {
                    if !old.same_domain(&next)
                        || old.values != next.values
                            && structural.expected.get() != Some(next.values)
                    {
                        structural.cancel()
                    }
                    structural.accepted.try_revise(|old| *old = next)?;
                    structural.expected.set(None);
                    structural.confirmed.set(None);
                }
                Ok(())
            })();
            if let Err(error) = result {
                structural.cancel();
                structural.report(error)
            }
        });
        let size = match self.common.size {
            crate::Size::ExtraSmall => 24.,
            crate::Size::Small => 30.,
            crate::Size::Medium => 36.,
            crate::Size::Large => 42.,
            crate::Size::ExtraLarge => 50.,
        };
        let horizontal = axis == Orientation::Horizontal;
        let default_width = Length::Px(if horizontal { 240. } else { size });
        let has_mark_labels = marks
            .iter()
            .any(|mark| !mark.hidden && mark.label.is_some());
        let default_height = Length::Px(if horizontal {
            size + if has_mark_labels { 24. } else { 0. }
        } else {
            240.
        });
        let (classes, styles) = self.common.appearance(
            scope,
            "sapid-range-slider",
            Style::new()
                .position(Position::Relative)
                .width(Length::Px(if horizontal { 240. } else { size }))
                .height(Length::Px(if horizontal {
                    size + if has_mark_labels { 24. } else { 0. }
                } else {
                    240.
                })),
        );
        let root_metadata = metadata.clone();
        let mut root = glory::ui::view()
            .role(Role::Group)
            .class(classes)
            .node_ref(reference)
            .style(scope.bond(move || {
                let mut style = styles.get();
                style.width = style.width.or(Some(default_width));
                style.height = style.height.or(Some(default_height));
                style.position = Some(Position::Relative);
                style.padding = Some(Edges::all(0.));
                style.border_width = Some(Edges::all(0.));
                style
            }))
            .semantics(scope.bond(move || {
                root_metadata.with(|m| {
                    let mut s = glory::render::Semantics {
                        id: Some(root_id.clone()),
                        ..Default::default()
                    };
                    if let Some(m) = m {
                        s.labelled_by = m.labelled_by.clone();
                        s.described_by = m.described_by.clone();
                    }
                    composite::metadata(s)
                })
            }));
        if let Some(name) = self.common.name.clone() {
            root = root.name(name)
        }
        let resize = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Resize)
                .unwrap_or_default(),
        );
        let resized = state.clone();
        let owner = scope.id();
        root = root.on(EventKind::Resize, move |event| {
            resized.cancel();
            resize.emit(owner, event)
        });
        for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
            let observers = Observers::new(handlers);
            root = root.on(kind, move |event| observers.emit(owner, event))
        }
        root.show_in(scope)?;
        let parent = scope.child_in(reference.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        let accent = self.common.theme.accent;
        let border = self.common.theme.border;
        let surface = self.common.theme.surface;
        let foreground = self.common.theme.foreground;
        let unstyled = self.common.unstyled;
        let inverted = self.inverted;
        let show_value_labels = self.show_value_labels;
        let value_labels_always_on = self.value_labels_always_on;
        for filled in [false, true] {
            glory::ui::view()
                .class(if filled {
                    "sapid-range-slider-fill"
                } else {
                    "sapid-range-slider-track"
                })
                .style(scope.bond(move || {
                    accepted.with(|state| {
                        let fractions = state.values.map(|v| {
                            (v - state.domain[0]) / (state.domain[1] - state.domain[0]) * 100.
                        });
                        let start = if filled { fractions[0] } else { 0. };
                        let length = if filled {
                            fractions[1] - fractions[0]
                        } else {
                            100.
                        };
                        let mut style = Style::new().position(Position::Absolute);
                        if horizontal {
                            style.top = Some(Length::Px(size / 2. - 2.));
                            style.height = Some(Length::Px(4.));
                            style.width = Some(Length::Percent(length));
                            if direction == Direction::Rtl {
                                style.right = Some(Length::Percent(start))
                            } else {
                                style.left = Some(Length::Percent(start))
                            }
                        } else {
                            style.left = Some(Length::Px(size / 2. - 2.));
                            style.width = Some(Length::Px(4.));
                            style.height = Some(Length::Percent(length));
                            style.bottom = Some(Length::Percent(start));
                        }
                        if !unstyled {
                            style.background =
                                Some(if filled ^ inverted { accent } else { border });
                            style.radius = Some(2.);
                        }
                        style
                    })
                }))
                .show_in(&parent)?;
        }
        let mark_label_color = self.common.theme.muted;
        for mark in marks.iter().cloned() {
            if mark.hidden {
                continue;
            }
            let marker_state = accepted;
            glory::ui::view()
                .class("sapid-range-slider-mark")
                .style(scope.bond(move || {
                    let state = marker_state.get();
                    let fraction = ((mark.value - state.domain[0])
                        / (state.domain[1] - state.domain[0])
                        * 100.)
                        .clamp(0., 100.);
                    let mut style = Style::new()
                        .position(Position::Absolute)
                        .width(Length::Px(4.))
                        .height(Length::Px(4.));
                    if horizontal {
                        style.left = Some(Length::Percent(if direction == Direction::Rtl {
                            100. - fraction
                        } else {
                            fraction
                        }));
                        style.top = Some(Length::Px(size / 2. - 2.));
                        style.margin = Some(Edges {
                            left: -2.,
                            top: 0.,
                            right: 0.,
                            bottom: 0.,
                        });
                    } else {
                        style.bottom = Some(Length::Percent(fraction));
                        style.left = Some(Length::Px(size / 2. - 2.));
                    }
                    if !unstyled {
                        style.background = Some(border);
                        style.radius = Some(2.);
                    }
                    style
                }))
                .show_in(&parent)?;
            if let Some(label) = mark.label {
                let label_state = accepted;
                glory::ui::text()
                    .class("sapid-range-slider-mark-label")
                    .content(label)
                    .style(scope.bond(move || {
                        let state = label_state.get();
                        let fraction = ((mark.value - state.domain[0])
                            / (state.domain[1] - state.domain[0])
                            * 100.)
                            .clamp(0., 100.);
                        let mut style = Style::new().position(Position::Absolute);
                        if horizontal {
                            style.left = Some(Length::Percent(if direction == Direction::Rtl {
                                100. - fraction
                            } else {
                                fraction
                            }));
                            style.top = Some(Length::Px(size));
                        } else {
                            style.bottom = Some(Length::Percent(fraction));
                            style.left = Some(Length::Px(size));
                        }
                        style.font_size = Some(11.);
                        style.color = Some(mark_label_color);
                        style
                    }))
                    .show_in(&parent)?;
            }
        }
        for thumb in [RangeThumb::Lower, RangeThumb::Upper] {
            let index = thumb.index();
            let meta = metadata.clone();
            let id = ids[index].clone();
            let focused = scope.cage(false);
            let focus_ring = self.common.theme.focus_ring;
            if show_value_labels {
                let label_state = accepted;
                let dragging = state.dragging;
                let label_content =
                    scope.bond(move || label_state.with(|state| state.display_value(index)));
                glory::ui::view()
                    .class("sapid-range-slider-value-label")
                    .semantics(Some(Box::new(Semantics {
                        inert: true,
                        ..Default::default()
                    })))
                    .style(scope.bond(move || {
                        let state = label_state.get();
                        let mut fraction = (state.values[index] - state.domain[0])
                            / (state.domain[1] - state.domain[0])
                            * 100.;
                        if horizontal && direction == Direction::Rtl {
                            fraction = 100. - fraction
                        }
                        let mut style = Style::new().position(Position::Absolute);
                        if horizontal {
                            style.left = Some(Length::Percent(fraction));
                            style.top = Some(Length::Px(-18.));
                            style.margin = Some(Edges {
                                left: -18.,
                                top: 0.,
                                right: 0.,
                                bottom: 0.,
                            });
                        } else {
                            style.bottom = Some(Length::Percent(fraction));
                            style.left = Some(Length::Px(size + 8.));
                        }
                        style.padding = Some(Edges {
                            left: 6.,
                            top: 2.,
                            right: 6.,
                            bottom: 2.,
                        });
                        style.font_size = Some(11.);
                        style.opacity =
                            Some(if value_labels_always_on || dragging.get() == Some(thumb) {
                                1.
                            } else {
                                0.
                            });
                        if !unstyled {
                            style.background = Some(surface);
                            style.color = Some(foreground);
                            style.radius = Some(4.);
                        }
                        style
                    }))
                    .fill(glory::ui::text().content(label_content))
                    .show_in(&parent)?;
            }
            let mut widget = glory::ui::view()
                .role(Role::Slider)
                .node_ref(thumbs[index])
                .class(if index == 0 {
                    "sapid-range-slider-lower"
                } else {
                    "sapid-range-slider-upper"
                })
                .name(scope.bond(move || accepted.with(|s| s.names[index].clone())))
                .focus(scope.bond(move || {
                    accepted.with(|s| {
                        if s.enabled {
                            FocusMode::TabStop
                        } else {
                            FocusMode::Programmatic
                        }
                    })
                }))
                .semantics(scope.bond(move || {
                    let mut metadata = meta.get().map(|m| *m).unwrap_or_default();
                    accepted.with(|s| {
                        let value = s.metadata(thumb);
                        metadata.id = Some(id.clone());
                        // The group retains the Field label. Each actual thumb
                        // has its own explicit name; aria-labelledby would
                        // otherwise override it in the browser name algorithm.
                        metadata.labelled_by = None;
                        metadata.readonly = s.readonly || value.range.min == value.range.max;
                        metadata.disabled = !s.enabled;
                        if let Some(control) = metadata.form_control.as_mut()
                            && control.value.is_none()
                        {
                            control.value = Some(s.values[index].to_string())
                        }
                        metadata.slider = Some(value);
                    });
                    composite::metadata(metadata)
                }))
                .style(scope.bond(move || {
                    accepted.with(|s| {
                        let mut fraction =
                            (s.values[index] - s.domain[0]) / (s.domain[1] - s.domain[0]) * 100.;
                        if horizontal && direction == Direction::Rtl {
                            fraction = 100. - fraction
                        }
                        let mut style = Style::new()
                            .position(Position::Absolute)
                            .width(Length::Px(size))
                            .height(Length::Px(size));
                        if horizontal {
                            style.left = Some(Length::Percent(fraction));
                            style.top = Some(Length::Px(0.));
                            style.margin = Some(Edges {
                                left: -size / 2.,
                                top: 0.,
                                right: 0.,
                                bottom: 0.,
                            });
                        } else {
                            style.bottom = Some(Length::Percent(fraction));
                            style.left = Some(Length::Px(0.));
                            style.margin = Some(Edges {
                                bottom: -size / 2.,
                                top: 0.,
                                right: 0.,
                                left: 0.,
                            });
                        }
                        if focused.get() && !unstyled {
                            style.border_width = Some(Edges::all(1.));
                            style.border_style = Some(glory::style::BorderStyle::Solid);
                            style.border_color = Some(focus_ring);
                            style.radius = Some(size / 2.);
                        }
                        style
                    })
                }));
            let ranged = state.clone();
            widget = widget.on(EventKind::Range, move |event| {
                if let Event::Range { command, .. } = event
                    && let Err(error) = ranged.key(thumb, command)
                {
                    ranged.report(error);
                    return;
                }
                ranged.range_events.emit(owner, event)
            });
            let dragged = state.clone();
            widget = widget.on(EventKind::Drag, move |event| {
                if let Err(error) = dragged.pointer(thumb, event) {
                    dragged.report(error)
                }
            });
            widget = widget.on(EventKind::FocusWithin, move |event| {
                if let Event::FocusWithin { focused: next, .. } = event {
                    focused.revise(|current| *current = next)
                }
            });
            widget.show_in(&parent)?;
            let visual = scope.child_in(thumbs[index].try_get()?.ok_or(glory::ui::Error::Gone)?)?;
            let mut circle = Style::new()
                .position(Position::Absolute)
                .width(Length::Px(size / 2.))
                .height(Length::Px(size / 2.))
                .left(Length::Px(size / 4.))
                .top(Length::Px(size / 4.));
            if !unstyled {
                circle.background = Some(surface);
                circle.border_width = Some(Edges::all(2.));
                circle.border_style = Some(glory::style::BorderStyle::Solid);
                circle.border_color = Some(accent);
                circle.radius = Some(size / 4.);
            }
            glory::ui::view()
                .class("sapid-range-slider-thumb")
                .style(circle)
                .show_in(&visual)?;
            scope.retain(visual)?;
        }
        scope.retain(parent)?;
        Ok(())
    }
}
