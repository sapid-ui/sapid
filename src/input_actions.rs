//! Shared geometry for native inputs/buttons with independent inset actions.
use crate::appearance::Common;
use glory::render::Semantics;
use glory::style::{
    Align, Color, Display, Edges, Justify, Length, Limit, PointerEvents, Position, Style,
};
use glory::ui::{Result, Widget};
use glory::{Scope, Vary};

pub(crate) fn side(common: &Common) -> f64 {
    common.theme.tokens.control(common.size).min_height
}

pub(crate) fn frame(scope: &Scope, common: &Common, class: &str) -> Result<Scope> {
    let reference = scope.cage(None);
    let own = common.style.clone();
    let side = side(common);
    glory::ui::view()
        .class(class.to_owned())
        .node_ref(reference)
        .style(scope.bond(move || {
            own.with(|own| Style {
                display: own.display.or(Some(Display::InlineFlex)),
                align: Some(Align::Stretch),
                position: Some(match own.position {
                    Some(position @ (Position::Absolute | Position::Fixed | Position::Sticky)) => {
                        position
                    }
                    _ => Position::Relative,
                }),
                top: own.top,
                right: own.right,
                bottom: own.bottom,
                left: own.left,
                width: own.width.or(Some(Length::Percent(100.))),
                min_width: own.min_width.or(Some(Length::Px(0.))),
                max_width: own.max_width.or(Some(Limit::Percent(100.))),
                height: own.height,
                min_height: own.min_height.or(Some(Length::Px(side))),
                max_height: own.max_height,
                margin: own.margin,
                grow: own.grow,
                shrink: own.shrink,
                ..Style::new()
            })
        }))
        .show_in(scope)?;
    scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)
}

pub(crate) fn display_style(
    scope: &Scope,
    common: &Common,
    enabled: Vary<bool>,
    metadata: Vary<Option<Box<Semantics>>>,
    actions: usize,
) -> Vary<Style> {
    let own = common.style.clone();
    let theme = common.theme.clone();
    let unstyled = common.unstyled;
    let mut base = if unstyled {
        Style::new()
    } else {
        theme.control(common.size)
    };
    let side = side(common);
    base.min_height.get_or_insert(Length::Px(side));
    scope
        .bond(move || {
            let own = own.get();
            let mut base = base.clone();
            if !unstyled && metadata.with(|value| value.as_ref().is_some_and(|value| value.invalid))
            {
                base.border_color = Some(theme.danger);
            }
            if !unstyled && !enabled.get() {
                base.color = Some(theme.disabled_text);
                base.background = Some(theme.disabled);
                base.border_color = Some(theme.border);
            }
            let height = own.height;
            let max_height = own.max_height;
            let mut style = crate::appearance::merge(base, own);
            // Layout belongs to the frame, while typography/paint stay on the
            // actual control. Percentages, margins and flex growth apply once.
            style.position = None;
            style.top = None;
            style.right = None;
            style.bottom = None;
            style.left = None;
            style.margin = Some(Edges::all(0.));
            style.grow = Some(1.);
            style.shrink = Some(1.);
            if height.is_some() {
                style.height = Some(Length::Auto);
            }
            if max_height.is_some() {
                style.max_height = Some(Limit::None);
            }
            style.width = Some(Length::Percent(100.));
            style.min_width = Some(Length::Px(0.));
            style.max_width = Some(Limit::Percent(100.));
            let mut padding = style.padding.unwrap_or(Edges::all(0.));
            padding.right = padding.right.max(side * actions as f64 + 4.);
            style.padding = Some(padding);
            style
        })
        .into()
}

pub(crate) fn action_style(common: &Common, index: usize) -> Style {
    let side = side(common);
    let mut style = Style {
        align: Some(Align::Center),
        justify: Some(Justify::Center),
        ..Style::new()
            .position(Position::Absolute)
            .right(Length::Px(side * index as f64))
            .top(Length::Px(0.))
            .width(Length::Px(side))
            .height(Length::Percent(100.))
            .min_width(Length::Px(0.))
            .min_height(Length::Px(0.))
            .padding(Edges::all(0.))
            .display(Display::InlineFlex)
    };
    if !common.unstyled {
        style.background = Some(Color(0, 0, 0, 0));
        style.border_width = Some(Edges::all(0.));
    }
    style
}

#[cfg(feature = "dates")]
pub(crate) fn flow_action_style(common: &Common) -> Style {
    let side = side(common);
    let mut style = Style {
        align: Some(Align::Center),
        justify: Some(Justify::Center),
        shrink: Some(0.),
        ..Style::new()
            .display(Display::InlineFlex)
            .width(Length::Px(side))
            .min_width(Length::Px(side))
            .height(Length::Px(side))
            .padding(Edges::all(0.))
    };
    if !common.unstyled {
        style.background = Some(Color(0, 0, 0, 0));
        style.border_width = Some(Edges::all(0.));
        style.radius = Some(common.theme.radius);
    }
    style
}

