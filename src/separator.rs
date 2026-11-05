use crate::appearance::{Common, configure};
use glory::render::{Orientation, Role, Semantics};
use glory::style::{Align, BorderStyle, Display, Edges, Length, Style};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};

/// A passive visual boundary; adjustable splitters are a separate interaction.
pub struct Separator {
    common: Common,
    axis: Orientation,
    decorative: bool,
    length: Option<Length>,
    thickness: f64,
    variant: SeparatorVariant,
    label: Option<Vary<String>>,
    label_position: SeparatorLabelPosition,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorVariant {
    #[default]
    Solid,
    Dashed,
    Dotted,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorLabelPosition {
    Start,
    #[default]
    Center,
    End,
}
pub fn separator() -> Separator {
    Separator {
        common: Common::default(),
        axis: Orientation::Horizontal,
        decorative: false,
        length: None,
        thickness: 1.0,
        variant: SeparatorVariant::Solid,
        label: None,
        label_position: SeparatorLabelPosition::Center,
    }
}
impl Separator {
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.common.theme = theme;
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
    pub fn class(mut self, class: impl Into<Vary<String>>) -> Self {
        self.common.class = class.into();
        self
    }
    pub fn style(mut self, style: impl Into<Vary<Style>>) -> Self {
        self.common.style = style.into();
        self
    }
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
    pub fn node_ref(mut self, reference: glory::Cage<Option<glory::render::NodeId>>) -> Self {
        self.common.node_ref = Some(reference);
        self
    }
    pub fn orientation(mut self, axis: Orientation) -> Self {
        self.axis = axis;
        self
    }
    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }
    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }
    pub fn thickness(mut self, thickness: f64) -> Self {
        self.thickness = thickness;
        self
    }
    pub fn variant(mut self, variant: SeparatorVariant) -> Self {
        self.variant = variant;
        self
    }
    /// A horizontal divider label, nonempty and at most 256 UTF-8 bytes.
    pub fn label(mut self, label: impl Into<Vary<String>>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn label_position(mut self, position: SeparatorLabelPosition) -> Self {
        self.label_position = position;
        self
    }
}
fn line_style(
    axis: Orientation,
    thickness: f64,
    variant: SeparatorVariant,
    color: Option<glory::style::Color>,
) -> Style {
    let mut style = Style::new();
    if let Some(color) = color {
        if variant == SeparatorVariant::Solid {
            style.background = Some(color);
        } else {
            style.border_style = Some(match variant {
                SeparatorVariant::Dashed => BorderStyle::Dashed,
                SeparatorVariant::Dotted => BorderStyle::Dotted,
                SeparatorVariant::Solid => unreachable!(),
            });
            style.border_color = Some(color);
            style.border_width = Some(match axis {
                Orientation::Horizontal => Edges {
                    top: thickness,
                    right: 0.,
                    bottom: 0.,
                    left: 0.,
                },
                Orientation::Vertical => Edges {
                    top: 0.,
                    right: 0.,
                    bottom: 0.,
                    left: thickness,
                },
            });
        }
    }
    style
}
impl Widget for Separator {
    fn build(&mut self, scope: &Scope) -> Result {
        if !self.thickness.is_finite() || self.thickness <= 0.0 {
            return Err(glory::render::Error::InvalidProps(
                "separator thickness must be finite and positive",
            )
            .into());
        }
        if self.decorative && self.common.name.is_some() {
            return Err(glory::render::Error::InvalidProps(
                "decorative separators cannot have names",
            )
            .into());
        }
        if self.label.is_some() && self.axis == Orientation::Vertical {
            return Err(glory::render::Error::InvalidProps(
                "separator labels require horizontal orientation",
            )
            .into());
        }
        let length = self.length.unwrap_or(match self.axis {
            Orientation::Horizontal => Length::Percent(100.0),
            Orientation::Vertical => Length::Px(24.0),
        });
        let (width, height) = match self.axis {
            Orientation::Horizontal => (length, Length::Px(self.thickness)),
            Orientation::Vertical => (Length::Px(self.thickness), length),
        };
        let mut base = Style {
            shrink: Some(0.0),
            ..Style::new().width(width).height(height)
        };
        let paint = line_style(
            self.axis,
            self.thickness,
            self.variant,
            Some(self.common.theme.border),
        );
        base = crate::appearance::merge(base, paint);
        base.validate()
            .map_err(glory::render::Error::InvalidProps)?;
        if let Some(label) = self.label.take() {
            let label = crate::composite::validated(scope, &label, |value| {
                if value.trim().is_empty()
                    || value.len() > 256
                    || value.contains(['\0', '\r', '\n', '\u{2028}', '\u{2029}'])
                {
                    Err(glory::render::Error::InvalidProps(
                        "separator label must be nonempty single-line text within 256 UTF-8 bytes",
                    )
                    .into())
                } else {
                    Ok(())
                }
            })?;
            let explicit = self.common.style.clone();
            let reactive = explicit.is_reactive();
            let geometry = move || {
                crate::appearance::merge(
                    Style {
                        align: Some(Align::Center),
                        shrink: Some(0.),
                        ..Style::new()
                            .display(Display::Flex)
                            .width(length)
                            .min_width(Length::Px(0.))
                            .gap(12.)
                    },
                    explicit.get(),
                )
            };
            self.common.style = if reactive {
                scope.bond(geometry).into()
            } else {
                geometry().into()
            };
            let color = (!self.common.unstyled).then_some(self.common.theme.border);
            let thickness = self.thickness;
            let variant = self.variant;
            let line = move || {
                glory::ui::view()
                    .class("sapid-separator-line")
                    .style(crate::appearance::merge(
                        Style {
                            grow: Some(1.),
                            shrink: Some(1.),
                            ..Style::new()
                                .width(Length::Px(0.))
                                .min_width(Length::Px(12.))
                                .height(Length::Px(thickness))
                        },
                        line_style(Orientation::Horizontal, thickness, variant, color),
                    ))
            };
            let mut root = configure!(
                self.common,
                glory::ui::view(),
                scope,
                "sapid-separator sapid-separator-horizontal sapid-separator-labelled",
                self.common.theme.description(self.common.size)
            );
            // A passive separator is an empty intrinsic node. Labeled rules
            // are ordinary text and decorative line parts in a legal box.
            if self.label_position != SeparatorLabelPosition::Start {
                root = root.fill(line());
            }
            root = root.fill(
                glory::ui::text()
                    .class("sapid-separator-label")
                    .content(label)
                    .style(Style {
                        shrink: Some(1.),
                        ..Style::new()
                            .min_width(Length::Px(0.))
                            .overflow_x(glory::style::Overflow::Hidden)
                    }),
            );
            if self.label_position != SeparatorLabelPosition::End {
                root = root.fill(line());
            }
            return root.show_in(scope);
        }
        let class = match self.axis {
            Orientation::Horizontal => "sapid-separator sapid-separator-horizontal",
            Orientation::Vertical => "sapid-separator sapid-separator-vertical",
        };
        let mut root = configure!(self.common, glory::ui::view(), scope, class, base);
        if !self.decorative {
            root = root
                .role(Role::Separator)
                .semantics(Some(Box::new(Semantics {
                    separator_axis: Some(self.axis),
                    ..Default::default()
                })));
        }
        root.show_in(scope)
    }
}

impl Default for Separator {
    fn default() -> Self {
        separator()
    }
}
