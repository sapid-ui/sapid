//! HSV hue and alpha domains over the shared finite Slider controller.
use crate::appearance::Common;
use crate::color_model::Hsva;
use crate::{composite, state::Value};
use glory::render::{Direction, Event, EventKind, GraphicPoint, GraphicProps, PathCommand};
use glory::style::{
    Align, Color, Display, Length, LinearGradient, Overflow, PointerEvents, Position, Style,
};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};

pub const MAX_COLOR_SLIDER_KEY_BYTES: usize = 58;
#[derive(Clone, Copy)]
enum Channel {
    Hue,
    Alpha,
}
struct ColorSlider {
    key: String,
    common: Common,
    channel: Channel,
    value: Value<f64>,
    step: Vary<f64>,
    page: Option<Vary<f64>>,
    color: Vary<Color>,
    direction: Direction,
    field: Option<crate::FieldControl>,
    required: Vary<bool>,
    readonly: Vary<bool>,
    invalid: Vary<bool>,
    text: Option<Vary<Option<String>>>,
    change: Option<Box<dyn FnMut(f64)>>,
    commit: Option<Box<dyn FnMut(f64)>>,
}
pub struct HueSlider(ColorSlider);
pub struct AlphaSlider(ColorSlider);
fn channel(key: String, channel: Channel) -> ColorSlider {
    let alpha = matches!(channel, Channel::Alpha);
    ColorSlider {
        key,
        common: Common::default(),
        channel,
        value: Value {
            source: None,
            default: if alpha { 1. } else { 0. },
        },
        step: if alpha { 0.01.into() } else { 1.0.into() },
        page: None,
        color: Color(255, 0, 0, 255).into(),
        direction: Direction::Ltr,
        field: None,
        required: false.into(),
        readonly: false.into(),
        invalid: false.into(),
        text: None,
        change: None,
        commit: None,
    }
}
pub fn hue_slider(key: impl Into<String>) -> HueSlider {
    HueSlider(channel(key.into(), Channel::Hue))
}
pub fn alpha_slider(key: impl Into<String>) -> AlphaSlider {
    AlphaSlider(channel(key.into(), Channel::Alpha))
}
macro_rules! methods {
    () => {
        pub fn value(mut self, value: impl Into<Vary<f64>>) -> Self {
            self.0.value.source = Some(value.into());
            self
        }
        pub fn default_value(mut self, value: f64) -> Self {
            self.0.value.default = value;
            self
        }
        pub fn step(mut self, value: impl Into<Vary<f64>>) -> Self {
            self.0.step = value.into();
            self
        }
        pub fn page_step(mut self, value: impl Into<Vary<f64>>) -> Self {
            self.0.page = Some(value.into());
            self
        }
        pub fn value_text(mut self, value: impl Into<Vary<Option<String>>>) -> Self {
            self.0.text = Some(value.into());
            self
        }
        pub fn field(mut self, value: crate::FieldControl) -> Self {
            self.0.common.theme = value.theme.clone();
            self.0.common.size = value.size;
            self.0.common.unstyled = value.unstyled;
            self.0.field = Some(value);
            self
        }
        pub fn theme(mut self, value: crate::Theme) -> Self {
            self.0.common.theme = value;
            self
        }
        pub fn size(mut self, value: crate::Size) -> Self {
            self.0.common.size = value;
            self
        }
        pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
            self.0.common.class = value.into();
            self
        }
        pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
            self.0.common.style = value.into();
            self
        }
        pub fn unstyled(mut self) -> Self {
            self.0.common.unstyled = true;
            self
        }
        pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
            self.0.common.name = Some(value.into());
            self
        }
        pub fn enabled(mut self, value: impl Into<Vary<bool>>) -> Self {
            self.0.common.enabled = value.into();
            self
        }
        pub fn required(mut self, value: impl Into<Vary<bool>>) -> Self {
            self.0.required = value.into();
            self
        }
        pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
            self.0.readonly = value.into();
            self
        }
        pub fn invalid(mut self, value: impl Into<Vary<bool>>) -> Self {
            self.0.invalid = value.into();
            self
        }
        pub fn direction(mut self, value: Direction) -> Self {
            self.0.direction = value;
            self
        }
        pub fn node_ref(mut self, value: glory::Cage<Option<glory::render::NodeId>>) -> Self {
            self.0.common.node_ref = Some(value);
            self
        }
        pub fn on_value_change(mut self, value: impl FnMut(f64) + 'static) -> Self {
            self.0.change = Some(Box::new(value));
            self
        }
        pub fn on_value_commit(mut self, value: impl FnMut(f64) + 'static) -> Self {
            self.0.commit = Some(Box::new(value));
            self
        }
        pub fn on(mut self, kind: EventKind, value: impl FnMut(Event) + 'static) -> Self {
            self.0
                .common
                .handlers
                .entry(kind)
                .or_default()
                .push(Box::new(value));
            self
        }
    };
}
impl HueSlider {
    methods!();
}
impl AlphaSlider {
    methods!();
    /// Base RGB; its alpha is ignored because the slider owns that channel.
    pub fn color(mut self, value: impl Into<Vary<Color>>) -> Self {
        self.0.color = value.into();
        self
    }
}
fn invalid(message: &'static str) -> glory::ui::Error {
    glory::render::Error::InvalidProps(message).into()
}
fn preview(channel: Channel, value: f64, Color(r, g, b, _): Color) -> Color {
    match channel {
        Channel::Hue => Hsva {
            hue: value,
            ..Hsva::default()
        }
        .to_rgba()
        .expect("validated hue"),
        Channel::Alpha => Color(r, g, b, (value * 255.).round() as u8),
    }
}
struct Trail {
    checker: [Color; 2],
    channel: Channel,
    color: Vary<Color>,
    direction: Direction,
}
fn transparency_checker(color: Color) -> GraphicProps {
    // One passive, bounded path. The centered drawing keeps its square cells;
    // a narrow track clips the outer parts instead of stretching them vertically.
    let mut commands = Vec::with_capacity(240);
    for y in 0..2 {
        for x in 0..48 {
            if (x + y) % 2 == 0 {
                let x = x as f64 * 4.;
                let y = y as f64 * 4.;
                commands.extend([
                    PathCommand::Move(GraphicPoint::new(x, y)),
                    PathCommand::Line(GraphicPoint::new(x + 4., y)),
                    PathCommand::Line(GraphicPoint::new(x + 4., y + 4.)),
                    PathCommand::Line(GraphicPoint::new(x, y + 4.)),
                    PathCommand::Close,
                ]);
            }
        }
    }
    GraphicProps {
        width: 192.,
        height: 8.,
        commands,
        fill: Some(color),
        ..Default::default()
    }
}
impl Widget for Trail {
    fn build(&mut self, scope: &Scope) -> Result {
        let reference = scope.cage(None);
        glory::ui::view()
            .class("sapid-color-slider-trail")
            .node_ref(reference)
            .style(
                Style::new()
                    .position(Position::Relative)
                    .width(Length::Percent(100.))
                    .height(Length::Percent(100.))
                    .overflow_x(Overflow::Hidden)
                    .overflow_y(Overflow::Hidden)
                    .pointer_events(PointerEvents::None),
            )
            .show_in(scope)?;
        let parent = scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
        let alpha = matches!(self.channel, Channel::Alpha);
        if alpha {
            glory::ui::view()
                .class("sapid-alpha-slider-background")
                .style(Style {
                    align: Some(Align::Center),
                    background: Some(self.checker[1]),
                    ..Style::new()
                        .display(Display::Flex)
                        .overflow_x(Overflow::Hidden)
                        .overflow_y(Overflow::Hidden)
                        .position(Position::Absolute)
                        .left(Length::Px(0.))
                        .top(Length::Px(0.))
                        .width(Length::Percent(100.))
                        .height(Length::Percent(100.))
                        .pointer_events(PointerEvents::None)
                })
                .fill(
                    glory::ui::graphic()
                        .class("sapid-alpha-slider-checker")
                        .drawing(transparency_checker(self.checker[0]))
                        .style(Style {
                            shrink: Some(0.),
                            ..Style::new()
                                .width(Length::Percent(100.))
                                .height(Length::Auto)
                                .pointer_events(PointerEvents::None)
                        }),
                )
                .show_in(&parent)?;
        }
        let color = self.color.clone();
        let channel = self.channel;
        let direction = self.direction;
        let style = scope.bond(move || {
            let stops = match channel {
                Channel::Hue => [0., 60., 120., 180., 240., 300., 360.]
                    .into_iter()
                    .map(|hue| {
                        Hsva {
                            hue,
                            ..Hsva::default()
                        }
                        .to_rgba()
                        .expect("finite hue stop")
                    })
                    .collect(),
                Channel::Alpha => {
                    let Color(r, g, b, _) = color.get();
                    vec![Color(0, 0, 0, 0), Color(r, g, b, 255)]
                }
            };
            Style::new()
                .background_image(LinearGradient {
                    angle: if direction == Direction::Rtl {
                        270.
                    } else {
                        90.
                    },
                    stops,
                })
                .position(Position::Absolute)
                .left(Length::Px(0.))
                .top(Length::Px(0.))
                .width(Length::Percent(100.))
                .height(Length::Percent(100.))
                .pointer_events(PointerEvents::None)
        });
        glory::ui::view()
            .class("sapid-color-slider-overlay")
            .style(style)
            .show_in(&parent)?;
        scope.retain(parent)?;
        Ok(())
    }
}
impl Widget for ColorSlider {
    fn build(&mut self, scope: &Scope) -> Result {
        composite::id("color-slider", &self.key, "control", "root")?;
        if self.key.len() > MAX_COLOR_SLIDER_KEY_BYTES {
            return Err(invalid("color slider keys require at most 58 ASCII bytes"));
        }
        let channel = self.channel;
        let hue = matches!(channel, Channel::Hue);
        let max = if hue { 360. } else { 1. };
        let control = std::mem::take(&mut self.value).mount(scope);
        let accepted = composite::validated(scope, &control.source, move |value| {
            if value.is_finite() && (0. ..=max).contains(value) {
                Ok(())
            } else {
                Err(invalid(
                    "color slider value lies outside its finite channel",
                ))
            }
        })?;
        let color = self.color.clone();
        let thumb = scope.bond(move || preview(channel, accepted.get(), color.get()));
        let text = self.text.take().unwrap_or_else(|| {
            scope
                .bond(move || {
                    Some(if hue {
                        format!("{:.2} degrees", accepted.get())
                    } else {
                        format!("{:.2}% opacity", accepted.get() * 100.)
                    })
                })
                .into()
        });
        let extra = self.common.class.clone();
        let prefix = if hue {
            "sapid-hue-slider"
        } else {
            "sapid-alpha-slider"
        };
        let mut slider = crate::slider(format!(
            "{}-{}",
            if hue { "hue" } else { "alpha" },
            self.key
        ))
        .theme(self.common.theme.clone())
        .size(self.common.size)
        .class(scope.bond(move || format!("{prefix} {}", extra.get())))
        .style(self.common.style.clone())
        .min(0.)
        .max(max)
        .step(self.step.clone())
        .value(control.source.clone())
        .value_text(text)
        .direction(self.direction)
        .enabled(self.common.enabled.clone())
        .required(self.required.clone())
        .readonly(self.readonly.clone())
        .invalid(self.invalid.clone())
        .track_content(Trail {
            checker: [
                self.common.theme.checker_dark,
                self.common.theme.checker_light,
            ],
            channel,
            color: self.color.clone(),
            direction: self.direction,
        })
        .thumb_color(thumb);
        let has_field = self.field.is_some();
        if let Some(field) = self.field.take() {
            slider = slider.field(field)
        }
        if let Some(name) = self.common.name.take() {
            slider = slider.name(name)
        } else if !has_field {
            // Slider preserves a Field's label relation, and uses this fallback only
            // when the widget has no Field metadata.
            slider = slider.name(if hue { "Hue" } else { "Opacity" });
        }
        if let Some(page) = self.page.take() {
            slider = slider.page_step(page)
        }
        if let Some(reference) = self.common.node_ref {
            slider = slider.node_ref(reference)
        }
        if self.common.unstyled {
            slider = slider.unstyled()
        }
        let mut changed = self.change.take();
        let weak = scope.downgrade();
        slider = slider.on_value_change(move |value| {
            if let Err(error) = control.accept(&value) {
                if let Some(scope) = weak.upgrade() {
                    scope.report_error(error)
                };
                return;
            }
            if let Some(callback) = changed.as_mut() {
                callback(value)
            }
        });
        if let Some(commit) = self.commit.take() {
            slider = slider.on_value_commit(commit)
        }
        for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
            for handler in handlers {
                slider = slider.on(kind, handler)
            }
        }
        slider.show_in(scope)
    }
}
impl Widget for HueSlider {
    fn build(&mut self, scope: &Scope) -> Result {
        self.0.build(scope)
    }
}
impl Widget for AlphaSlider {
    fn build(&mut self, scope: &Scope) -> Result {
        self.0.build(scope)
    }
}
