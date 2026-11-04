//! A single real editor with font-aligned passive cells and a bounded String model.
use crate::FieldControl;
use crate::appearance::{Common, common_methods};
use crate::composite::{self, Callback, Observers};
use crate::field::control_metadata;
use crate::pin_model::{PinAlphabet, PinDomain, PinRejection};
use crate::state::{Control, Dirty, Value};
use glory::render::{Event, EventKind, InputAssist, InputAutofill, InputMode, Purpose, Semantics};
use glory::style::{
    BorderStyle, Color, Display, Edges, Family, Length, Limit, Overflow, PointerEvents, Position,
    Style,
};
use glory::ui::{Result, WeakScope, Widget};
use glory::{Cage, Scope, Vary};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
const MAX_PREEDIT_BYTES: usize = 128;
fn invalid() -> glory::ui::Error {
    glory::render::Error::InvalidProps("invalid bounded PIN input configuration").into()
}
#[derive(Clone, PartialEq)]
struct Snapshot {
    value: String,
    enabled: bool,
    metadata: Semantics,
}
impl Snapshot {
    fn writable(&self) -> bool {
        self.enabled && !self.metadata.readonly
    }
}
struct Controller {
    owner: glory::core::InstanceId,
    weak: WeakScope,
    live: Cell<bool>,
    domain: PinDomain,
    control: Control<String>,
    accepted: Cage<Snapshot>,
    dirty: Dirty,
    changed: Callback<String>,
    committed: Callback<String>,
    completed: Callback<String>,
    rejected: Callback<PinRejection>,
    composing: Cell<bool>,
    draft: RefCell<Option<String>>,
    ended: Cell<bool>,
    echo: RefCell<Option<(u64, String)>>,
}
impl Controller {
    fn ensure(&self) -> Result {
        if self.live.get() && self.owner.is_alive() {
            Ok(())
        } else {
            Err(glory::ui::Error::Gone)
        }
    }
    fn current(&self) -> Result<String> {
        Ok(self
            .control
            .source
            .try_with(|v| self.domain.validate_value(v).ok().map(|()| v.clone()))?
            .unwrap_or(self.accepted.try_with(|s| s.value.clone())?))
    }
    fn propose(&self, value: &str, commit: bool) -> Result {
        self.ensure()?;
        if !self.accepted.try_with(Snapshot::writable)? {
            return Ok(());
        }
        if let Err(error) = self.domain.validate_value(value) {
            self.dirty.touch()?;
            self.rejected.emit(self.owner, error);
            return Ok(());
        }
        let before = self.current()?;
        let proposal = value.to_owned();
        self.control.accept(&proposal)?;
        self.dirty.touch()?;
        if commit {
            self.committed.emit(self.owner, proposal.clone());
        } else {
            self.changed.emit(self.owner, proposal.clone());
        }
        if self.ensure().is_ok()
            && self
                .domain
                .completed_user_transition(&before, &proposal, &self.current()?)
        {
            self.completed.emit(self.owner, proposal);
        }
        Ok(())
    }
    fn text(&self, value: &str, commit: bool) -> Result {
        if self.composing.get() {
            if value.len() <= MAX_PREEDIT_BYTES {
                *self.draft.borrow_mut() = Some(value.to_owned());
            } else {
                self.draft.borrow_mut().take();
                self.dirty.touch()?;
                self.rejected.emit(self.owner, PinRejection::TooLong);
            }
            return Ok(());
        }
        self.propose(value, commit)
    }
    fn event(&self, event: &Event) -> Result {
        match event {
            Event::ValueChanged {
                value,
                composing,
                revision,
            } => {
                if *composing {
                    self.composing.set(true);
                }
                let echo = self.echo.borrow_mut().take();
                if !composing
                    && echo
                        .as_ref()
                        .is_some_and(|(r, v)| r == revision && v == value)
                {
                    return Ok(());
                }
                let ended =
                    !composing && (self.ended.replace(false) || self.composing.replace(false));
                if ended {
                    self.draft.borrow_mut().take();
                }
                self.text(value, false)?;
                if ended && value.len() <= MAX_PREEDIT_BYTES && self.ensure().is_ok() {
                    *self.echo.borrow_mut() = Some((*revision, value.clone()));
                    self.dirty.touch()?;
                }
                Ok(())
            }
            Event::Commit { value, .. } => self.text(value, true),
            Event::Composition { active } => {
                self.composing.set(*active);
                if *active {
                    self.ended.set(false);
                    self.echo.borrow_mut().take();
                    Ok(())
                } else {
                    self.ended.set(true);
                    self.dirty.touch()
                }
            }
            _ => Ok(()),
        }
    }
    fn report(&self, result: Result) {
        if let Err(error) = result
            && let Some(scope) = self.weak.upgrade()
        {
            scope.report_error(error);
        }
    }
    fn retire(&self) {
        self.live.set(false);
        self.changed.clear();
        self.committed.clear();
        self.completed.clear();
        self.rejected.clear();
        self.draft.borrow_mut().take();
        self.echo.borrow_mut().take();
    }
}
pub struct PinInput {
    common: Common,
    value: Value<String>,
    domain: PinDomain,
    masked: bool,
    gap: f64,
    cell_padding: Option<f64>,
    autofill: Option<InputAutofill>,
    field: Option<FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    form_name: Option<Vary<String>>,
    changed: Option<Box<dyn FnMut(String)>>,
    committed: Option<Box<dyn FnMut(String)>>,
    completed: Option<Box<dyn FnMut(String)>>,
    rejected: Option<Box<dyn FnMut(PinRejection)>>,
}
impl Default for PinInput {
    fn default() -> Self {
        pin_input()
    }
}
pub fn pin_input() -> PinInput {
    PinInput {
        common: Common::default(),
        value: Value::default(),
        domain: PinDomain::default(),
        masked: false,
        gap: 8.,
        cell_padding: None,
        autofill: None,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        form_name: None,
        changed: None,
        committed: None,
        completed: None,
        rejected: None,
    }
}
/// The same single editor with an explicit one-time-code autofill hint.
pub fn input_otp() -> PinInput {
    pin_input().autofill(InputAutofill::OneTimeCode)
}
impl PinInput {
    common_methods!();
    /// Space between passive glyph cells; one native editor owns the whole code.
    pub fn gap(mut self, value: f64) -> Self {
        self.gap = value;
        self
    }
    /// Horizontal padding on each side of the actual zero-glyph advance.
    pub fn cell_padding(mut self, value: f64) -> Self {
        self.cell_padding = Some(value);
        self
    }
    pub fn length(mut self, length: usize) -> Self {
        self.domain.length = length;
        self
    }
    pub fn alphabet(mut self, alphabet: PinAlphabet) -> Self {
        self.domain.alphabet = alphabet;
        self
    }
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }
    pub fn autofill(mut self, hint: InputAutofill) -> Self {
        self.autofill = Some(hint);
        self
    }
    pub fn value(mut self, value: impl Into<Vary<String>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.value.default = value.into();
        self
    }
    pub fn required(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.required = value.into();
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn invalid(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.invalid = value.into();
        self
    }
    pub fn form_name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(value.into());
        self
    }
    pub fn field(mut self, field: FieldControl) -> Self {
        self.common.theme = field.theme.clone();
        self.common.size = field.size;
        self.common.unstyled = field.unstyled;
        self.field = Some(field);
        self
    }
    pub fn on_value_change(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.changed = Some(Box::new(handler));
        self
    }
    pub fn on_commit(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.committed = Some(Box::new(handler));
        self
    }
    pub fn on_complete(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.completed = Some(Box::new(handler));
        self
    }
    pub fn on_reject(mut self, handler: impl FnMut(PinRejection) + 'static) -> Self {
        self.rejected = Some(Box::new(handler));
        self
    }
}
impl Widget for PinInput {
    fn build(&mut self, scope: &Scope) -> Result {
        let domain = self.domain;
        domain.validate().map_err(|_| invalid())?;
        let gap = self.gap;
        let padding = self
            .cell_padding
            .unwrap_or(self.common.theme.tokens.control(self.common.size).padding_x);
        if !gap.is_finite()
            || !(0.0..=32.0).contains(&gap)
            || !padding.is_finite()
            || !(0.0..=32.0).contains(&padding)
        {
            return Err(invalid());
        }
        let tracking = padding * 2. + gap;
        // Reserve the native editor's trailing advance and caret. Cell geometry
        // and text advances share the same real font-relative ch term.
        let editor_width = Length::ChPx {
            ch: domain.length as f64,
            px: domain.length as f64 * tracking + padding + 1.,
        };
        if let Some(source) = &self.value.source {
            source
                .try_with(|v| domain.validate_value(v))?
                .map_err(|_| invalid())?;
        } else {
            domain
                .validate_value(&self.value.default)
                .map_err(|_| invalid())?;
        }
        let control = std::mem::take(&mut self.value).mount(scope);
        let value = composite::validated(scope, &control.source, move |v| {
            domain.validate_value(v).map_err(|_| invalid())
        })?;
        let enabled = self
            .field
            .as_ref()
            .map(|f| f.enabled(scope, self.common.enabled.clone()))
            .unwrap_or_else(|| self.common.enabled.clone());
        let metadata = control_metadata(
            scope,
            self.field.as_ref(),
            self.common.name.is_some(),
            self.required.clone(),
            self.readonly.clone(),
            self.invalid.clone(),
        );
        let form_name = composite::form_name(scope, &self.form_name)?;
        let assist = InputAssist {
            autofill: self.autofill,
            mode: Some(match domain.alphabet {
                PinAlphabet::Digits => InputMode::Numeric,
                PinAlphabet::AsciiAlphanumeric => InputMode::Text,
            }),
        };
        let source = scope.bond(move || {
            let mut metadata = metadata.get().map(|s| *s).unwrap_or_default();
            metadata.input_assist = Some(assist);
            if let Some(name) = form_name {
                metadata.form_control = Some(glory::render::FormControl {
                    name: name.get(),
                    value: None,
                });
            }
            Snapshot {
                value: value.get(),
                enabled: enabled.get(),
                metadata,
            }
        });
        let accepted = composite::validated(scope, &source.into(), move |s: &Snapshot| {
            domain.validate_value(&s.value).map_err(|_| invalid())
        })?;
        let styles = composite::validated(scope, &self.common.style, |style: &Style| {
            if style
                .font_family
                .as_ref()
                .is_some_and(|faces| faces.len() != 1 || faces[0] != Family::Mono)
            {
                return Err(invalid());
            }
            if style.letter_spacing.is_some() {
                return Err(invalid());
            }
            Ok(())
        })?;
        let dirty = Dirty::new(scope);
        let state = Rc::new(Controller {
            owner: scope.id(),
            weak: scope.downgrade(),
            live: Cell::new(true),
            domain,
            control,
            accepted,
            dirty: dirty.clone(),
            changed: Callback::new(self.changed.take()),
            committed: Callback::new(self.committed.take()),
            completed: Callback::new(self.completed.take()),
            rejected: Callback::new(self.rejected.take()),
            composing: Cell::new(false),
            draft: RefCell::new(None),
            ended: Cell::new(false),
            echo: RefCell::new(None),
        });
        let cleanup = Rc::downgrade(&state);
        scope.on_cleanup(move || {
            if let Some(s) = cleanup.upgrade() {
                s.retire();
            }
        })?;
        let base = self.common.theme.control(self.common.size);
        let font = base.font_size.unwrap_or(14.);
        let height = base.min_height.unwrap_or(Length::Px(36.));
        let theme = self.common.theme.clone();
        let themed = !self.common.unstyled;
        self.common.style = scope
            .bond(move || {
                let mut s = styles.get();
                let font = s.font_size.unwrap_or(font);
                s.font_family = Some(vec![Family::Mono]);
                s.font_size.get_or_insert(font);
                s.line_height.get_or_insert(font * 1.55);
                s.width.get_or_insert(editor_width);
                s.min_width.get_or_insert(Length::Px(0.));
                s.max_width.get_or_insert(Limit::Percent(100.));
                s.overflow_x.get_or_insert(Overflow::Auto);
                let default_height = match height {
                    Length::Px(h) => Length::Px(h.max(font * 1.55)),
                    _ => height,
                };
                s.height.get_or_insert(default_height);
                if themed {
                    s.color.get_or_insert(if accepted.with(|v| v.enabled) {
                        theme.foreground
                    } else {
                        theme.disabled_text
                    });
                }
                s
            })
            .into();
        let geometry = self.common.style.clone();
        let (classes, appearance) = self.common.appearance(
            scope,
            "sapid-pin-input",
            Style::new().display(Display::InlineFlex),
        );
        let outer = scope.cage(None);
        let focused = scope.cage(false);
        glory::ui::view()
            .class(classes)
            .style(appearance)
            .node_ref(outer)
            .on(EventKind::FocusWithin, move |event| {
                if let Event::FocusWithin { focused: value, .. } = event {
                    focused.revise(|old| *old = value);
                }
            })
            .show_in(scope)?;
        let parent = scope.child_in(outer.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        let frame = scope.cage(None);
        glory::ui::view()
            .class("sapid-pin-frame")
            .style(scope.bond(move || {
                let s = geometry.get();
                Style {
                    shrink: Some(0.),
                    ..Style::new()
                }
                .position(Position::Relative)
                .width(editor_width)
                .height(s.height.filter(|h| *h != Length::Auto).unwrap_or(height))
            }))
            .node_ref(frame)
            .show_in(&parent)?;
        let content = scope.child_in(frame.try_get()?.ok_or(glory::ui::Error::Gone)?)?;
        for index in 0..domain.length {
            let theme = self.common.theme.clone();
            glory::ui::view()
                .class("sapid-pin-cell")
                .style(scope.bond(move || {
                    let mut s = Style::new()
                        .position(Position::Absolute)
                        .left(Length::ChPx {
                            ch: index as f64,
                            px: index as f64 * tracking,
                        })
                        .top(Length::Px(0.))
                        .width(Length::ChPx {
                            ch: 1.,
                            px: padding * 2.,
                        })
                        .height(Length::Percent(100.))
                        .pointer_events(PointerEvents::None);
                    if themed {
                        s = s
                            .border_width(Edges {
                                left: if index == 0 || gap > 0. { 1. } else { 0. },
                                right: 1.,
                                top: 1.,
                                bottom: 1.,
                            })
                            .border_style(BorderStyle::Solid)
                            .radius(theme.radius)
                            .border_color(if accepted.with(|v| v.metadata.invalid) {
                                theme.danger
                            } else if focused.get() {
                                theme.accent
                            } else {
                                theme.border
                            });
                        s.background =
                            Some(if accepted.with(|v| v.enabled && !v.metadata.readonly) {
                                theme.input
                            } else {
                                theme.disabled
                            });
                    }
                    s
                }))
                .show_in(&content)?;
        }
        let initial = accepted.try_with(|s| s.value.clone())?;
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let mut input = glory::ui::input()
            .class("sapid-pin-editor")
            .purpose(if self.masked {
                Purpose::Password
            } else {
                Purpose::Text
            })
            .value(initial.clone())
            .focus(self.common.focus)
            .enabled(scope.bond(move || accepted.with(|s| s.enabled)))
            .semantics(
                scope.bond(move || composite::metadata(accepted.with(|s| s.metadata.clone()))),
            )
            .node_ref(reference)
            .style(
                Style {
                    background: Some(Color(0, 0, 0, 0)),
                    ..Style::new()
                }
                .position(Position::Relative)
                .width(editor_width)
                .height(Length::Percent(100.))
                .min_width(Length::Px(0.))
                .padding(Edges {
                    left: padding,
                    right: 1.,
                    top: 0.,
                    bottom: 0.,
                })
                .border_width(Edges::all(0.))
                .letter_spacing(tracking)
                .font_family([Family::Mono]),
            );
        if let Some(name) = self.common.name.take() {
            input = input.name(name);
        }
        for kind in [
            EventKind::ValueChanged,
            EventKind::Commit,
            EventKind::Composition,
        ] {
            let s = state.clone();
            let observers = Observers::new(self.common.handlers.remove(&kind).unwrap_or_default());
            let cleanup = observers.clone();
            scope.on_cleanup(move || cleanup.clear())?;
            input = input.on(kind, move |event| { if s.ensure().is_err() { return; } s.report(s.event(&event)); if !matches!(&event, Event::ValueChanged {value, ..} | Event::Commit {value, ..} if value.len() > MAX_PREEDIT_BYTES) { observers.emit(s.owner, event); } });
        }
        for (kind, mut handlers) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            input = input.on(kind, move |event| {
                crate::appearance::observe(&mut handlers, owner, event)
            });
        }
        input.show_in(&content)?;
        let node = reference.try_get()?.ok_or(glory::ui::Error::Gone)?;
        scope.retain(content)?;
        scope.retain(parent)?;
        let weak = Rc::downgrade(&state);
        let owner = scope.downgrade();
        let mut previous = initial;
        scope.effect(move || {
            let (Some(s), Some(scope)) = (weak.upgrade(), owner.upgrade()) else {
                return;
            };
            let result = (|| -> Result {
                let edited = dirty.take();
                if s.ended.replace(false) {
                    let draft = s.draft.borrow_mut().take();
                    if let Some(value) = draft {
                        s.propose(&value, false)?;
                        return Ok(());
                    }
                }
                let value = accepted.try_with(|s| s.value.clone())?;
                if !s.composing.get() && (edited || value != previous) {
                    scope.set_input_value(node, value.clone())?;
                    previous = value;
                    s.echo.borrow_mut().take();
                }
                Ok(())
            })();
            s.report(result);
        });
        scope.retain(state)?;
        Ok(())
    }
}
