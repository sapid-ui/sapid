//! Editable hexadecimal color drafts with the shared Popover/ColorPicker.
use crate::{
    appearance::{Common, observe},
    color_model::{ColorFormat, Hsva, format_color, parse_color, parse_hex_color},
    composite::{self, Callback},
    state::Value,
};
use glory::render::{Direction, Event, EventKind, NodeId, Semantics};
use glory::style::Style;
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::Cell, future::Future, pin::Pin, rc::Rc};
const MAX_DRAFT_BYTES: usize = 256;

pub type ColorEyeDropperResult = std::result::Result<Option<glory::style::Color>, String>;
type ColorEyeDropperFuture = Pin<Box<dyn Future<Output = ColorEyeDropperResult>>>;

fn source_version<T>(source: &Vary<T>) -> Option<u64> {
    match source {
        Vary::Cage(source) => source.try_version().ok(),
        _ => None,
    }
}

/// An application-supplied color selection service. Sapid never reaches for an
/// ambient browser or operating-system eyedropper capability on its own.
#[derive(Clone)]
pub struct ColorEyeDropper {
    pick: Rc<dyn Fn() -> ColorEyeDropperFuture>,
}
impl ColorEyeDropper {
    pub fn new<F: Future<Output = ColorEyeDropperResult> + 'static>(
        pick: impl Fn() -> F + 'static,
    ) -> Self {
        Self {
            pick: Rc::new(move || Box::pin(pick())),
        }
    }
    fn pick(&self) -> ColorEyeDropperFuture {
        (self.pick)()
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorValidity {
    Empty,
    Valid(glory::style::Color),
    Invalid(crate::ColorError),
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorInputSectionPosition {
    Left,
    #[default]
    Right,
}
fn draft(value: &str) -> Result {
    if value.len() > MAX_DRAFT_BYTES || value.contains(['\0', '\r', '\n']) {
        Err(glory::render::Error::InvalidProps(
            "color drafts require at most 256 canonical single-line bytes",
        )
        .into())
    } else {
        Ok(())
    }
}
// Keep exact picker coordinates while the source echoes their formatted color.
// Re-decoding that echo would round hue/alpha and look like an external write
// to the shared Slider controller, cancelling an otherwise accepted drag.
fn picker_coordinates(color: glory::style::Color, format: ColorFormat, previous: Hsva) -> Hsva {
    let encoded = previous
        .to_rgba()
        .ok()
        .and_then(|color| parse_color(&format_color(color, format), format).ok());
    if encoded == Some(color) {
        return previous;
    }
    let mut next = Hsva::from_rgba(color);
    if next.saturation == 0. {
        next.hue = previous.hue;
    }
    next
}

pub struct ColorInput {
    key: String,
    common: Common,
    value: Value<String>,
    field: Option<crate::FieldControl>,
    readonly: Vary<bool>,
    required: Vary<bool>,
    invalid: Vary<bool>,
    success: Vary<bool>,
    open: Value<bool>,
    open_change: Option<Box<dyn FnMut(bool)>>,
    change: Option<Box<dyn FnMut(String)>>,
    commit: Option<Box<dyn FnMut(String)>>,
    validation: Option<Box<dyn FnMut(ColorValidity)>>,
    alpha: bool,
    picker: bool,
    direction: Direction,
    format: ColorFormat,
    fix_on_blur: bool,
    disallow_input: bool,
    preview: bool,
    swatches: Vec<String>,
    swatches_per_row: usize,
    close_on_swatch: bool,
    loading: Vary<bool>,
    loading_position: ColorInputSectionPosition,
    left: Option<Children>,
    right: Option<Children>,
    eye_dropper: Option<ColorEyeDropper>,
    eye_dropper_label: Vary<String>,
    eye_dropper_icon: Option<Children>,
    eye_dropper_error: Option<Box<dyn FnMut(String)>>,
}
pub fn color_input(key: impl Into<String>) -> ColorInput {
    ColorInput {
        key: key.into(),
        common: Common::default(),
        value: Value {
            source: None,
            default: "#ff0000".into(),
        },
        field: None,
        readonly: false.into(),
        required: false.into(),
        invalid: false.into(),
        success: false.into(),
        open: Value::default(),
        open_change: None,
        change: None,
        commit: None,
        validation: None,
        alpha: false,
        picker: true,
        direction: Direction::Ltr,
        format: ColorFormat::Hex,
        fix_on_blur: true,
        disallow_input: false,
        preview: true,
        swatches: Vec::new(),
        swatches_per_row: 7,
        close_on_swatch: false,
        loading: false.into(),
        loading_position: ColorInputSectionPosition::Right,
        left: None,
        right: None,
        eye_dropper: None,
        eye_dropper_label: "Pick color from screen".into(),
        eye_dropper_icon: None,
        eye_dropper_error: None,
    }
}
impl ColorInput {
    pub fn format(mut self, value: ColorFormat) -> Self {
        self.format = value;
        self.alpha = value.alpha();
        self
    }
    pub fn fix_on_blur(mut self, value: bool) -> Self {
        self.fix_on_blur = value;
        self
    }
    pub fn disallow_input(mut self, value: bool) -> Self {
        self.disallow_input = value;
        self
    }
    pub fn with_preview(mut self, value: bool) -> Self {
        self.preview = value;
        self
    }
    pub fn swatches(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.swatches = values.into_iter().map(Into::into).collect();
        self
    }
    pub fn swatches_per_row(mut self, value: usize) -> Self {
        self.swatches_per_row = value;
        self
    }
    pub fn close_on_color_swatch_click(mut self, value: bool) -> Self {
        self.close_on_swatch = value;
        self
    }
    pub fn loading(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.loading = value.into();
        self
    }
    pub fn loading_position(mut self, value: ColorInputSectionPosition) -> Self {
        self.loading_position = value;
        self
    }
    pub fn left_section(mut self, children: impl IntoChildren) -> Self {
        self.left = Some(children.into_children());
        self
    }
    pub fn right_section(mut self, children: impl IntoChildren) -> Self {
        self.right = Some(children.into_children());
        self
    }
    pub fn eye_dropper(mut self, service: ColorEyeDropper) -> Self {
        self.eye_dropper = Some(service);
        self
    }
    pub fn eye_dropper_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.eye_dropper_label = value.into();
        self
    }
    pub fn eye_dropper_icon(mut self, children: impl IntoChildren) -> Self {
        self.eye_dropper_icon = Some(children.into_children());
        self
    }
    pub fn on_eye_dropper_error(mut self, value: impl FnMut(String) + 'static) -> Self {
        self.eye_dropper_error = Some(Box::new(value));
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
    pub fn on_value_change(mut self, value: impl FnMut(String) + 'static) -> Self {
        self.change = Some(Box::new(value));
        self
    }
    pub fn on_commit(mut self, value: impl FnMut(String) + 'static) -> Self {
        self.commit = Some(Box::new(value));
        self
    }
    pub fn on_validation(mut self, value: impl FnMut(ColorValidity) + 'static) -> Self {
        self.validation = Some(Box::new(value));
        self
    }
    pub fn open(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.open.source = Some(value.into());
        self
    }
    pub fn default_open(mut self, value: bool) -> Self {
        self.open.default = value;
        self
    }
    pub fn on_open_change(mut self, value: impl FnMut(bool) + 'static) -> Self {
        self.open_change = Some(Box::new(value));
        self
    }
    pub fn field(mut self, value: crate::FieldControl) -> Self {
        self.common.theme = value.theme.clone();
        self.common.size = value.size;
        self.common.unstyled = value.unstyled;
        self.field = Some(value);
        self
    }
    pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(value.into());
        self
    }
    pub fn theme(mut self, value: crate::Theme) -> Self {
        self.common.theme = value;
        self
    }
    pub fn size(mut self, value: crate::Size) -> Self {
        self.common.size = value;
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
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn enabled(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.common.enabled = value.into();
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn required(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.required = value.into();
        self
    }
    pub fn invalid(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.invalid = value.into();
        self
    }
    pub fn success(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.success = value.into();
        self
    }
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    /// Explicit hex-editor-only mode for hosts without top-layer support.
    pub fn show_picker(mut self, value: bool) -> Self {
        self.picker = value;
        self
    }
    pub fn show_alpha(mut self, value: bool) -> Self {
        self.alpha = value;
        self
    }
    pub fn node_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn on(mut self, kind: EventKind, value: impl FnMut(Event) + 'static) -> Self {
        self.common
            .handlers
            .entry(kind)
            .or_default()
            .push(Box::new(value));
        self
    }
}
impl Widget for ColorInput {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.len() > crate::MAX_COLOR_PICKER_KEY_BYTES - 2 {
            return Err(glory::render::Error::InvalidProps(
                "color input keys are limited to 46 bytes",
            )
            .into());
        }
        if !self.picker
            && self.swatches.is_empty()
            && (self.open.source.is_some() || self.open.default || self.open_change.is_some())
        {
            return Err(glory::render::Error::InvalidProps(
                "a color input without picker or swatches cannot configure a popup",
            )
            .into());
        }
        if self.swatches.len() > 64 || !(1..=16).contains(&self.swatches_per_row) {
            return Err(glory::render::Error::InvalidProps(
                "color input requires at most 64 swatches and 1..=16 swatches per row",
            )
            .into());
        }
        let swatches = self
            .swatches
            .iter()
            .map(|value| {
                parse_hex_color(value)
                    .map(|color| (value.clone(), color))
                    .map_err(|_| {
                        glory::ui::Error::from(glory::render::Error::InvalidProps(
                            "color input swatches require hexadecimal colors",
                        ))
                    })
            })
            .collect::<Result<Vec<_>>>()?;
        let root_id = composite::id("color-input", &self.key, "root", "root")?;
        let error_id = composite::id("color-input", &self.key, "error", "root")?;
        let control = std::mem::take(&mut self.value).mount(scope);
        let accepted = composite::validated(scope, &control.source, |value| draft(value))?;
        let format = self.format;
        let status = scope.bond(move || {
            accepted.with(|value| {
                if value.is_empty() {
                    ColorValidity::Empty
                } else {
                    match parse_color(value, format) {
                        Ok(value) => ColorValidity::Valid(value),
                        Err(error) => ColorValidity::Invalid(error),
                    }
                }
            })
        });
        let palette = scope.cage(Hsva::default());
        let validation = Callback::new(self.validation.take());
        let change = Callback::new(self.change.take());
        let commit = Callback::new(self.commit.take());
        let opened = Callback::new(self.open_change.take());
        let owner = scope.id();
        let weak = scope.downgrade();
        let observed = validation.clone();
        let mut previous = None;
        scope.effect(move || {
            let next = status.get();
            if let ColorValidity::Valid(color) = next {
                let old = palette.get();
                let hsv = picker_coordinates(color, format, old);
                if hsv != old {
                    palette.revise(|value| *value = hsv);
                }
            }
            if previous != Some(next) {
                previous = Some(next);
                if weak.upgrade().is_some() {
                    glory::core::untracked(|| observed.emit(owner, next));
                }
            }
        });
        for callback in [change.clone(), commit.clone()] {
            scope.on_cleanup(move || callback.clear())?;
        }
        scope.on_cleanup(move || validation.clear())?;
        let cleanup = opened.clone();
        scope.on_cleanup(move || cleanup.clear())?;
        let open = std::mem::take(&mut self.open).mount(scope);
        let enabled = self.field.as_ref().map_or_else(
            || self.common.enabled.clone(),
            |field| field.enabled(scope, self.common.enabled.clone()),
        );
        let readonly = self.field.as_ref().map_or_else(
            || self.readonly.clone(),
            |field| field.readonly_state(scope, self.readonly.clone()),
        );
        let composing = scope.cage(false);
        let read = readonly.clone();
        let active = enabled.clone();
        let picker_enabled = scope.bond(move || active.get() && !read.get() && !composing.get());
        let parse_message = scope.bond(move || match status.get() {
            ColorValidity::Invalid(error) => error.to_string(),
            _ => String::new(),
        });
        let invalid = self.invalid.clone();
        let error =
            scope.bond(move || invalid.get() || matches!(status.get(), ColorValidity::Invalid(_)));
        let last_valid = scope.cage(control.source.get());
        let remember = last_valid;
        let watch = control.source.clone();
        scope.effect(move || {
            let value = watch.get();
            if value.is_empty() || parse_color(&value, format).is_ok() {
                remember.revise(|old| *old = value);
            }
        });
        let has_left = self.preview
            || self.left.is_some()
            || self.loading_position == ColorInputSectionPosition::Left;
        let has_right = self.right.is_some()
            || self.eye_dropper.is_some()
            || self.loading_position == ColorInputSectionPosition::Right;
        let success = self.success.clone();
        let success_color = self.common.theme.success;
        let input_style =
            if has_left || has_right || self.success.is_reactive() || self.success.get() {
                let custom = self.common.style.clone();
                let mut fallback = self
                    .common
                    .theme
                    .control(self.common.size)
                    .padding
                    .unwrap_or_else(|| glory::style::Edges::all(0.));
                if has_left {
                    fallback.left = fallback.left.max(38.);
                }
                if has_right {
                    fallback.right = fallback.right.max(38.);
                }
                scope
                    .bond(move || {
                        let mut style = custom.get();
                        let mut padding = style.padding.unwrap_or(fallback);
                        if has_left {
                            padding.left = padding.left.max(38.);
                        }
                        if has_right {
                            padding.right = padding.right.max(38.);
                        }
                        style.padding = Some(padding);
                        if success.get() && !error.get() {
                            style = style.border_color(success_color);
                        }
                        style
                    })
                    .into()
            } else {
                self.common.style.clone()
            };
        let mut input = crate::text_field()
            .theme(self.common.theme.clone())
            .size(self.common.size)
            .value(accepted)
            .enabled(enabled.clone())
            .readonly(scope.bond({
                let readonly = self.readonly.clone();
                let disallow = self.disallow_input;
                move || readonly.get() || disallow
            }))
            .required(self.required.clone())
            .invalid(error)
            .class("sapid-color-editor")
            .style(input_style)
            .additional_error(error_id.clone(), parse_message.into());
        let input_id = self
            .field
            .as_ref()
            .map(|field| field.id.clone())
            .unwrap_or_else(|| root_id.clone());
        input = input.semantic_id(input_id.clone());
        if self.common.unstyled {
            input = input.unstyled();
        }
        if let Some(field) = self.field.take() {
            input = input.field(field);
        }
        if let Some(name) = self.common.name.take() {
            input = input.name(name);
        } else if input.field.is_none() {
            input = input.name("Color");
        }
        let input_reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        input = input.node_ref(input_reference);
        let returning_focus = Rc::new(Cell::new(false));
        let editing = control.clone();
        let changed = change.clone();
        let weak = scope.downgrade();
        input = input.on_value_change(move |value| {
            let result = draft(&value).and_then(|()| editing.accept(&value));
            match result {
                Ok(()) => changed.emit(owner, value),
                Err(error) => {
                    if let Some(scope) = weak.upgrade() {
                        scope.report_error(error)
                    }
                }
            }
        });
        let committed = commit.clone();
        let corrected = change.clone();
        let fixing = control.clone();
        let fix_on_blur = self.fix_on_blur;
        let weak = scope.downgrade();
        let finish = Rc::new(move |value: String| {
            if fix_on_blur && !value.is_empty() && parse_color(&value, format).is_err() {
                let restored = last_valid.get();
                if let Err(error) = fixing.accept(&restored) {
                    if let Some(scope) = weak.upgrade() {
                        scope.report_error(error);
                    }
                    return;
                }
                corrected.emit(owner, restored.clone());
                committed.emit(owner, restored);
            } else {
                committed.emit(owner, value)
            }
        });
        let submit = finish.clone();
        input = input.on_commit(move |value| submit(value));
        // A browser text input commits on Enter; leaving it is a distinct
        // focus notification. Restore invalid drafts on that real blur too.
        let draft_source = control.source.clone();
        let correcting_enabled = enabled.clone();
        let correcting_readonly = readonly.clone();
        input = input.on(EventKind::FocusChanged, move |event| {
            if matches!(event, Event::FocusChanged(false))
                && fix_on_blur
                && correcting_enabled.try_get().is_ok_and(|active| active)
                && correcting_readonly
                    .try_get()
                    .is_ok_and(|readonly| !readonly)
                && composing.try_get().is_ok_and(|active| !active)
                && let Ok(value) = draft_source.try_get()
                && !value.is_empty()
                && parse_color(&value, format).is_err()
            {
                finish(value);
            }
        });
        let mut observers = self
            .common
            .handlers
            .remove(&EventKind::Composition)
            .unwrap_or_default();
        input = input.on(EventKind::Composition, move |event| {
            if let Event::Composition { active } = event {
                composing.revise(|value| *value = active);
            }
            observe(&mut observers, owner, event);
        });
        if self.picker || !self.swatches.is_empty() {
            let focus_open = open.clone();
            let returning = returning_focus.clone();
            let changed = opened.clone();
            let available = picker_enabled;
            let weak = scope.downgrade();
            input = input.on(EventKind::FocusChanged, move |event| {
                if let Event::FocusChanged(active) = event {
                    if !active {
                        returning.set(false);
                    } else if !returning.replace(false) {
                        let result = (|| -> Result {
                            if !available.try_get()? || focus_open.source.try_get()? {
                                return Ok(());
                            }
                            focus_open.accept(&true)?;
                            changed.emit(owner, true);
                            Ok(())
                        })();
                        if let Err(error) = result
                            && let Some(scope) = weak.upgrade()
                        {
                            scope.report_error(error);
                        }
                    }
                }
            });
        }
        for (kind, mut observers) in std::mem::take(&mut self.common.handlers) {
            input = input.on(kind, move |event| observe(&mut observers, owner, event));
        }
        let extra = self.common.class.clone();
        let prefix = format!(
            "sapid-color-input {}{}",
            self.common.theme.class(),
            if self.common.unstyled {
                " sapid-unstyled"
            } else {
                ""
            }
        );
        let theme = self.common.theme.clone();
        let editor_theme = self.common.theme.clone();
        let size = self.common.size;
        let alpha = self.alpha;
        let direction = self.direction;
        let unstyled = self.common.unstyled;
        let edited = control.clone();
        let on_change = change.clone();
        let on_commit = commit.clone();
        let ready = picker_enabled;
        let palette_ready = picker_enabled;
        let read_palette = palette;
        let raw = control.source.clone();
        let picker_value = scope.bond(move || {
            raw.with(|value| match parse_color(value, format) {
                Ok(color) => picker_coordinates(color, format, read_palette.get()),
                Err(_) => read_palette.get(),
            })
        });
        let picker_key = format!("{}-c", self.key);
        let with_picker = self.picker;
        let close_on_swatch = self.close_on_swatch;
        let swatches_per_row = self.swatches_per_row;
        let swatch_open = open.clone();
        let swatch_changed = opened.clone();
        let swatch_returning = returning_focus.clone();
        let preview_open = open.clone();
        let preview_changed = opened.clone();
        let open_scope = scope.downgrade();
        let focus_scope = scope.downgrade();
        let return_enabled = enabled.clone();
        let palette_scope = scope.downgrade();
        let popup = crate::popover(format!("{}-p", self.key))
            .label("Choose color")
            .name("Choose color")
            .show_close(false)
            .theme(theme.clone())
            .size(size)
            .style(
                Style::new()
                    .width(glory::style::Length::Px(226.))
                    .padding(glory::style::Edges::all(12.))
                    .gap(8.),
            )
            .enabled(ready)
            .external_trigger(input_id)
            .on_dismiss(move |reason| {
                if reason == glory::render::DismissReason::Escape
                    && return_enabled.try_get().is_ok_and(|value| value)
                    && let Some(scope) = focus_scope.upgrade()
                    && scope
                        .capabilities()
                        .is_ok_and(|capabilities| capabilities.focus)
                    && let Ok(Some(input)) = input_reference.try_get()
                {
                    returning_focus.set(true);
                    if let Err(error) = scope.action(glory::render::Action::Focus(input)) {
                        returning_focus.set(false);
                        scope.report_error(error);
                    }
                }
            })
            .open(open.source.clone())
            .on_open_change(move |value| {
                if let Err(error) = open.accept(&value)
                    && let Some(scope) = open_scope.upgrade()
                {
                    scope.report_error(error)
                }
                opened.emit(owner, value);
            })
            .content(move || {
                let source = picker_value;
                let editing = edited.clone();
                let changed = on_change.clone();
                let callback = on_commit.clone();
                let weak = palette_scope.clone();
                let mut picker = crate::color_picker(picker_key.clone())
                    .value(source)
                    .enabled(palette_ready)
                    .theme(theme.clone())
                    .size(size)
                    .direction(direction)
                    .show_alpha(alpha)
                    .show_precision_sliders(false)
                    .plane_size(200.)
                    .on_value_change(move |value| {
                        let text =
                            format_color(value.to_rgba().expect("validated picker value"), format);
                        match editing.accept(&text) {
                            Ok(()) => {
                                let coordinates = Hsva {
                                    alpha: if format.alpha() { value.alpha } else { 1. },
                                    ..value
                                };
                                palette.revise(|current| *current = coordinates);
                                changed.emit(owner, text);
                            }
                            Err(error) => {
                                if let Some(scope) = weak.upgrade() {
                                    scope.report_error(error)
                                }
                            }
                        }
                    })
                    .on_value_commit(move |value| {
                        let text =
                            format_color(value.to_rgba().expect("validated picker value"), format);
                        callback.emit(owner, text);
                    });
                if unstyled {
                    picker = picker.unstyled();
                }
                let mut content: Vec<Box<dyn Widget>> = Vec::with_capacity(2);
                if with_picker {
                    content.push(Box::new(picker));
                }
                let mut swatch_widgets: Vec<Box<dyn Widget>> = Vec::with_capacity(swatches.len());
                for (label, color) in swatches.clone() {
                    let editing = edited.clone();
                    let changed = on_change.clone();
                    let committed = on_commit.clone();
                    let open = swatch_open.clone();
                    let changed_open = swatch_changed.clone();
                    let returning = swatch_returning.clone();
                    let available = palette_ready;
                    let weak = palette_scope.clone();
                    swatch_widgets.push(Box::new(
                        crate::color_swatch(color)
                            .theme(theme.clone())
                            .style(crate::color_swatch::palette_cell_style())
                            .side(
                                (240. - 6. * (swatches_per_row.saturating_sub(1)) as f64)
                                    / swatches_per_row as f64,
                            )
                            .name(label)
                            .interactive(true)
                            .pressed(palette.map(move |value| {
                                value.to_rgba().is_ok_and(|value| value == color)
                            }))
                            .on_press(move || {
                                let result = (|| -> Result {
                                    if !owner.is_alive() || !available.try_get()? {
                                        return Ok(());
                                    }
                                    let text = format_color(color, format);
                                    editing.accept(&text)?;
                                    changed.emit(owner, text.clone());
                                    committed.emit(owner, text);
                                    if close_on_swatch && owner.is_alive() {
                                        open.accept(&false)?;
                                        changed_open.emit(owner, false);
                                        if owner.is_alive()
                                            && !open.source.try_get()?
                                            && available.try_get()?
                                            && let Some(scope) = weak.upgrade()
                                            && scope.capabilities()?.focus
                                            && let Some(input) = input_reference.try_get()?
                                        {
                                            returning.set(true);
                                            if let Err(error) =
                                                scope.action(glory::render::Action::Focus(input))
                                            {
                                                returning.set(false);
                                                return Err(error);
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                if let Err(error) = result
                                    && let Some(scope) = weak.upgrade()
                                {
                                    scope.report_error(error);
                                }
                            }),
                    ));
                }
                if !swatch_widgets.is_empty() {
                    content.push(Box::new(
                        crate::group()
                            .theme(theme.clone())
                            .style(crate::color_swatch::palette_style(swatches_per_row))
                            .gap(6.)
                            .wrap(glory::style::FlexWrap::Wrap)
                            .fill(swatch_widgets),
                    ));
                }
                crate::stack().theme(theme.clone()).gap(8.).fill(content)
            });
        let preview_color = control.source.clone();
        let mut editor = glory::ui::view()
            .class("sapid-color-input-editor")
            .style(Style::new().position(glory::style::Position::Relative));
        editor = editor.fill(input);
        let section_style = |left: bool| {
            let mut style = Style::new()
                .position(glory::style::Position::Absolute)
                .top(glory::style::Length::Px(8.));
            if left {
                style = style.left(glory::style::Length::Px(9.));
            } else {
                style = style.right(glory::style::Length::Px(9.));
            }
            style
        };
        if let Some(children) = self.left.take() {
            editor = editor.fill(
                glory::ui::view()
                    .class("sapid-color-input-left-section")
                    .style(section_style(true))
                    .fill(children),
            );
        } else if self.preview {
            let mut preview = crate::color_swatch(scope.bond(move || {
                parse_color(&preview_color.get(), format)
                    .unwrap_or(glory::style::Color(255, 255, 255, 255))
            }))
            .theme(editor_theme)
            .side(20.)
            .round(true)
            .class("sapid-color-input-preview")
            .style(
                Style::new()
                    .position(glory::style::Position::Absolute)
                    .left(glory::style::Length::Px(9.))
                    .top(glory::style::Length::Px(8.))
                    .pointer_events(if self.picker || !self.swatches.is_empty() {
                        glory::style::PointerEvents::Auto
                    } else {
                        glory::style::PointerEvents::None
                    }),
            );
            if self.picker || !self.swatches.is_empty() {
                let weak = scope.downgrade();
                let available = if self.loading_position == ColorInputSectionPosition::Left {
                    let loading = self.loading.clone();
                    scope.bond(move || picker_enabled.get() && !loading.get())
                } else {
                    picker_enabled
                };
                preview = preview
                    .interactive(true)
                    .name("Open color picker")
                    .enabled(available)
                    .on_press(move || {
                        let result = (|| -> Result {
                            if !owner.is_alive()
                                || !available.try_get()?
                                || preview_open.source.try_get()?
                            {
                                return Ok(());
                            }
                            preview_open.accept(&true)?;
                            preview_changed.emit(owner, true);
                            Ok(())
                        })();
                        if let Err(error) = result
                            && let Some(scope) = weak.upgrade()
                        {
                            scope.report_error(error);
                        }
                    });
            }
            if self.loading_position == ColorInputSectionPosition::Left {
                let loading = self.loading.clone();
                editor = editor.fill(
                    glory::ui::view()
                        .semantics(scope.bond(move || {
                            composite::metadata(Semantics {
                                hidden: loading.get(),
                                ..Default::default()
                            })
                        }))
                        .fill(preview),
                );
            } else {
                editor = editor.fill(preview);
            }
        }
        if let Some(children) = self.right.take() {
            editor = editor.fill(
                glory::ui::view()
                    .class("sapid-color-input-right-section")
                    .style(section_style(false))
                    .fill(children),
            );
        }
        if let Some(service) = self.eye_dropper.take() {
            let pending = scope.cage(false);
            let revision = scope.cage(());
            let task = Rc::new(Cell::new(None::<glory::core::TaskToken>));
            let cancel = task.clone();
            scope.on_cleanup(move || {
                if let Some(task) = cancel.take() {
                    task.cancel();
                }
            })?;
            let cancel = task.clone();
            let source = control.source.clone();
            let watch_enabled = enabled.clone();
            let watch_readonly = readonly.clone();
            scope.structural(move || {
                // Every source revision revokes the operation, including an
                // equal-value write or disabling and subsequently re-enabling.
                source.with(|_| ());
                watch_enabled.get();
                watch_readonly.get();
                composing.get();
                revision.revise(|_| ());
                let cancelled = cancel.take();
                pending.revise(|value| *value = false);
                if let Some(task) = cancelled {
                    task.cancel();
                }
            });
            let failure = Callback::new(self.eye_dropper_error.take());
            let cleanup = failure.clone();
            scope.on_cleanup(move || cleanup.clear())?;
            let editing = control.clone();
            let changed = change.clone();
            let committed = commit.clone();
            let weak = scope.downgrade();
            let permission_enabled = enabled.clone();
            let permission_readonly = readonly.clone();
            let enabled = picker_enabled;
            let mut button = crate::button()
                .theme(self.common.theme.clone())
                .class("sapid-color-input-eye-dropper")
                .variant(crate::ButtonVariant::Subtle)
                .compact(true)
                .label("")
                .name(self.eye_dropper_label.clone())
                .enabled(enabled)
                .loading(pending)
                .style(
                    section_style(false)
                        .width(glory::style::Length::Px(24.))
                        .height(glory::style::Length::Px(24.))
                        .min_height(glory::style::Length::Px(24.))
                        .padding(glory::style::Edges::all(0.)),
                )
                .on_press(move || {
                    if pending.get_untracked() || !enabled.get() {
                        return;
                    }
                    revision.revise(|_| ());
                    let version = revision.try_version().ok();
                    let value = editing.source.get();
                    let input_version = source_version(&editing.source);
                    let enabled_version = source_version(&permission_enabled);
                    let readonly_version = source_version(&permission_readonly);
                    pending.revise(|value| *value = true);
                    let future = service.pick();
                    let editing = editing.clone();
                    let changed = changed.clone();
                    let committed = committed.clone();
                    let failure = failure.clone();
                    let weak_task = weak.clone();
                    let task_slot = task.clone();
                    let permission_enabled = permission_enabled.clone();
                    let permission_readonly = permission_readonly.clone();
                    let spawned = weak
                        .upgrade()
                        .ok_or(glory::ui::Error::Gone)
                        .and_then(|scope| {
                            let token = scope.spawn(async move {
                                let result = future.await;
                                let Some(scope) = weak_task.upgrade() else {
                                    return;
                                };
                                // An arbitrary Future destructor may have
                                // cancelled this pick and started a newer one.
                                if revision.try_version().ok() != version {
                                    return;
                                }
                                if !enabled.try_get().unwrap_or(false)
                                    || source_version(&editing.source) != input_version
                                    || source_version(&permission_enabled) != enabled_version
                                    || source_version(&permission_readonly) != readonly_version
                                    || !editing
                                        .source
                                        .try_with(|current| current == &value)
                                        .unwrap_or(false)
                                {
                                    task_slot.take();
                                    let _ = pending.try_revise(|value| *value = false);
                                    return;
                                }
                                task_slot.take();
                                pending.revise(|value| *value = false);
                                match result {
                                    Ok(Some(color)) => {
                                        let text = format_color(color, format);
                                        match editing.accept(&text) {
                                            Ok(()) => {
                                                changed.emit(owner, text.clone());
                                                committed.emit(owner, text);
                                            }
                                            Err(error) => scope.report_error(error),
                                        }
                                    }
                                    Ok(None) => {}
                                    Err(error) => failure.emit(owner, error),
                                }
                            })?;
                            // The service factory may re-enter application state.
                            if revision.try_version().ok() == version
                                && enabled.try_get().unwrap_or(false)
                                && token.is_current()
                            {
                                task.set(Some(token));
                            } else {
                                token.cancel();
                                if revision.try_version().ok() == version {
                                    let _ = pending.try_revise(|value| *value = false);
                                }
                            }
                            Ok(())
                        });
                    if let Err(error) = spawned {
                        if revision.try_version().ok() == version {
                            let _ = pending.try_revise(|value| *value = false);
                        }
                        if let Some(scope) = weak.upgrade() {
                            scope.report_error(error)
                        }
                    }
                });
            if let Some(icon) = self.eye_dropper_icon.take() {
                button = button.fill(icon);
            } else {
                button = button.fill(glory::ui::text().content("◉"));
            }
            editor = editor.fill(button);
        }
        editor = editor.fill(
            crate::spinner()
                .theme(self.common.theme.clone())
                .diameter(18.)
                .thickness(2.)
                .visible(self.loading.clone())
                .class(match self.loading_position {
                    ColorInputSectionPosition::Left => "sapid-color-input-loading is-left",
                    ColorInputSectionPosition::Right => "sapid-color-input-loading is-right",
                })
                .style(section_style(
                    self.loading_position == ColorInputSectionPosition::Left,
                )),
        );
        let mut root = glory::ui::view()
            .class(scope.bond(move || format!("{prefix} {}", extra.get())))
            .style(composite::column(6.))
            .fill(editor);
        if self.picker || !self.swatches.is_empty() {
            root = root.fill(popup);
        }
        root.fill(
            glory::ui::view()
                .class("sapid-color-validation")
                .semantics(scope.bond(move || {
                    Some(Box::new(Semantics {
                        id: Some(error_id.clone()),
                        hidden: parse_message.with(|value| value.is_empty()),
                        ..Default::default()
                    }))
                }))
                .fill(glory::ui::text().content(parse_message)),
        )
        .show_in(scope)
    }
}
