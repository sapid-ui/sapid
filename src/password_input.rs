use glory::render::Purpose;
use glory::style::{Align, Color, Display, Edges, Justify, Length, Style};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};

use crate::state::Value;
use crate::text_input::text_input_methods;

fn eye(visible: bool, color: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as C};
    let mut commands = vec![
        C::Move(P::new(2., 12.)),
        C::Cubic {
            first: P::new(6., 4.),
            second: P::new(18., 4.),
            end: P::new(22., 12.),
        },
        C::Cubic {
            first: P::new(18., 20.),
            second: P::new(6., 20.),
            end: P::new(2., 12.),
        },
        C::Close,
        C::Move(P::new(15., 12.)),
        C::Cubic {
            first: P::new(15., 8.),
            second: P::new(9., 8.),
            end: P::new(9., 12.),
        },
        C::Cubic {
            first: P::new(9., 16.),
            second: P::new(15., 16.),
            end: P::new(15., 12.),
        },
        C::Close,
    ];
    if visible {
        commands.extend([C::Move(P::new(3., 3.)), C::Line(P::new(21., 21.))]);
    }
    glory::render::GraphicProps {
        commands,
        stroke: Some(glory::render::GraphicStroke {
            color,
            width: 1.5,
            cap: glory::render::StrokeCap::Round,
        }),
        ..Default::default()
    }
}

/// A single input identity with an explicitly named visibility action.
/// Revealing does not copy its value or write a new input revision.
pub struct PasswordInput {
    input: crate::TextField,
    visible: Value<bool>,
    on_visibility: Option<Box<dyn FnMut(bool)>>,
    show_label: String,
    hide_label: String,
}
pub fn password_input() -> PasswordInput {
    let mut input = crate::text_field();
    input.widget_class = "sapid-text-field sapid-password-input";
    PasswordInput {
        input,
        visible: Value::default(),
        on_visibility: None,
        show_label: "Show password".into(),
        hide_label: "Hide password".into(),
    }
}
impl PasswordInput {
    text_input_methods!();
    pub fn visible(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.visible.source = Some(value.into());
        self
    }
    pub fn default_visible(mut self, value: bool) -> Self {
        self.visible.default = value;
        self
    }
    pub fn on_visibility_change(mut self, handler: impl FnMut(bool) + 'static) -> Self {
        self.on_visibility = Some(Box::new(handler));
        self
    }
    /// Localized action names must both be nonempty.
    pub fn visibility_labels(mut self, show: impl Into<String>, hide: impl Into<String>) -> Self {
        self.show_label = show.into();
        self.hide_label = hide.into();
        self
    }
}
impl Widget for PasswordInput {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.show_label.trim().is_empty() || self.hide_label.trim().is_empty() {
            return Err(glory::render::Error::InvalidProps(
                "password visibility actions require names",
            )
            .into());
        }
        let control = std::mem::take(&mut self.visible).mount(scope);
        let source = control.source.clone();
        let purpose = scope.bond(move || {
            if source.get() {
                Purpose::Text
            } else {
                Purpose::Password
            }
        });
        // A caller-supplied right section replaces the default visibility action.
        if self.input.has_right_section() {
            return std::mem::take(&mut self.input)
                .purpose(purpose)
                .build(scope);
        }
        let source = control.source.clone();
        let show = self.show_label.clone();
        let hide = self.hide_label.clone();
        let label = scope.bond(move || {
            if source.get() {
                hide.clone()
            } else {
                show.clone()
            }
        });
        let common = &self.input.common;
        let enabled = self
            .input
            .field
            .as_ref()
            .map(|field| field.enabled(scope, common.enabled.clone()))
            .unwrap_or_else(|| common.enabled.clone());
        let theme = common.theme.clone();
        let size = common.size;
        let unstyled = common.unstyled;
        let loading = self.input.right_section_loading();
        let weak = scope.downgrade();
        let mut callback = self.on_visibility.take();
        let side = theme.tokens.control(size).min_height;
        let source = control.source.clone();
        let action_enabled = enabled.clone();
        let action_loading = loading.clone();
        let enabled_icon = enabled.clone();
        let colors = theme.clone();
        let drawing = scope.bond(move || {
            eye(
                source.get(),
                if enabled_icon.get() {
                    colors.foreground
                } else {
                    colors.disabled_text
                },
            )
        });
        let mut toggle = glory::ui::button()
            .class(format!(
                "sapid-password-toggle {}{}",
                theme.class(),
                if unstyled { " sapid-unstyled" } else { "" }
            ))
            .name(label)
            .fill(
                glory::ui::graphic().drawing(drawing).style(
                    Style::new()
                        .width(Length::Px(side * 0.5))
                        .height(Length::Px(side * 0.5)),
                ),
            )
            .enabled(enabled)
            .on(glory::render::EventKind::Activate, move |_| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !action_enabled.try_get()?
                        || action_loading
                            .as_ref()
                            .map(Vary::try_get)
                            .transpose()?
                            .unwrap_or(false)
                    {
                        return Ok(());
                    }
                    let proposal = !control.source.try_get()?;
                    control.accept(&proposal)?;
                    if let Some(callback) = &mut callback {
                        callback(proposal);
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            });
        if !unstyled {
            toggle = toggle.style(Style {
                align: Some(Align::Center),
                justify: Some(Justify::Center),
                background: Some(Color(0, 0, 0, 0)),
                ..Style::new()
                    .display(Display::InlineFlex)
                    .width(Length::Px(side - 6.))
                    .height(Length::Px(side - 6.))
                    .padding(Edges::all(0.))
                    .border_width(Edges::all(0.))
                    .radius(theme.radius)
            });
        }
        std::mem::take(&mut self.input)
            .purpose(purpose)
            .right_section_width(side.max(20.))
            .right_section(
                glory::ui::view()
                    .class("sapid-password-visibility")
                    .semantics(scope.bond(move || {
                        crate::composite::metadata(glory::render::Semantics {
                            hidden: loading.as_ref().is_some_and(|loading| loading.get()),
                            ..Default::default()
                        })
                    }))
                    .fill(toggle),
            )
            .build(scope)
    }
}

impl Default for PasswordInput {
    fn default() -> Self {
        password_input()
    }
}
