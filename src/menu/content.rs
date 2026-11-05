//! Passive menu content. Intrinsic controls retain ownership of activation.
use super::*;
use glory::ui::{Children, IntoChildren};

/// Keep filtered controls and their owners mounted, while hiding their whole
/// physical shape (including an intrinsic Check's label and input together).
pub(super) fn member_scope(scope: &Scope, visible: Vary<bool>) -> Result<Scope> {
    let owner = scope.child()?;
    if visible.is_reactive() || !visible.try_get()? {
        let reference = owner.cage(None);
        glory::ui::view()
            .class("sapid-menu-filtered-item")
            .node_ref(reference)
            .semantics(owner.bond(move || {
                composite::metadata(Semantics {
                    hidden: !visible.get(),
                    ..Default::default()
                })
            }))
            .show_in(&owner)?;
        let content = owner.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
        scope.retain(owner)?;
        Ok(content)
    } else {
        Ok(owner)
    }
}

pub(super) fn filter_enabled(
    scope: &Scope,
    enabled: Vary<bool>,
    visible: Vary<bool>,
) -> Vary<bool> {
    if visible.is_reactive() || !visible.get() {
        scope.bond(move || enabled.get() && visible.get()).into()
    } else {
        enabled
    }
}

/// An independent visible group caption, excluded from keyboard navigation.
pub struct MenuLabel {
    pub(super) common: Common,
    pub(super) key: String,
    pub(super) label: Vary<String>,
    pub(super) visible: Vary<bool>,
}

pub fn menu_label(key: impl Into<String>, label: impl Into<Vary<String>>) -> MenuLabel {
    MenuLabel {
        common: Common::default(),
        key: key.into(),
        label: label.into(),
        visible: true.into(),
    }
}

impl MenuLabel {
    common_methods!();
    pub fn visible(mut self, visible: impl Into<Vary<bool>>) -> Self {
        self.visible = visible.into();
        self
    }
}

pub(super) fn build_label(
    scope: &Scope,
    parent: &Common,
    namespace: &str,
    path: &[String],
    mut label: MenuLabel,
) -> Result {
    let text = composite::validated(scope, &label.label, |text| {
        if text.len() > 256 || text.trim().is_empty() {
            Err(invalid("menu captions require 1–256 bytes"))
        } else {
            Ok(())
        }
    })?;
    label.common.theme = parent.theme.clone();
    label.common.size = parent.size;
    label.common.unstyled |= parent.unstyled;
    let mut path = path.to_vec();
    path.push(label.key);
    let id = id(namespace, &path, "item")?;
    let visible = label.visible;
    let base = glory::style::Style {
        color: Some(parent.theme.muted),
        font_size: Some(12.),
        font_weight: Some(600.),
        ..glory::style::Style::new().padding(Edges {
            top: 6.,
            bottom: 4.,
            left: 9.,
            right: 9.,
        })
    };
    let mut widget = configure!(
        label.common,
        glory::ui::view(),
        scope,
        "sapid-menu-label",
        base
    )
    .semantics(scope.bond(move || {
        composite::metadata(Semantics {
            id: Some(id.clone()),
            hidden: !visible.get(),
            ..Default::default()
        })
    }))
    .fill(glory::ui::text().content(text));
    let owner = scope.id();
    for (kind, mut handlers) in label.common.handlers {
        widget = widget.on(kind, move |event| observe(&mut handlers, owner, event));
    }
    widget.show_in(scope)
}

#[derive(Default)]
pub(super) struct Sections {
    pub left: Option<Children>,
    pub right: Option<Children>,
}

impl Sections {
    pub fn is_empty(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn decorate(self, button: glory::ui::Button, label: Vary<String>) -> glory::ui::Button {
        if self.is_empty() {
            return button.label(label);
        }
        // Group flattens passive children; nested controls remain rejected by
        // Glory's shared restricted-button placement contract.
        button.label("").name(label.clone()).fill((
            glory::ui::group().fill(self.left.unwrap_or_else(|| ().into_children())),
            glory::ui::text()
                .class("sapid-menu-item-label")
                .content(label)
                .selectable(false)
                .style(glory::style::Style {
                    grow: Some(1.),
                    ..glory::style::Style::new()
                        .min_width(Length::Px(0.))
                        .white_space(glory::style::WhiteSpace::NoWrap)
                        .overflow_x(glory::style::Overflow::Hidden)
                }),
            glory::ui::group().fill(self.right.unwrap_or_else(|| ().into_children())),
        ))
    }
}
