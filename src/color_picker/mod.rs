//! Finite HSV picker with a pointer plane and genuine keyboard Slider alternatives.
use crate::{
    appearance::Common,
    color_model::{ColorFormat, Hsva, format_color, parse_hex_color},
    composite::{self, Callback, Observers},
    state::{Control, Value},
};
use glory::render::{
    Direction, DragCapture, Event, EventKind, FormControl, NodeId, Role, Semantics,
};
use glory::style::{Color, Display, Length, LinearGradient, PointerEvents, Position, Style};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
use std::rc::Rc;
mod pad;
pub const MAX_COLOR_PICKER_KEY_BYTES: usize = 48;
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn validate_name(value: &str) -> Result {
    if value.trim().is_empty() || value.len() > 256 || value.contains(['\0', '\r', '\n']) {
        Err(invalid(
            "color names require 1..=256 canonical single-line bytes",
        ))
    } else {
        Ok(())
    }
}
pub(super) struct PickerState {
    owner: glory::core::InstanceId,
    control: Control<Hsva>,
    accepted: Cage<Hsva>,
    enabled: Vary<bool>,
    readonly: Vary<bool>,
    direction: Direction,
    change: Callback<Hsva>,
    commit: Callback<Hsva>,
    drag_events: Observers,
    range_events: Observers,
}
impl PickerState {
    fn current(&self) -> Hsva {
        let value = self.control.source.get();
        if value.validate().is_ok() {
            value
        } else {
            self.accepted.get()
        }
    }
    fn ready(&self) -> bool {
        self.owner.is_alive() && self.enabled.get() && !self.readonly.get()
    }
    fn propose(&self, value: Hsva, commit: bool) -> Result {
        if !self.ready() {
            return Ok(());
        }
        value
            .validate()
            .map_err(|_| invalid("invalid color proposal"))?;
        if self.current() != value {
            self.control.accept(&value)?;
            self.change.emit(self.owner, value);
        }
        if commit && self.ready() {
            self.commit.emit(self.owner, value);
        }
        Ok(())
    }
    fn channel(&self, index: u8, value: f64, commit: bool) -> Result {
        let mut next = self.current();
        match index {
            0 => next.hue = value,
            1 => next.saturation = value,
            2 => next.value = value,
            _ => next.alpha = value,
        };
        if commit {
            if self.ready() {
                self.commit.emit(self.owner, next);
            }
            Ok(())
        } else {
            self.propose(next, false)
        }
    }
}
pub struct ColorPicker {
    key: String,
    common: Common,
    value: Value<Hsva>,
    readonly: Vary<bool>,
    direction: Direction,
    alpha: bool,
    precision_sliders: bool,
    picker: bool,
    full_width: bool,
    swatches: Vec<String>,
    swatches_per_row: usize,
    saturation_label: Vary<String>,
    brightness_label: Vary<String>,
    hue_label: Vary<String>,
    alpha_label: Vary<String>,
    form_name: Option<Vary<String>>,
    form_format: ColorFormat,
    side: f64,
    change: Option<Box<dyn FnMut(Hsva)>>,
    commit: Option<Box<dyn FnMut(Hsva)>>,
}
pub fn color_picker(key: impl Into<String>) -> ColorPicker {
    ColorPicker {
        key: key.into(),
        common: Common::default(),
        value: Value {
            source: None,
            default: Hsva::default(),
        },
        readonly: false.into(),
        direction: Direction::Ltr,
        alpha: true,
        precision_sliders: false,
        picker: true,
        full_width: false,
        swatches: Vec::new(),
        swatches_per_row: 7,
        saturation_label: "Saturation".into(),
        brightness_label: "Brightness".into(),
        hue_label: "Hue".into(),
        alpha_label: "Opacity".into(),
        form_name: None,
        form_format: ColorFormat::Hexa,
        side: 240.,
        change: None,
        commit: None,
    }
}
impl ColorPicker {
    pub fn value(mut self, value: impl Into<Vary<Hsva>>) -> Self {
        self.value.source = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: Hsva) -> Self {
        self.value.default = value;
        self
    }
    pub fn on_value_change(mut self, value: impl FnMut(Hsva) + 'static) -> Self {
        self.change = Some(Box::new(value));
        self
    }
    pub fn on_value_commit(mut self, value: impl FnMut(Hsva) + 'static) -> Self {
        self.commit = Some(Box::new(value));
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
    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = value;
        self
    }
    pub fn show_alpha(mut self, value: bool) -> Self {
        self.alpha = value;
        self
    }
    /// Show the keyboard precision controls for saturation and brightness.
    /// Compact picker popovers use the two-dimensional plane instead.
    pub fn show_precision_sliders(mut self, value: bool) -> Self {
        self.precision_sliders = value;
        self
    }
    /// Show the continuous picker. Disable it to expose only declared swatches.
    pub fn show_picker(mut self, value: bool) -> Self {
        self.picker = value;
        self
    }
    /// Stretch the picker root to its parent's width while retaining finite channel geometry.
    pub fn full_width(mut self, value: bool) -> Self {
        self.full_width = value;
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
    pub fn saturation_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.saturation_label = value.into();
        self
    }
    pub fn brightness_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.brightness_label = value.into();
        self
    }
    pub fn hue_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.hue_label = value.into();
        self
    }
    pub fn alpha_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.alpha_label = value.into();
        self
    }
    /// Publish the current color through a real hidden successful control.
    /// The picker keeps its original HSVA value; only the submitted snapshot is formatted.
    pub fn form_name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.form_name = Some(value.into());
        self
    }
    pub fn form_format(mut self, value: ColorFormat) -> Self {
        self.form_format = value;
        self
    }
    /// Intrinsic plane side; custom outer style does not turn the plane into a zero-height box.
    pub fn plane_size(mut self, value: f64) -> Self {
        self.side = value;
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
fn surface(color: Color) -> Style {
    Style {
        background: Some(color),
        ..Style::new()
    }
}
fn channel(state: Rc<PickerState>, scope: &Scope, index: u8) -> Vary<f64> {
    scope
        .bond(move || {
            let value = state.current();
            match index {
                0 => value.hue,
                1 => value.saturation,
                2 => value.value,
                _ => value.alpha,
            }
        })
        .into()
}
impl Widget for ColorPicker {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.key.len() > MAX_COLOR_PICKER_KEY_BYTES {
            return Err(invalid("color picker keys are limited to 48 bytes"));
        }
        if !self.side.is_finite() || !(128. ..=512.).contains(&self.side) {
            return Err(invalid("color plane side requires 128..=512 pixels"));
        }
        if self.swatches.len() > 64 || !(1..=16).contains(&self.swatches_per_row) {
            return Err(invalid(
                "color picker requires at most 64 swatches and 1..=16 swatches per row",
            ));
        }
        if !self.picker && self.swatches.is_empty() {
            return Err(invalid(
                "swatches-only color picker requires at least one swatch",
            ));
        }
        let swatches = self
            .swatches
            .iter()
            .map(|value| {
                parse_hex_color(value)
                    .map(|color| (value.clone(), color))
                    .map_err(|_| invalid("color picker swatches require hexadecimal colors"))
            })
            .collect::<Result<Vec<_>>>()?;
        let id = composite::id("color-picker", &self.key, "root", "root")?;
        let control = std::mem::replace(
            &mut self.value,
            Value {
                source: None,
                default: Hsva::default(),
            },
        )
        .mount(scope);
        let accepted = composite::validated(scope, &control.source, |value| {
            value.validate().map_err(|_| invalid("invalid HSV source"))
        })?;
        let name = self.common.name.take().unwrap_or_else(|| "Color".into());
        let name = composite::validated(scope, &name, |value| validate_name(value))?;
        let saturation_label =
            composite::validated(scope, &self.saturation_label, |value| validate_name(value))?;
        let brightness_label =
            composite::validated(scope, &self.brightness_label, |value| validate_name(value))?;
        let hue_label = composite::validated(scope, &self.hue_label, |value| validate_name(value))?;
        let alpha_label =
            composite::validated(scope, &self.alpha_label, |value| validate_name(value))?;
        let state = Rc::new(PickerState {
            owner: scope.id(),
            control,
            accepted,
            enabled: self.common.enabled.clone(),
            readonly: self.readonly.clone(),
            direction: self.direction,
            change: Callback::new(self.change.take()),
            commit: Callback::new(self.commit.take()),
            drag_events: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Drag)
                    .unwrap_or_default(),
            ),
            range_events: Observers::new(
                self.common
                    .handlers
                    .remove(&EventKind::Range)
                    .unwrap_or_default(),
            ),
        });
        let cleanup = state.clone();
        scope.on_cleanup(move || {
            cleanup.change.clear();
            cleanup.commit.clear();
            cleanup.drag_events.clear();
            cleanup.range_events.clear();
        })?;
        let (class, style) = self
            .common
            .appearance(scope, "sapid-color-picker", Style::new());
        let side = self.side;
        let full_width = self.full_width;
        let style = scope.bond(move || {
            let mut value = style.get();
            if value.width.is_none() {
                value.width = Some(if full_width {
                    Length::Percent(100.)
                } else {
                    Length::Px(side)
                });
            }
            crate::appearance::merge(composite::column(8.), value)
        });
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let mut root = glory::ui::view()
            .role(Role::Group)
            .name(name)
            .class(class)
            .style(style)
            .node_ref(reference)
            .semantics(composite::metadata(Semantics {
                id: Some(id),
                ..Default::default()
            }));
        for (kind, mut callbacks) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            root = root.on(kind, move |event| {
                crate::appearance::observe(&mut callbacks, owner, event)
            });
        }
        root.show_in(scope)?;
        let parent = scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
        if let Some(form_name) = self.form_name.take() {
            let form_name = composite::validated(scope, &form_name, |value| validate_name(value))?;
            let form_state = state.clone();
            let form_format = self.form_format;
            glory::ui::input()
                .class("sapid-color-picker-form-proxy")
                .value(scope.bond(move || {
                    format_color(
                        form_state
                            .current()
                            .to_rgba()
                            .expect("validated color picker form value"),
                        form_format,
                    )
                }))
                .enabled(self.common.enabled.clone())
                .style(Style::new().display(Display::None))
                .semantics(scope.bond(move || {
                    Some(Box::new(Semantics {
                        form_control: Some(FormControl {
                            name: form_name.get(),
                            value: None,
                        }),
                        ..Default::default()
                    }))
                }))
                .show_in(&parent)?;
        }
        if !self.picker {
            show_swatches(
                &parent,
                scope,
                state,
                swatches,
                self.swatches_per_row,
                side,
                self.common.theme.clone(),
                self.common.enabled.clone(),
                self.readonly.clone(),
            )?;
            scope.retain(parent)?;
            return Ok(());
        }
        let pad_ref = scope.cage(None);
        let driver = pad::Pad::new(scope, state.clone(), pad_ref)?;
        let enabled = self.common.enabled.clone();
        let readonly = self.readonly.clone();
        let weak = scope.downgrade();
        let drag = driver.clone();
        glory::ui::view()
            .class("sapid-color-plane")
            .node_ref(pad_ref)
            .semantics(scope.bond(move || {
                Some(Box::new(Semantics {
                    drag_capture: Some(DragCapture::Press),
                    disabled: !enabled.get() || readonly.get(),
                    ..Default::default()
                }))
            }))
            .style(
                Style::new()
                    .position(Position::Relative)
                    .width(Length::Percent(100.))
                    .height(Length::Px(side))
                    .min_height(Length::Px(side)),
            )
            .on(EventKind::Drag, move |event| {
                if let Err(error) = drag.event(event)
                    && let Some(scope) = weak.upgrade()
                {
                    scope.report_error(error)
                }
            })
            .show_in(&parent)?;
        let plane = scope.child_in(pad_ref.get().ok_or(glory::ui::Error::Gone)?)?;
        let hue = state.clone();
        glory::ui::view()
            .class("sapid-color-plane-hue")
            .style(scope.bond(move || {
                surface(
                    Hsva {
                        hue: hue.current().hue,
                        saturation: 1.,
                        value: 1.,
                        alpha: 1.,
                    }
                    .to_rgba()
                    .expect("validated color plane hue"),
                )
                .position(Position::Absolute)
                .left(Length::Percent(0.))
                .top(Length::Percent(0.))
                .width(Length::Percent(100.))
                .height(Length::Percent(100.))
                .pointer_events(PointerEvents::None)
            }))
            .show_in(&plane)?;
        let horizontal_stops = if self.direction == Direction::Rtl {
            vec![Color(255, 255, 255, 0), Color(255, 255, 255, 255)]
        } else {
            vec![Color(255, 255, 255, 255), Color(255, 255, 255, 0)]
        };
        for (class, gradient) in [
            (
                "sapid-color-plane-saturation",
                LinearGradient {
                    angle: 90.,
                    stops: horizontal_stops,
                },
            ),
            (
                "sapid-color-plane-value",
                LinearGradient {
                    angle: 180.,
                    stops: vec![Color(0, 0, 0, 0), Color(0, 0, 0, 255)],
                },
            ),
        ] {
            glory::ui::view()
                .class(class)
                .style(
                    Style::new()
                        .background_image(gradient)
                        .position(Position::Absolute)
                        .left(Length::Percent(0.))
                        .top(Length::Percent(0.))
                        .width(Length::Percent(100.))
                        .height(Length::Percent(100.))
                        .pointer_events(PointerEvents::None),
                )
                .show_in(&plane)?;
        }
        let read = state.clone();
        let marker_border = self.common.theme.control_thumb;
        glory::ui::view()
            .class("sapid-color-plane-indicator")
            .style(scope.bond(move || {
                let value = read.current();
                let x = if read.direction == Direction::Rtl {
                    1. - value.saturation
                } else {
                    value.saturation
                };
                surface(value.to_rgba().expect("validated color indicator"))
                    .position(Position::Absolute)
                    .left(Length::Percent(x * 100.))
                    .top(Length::Percent((1. - value.value) * 100.))
                    .width(Length::Px(10.))
                    .height(Length::Px(10.))
                    .margin(glory::style::Edges {
                        top: -5.,
                        left: -5.,
                        right: 0.,
                        bottom: 0.,
                    })
                    .radius(5.)
                    .border_width(glory::style::Edges::all(2.))
                    .border_style(glory::style::BorderStyle::Solid)
                    .border_color(marker_border)
                    .pointer_events(PointerEvents::None)
            }))
            .show_in(&plane)?;
        for index in if self.precision_sliders {
            &[1, 2][..]
        } else {
            &[][..]
        } {
            let index = *index;
            let value = channel(state.clone(), scope, index);
            let changed = state.clone();
            let committed = state.clone();
            let weak = scope.downgrade();
            let weak_commit = scope.downgrade();
            let observe_range = state.clone();
            let observe_drag = state.clone();
            let label = if index == 1 {
                saturation_label
            } else {
                brightness_label
            };
            let mut slider = crate::slider(format!("{}-{index}", self.key))
                .value(value)
                .min(0.)
                .max(1.)
                .step(0.01)
                .name(label)
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .style(Style::new().width(Length::Percent(100.)))
                .enabled(self.common.enabled.clone())
                .readonly(self.readonly.clone())
                .direction(self.direction)
                .on(EventKind::Range, move |event| {
                    observe_range.range_events.emit(observe_range.owner, event)
                })
                .on(EventKind::Drag, move |event| {
                    observe_drag.drag_events.emit(observe_drag.owner, event)
                })
                .on_value_change(move |value| {
                    if let Err(error) = changed.channel(index, value, false)
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error)
                    }
                })
                .on_value_commit(move |value| {
                    if let Err(error) = committed.channel(index, value, true)
                        && let Some(scope) = weak_commit.upgrade()
                    {
                        scope.report_error(error)
                    }
                });
            if self.common.unstyled {
                slider = slider.unstyled();
            }
            slider.show_in(&parent)?;
        }
        let changed = state.clone();
        let committed = state.clone();
        let weak = scope.downgrade();
        let weak_commit = scope.downgrade();
        let observe_range = state.clone();
        let observe_drag = state.clone();
        let mut hue = crate::hue_slider(format!("{}-h", self.key))
            .value(channel(state.clone(), scope, 0))
            .name(hue_label)
            .theme(self.common.theme.clone())
            .size(self.common.size)
            .style(Style::new().width(Length::Percent(100.)))
            .enabled(self.common.enabled.clone())
            .readonly(self.readonly.clone())
            .direction(self.direction)
            .on(EventKind::Range, move |event| {
                observe_range.range_events.emit(observe_range.owner, event)
            })
            .on(EventKind::Drag, move |event| {
                observe_drag.drag_events.emit(observe_drag.owner, event)
            })
            .on_value_change(move |value| {
                if let Err(error) = changed.channel(0, value, false)
                    && let Some(scope) = weak.upgrade()
                {
                    scope.report_error(error)
                }
            })
            .on_value_commit(move |value| {
                if let Err(error) = committed.channel(0, value, true)
                    && let Some(scope) = weak_commit.upgrade()
                {
                    scope.report_error(error)
                }
            });
        if self.common.unstyled {
            hue = hue.unstyled();
        }
        hue.show_in(&parent)?;
        if self.alpha {
            let preview = state.clone();
            let color = scope.bond(move || {
                preview
                    .current()
                    .to_rgba()
                    .expect("validated color preview")
            });
            let changed = state.clone();
            let committed = state.clone();
            let weak = scope.downgrade();
            let weak_commit = scope.downgrade();
            let observe_range = state.clone();
            let observe_drag = state.clone();
            let mut alpha = crate::alpha_slider(format!("{}-a", self.key))
                .value(channel(state.clone(), scope, 3))
                .name(alpha_label)
                .color(color)
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .style(Style::new().width(Length::Percent(100.)))
                .enabled(self.common.enabled.clone())
                .readonly(self.readonly.clone())
                .direction(self.direction)
                .on(EventKind::Range, move |event| {
                    observe_range.range_events.emit(observe_range.owner, event)
                })
                .on(EventKind::Drag, move |event| {
                    observe_drag.drag_events.emit(observe_drag.owner, event)
                })
                .on_value_change(move |value| {
                    if let Err(error) = changed.channel(3, value, false)
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error)
                    }
                })
                .on_value_commit(move |value| {
                    if let Err(error) = committed.channel(3, value, true)
                        && let Some(scope) = weak_commit.upgrade()
                    {
                        scope.report_error(error)
                    }
                });
            if self.common.unstyled {
                alpha = alpha.unstyled();
            }
            alpha.show_in(&parent)?;
        }
        if !swatches.is_empty() {
            show_swatches(
                &parent,
                scope,
                state,
                swatches,
                self.swatches_per_row,
                side,
                self.common.theme.clone(),
                self.common.enabled.clone(),
                self.readonly.clone(),
            )?;
        }
        scope.retain(plane)?;
        scope.retain(parent)?;
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn show_swatches(
    parent: &Scope,
    scope: &Scope,
    state: Rc<PickerState>,
    swatches: Vec<(String, Color)>,
    per_row: usize,
    width: f64,
    theme: crate::Theme,
    enabled: Vary<bool>,
    readonly: Vary<bool>,
) -> Result {
    let mut children: Vec<Box<dyn Widget>> = Vec::with_capacity(swatches.len());
    let side = ((width - 6. * (per_row.saturating_sub(1)) as f64) / per_row as f64).clamp(8., 128.);
    for (label, color) in swatches {
        let next = Hsva::from_rgba(color);
        let select = state.clone();
        let weak = scope.downgrade();
        let ready = {
            let enabled = enabled.clone();
            let readonly = readonly.clone();
            scope.bond(move || enabled.get() && !readonly.get())
        };
        children.push(Box::new(
            crate::color_swatch(color)
                .theme(theme.clone())
                .class("sapid-color-picker-swatch")
                .side(side)
                .name(label)
                .enabled(ready)
                .interactive(true)
                .on_press(move || {
                    if let Err(error) = select.propose(next, true)
                        && let Some(scope) = weak.upgrade()
                    {
                        scope.report_error(error)
                    }
                }),
        ));
    }
    crate::group()
        .theme(theme)
        .class("sapid-color-picker-swatches")
        .gap(6.)
        .wrap(glory::style::FlexWrap::Wrap)
        .fill(children)
        .show_in(parent)
}
