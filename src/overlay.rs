pub use glory::render::{
    AnchorAlign, AnchorArrow, AnchorArrowPosition, AnchorPlacement, AnchorSide,
};
use glory::render::{
    DismissReason, Event, EventKind, FocusMode, Layer, LayerPlacement, NodeId, Role, SemanticId,
    Semantics, TextKind,
};
use glory::style::{Edges, Length, Style};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};

use crate::appearance::Common;
use crate::composite::{self, Callback, ContentPolicy, Factory, Observers};
use crate::state::Value;

/// Drawer edge in viewport coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}
/// Finite Popover panel width policy.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PopoverWidth {
    /// Use the component's intrinsic content width.
    Auto,
    /// Match the live trigger border-box width in the host.
    Target,
    /// Use a fixed CSS-pixel width.
    #[default]
    Default,
    Px(f64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertCancelReason {
    Cancel,
    Dismiss(DismissReason),
}

struct Overlay {
    key: String,
    common: Common,
    kind: &'static str,
    label: Vary<String>,
    title: Option<Vary<String>>,
    description: Option<Vary<String>>,
    close_label: Vary<String>,
    show_close: bool,
    show_trigger: bool,
    open: Value<bool>,
    change: Option<Box<dyn FnMut(bool)>>,
    dismiss: Option<Box<dyn FnMut(DismissReason)>>,
    content: Option<Factory>,
    trigger_content: Option<Factory>,
    trigger_style: Option<Vary<Style>>,
    close_content: Option<Factory>,
    policy: ContentPolicy,
    trigger: Option<Cage<Option<NodeId>>>,
    external_anchor: Option<SemanticId>,
    panel: Option<Cage<Option<NodeId>>>,
    config: Layer,
    snaps: Option<Vary<crate::DrawerSnaps>>,
    snap: Value<usize>,
    snap_change: Option<Box<dyn FnMut(usize)>>,
    confirm_label: Vary<String>,
    confirm: Option<Box<dyn FnMut(())>>,
    cancel: Option<Box<dyn FnMut(AlertCancelReason)>>,
    confirm_enabled: Vary<bool>,
    confirm_variant: crate::ButtonVariant,
    full_screen: bool,
    popover_width: PopoverWidth,
}
impl Overlay {
    fn new(
        key: impl Into<String>,
        kind: &'static str,
        modal: bool,
        placement: LayerPlacement,
    ) -> Self {
        Self {
            key: key.into(),
            common: Common::default(),
            kind,
            label: "Open".into(),
            title: None,
            description: None,
            close_label: if kind == "alert-dialog" {
                "Cancel"
            } else {
                "Close"
            }
            .into(),
            show_close: true,
            show_trigger: true,
            open: Value::default(),
            change: None,
            dismiss: None,
            content: None,
            trigger_content: None,
            trigger_style: None,
            close_content: None,
            policy: ContentPolicy::Unmount,
            trigger: None,
            external_anchor: None,
            panel: None,
            snaps: None,
            snap: Value::default(),
            snap_change: None,
            config: Layer {
                modal,
                placement,
                outside: !matches!(kind, "alert-dialog" | "floating-dialog"),
                focus: if kind == "floating-dialog" {
                    glory::render::LayerFocus::Preserve
                } else {
                    glory::render::LayerFocus::Manage
                },
                ..Default::default()
            },
            confirm_label: "Confirm".into(),
            confirm: None,
            cancel: None,
            confirm_enabled: true.into(),
            confirm_variant: crate::ButtonVariant::Primary,
            full_screen: false,
            popover_width: PopoverWidth::Default,
        }
    }
}

macro_rules! overlay {
    ($name:ident, $function:ident, $kind:literal, $modal:literal, $placement:expr) => {
        pub struct $name {
            inner: Overlay,
        }
        pub fn $function(key: impl Into<String>) -> $name {
            $name {
                inner: Overlay::new(key, $kind, $modal, $placement),
            }
        }
        impl $name {
            /// Controlled input; changes are proposals delivered to the callback.
            pub fn open(mut self, value: impl Into<Vary<bool>>) -> Self {
                self.inner.open.source = Some(value.into());
                self
            }
            pub fn default_open(mut self, value: bool) -> Self {
                self.inner.open.default = value;
                self
            }
            pub fn label(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.label = value.into();
                self
            }
            pub fn title(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.title = Some(value.into());
                self
            }
            pub fn description(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.description = Some(value.into());
                self
            }
            pub fn close_label(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.close_label = value.into();
                self
            }
            /// Visual children of the real trigger Button. `label` remains its
            /// accessible name; interactive descendants are rejected by Glory.
            pub fn trigger_content<W: Widget>(
                mut self,
                mut factory: impl FnMut() -> W + 'static,
            ) -> Self {
                self.inner.trigger_content = Some(Box::new(move || Box::new(factory())));
                self
            }
            /// Style only the real trigger; panel geometry remains independent.
            pub fn trigger_style(mut self, value: impl Into<Vary<Style>>) -> Self {
                self.inner.trigger_style = Some(value.into());
                self
            }
            /// Visual children of the real close/cancel Button. `close_label`
            /// remains its accessible name.
            pub fn close_content<W: Widget>(
                mut self,
                mut factory: impl FnMut() -> W + 'static,
            ) -> Self {
                self.inner.close_content = Some(Box::new(move || Box::new(factory())));
                self
            }
            pub fn show_close(mut self, value: bool) -> Self {
                self.inner.show_close = value;
                self
            }
            /// An externally controlled dialog may omit its built-in trigger.
            /// Provide a title or name; anchored Popovers require their trigger.
            pub fn show_trigger(mut self, value: bool) -> Self {
                self.inner.show_trigger = value;
                self
            }
            pub fn on_open_change(mut self, callback: impl FnMut(bool) + 'static) -> Self {
                self.inner.change = Some(Box::new(callback));
                self
            }
            pub fn on_dismiss(mut self, callback: impl FnMut(DismissReason) + 'static) -> Self {
                self.inner.dismiss = Some(Box::new(callback));
                self
            }
            pub fn dismiss_on_escape(mut self, value: bool) -> Self {
                self.inner.config.escape = value;
                self
            }
            pub fn dismiss_on_outside(mut self, value: bool) -> Self {
                self.inner.config.outside = value;
                self
            }
            pub fn content<W: Widget>(mut self, mut factory: impl FnMut() -> W + 'static) -> Self {
                self.inner.content = Some(Box::new(move || Box::new(factory())));
                self
            }
            pub fn content_policy(mut self, value: ContentPolicy) -> Self {
                self.inner.policy = value;
                self
            }
            pub fn trigger_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
                self.inner.trigger = Some(value);
                self
            }
            pub fn panel_ref(mut self, value: Cage<Option<NodeId>>) -> Self {
                self.inner.panel = Some(value);
                self
            }
            pub fn class(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.common.class = value.into();
                self
            }
            pub fn name(mut self, value: impl Into<Vary<String>>) -> Self {
                self.inner.common.name = Some(value.into());
                self
            }
            pub fn style(mut self, value: impl Into<Vary<Style>>) -> Self {
                self.inner.common.style = value.into();
                self
            }
            pub fn theme(mut self, value: crate::Theme) -> Self {
                self.inner.common.theme = value;
                self
            }
            pub fn size(mut self, value: crate::Size) -> Self {
                self.inner.common.size = value;
                self
            }
            pub fn unstyled(mut self) -> Self {
                self.inner.common.unstyled = true;
                self
            }
            pub fn enabled(mut self, value: impl Into<Vary<bool>>) -> Self {
                self.inner.common.enabled = value.into();
                self
            }
            pub fn on(mut self, kind: EventKind, callback: impl FnMut(Event) + 'static) -> Self {
                self.inner
                    .common
                    .handlers
                    .entry(kind)
                    .or_default()
                    .push(Box::new(callback));
                self
            }
        }
        impl Widget for $name {
            fn build(&mut self, scope: &Scope) -> Result {
                self.inner.build(scope)
            }
        }
    };
}
overlay!(Dialog, dialog, "dialog", true, LayerPlacement::Center);
overlay!(Drawer, drawer, "drawer", true, LayerPlacement::Right);
overlay!(
    Popover,
    popover,
    "popover",
    false,
    LayerPlacement::Anchor(AnchorPlacement::default())
);
overlay!(
    AlertDialog,
    alert_dialog,
    "alert-dialog",
    true,
    LayerPlacement::Center
);
overlay!(
    FloatingDialog,
    floating_dialog,
    "floating-dialog",
    false,
    LayerPlacement::Center
);
macro_rules! managed_focus {
    ($($name:ident),+)=>{$(impl $name {
        /// Resolve a stable semantic name inside the live panel; otherwise the
        /// host chooses its first enabled tab stop, then the panel.
        pub fn initial_focus(mut self,id:SemanticId)->Self{self.inner.config.initial_focus=Some(id);self}
        pub fn return_focus(mut self,id:SemanticId)->Self{self.inner.config.return_focus=Some(id);self}
    })+};
}
managed_focus!(Dialog, Drawer, Popover, AlertDialog);
impl FloatingDialog {
    /// Position the nonmodal notice in viewport CSS pixels. The shared host
    /// constrains it to the viewport; opening still preserves actual focus.
    pub fn position(mut self, position: glory::render::ViewportPlacement) -> Self {
        self.inner.config.placement = LayerPlacement::Viewport(position);
        self
    }
}
impl Dialog {
    /// Fill the host viewport while preserving the same modal focus and dismissal protocol.
    pub fn full_screen(mut self, value: bool) -> Self {
        self.inner.full_screen = value;
        self
    }
}
impl AlertDialog {
    pub fn confirm_enabled(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.inner.confirm_enabled = value.into();
        self
    }
    pub fn confirm_variant(mut self, value: crate::ButtonVariant) -> Self {
        self.inner.confirm_variant = value;
        self
    }
    pub fn confirm_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.inner.confirm_label = value.into();
        self
    }
    pub fn cancel_label(mut self, value: impl Into<Vary<String>>) -> Self {
        self.inner.close_label = value.into();
        self
    }
    pub fn on_confirm(mut self, mut callback: impl FnMut() + 'static) -> Self {
        self.inner.confirm = Some(Box::new(move |()| callback()));
        self
    }
    pub fn on_cancel(mut self, callback: impl FnMut(AlertCancelReason) + 'static) -> Self {
        self.inner.cancel = Some(Box::new(callback));
        self
    }
}
impl Drawer {
    /// Enable a bounded draggable sheet, with explicit accessible snap labels.
    pub fn snap_points(mut self, points: impl IntoIterator<Item = crate::DrawerSnap>) -> Self {
        self.inner.snaps = Some(crate::DrawerSnaps::new(points).into());
        self
    }
    pub fn snap_config(mut self, options: impl Into<Vary<crate::DrawerSnaps>>) -> Self {
        self.inner.snaps = Some(options.into());
        self
    }
    pub fn snap_index(mut self, value: impl Into<Vary<usize>>) -> Self {
        self.inner.snap.source = Some(value.into());
        self
    }
    pub fn default_snap(mut self, value: usize) -> Self {
        self.inner.snap.default = value;
        self
    }
    pub fn on_snap_change(mut self, callback: impl FnMut(usize) + 'static) -> Self {
        self.inner.snap_change = Some(Box::new(callback));
        self
    }
    pub fn side(mut self, side: DrawerSide) -> Self {
        self.inner.config.placement = match side {
            DrawerSide::Left => LayerPlacement::Left,
            DrawerSide::Right => LayerPlacement::Right,
            DrawerSide::Top => LayerPlacement::Top,
            DrawerSide::Bottom => LayerPlacement::Bottom,
        };
        self
    }
}
impl Popover {
    /// Use an existing semantic control as the trigger and anchor. The caller
    /// owns activation and the controlled `open` state.
    pub(crate) fn external_trigger(mut self, anchor: SemanticId) -> Self {
        self.inner.show_trigger = false;
        self.inner.external_anchor = Some(anchor);
        // Editor-anchored popups leave editing focus in place. Their caller
        // owns any explicit focus return after interacting with popup controls.
        self.inner.config.focus = glory::render::LayerFocus::Preserve;
        self
    }
    pub fn placement(mut self, placement: AnchorPlacement) -> Self {
        self.inner.config.placement = LayerPlacement::Anchor(placement);
        self
    }
    /// Configure intrinsic, trigger-matched or fixed panel width.
    pub fn width(mut self, width: PopoverWidth) -> Self {
        self.inner.popover_width = width;
        self
    }
    /// Set the anchor-to-panel gap in CSS pixels.
    pub fn offset(mut self, offset: f64) -> Self {
        if let LayerPlacement::Anchor(mut placement) = self.inner.config.placement {
            placement.gap = offset;
            self.inner.config.placement = LayerPlacement::Anchor(placement);
        }
        self
    }
    /// Enable the default collision-aware arrow.
    pub fn with_arrow(mut self, enabled: bool) -> Self {
        if let LayerPlacement::Anchor(mut placement) = self.inner.config.placement {
            placement.arrow = enabled.then(AnchorArrow::default);
            self.inner.config.placement = LayerPlacement::Anchor(placement);
        }
        self
    }
    /// Configure a collision-aware arrow using bounded host geometry.
    pub fn arrow(mut self, arrow: AnchorArrow) -> Self {
        if let LayerPlacement::Anchor(mut placement) = self.inner.config.placement {
            placement.arrow = Some(arrow);
            self.inner.config.placement = LayerPlacement::Anchor(placement);
        }
        self
    }
    pub fn modal(mut self, modal: bool) -> Self {
        self.inner.config.modal = modal;
        self
    }
}
fn icon_close(
    mut button: glory::ui::Button,
    label: Vary<String>,
    theme: &crate::Theme,
    size: crate::Size,
    unstyled: bool,
) -> glory::ui::Button {
    let mut style = if unstyled {
        Style::new()
    } else {
        theme.label(size)
    };
    style.width = Some(Length::Px(24.));
    style.height = Some(Length::Px(24.));
    style.min_width = Some(Length::Px(24.));
    style.min_height = Some(Length::Px(24.));
    style.padding = Some(Edges::all(0.));
    style.border_width = Some(Edges::all(0.));
    style.background = Some(glory::style::Color(0, 0, 0, 0));
    style.display = Some(glory::style::Display::InlineFlex);
    style.align = Some(glory::style::Align::Center);
    style.justify = Some(glory::style::Justify::Center);
    style.shrink = Some(0.);
    button = button.name(label).label("").style(style).fill(
        glory::ui::graphic()
            .drawing(crate::Icon::Close.drawing(theme.foreground))
            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
    );
    button
}
fn layer_header(
    scope: &Scope,
    title: Option<Vary<String>>,
    title_id: &SemanticId,
    close: Option<glory::ui::Button>,
    theme: &crate::Theme,
    size: crate::Size,
    unstyled: bool,
) -> Result {
    if title.is_none() && close.is_none() {
        return Ok(());
    }
    let mut header = glory::ui::view().class("sapid-layer-header").style(Style {
        align: Some(glory::style::Align::Center),
        justify: Some(glory::style::Justify::End),
        ..Style::new()
            .display(glory::style::Display::Flex)
            .gap(12.)
            .min_width(Length::Px(0.))
    });
    if let Some(title) = title {
        let mut style = if unstyled {
            Style::new()
        } else {
            theme.label(size).font_weight(600.)
        };
        style.grow = Some(1.);
        style.min_width = Some(Length::Px(0.));
        style.margin = Some(Edges::all(0.));
        header = header.fill(
            glory::ui::text()
                .class("sapid-layer-title")
                .kind(TextKind::Heading(2))
                .content(title)
                .style(style)
                .semantics(composite::metadata(Semantics {
                    id: Some(title_id.clone()),
                    ..Default::default()
                })),
        );
    }
    if let Some(button) = close {
        header = header.fill(button);
    }
    header.show_in(scope)
}

