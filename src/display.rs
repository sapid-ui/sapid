//! Passive display widgets and a separately named removable Pill action.
use glory::render::{EventKind, NodeId, Semantics, TextKind};
use glory::style::{
    Align, BorderStyle, BoxShadow, Display, Edges, FlexDirection, Length, Shadow, Style,
};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};

use crate::appearance::{Common, configure, observe};
use crate::state::Value;

macro_rules! display_methods {
    () => {
        pub fn theme(mut self, theme: crate::Theme) -> Self {
            self.common.theme = theme;
            self
        }
        pub fn size(mut self, size: crate::Size) -> Self {
            self.common.size = size;
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
        pub fn node_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
            self.common.node_ref = Some(reference);
            self
        }
    };
}

fn column(gap: f64) -> Style {
    Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .gap(gap)
        .min_width(Length::Px(0.0))
}
fn take(children: &mut Children) -> Children {
    std::mem::replace(children, ().into_children())
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShadowSize {
    #[default]
    None,
    Small,
    Medium,
    Large,
}
impl ShadowSize {
    fn value(self, theme: &crate::Theme) -> BoxShadow {
        let (y, blur, spread, color) = match self {
            Self::None => return BoxShadow::None,
            Self::Small => (1., 3., 0., theme.shadow_small),
            Self::Medium => (4., 12., -2., theme.shadow_medium),
            Self::Large => (8., 24., -4., theme.shadow_large),
        };
        BoxShadow::Outset(Shadow {
            x: 0.0,
            y,
            blur,
            spread,
            color,
        })
    }
}
fn surface(common: &Common, bordered: bool, shadow: Option<ShadowSize>) -> Style {
    let mut style = Style {
        background: Some(common.theme.surface),
        color: Some(common.theme.foreground),
        ..column(12.0)
            .padding(Edges::all(16.0))
            .radius(common.theme.radius)
    };
    if bordered {
        style = style
            .border_width(Edges::all(1.0))
            .border_style(BorderStyle::Solid)
            .border_color(common.theme.border);
    }
    style.box_shadow = shadow.map(|shadow| shadow.value(&common.theme));
    style
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Soft,
    Solid,
    Outline,
    Dot,
}
/// Finite passive side content, without nested interactive controls.
#[derive(Clone, Debug)]
pub enum BadgeSection {
    Icon(crate::Icon),
    Text(String),
}
impl From<crate::Icon> for BadgeSection {
    fn from(value: crate::Icon) -> Self {
        Self::Icon(value)
    }
}
impl From<String> for BadgeSection {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<&str> for BadgeSection {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// A passive text label. It has no implicit status/live-region behavior.
pub struct Badge {
    common: Common,
    content: Vary<String>,
    variant: BadgeVariant,
    color: Option<glory::style::Color>,
    left: Option<BadgeSection>,
    right: Option<BadgeSection>,
}
pub fn badge() -> Badge {
    Badge {
        common: Common::default(),
        content: "".into(),
        variant: BadgeVariant::Soft,
        color: None,
        left: None,
        right: None,
    }
}
impl Badge {
    display_methods!();
    pub fn color(mut self, color: glory::style::Color) -> Self {
        self.color = Some(color);
        self
    }
    /// A finite icon or single-line text within 32 UTF-8 bytes.
    pub fn left_section(mut self, section: impl Into<BadgeSection>) -> Self {
        self.left = Some(section.into());
        self
    }
    pub fn right_section(mut self, section: impl Into<BadgeSection>) -> Self {
        self.right = Some(section.into());
        self
    }
    pub fn content(mut self, content: impl Into<Vary<String>>) -> Self {
        self.content = content.into();
        self
    }
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}
fn badge_side(
    scope: &Scope,
    section: BadgeSection,
    class: &str,
    color: glory::style::Color,
    custom: Vary<Style>,
) -> glory::ui::View {
    let mut wrapper = glory::ui::view().class(class.to_owned()).style(Style {
        align: Some(Align::Center),
        shrink: Some(0.),
        ..Style::new()
            .display(Display::InlineFlex)
            .min_width(glory::style::Length::Px(0.))
    });
    match section {
        BadgeSection::Text(text) => wrapper = wrapper.fill(glory::ui::text().content(text)),
        BadgeSection::Icon(icon) => {
            let reactive = custom.is_reactive();
            let drawing = move || icon.drawing(custom.with(|style| style.color.unwrap_or(color)));
            let drawing: Vary<glory::render::GraphicProps> = if reactive {
                scope.bond(drawing).into()
            } else {
                drawing().into()
            };
            wrapper = wrapper.fill(
                glory::ui::graphic().drawing(drawing).style(
                    Style::new()
                        .width(glory::style::Length::Px(14.))
                        .height(glory::style::Length::Px(14.)),
                ),
            );
        }
    }
    wrapper
}
impl Widget for Badge {
    fn build(&mut self, scope: &Scope) -> Result {
        for section in [&self.left, &self.right].into_iter().flatten() {
            if let BadgeSection::Text(text) = section
                && (text.trim().is_empty()
                    || text.len() > 32
                    || text.contains(['\0', '\r', '\n', '\u{2028}', '\u{2029}']))
            {
                return Err(glory::render::Error::InvalidProps(
                    "badge sections require nonempty single-line text within 32 UTF-8 bytes",
                )
                .into());
            }
        }
        let theme = &self.common.theme;
        let metrics = theme.typography(crate::TypographyRole::Caption, self.common.size);
        let mut base = theme
            .label(self.common.size)
            .font_weight(600.)
            .padding(Edges {
                top: 2.,
                bottom: 2.,
                left: 8.,
                right: 8.,
            })
            .radius(100.)
            .display(Display::InlineFlex)
            .min_width(glory::style::Length::Px(0.))
            .max_width(glory::style::Limit::Percent(100.))
            .white_space(glory::style::WhiteSpace::NoWrap)
            .overflow_x(glory::style::Overflow::Hidden)
            .overflow_y(glory::style::Overflow::Hidden);
        base.font_size = Some(metrics.font_size);
        base.line_height = Some(metrics.line_height);
        let accent = self.color.unwrap_or(theme.accent);
        let class = match self.variant {
            BadgeVariant::Soft => {
                let fill = self.color.map_or(theme.accent_soft, |c| {
                    glory::style::Color(c.0, c.1, c.2, (f64::from(c.3) * 0.12).round() as u8)
                });
                base.background = Some(fill);
                base.color = Some(self.color.map_or(theme.accent_text, |c| {
                    crate::alert::filled_foreground(fill, theme.surface, c)
                }));
                "sapid-badge sapid-badge-soft"
            }
            BadgeVariant::Solid => {
                base.background = Some(accent);
                base.color = Some(crate::alert::filled_foreground(
                    accent,
                    theme.surface,
                    theme.on_accent,
                ));
                "sapid-badge sapid-badge-solid"
            }
            BadgeVariant::Outline => {
                base.color = Some(self.color.map_or(theme.accent_text, |c| {
                    crate::alert::filled_foreground(theme.surface, theme.surface, c)
                }));
                base.border_width = Some(Edges::all(1.));
                base.border_style = Some(BorderStyle::Solid);
                base.border_color = Some(accent);
                "sapid-badge sapid-badge-outline"
            }
            BadgeVariant::Dot => {
                base.color = Some(theme.foreground);
                base.background = Some(theme.surface);
                base.border_width = Some(Edges::all(1.));
                base.border_style = Some(BorderStyle::Solid);
                base.border_color = Some(theme.border);
                "sapid-badge sapid-badge-dot"
            }
        };
        // The ordinary text-only form keeps its existing Text identity and
        // passive Button-content contract. Rich declarations use a boxed row.
        if self.left.is_none() && self.right.is_none() && self.variant != BadgeVariant::Dot {
            return configure!(self.common, glory::ui::text(), scope, class, base)
                .content(self.content.clone())
                .show_in(scope);
        }
        base.gap = Some(6.);
        base.align = Some(Align::Center);
        let ink = base.color.unwrap_or(theme.foreground);
        let custom = self.common.style.clone();
        let mut root = configure!(self.common, glory::ui::view(), scope, class, base);
        if self.variant == BadgeVariant::Dot {
            let mut drawing = crate::Icon::Circle.drawing(accent);
            drawing.stroke = None;
            drawing.fill = Some(accent);
            root = root.fill(
                glory::ui::graphic()
                    .class("sapid-badge-dot-marker")
                    .drawing(drawing)
                    .style(Style {
                        shrink: Some(0.),
                        ..Style::new()
                            .width(glory::style::Length::Px(6.))
                            .height(glory::style::Length::Px(6.))
                    }),
            );
        }
        if let Some(section) = self.left.take() {
            root = root.fill(badge_side(
                scope,
                section,
                "sapid-badge-left",
                ink,
                custom.clone(),
            ));
        }
        root = root.fill(
            glory::ui::text()
                .class("sapid-badge-content")
                .content(self.content.clone())
                .style(
                    Style::new()
                        .min_width(glory::style::Length::Px(0.))
                        .overflow_x(glory::style::Overflow::Hidden)
                        .white_space(glory::style::WhiteSpace::NoWrap),
                ),
        );
        if let Some(section) = self.right.take() {
            root = root.fill(badge_side(scope, section, "sapid-badge-right", ink, custom));
        }
        root.show_in(scope)
    }
}

/// Removable text with its own native close button. A controlled visible source
/// is never written; on_remove proposes dismissal. Hidden content keeps identity.
pub struct Pill {
    common: Common,
    content: Vary<String>,
    visible: Value<bool>,
    remove_label: Option<String>,
    on_remove: Option<Box<dyn FnMut()>>,
    remove_ref: Option<Cage<Option<NodeId>>>,
}
pub fn pill() -> Pill {
    Pill {
        common: Common::default(),
        content: "".into(),
        visible: Value {
            source: None,
            default: true,
        },
        remove_label: None,
        on_remove: None,
        remove_ref: None,
    }
}
impl Pill {
    display_methods!();
    pub fn content(mut self, content: impl Into<Vary<String>>) -> Self {
        self.content = content.into();
        self
    }
    /// Add removal with a localized, nonempty accessible action name.
    pub fn removable(mut self, label: impl Into<String>) -> Self {
        self.remove_label = Some(label.into());
        self
    }
    pub fn visible(mut self, source: impl Into<Vary<bool>>) -> Self {
        self.visible.source = Some(source.into());
        self
    }
    pub fn default_visible(mut self, visible: bool) -> Self {
        self.visible.default = visible;
        self
    }
    pub fn enabled(mut self, enabled: impl Into<Vary<bool>>) -> Self {
        self.common.enabled = enabled.into();
        self
    }
    pub fn on_remove(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_remove = Some(Box::new(callback));
        self
    }
    pub fn remove_ref(mut self, reference: Cage<Option<NodeId>>) -> Self {
        self.remove_ref = Some(reference);
        self
    }
    /// Observe removal activation after internal behavior and on_remove.
    pub fn on_remove_press(mut self, observer: impl FnMut(glory::render::Event) + 'static) -> Self {
        self.common
            .handlers
            .entry(EventKind::Activate)
            .or_default()
            .push(Box::new(observer));
        self
    }
}
impl Widget for Pill {
    fn build(&mut self, scope: &Scope) -> Result {
        if let Some(label) = &self.remove_label
            && label.trim().is_empty()
        {
            return Err(glory::render::Error::InvalidProps("pill removal requires a name").into());
        }
        let visible = std::mem::replace(
            &mut self.visible,
            Value {
                source: None,
                default: true,
            },
        )
        .mount(scope);
        let source = visible.source.clone();
        let metadata = scope.bond(move || {
            Some(Box::new(Semantics {
                hidden: !source.get(),
                ..Default::default()
            }))
        });
        let base = crate::tag::style(&self.common.theme, self.common.size, true);
        let mut root = configure!(self.common, glory::ui::view(), scope, "sapid-pill", base)
            .semantics(metadata)
            .fill(
                glory::ui::text()
                    .class("sapid-pill-label")
                    .style(crate::tag::label_style())
                    .content(self.content.clone()),
            );
        if let Some(label) = self.remove_label.take() {
            let weak = scope.downgrade();
            let owner = scope.id();
            let mut callback = self.on_remove.take();
            let mut observers = self
                .common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default();
            let enabled = self.common.enabled.clone();
            let source = visible.source.clone();
            let button_enabled = scope.bond(move || enabled.get() && source.get());
            let mut button = crate::close_button()
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .style(crate::tag::remove_style(self.common.size))
                .class("sapid-pill-remove")
                .name(label)
                .enabled(button_enabled)
                .on_press(move || {
                    let Some(scope) = weak.upgrade() else { return };
                    let result = (|| -> Result {
                        if !visible.source.try_get()? {
                            return Ok(());
                        }
                        visible.accept(&false)?;
                        if let Some(callback) = &mut callback {
                            callback();
                        }
                        observe(&mut observers, owner, glory::render::Event::Activate);
                        Ok(())
                    })();
                    if let Err(error) = result {
                        scope.report_error(error);
                    }
                });
            if self.common.unstyled {
                button = button.unstyled();
            }
            if let Some(reference) = self.remove_ref {
                button = button.node_ref(reference);
            }
            root = root.fill(button);
        }
        root.show_in(scope)
    }
}

/// Passive surface. Interactive content retains its own intrinsic semantics.
pub struct Paper {
    common: Common,
    children: Children,
    bordered: bool,
    shadow: Option<ShadowSize>,
}
pub fn paper() -> Paper {
    Paper {
        common: Common::default(),
        children: ().into_children(),
        bordered: false,
        shadow: None,
    }
}
impl Paper {
    display_methods!();
    pub fn fill(mut self, children: impl IntoChildren) -> Self {
        self.children = children.into_children();
        self
    }
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
    pub fn shadow(mut self, shadow: ShadowSize) -> Self {
        self.shadow = Some(shadow);
        self
    }
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
}
impl Widget for Paper {
    fn build(&mut self, scope: &Scope) -> Result {
        let base = surface(&self.common, self.bordered, self.shadow);
        configure!(self.common, glory::ui::view(), scope, "sapid-paper", base)
            .fill(take(&mut self.children))
            .show_in(scope)
    }
}

/// A passive card with ordered header, title/description, body and footer slots.
pub struct Card {
    common: Common,
    title: Option<Vary<String>>,
    description: Option<Vary<String>>,
    level: u8,
    header: Children,
    body: Children,
    footer: Children,
    bordered: bool,
    shadow: Option<ShadowSize>,
}
pub fn card() -> Card {
    Card {
        common: Common::default(),
        title: None,
        description: None,
        level: 3,
        header: ().into_children(),
        body: ().into_children(),
        footer: ().into_children(),
        bordered: true,
        shadow: None,
    }
}
impl Card {
    display_methods!();
    pub fn title(mut self, title: impl Into<Vary<String>>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn description(mut self, description: impl Into<Vary<String>>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn heading_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
    pub fn header(mut self, children: impl IntoChildren) -> Self {
        self.header = children.into_children();
        self
    }
    pub fn fill(mut self, children: impl IntoChildren) -> Self {
        self.body = children.into_children();
        self
    }
    pub fn footer(mut self, children: impl IntoChildren) -> Self {
        self.footer = children.into_children();
        self
    }
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
    pub fn shadow(mut self, shadow: ShadowSize) -> Self {
        self.shadow = Some(shadow);
        self
    }
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
}
impl Widget for Card {
    fn build(&mut self, scope: &Scope) -> Result {
        validate_level(self.level)?;
        let base = surface(&self.common, self.bordered, self.shadow);
        let mut header = glory::ui::view()
            .class("sapid-card-header")
            .fill(take(&mut self.header));
        if let Some(title) = self.title.take() {
            let mut title = glory::ui::text()
                .class("sapid-card-title")
                .kind(TextKind::Heading(self.level))
                .content(title);
            if !self.common.unstyled {
                title = title.style(
                    self.common
                        .theme
                        .label(self.common.size)
                        .font_weight(600.0)
                        .margin(Edges::all(0.)),
                );
            }
            header = header.fill(title);
        }
        if let Some(description) = self.description.take() {
            let mut text = glory::ui::text()
                .class("sapid-card-description")
                .kind(TextKind::Paragraph)
                .content(description);
            if !self.common.unstyled {
                text = text.style(
                    self.common
                        .theme
                        .description(self.common.size)
                        .margin(Edges::all(0.)),
                );
            }
            header = header.fill(text);
        }
        if !self.common.unstyled {
            header = header.style(column(4.0));
        }
        configure!(self.common, glory::ui::view(), scope, "sapid-card", base)
            .fill(header)
            .fill(
                glory::ui::view()
                    .class("sapid-card-body")
                    .fill(take(&mut self.body)),
            )
            .fill(
                glory::ui::view()
                    .class("sapid-card-footer")
                    .fill(take(&mut self.footer)),
            )
            .show_in(scope)
    }
}

fn validate_level(level: u8) -> Result {
    if !(1..=6).contains(&level) {
        return Err(glory::render::Error::InvalidProps(
            "heading level must be between one and six",
        )
        .into());
    }
    Ok(())
}

/// Ordinary empty content, without unsolicited alert/live announcements.
pub struct EmptyState {
    common: Common,
    title: Vary<String>,
    description: Option<Vary<String>>,
    level: u8,
    illustration: Children,
    actions: Children,
    children: Children,
}
pub fn empty_state() -> EmptyState {
    EmptyState {
        common: Common::default(),
        title: "No results".into(),
        description: None,
        level: 2,
        illustration: ().into_children(),
        actions: ().into_children(),
        children: ().into_children(),
    }
}
impl EmptyState {
    display_methods!();
    pub fn title(mut self, title: impl Into<Vary<String>>) -> Self {
        self.title = title.into();
        self
    }
    pub fn description(mut self, description: impl Into<Vary<String>>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn heading_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
    pub fn illustration(mut self, children: impl IntoChildren) -> Self {
        self.illustration = children.into_children();
        self
    }
    pub fn actions(mut self, children: impl IntoChildren) -> Self {
        self.actions = children.into_children();
        self
    }
    pub fn fill(mut self, children: impl IntoChildren) -> Self {
        self.children = children.into_children();
        self
    }
}
impl Widget for EmptyState {
    fn build(&mut self, scope: &Scope) -> Result {
        validate_level(self.level)?;
        let base = Style {
            align: Some(Align::Center),
            ..column(12.0).padding(Edges::all(24.0))
        };
        let mut title = glory::ui::text()
            .class("sapid-empty-title")
            .kind(TextKind::Heading(self.level))
            .content(self.title.clone());
        if !self.common.unstyled {
            title = title.style(
                self.common
                    .theme
                    .label(self.common.size)
                    .font_weight(600.0)
                    .margin(Edges::all(0.)),
            );
        }
        let mut root = configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-empty-state",
            base
        );
        let illustration = glory::ui::fragment(take(&mut self.illustration));
        if !illustration.is_empty() {
            root = root.fill(
                glory::ui::view()
                    .class("sapid-empty-illustration")
                    .fill(illustration),
            );
        }
        root = root.fill(title);
        if let Some(description) = self.description.take() {
            let mut text = glory::ui::text()
                .class("sapid-empty-description")
                .kind(TextKind::Paragraph)
                .content(description);
            if !self.common.unstyled {
                text = text.style(
                    self.common
                        .theme
                        .description(self.common.size)
                        .margin(Edges::all(0.)),
                );
            }
            root = root.fill(text);
        }
        let children = glory::ui::fragment(take(&mut self.children));
        if !children.is_empty() {
            root = root.fill(glory::ui::view().class("sapid-empty-body").fill(children));
        }
        let actions = glory::ui::fragment(take(&mut self.actions));
        if !actions.is_empty() {
            root = root.fill(
                glory::ui::view()
                    .class("sapid-empty-actions")
                    .style(if self.common.unstyled {
                        Style::new()
                    } else {
                        Style {
                            align: Some(Align::Center),
                            justify: Some(glory::style::Justify::Center),
                            flex_wrap: Some(glory::style::FlexWrap::Wrap),
                            ..Style::new().display(Display::Flex).gap(8.)
                        }
                    })
                    .fill(actions),
            );
        }
        root.show_in(scope)
    }
}

impl Default for Badge {
    fn default() -> Self {
        badge()
    }
}

impl Default for Pill {
    fn default() -> Self {
        pill()
    }
}

impl Default for Paper {
    fn default() -> Self {
        paper()
    }
}

impl Default for Card {
    fn default() -> Self {
        card()
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        empty_state()
    }
}