#[cfg(feature = "dates")]
pub(crate) fn footer_row(scope: &Scope, class: &str) -> Result<Scope> {
    let reference = scope.cage(None);
    glory::ui::view()
        .class(class.to_owned())
        .node_ref(reference)
        .style(Style {
            align: Some(Align::Center),
            justify: Some(Justify::End),
            ..Style::new()
                .display(Display::Flex)
                .flex_wrap(glory::style::FlexWrap::Wrap)
                .gap(8.)
                .min_width(Length::Px(0.))
        })
        .show_in(scope)?;
    scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)
}

/// Keep separators with their editors and wrap the action group as a unit.
#[cfg(feature = "dates")]
pub(crate) fn segmented_row(
    scope: &Scope,
    class: &str,
    actions_visible: Vary<bool>,
) -> Result<(Scope, Scope)> {
    let row = scope.cage(None);
    glory::ui::view()
        .class(class.to_owned())
        .node_ref(row)
        .style(Style {
            align: Some(Align::Center),
            ..Style::new()
                .display(Display::Flex)
                .flex_wrap(glory::style::FlexWrap::Wrap)
                .gap(8.)
                .min_width(Length::Px(0.))
        })
        .show_in(scope)?;
    let parent = scope.child_in(row.get().ok_or(glory::ui::Error::Gone)?)?;
    let fields = scope.cage(None);
    glory::ui::view()
        .class("sapid-time-fields")
        .node_ref(fields)
        .style(Style {
            align: Some(Align::Center),
            shrink: Some(1.),
            ..Style::new()
                .display(Display::InlineFlex)
                .gap(4.)
                .min_width(Length::Px(0.))
        })
        .show_in(&parent)?;
    let actions = scope.cage(None);
    glory::ui::view()
        .class("sapid-time-actions")
        .node_ref(actions)
        .style(scope.bond(move || {
            Style {
                align: Some(Align::Center),
                shrink: Some(0.),
                ..Style::new()
                    .display(if actions_visible.get() {
                        Display::InlineFlex
                    } else {
                        Display::None
                    })
                    .gap(4.)
            }
        }))
        .show_in(&parent)?;
    let fields = scope.child_in(fields.get().ok_or(glory::ui::Error::Gone)?)?;
    let actions = scope.child_in(actions.get().ok_or(glory::ui::Error::Gone)?)?;
    scope.retain(parent)?;
    Ok((fields, actions))
}

pub(crate) fn glyph(icon: crate::Icon, color: Color) -> glory::ui::Graphic {
    glory::ui::graphic()
        .class("sapid-input-action-icon")
        .drawing(icon.drawing(color))
        .style(Style {
            shrink: Some(0.),
            ..Style::new()
                .display(Display::Block)
                .pointer_events(PointerEvents::None)
                .width(Length::Px(16.))
                .height(Length::Px(16.))
        })
}

#[cfg(feature = "dates")]
pub(crate) fn display_metadata(
    scope: &Scope,
    metadata: Vary<Option<Box<Semantics>>>,
) -> Vary<Option<Box<Semantics>>> {
    scope
        .bond(move || {
            metadata.get().map(|mut value| {
                value.form_control = None;
                value
            })
        })
        .into()
}

/// Keep serialization independent from the readonly display spelling. The real
/// input is hidden from layout/AX, as with Select's intrinsic successful proxy.
#[cfg(feature = "dates")]
pub(crate) fn submission(
    scope: &Scope,
    metadata: Vary<Option<Box<Semantics>>>,
    value: Vary<String>,
    enabled: Vary<bool>,
) -> Result {
    if metadata
        .get()
        .is_none_or(|value| value.form_control.is_none())
    {
        return Ok(());
    }
    let reference = scope.cage(None);
    glory::ui::view()
        .class("sapid-picker-submission")
        .node_ref(reference)
        .semantics(Some(Box::new(Semantics {
            hidden: true,
            ..Default::default()
        })))
        .show_in(scope)?;
    let content = scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)?;
    glory::ui::input()
        .value(value)
        .enabled(enabled)
        .focus(glory::render::FocusMode::Programmatic)
        .semantics(scope.bond(move || {
            let metadata = metadata.get().unwrap_or_default();
            Some(Box::new(Semantics {
                form_control: metadata.form_control,
                required: metadata.required,
                invalid: metadata.invalid,
                readonly: true,
                ..Default::default()
            }))
        }))
        .show_in(&content)?;
    scope.retain(content)
}
