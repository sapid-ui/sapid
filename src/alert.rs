//! Inline messages use intrinsic live-region roles; severity is independent of
//! whether the application requests an announcement.
use crate::appearance::Common;
use crate::composite::{self, Callback};
use crate::state::Value;
use glory::render::{Role, Semantics, TextKind};
use glory::style::{Align, BorderStyle, Color, Display, Edges, FlexDirection, Length, Style};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Scope, Vary};

pub const MAX_ALERT_TEXT_BYTES: usize = 64 * 1024;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Announcement {
    Off,
    #[default]
    Polite,
    Assertive,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Light,
    Filled,
    Outline,
    Transparent,
}
pub struct Alert {
    key: String,
    common: Common,
    content: Vary<String>,
    title: Option<Vary<String>>,
    severity: AlertSeverity,
    variant: AlertVariant,
    color: Option<Color>,
    icon: Option<Children>,
    announcement: Announcement,
    visible: Value<bool>,
    closable: bool,
    close_label: Vary<String>,
    close: Option<Box<dyn FnMut(())>>,
    children: Option<Children>,
}
pub fn alert(key: impl Into<String>) -> Alert {
    Alert {
        key: key.into(),
        common: Common::default(),
        content: "".into(),
        title: None,
        severity: AlertSeverity::Info,
        variant: AlertVariant::Light,
        color: None,
        icon: None,
        announcement: Announcement::Polite,
        visible: Value {
            source: None,
            default: true,
        },
        closable: false,
        close_label: "Dismiss message".into(),
        close: None,
        children: None,
    }
}
impl Alert {
    pub fn variant(mut self, value: AlertVariant) -> Self {
        self.variant = value;
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
    pub fn icon(mut self, value: impl IntoChildren) -> Self {
        self.icon = Some(value.into_children());
        self
    }
    pub fn content(mut self, value: impl Into<Vary<String>>) -> Self {
        self.content = value.into();
        self
    }
    pub fn title(mut self, value: impl Into<Vary<String>>) -> Self {
        self.title = Some(value.into());
        self
    }
    pub fn severity(mut self, value: AlertSeverity) -> Self {
        self.severity = value;
        self
    }
    pub fn announcement(mut self, value: Announcement) -> Self {
        self.announcement = value;
        self
    }
    pub fn visible(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.visible.source = Some(value.into());
        self
    }
    pub fn closable(mut self, value: bool) -> Self {
        self.closable = value;
        self
    }
    pub fn close_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.close_label = value.into();
        self
    }
    pub fn on_close(mut self, mut handler: impl FnMut() + 'static) -> Self {
        self.close = Some(Box::new(move |()| handler()));
        self
    }
    pub fn fill(mut self, value: impl IntoChildren) -> Self {
        self.children = Some(value.into_children());
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
    pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(value.into());
        self
    }
    pub fn node_ref(mut self, value: glory::Cage<Option<glory::render::NodeId>>) -> Self {
        self.common.node_ref = Some(value);
        self
    }
    pub fn unstyled(mut self) -> Self {
        self.common.unstyled = true;
        self
    }
}
fn message(value: &str) -> Result {
    if value.trim().is_empty() || value.len() > MAX_ALERT_TEXT_BYTES {
        return Err(glory::render::Error::InvalidProps(
            "alert text must be nonempty and within 64 KiB",
        )
        .into());
    }
    Ok(())
}
impl Widget for Alert {
    fn build(&mut self, scope: &Scope) -> Result {
        let id = composite::id("alert", &self.key, "root", "root")?;
        let title_id = composite::id("alert", &self.key, "title", "root")?;
        let content_id = composite::id("alert", &self.key, "message", "root")?;
        let content = composite::validated(scope, &self.content, |value| message(value))?;
        let title = self
            .title
            .as_ref()
            .map(|value| composite::validated(scope, value, |value| message(value)))
            .transpose()?;
        let close_label = if self.closable {
            Some(composite::validated(scope, &self.close_label, |value| {
                message(value)
            })?)
        } else {
            None
        };
        let visible = std::mem::take(&mut self.visible).mount(scope);
        let source = visible.source.clone();
        let mut root = glory::ui::view();
        if let Some(role) = match self.announcement {
            Announcement::Off => None,
            Announcement::Polite => Some(Role::Status),
            Announcement::Assertive => Some(Role::Alert),
        } {
            root = root.role(role);
        }
        let labelled_by = if self.common.name.is_none() && title.is_some() {
            Some(title_id.clone())
        } else {
            None
        };
        let description_id = content_id.clone();
        root = root.semantics(scope.bond(move || {
            Some(Box::new(Semantics {
                id: Some(id.clone()),
                labelled_by: labelled_by.clone(),
                described_by: Some(description_id.clone()),
                hidden: !source.get(),
                ..Default::default()
            }))
        }));
        let theme = &self.common.theme;
        let (background, color) = match self.severity {
            AlertSeverity::Info => (theme.accent_soft, theme.accent_text),
            AlertSeverity::Success => (theme.success_soft, theme.success),
            AlertSeverity::Warning => (theme.warning_soft, theme.warning),
            AlertSeverity::Danger => (theme.danger_soft, theme.danger),
        };
        let color = self.color.unwrap_or(color);
        let background = if self.color.is_some() {
            Color(color.0, color.1, color.2, 26)
        } else {
            background
        };
        let filled = self.variant == AlertVariant::Filled;
        let foreground = if filled { theme.on_accent } else { color };
        let message_color = if filled {
            theme.on_accent
        } else {
            theme.foreground
        };
        let mut base = Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Row)
            .gap(16.0)
            .min_width(Length::Px(0.0))
            .padding(Edges::all(16.0))
            .radius(theme.radius)
            .border_width(Edges::all(if self.variant == AlertVariant::Outline {
                1.0
            } else {
                0.0
            }))
            .border_style(BorderStyle::Solid)
            .border_color(color);
        base.align = Some(Align::Start);
        base.background = Some(match self.variant {
            AlertVariant::Light => background,
            AlertVariant::Filled => color,
            AlertVariant::Outline | AlertVariant::Transparent => Color(0, 0, 0, 0),
        });
        base.color = Some(foreground);
        let (classes, style) = self.common.appearance(scope, "sapid-alert", base);
        root = root.class(classes).style(style);
        if let Some(name) = self.common.name.take() {
            root = root.name(name)
        }
        if let Some(reference) = self.common.node_ref {
            root = root.node_ref(reference)
        }
        let mut children = self.children.take();
        let mut icon = self.icon.take();
        let close = Callback::new(self.close.take());
        let weak = scope.downgrade();
        let owner = scope.id();
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        // All child captures belong to the caller's mounting Owner.
        root.fill(glory::ui::widget(move |scope| {
            if let Some(icon) = icon.take() {
                glory::ui::view()
                    .class("sapid-alert-icon")
                    .style(Style {
                        shrink: Some(0.0),
                        ..Style::new()
                            .width(Length::Px(20.0))
                            .height(Length::Px(20.0))
                    })
                    .fill(icon)
                    .show_in(scope)?;
            }
            let title_id = title_id.clone();
            let content_id = content_id.clone();
            let mut children = children.take();
            glory::ui::view()
                .class("sapid-alert-body")
                .style(Style {
                    grow: Some(1.0),
                    ..composite::column(8.0).min_width(Length::Px(0.0))
                })
                .fill(glory::ui::widget(move |scope| {
                    if let Some(title) = title {
                        glory::ui::text()
                            .class("sapid-alert-title")
                            .kind(TextKind::Heading(3))
                            .content(title)
                            .style(if unstyled {
                                Style::new()
                            } else {
                                Style {
                                    font_size: Some(14.0),
                                    line_height: Some(21.7),
                                    ..Style::new().font_weight(700.0).margin(Edges::all(0.0))
                                }
                            })
                            .semantics(Some(Box::new(Semantics {
                                id: Some(title_id.clone()),
                                ..Default::default()
                            })))
                            .show_in(scope)?;
                    }
                    glory::ui::text()
                        .class("sapid-alert-content")
                        .content(content)
                        .style(if unstyled {
                            Style::new()
                        } else {
                            Style {
                                color: Some(message_color),
                                font_size: Some(14.0),
                                line_height: Some(21.7),
                                ..Style::new()
                            }
                        })
                        .semantics(Some(Box::new(Semantics {
                            id: Some(content_id.clone()),
                            ..Default::default()
                        })))
                        .show_in(scope)?;
                    if let Some(children) = children.take() {
                        glory::ui::group().fill(children).show_in(scope)?;
                    }
                    Ok(())
                }))
                .show_in(scope)?;
            if let Some(label) = close_label {
                let weak = weak.clone();
                let visible = visible.clone();
                let close = close.clone();
                let button = crate::button()
                    .theme(theme.clone())
                    .size(size)
                    .unstyled()
                    .class("sapid-alert-close")
                    .name(label)
                    .label("×")
                    .style(Style {
                        color: (!unstyled).then_some(foreground),
                        shrink: Some(0.0),
                        font_size: Some(20.0),
                        line_height: Some(20.0),
                        border_width: Some(Edges::all(0.0)),
                        background: Some(Color(0, 0, 0, 0)),
                        ..Style::new()
                            .width(Length::Px(20.0))
                            .height(Length::Px(20.0))
                            .padding(Edges::all(0.0))
                            .radius(3.0)
                    })
                    .on_press(move || {
                        let Some(scope) = weak.upgrade() else { return };
                        match visible.accept(&false) {
                            Ok(()) => close.emit(owner, ()),
                            Err(error) => scope.report_error(error),
                        }
                    });
                button.show_in(scope)?;
            }
            Ok(())
        }))
        .show_in(scope)
    }
}