impl Widget for Overlay {
    fn build(&mut self, scope: &Scope) -> Result {
        if let LayerPlacement::Anchor(placement) = self.config.placement {
            placement.validate()?;
        }
        if self.kind == "popover"
            && let PopoverWidth::Px(width) = self.popover_width
            && (!width.is_finite() || !(1.0..=4096.0).contains(&width))
        {
            return Err(glory::render::Error::InvalidProps(
                "popover width must be finite and within 1..=4096 CSS pixels",
            )
            .into());
        }
        let alert = self.kind == "alert-dialog";
        let floating = self.kind == "floating-dialog";
        if !self.show_trigger
            && (self.title.is_none() && self.common.name.is_none()
                || matches!(self.config.placement, LayerPlacement::Anchor(_))
                    && self.external_anchor.is_none())
        {
            return Err(glory::render::Error::InvalidProps(
                "a triggerless dialog requires a name/title and non-anchored placement",
            )
            .into());
        }
        let trigger_id = composite::id(self.kind, &self.key, "trigger", "root")?;
        let panel_id = composite::id(self.kind, &self.key, "panel", "root")?;
        let title_id = composite::id(self.kind, &self.key, "title", "root")?;
        let description_id = composite::id(self.kind, &self.key, "description", "root")?;
        let close_id = composite::id(self.kind, &self.key, "close", "root")?;
        if alert || floating || !self.show_trigger {
            if (alert && (!self.show_close || self.description.is_none()))
                || (self.title.is_none() && self.common.name.is_none())
            {
                return Err(glory::render::Error::InvalidProps(if alert {
                    "alert dialog requires a name/title, description and safe cancel action"
                } else {
                    "floating dialog requires a title or explicit name"
                })
                .into());
            }
            fn nonempty(value: &str) -> Result {
                if value.trim().is_empty() || value.len() > crate::MAX_ALERT_TEXT_BYTES {
                    return Err(glory::render::Error::InvalidProps(
                        "alert dialog labels must be nonempty and within 64 KiB",
                    )
                    .into());
                }
                Ok(())
            }
            self.label = composite::validated(scope, &self.label, |value| nonempty(value))?.into();
            for text in [
                &mut self.title,
                &mut self.description,
                &mut self.common.name,
            ]
            .into_iter()
            .flatten()
            {
                *text = composite::validated(scope, text, |value| nonempty(value))?.into();
            }
            self.close_label =
                composite::validated(scope, &self.close_label, |value| nonempty(value))?.into();
            if alert {
                self.confirm_label =
                    composite::validated(scope, &self.confirm_label, |value| nonempty(value))?
                        .into();
            }
            if alert && self.config.initial_focus.is_none() {
                self.config.initial_focus = Some(close_id.clone());
            }
        }
        if self.snaps.is_some() && !self.show_close {
            return Err(glory::render::Error::InvalidProps(
                "a sheet requires its accessible close action",
            )
            .into());
        }
        let snap_control = std::mem::take(&mut self.snap).mount(scope);
        let sheet = self
            .snaps
            .as_ref()
            .map(|options| crate::sheet::mount(scope, options, &snap_control))
            .transpose()?;
        let snap_change = Callback::new(self.snap_change.take());
        let sheet_observers = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Sheet)
                .unwrap_or_default(),
        );
        let control = std::mem::take(&mut self.open).mount(scope);
        let source = control.source.clone();
        let change = Callback::new(self.change.take());
        let dismiss = Callback::new(self.dismiss.take());
        let cancel = Callback::new(self.cancel.take());
        let confirm = Callback::new(self.confirm.take());
        let owner = scope.id();
        let weak = scope.downgrade();
        let trigger_observers = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Activate)
                .unwrap_or_default(),
        );
        let c = control.clone();
        let cb = change.clone();
        let id = trigger_id.clone();
        let target = panel_id.clone();
        let open = source.clone();
        let trigger_enabled = self.common.enabled.clone();
        let mut trigger = glory::ui::button()
            .label(self.label.clone())
            .class(format!("sapid-{}-trigger", self.kind))
            .enabled(self.common.enabled.clone())
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    controls: Some(target.clone()),
                    expanded: Some(open.get()),
                    popup: Some(glory::render::PopupKind::Dialog),
                    ..Default::default()
                })
            }))
            .on(EventKind::Activate, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !trigger_enabled.try_get()? {
                        return Ok(());
                    }
                    let next = !c.source.try_get()?;
                    c.accept(&next)?;
                    cb.emit(owner, next);
                    if owner.is_alive() {
                        trigger_observers.emit(owner, event);
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            });
        if let Some(reference) = self.trigger {
            trigger = trigger.node_ref(reference);
        }
        if self.show_trigger
            && let Some(mut factory) = self.trigger_content.take()
        {
            trigger = trigger
                .name(self.label.clone())
                .label("")
                .fill(glory::ui::widget(move |scope| factory().show_in(scope)));
        }
        let trigger_base = if self.common.unstyled {
            Style::new()
        } else {
            self.common.theme.control(self.common.size)
        };
        trigger = if let Some(own) = self.trigger_style.take() {
            if own.is_reactive() {
                trigger.style(
                    scope.bond(move || crate::appearance::merge(trigger_base.clone(), own.get())),
                )
            } else {
                trigger.style(crate::appearance::merge(trigger_base, own.get()))
            }
        } else {
            trigger.style(trigger_base)
        };
        let mut config = self.config.clone();
        if self.kind == "popover"
            && let LayerPlacement::Anchor(mut placement) = config.placement
        {
            placement.match_width = self.popover_width == PopoverWidth::Target;
            config.placement = LayerPlacement::Anchor(placement);
        }
        if (self.show_trigger || self.external_anchor.is_some())
            && config.focus == glory::render::LayerFocus::Manage
            && config.return_focus.is_none()
        {
            config.return_focus = Some(
                self.external_anchor
                    .clone()
                    .unwrap_or_else(|| trigger_id.clone()),
            );
        }
        if matches!(config.placement, LayerPlacement::Anchor(_)) {
            config.anchor = Some(
                self.external_anchor
                    .clone()
                    .unwrap_or_else(|| trigger_id.clone()),
            );
        }
        let label = if self.common.name.is_some() {
            None
        } else {
            Some(if self.title.is_some() {
                title_id.clone()
            } else if let Some(anchor) = self.external_anchor.clone() {
                anchor
            } else {
                trigger_id
            })
        };
        let description = self.description.as_ref().map(|_| description_id.clone());
        let id = panel_id;
        let open = source.clone();
        let dismiss_observers = Observers::new(
            self.common
                .handlers
                .remove(&EventKind::Dismiss)
                .unwrap_or_default(),
        );
        let c = control.clone();
        let cb = change.clone();
        let weak = scope.downgrade();
        let cancel_dismiss = cancel.clone();
        let sheet_dismiss = dismiss.clone();
        let mut panel = glory::ui::view()
            .role(if alert {
                Role::AlertDialog
            } else {
                Role::Dialog
            })
            .focus(FocusMode::Programmatic)
            .semantics(scope.bond(move || {
                composite::metadata(Semantics {
                    id: Some(id.clone()),
                    labelled_by: label.clone(),
                    described_by: description.clone(),
                    layer: Some({
                        let mut next = config.clone();
                        if let Some(sheet) = sheet {
                            next.sheet = Some(sheet.get().protocol);
                        }
                        next
                    }),
                    hidden: !open.get(),
                    ..Default::default()
                })
            }))
            .on(EventKind::Dismiss, move |event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !c.source.try_get()? {
                        return Ok(());
                    }
                    if let Event::Dismiss(reason) = event {
                        c.accept(&false)?;
                        cancel_dismiss.emit(owner, AlertCancelReason::Dismiss(reason));
                        dismiss.emit(owner, reason);
                        if owner.is_alive() {
                            cb.emit(owner, false);
                        }
                        if owner.is_alive() {
                            dismiss_observers.emit(owner, Event::Dismiss(reason));
                        }
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            });
        let panel_reference = self.panel.unwrap_or_else(|| scope.cage(None));
        let sheet_handler = if let Some(accepted) = sheet {
            let open = control.clone();
            let on_open = change.clone();
            let on_snap = snap_change.clone();
            let observers = sheet_observers.clone();
            let snap = snap_control.clone();
            let weak = scope.downgrade();
            Some(Callback::new(Some(Box::new(move |event: Event| {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    if !open.source.try_get()? {
                        return Ok(());
                    }
                    let Event::Sheet {
                        revision, command, ..
                    } = event
                    else {
                        return Ok(());
                    };
                    if accepted.try_with(|state| state.protocol.revision != revision)? {
                        return Ok(());
                    }
                    match command {
                        glory::render::SheetCommand::Snap(index) => {
                            if accepted.try_with(|state| {
                                index >= state.options.points.len()
                                    || index == state.protocol.current
                            })? {
                                return Ok(());
                            }
                            snap.accept(&index)?;
                            on_snap.emit(owner, index);
                        }
                        glory::render::SheetCommand::Dismiss => {
                            if !accepted.try_with(|state| state.protocol.dismissible)? {
                                return Ok(());
                            }
                            open.accept(&false)?;
                            sheet_dismiss.emit(owner, DismissReason::Gesture);
                            on_open.emit(owner, false);
                        }
                    }
                    observers.emit(owner, event);
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            }))))
        } else {
            None
        };
        if let Some(handler) = sheet_handler.clone() {
            panel = panel.on(EventKind::Sheet, move |event| handler.emit(owner, event));
        }
        if sheet.is_some() {
            panel = panel.node_ref(panel_reference);
        }
        let mut base = composite::column(12.0)
            .width(Length::Px(360.0))
            .padding(Edges::all(20.0))
            .radius(self.common.theme.radius);
        if self.kind == "popover" {
            base.width = match self.popover_width {
                PopoverWidth::Auto => None,
                PopoverWidth::Target => Some(Length::Px(360.0)),
                PopoverWidth::Default => Some(Length::Px(360.0)),
                PopoverWidth::Px(width) => Some(Length::Px(width)),
            };
        }
        if self.full_screen {
            if self.kind != "dialog" {
                return Err(glory::render::Error::InvalidProps(
                    "full-screen presentation requires a dialog",
                )
                .into());
            }
            base.width = Some(Length::Percent(100.));
            base.height = Some(Length::Percent(100.));
            base.max_width = Some(glory::style::Limit::None);
            base.max_height = Some(glory::style::Limit::None);
            base.radius = Some(0.);
        }
        let text = self
            .common
            .theme
            .typography(crate::TypographyRole::Body, self.common.size);
        base.font_family = Some(vec![glory::style::Family::System]);
        base.font_size = Some(text.font_size);
        base.line_height = Some(text.line_height);
        base.font_weight = text.weight;
        base.background = Some(self.common.theme.surface);
        base.color = Some(self.common.theme.foreground);
        let presentation = if self.full_screen {
            format!("sapid-{} sapid-layer sapid-dialog-full-screen", self.kind)
        } else {
            format!("sapid-{} sapid-layer", self.kind)
        };
        let (classes, styles) = self.common.appearance(scope, &presentation, base);
        panel = panel.class(classes);
        if sheet.is_some() {
            let edge = self.config.placement;
            panel = panel.style(scope.bond(move || {
                let mut style = styles.get();
                let mut padding = style.padding.unwrap_or_else(|| Edges::all(0.0));
                match edge {
                    LayerPlacement::Left => padding.right = padding.right.max(24.0),
                    LayerPlacement::Right => padding.left = padding.left.max(24.0),
                    LayerPlacement::Top => padding.bottom = padding.bottom.max(24.0),
                    LayerPlacement::Bottom => padding.top = padding.top.max(24.0),
                    _ => {}
                }
                style.padding = Some(padding);
                style
            }));
        } else {
            panel = panel.style(styles);
        }
        if let Some(name) = self.common.name.clone() {
            panel = panel.name(name);
        }
        if let Some(reference) = self.panel {
            panel = panel.node_ref(reference);
        }
        let title = self.title.take();
        let description = self.description.take();
        let close_label = self.close_label.clone();
        let confirm_label = self.confirm_label.clone();
        let confirm_enabled = self.confirm_enabled.clone();
        let confirm_variant = self.confirm_variant;
        let theme = self.common.theme.clone();
        let size = self.common.size;
        let unstyled = self.common.unstyled;
        let show_close = self.show_close;
        let policy = self.policy;
        let mut factory = self.content.take();
        let mut close_content = self.close_content.take();
        let content_open = source;
        let weak = scope.downgrade();
        panel = panel.fill(glory::ui::widget(move |scope| {
            if let (Some(accepted), Some(handler)) = (sheet, sheet_handler.clone()) {
                crate::sheet::controls(
                    scope,
                    accepted,
                    panel_reference,
                    handler,
                    theme.muted,
                    unstyled,
                )?;
            }
            let mut close = if show_close {
                let c = control.clone();
                let cb = change.clone();
                let weak = weak.clone();
                let cancel = cancel.clone();
                let mut cancel_button = glory::ui::button()
                    .class("sapid-layer-close")
                    .label(close_label.clone())
                    .semantics(composite::metadata(Semantics {
                        id: Some(close_id.clone()),
                        ..Default::default()
                    }))
                    .on(EventKind::Activate, move |_| {
                        let Some(scope) = weak.upgrade() else { return };
                        let result = (|| -> Result {
                            c.accept(&false)?;
                            cancel.emit(owner, AlertCancelReason::Cancel);
                            cb.emit(owner, false);
                            Ok(())
                        })();
                        if let Err(error) = result {
                            scope.report_error(error);
                        }
                    });
                if alert && !unstyled {
                    cancel_button = cancel_button.style(theme.control(size));
                }
                let custom_close = close_content.is_some();
                if let Some(mut factory) = close_content.take() {
                    cancel_button = cancel_button
                        .name(close_label.clone())
                        .label("")
                        .fill(glory::ui::widget(move |scope| factory().show_in(scope)));
                }
                if !alert && !custom_close {
                    cancel_button =
                        icon_close(cancel_button, close_label.clone(), &theme, size, unstyled);
                } else if !alert && !unstyled {
                    cancel_button = cancel_button.style(theme.control(size));
                }
                Some(cancel_button)
            } else {
                None
            };
            layer_header(
                scope,
                title.clone(),
                &title_id,
                if alert { None } else { close.take() },
                &theme,
                size,
                unstyled,
            )?;
            if let Some(description) = description.clone() {
                glory::ui::text()
                    .content(description)
                    .semantics(composite::metadata(Semantics {
                        id: Some(description_id.clone()),
                        ..Default::default()
                    }))
                    .show_in(scope)?;
            }
            if let Some(mut factory) = factory.take() {
                match policy {
                    ContentPolicy::Unmount => {
                        glory::ui::when(content_open.clone(), factory).show_in(scope)?
                    }
                    ContentPolicy::KeepMounted => factory().show_in(scope)?,
                }
            }
            if alert {
                if let Some(button) = close.take() {
                    button.show_in(scope)?;
                }
                let c = control.clone();
                let cb = change.clone();
                let confirm = confirm.clone();
                let weak = weak.clone();
                let mut button = crate::button()
                    .theme(theme.clone())
                    .size(size)
                    .variant(confirm_variant)
                    .enabled(confirm_enabled.clone())
                    .class("sapid-alert-dialog-confirm")
                    .label(confirm_label.clone())
                    .on_press(move || {
                        let Some(scope) = weak.upgrade() else { return };
                        let result = (|| -> Result {
                            c.accept(&false)?;
                            confirm.emit(owner, ());
                            cb.emit(owner, false);
                            Ok(())
                        })();
                        if let Err(error) = result {
                            scope.report_error(error);
                        }
                    });
                if unstyled {
                    button = button.unstyled();
                }
                button.show_in(scope)?;
            }
            Ok(())
        }));
        for (kind, handlers) in std::mem::take(&mut self.common.handlers) {
            let observers = Observers::new(handlers);
            panel = panel.on(kind, move |event| observers.emit(owner, event));
        }
        if self.show_trigger {
            glory::ui::view()
                .class("sapid-layer-root")
                .fill((trigger, panel))
                .show_in(scope)
        } else {
            glory::ui::view()
                .class("sapid-layer-root")
                .fill(panel)
                .show_in(scope)
        }
    }
}
