//! Shared dimensions for passive pills and accepted editable tags.
use crate::{Size, Theme};
use glory::style::{Align, Display, Edges, FlexDirection, Length, Limit, Style};

pub(crate) fn remove_side(size: Size) -> f64 {
    match size {
        Size::ExtraSmall => 16.0,
        Size::Small => 18.0,
        Size::Medium => 20.0,
        Size::Large => 24.0,
        Size::ExtraLarge => 28.0,
    }
}

pub(crate) fn style(theme: &Theme, size: Size, rounded: bool) -> Style {
    let mut style = theme.label(size);
    style.display = Some(Display::InlineFlex);
    style.flex_direction = Some(FlexDirection::Row);
    style.align = Some(Align::Center);
    style.gap = Some(4.0);
    style.shrink = Some(0.);
    style.max_width = Some(Limit::Percent(100.0));
    style.background = Some(theme.accent_soft);
    style.color = Some(theme.accent_text);
    style.font_size = Some(
        (theme
            .typography(crate::TypographyRole::Control, size)
            .font_size
            - 2.0)
            .max(0.0),
    );
    style.line_height = Some(remove_side(size));
    style.padding = Some(Edges {
        top: 2.0,
        bottom: 2.0,
        left: 8.0,
        right: 4.0,
    });
    style.radius = Some(if rounded { 999.0 } else { theme.radius });
    style
}

pub(crate) fn label_style() -> Style {
    Style {
        shrink: Some(1.),
        ..Style::new()
            .min_width(Length::Px(0.))
            .overflow_x(glory::style::Overflow::Hidden)
            .white_space(glory::style::WhiteSpace::NoWrap)
    }
}

pub(crate) fn remove_style(size: Size) -> Style {
    let side = Length::Px(remove_side(size));
    Style::new()
        .width(side)
        .height(side)
        .min_width(side)
        .min_height(side)
        .padding(Edges::all(0.0))
        .radius(999.0)
}

/// One wrapping surface for accepted tokens and their existing intrinsic editor.
/// Presentation sources never own editing, selection or composition state.
pub(crate) struct EditorFrame {
    classes: String,
    pub style: glory::Vary<Style>,
    pub focused: glory::Cage<bool>,
    pub input: glory::Vary<Style>,
}
#[derive(Clone, Copy)]
pub(crate) struct EditorActions {
    pub wrapped: bool,
    pub leading: bool,
    pub trailing: u8,
}
pub(crate) fn editor_frame(
    scope: &glory::Scope,
    common: &crate::appearance::Common,
    enabled: glory::Vary<bool>,
    metadata: glory::Vary<Option<Box<glory::render::Semantics>>>,
    actions: EditorActions,
) -> EditorFrame {
    use glory::style::{Color, FlexWrap, Position};
    let focused = scope.cage(false);
    let own = common.style.clone();
    let theme = common.theme.clone();
    let size = common.size;
    let plain = common.unstyled;
    let frame_enabled = enabled.clone();
    let style = scope.bond(move || {
        let mut style = if plain {
            Style::new()
        } else {
            theme.control(size)
        };
        style.display = Some(Display::Flex);
        style.flex_direction = Some(FlexDirection::Row);
        style.flex_wrap = Some(FlexWrap::Wrap);
        style.align = Some(Align::Center);
        style.gap = Some(4.);
        style.position = Some(Position::Relative);
        style.width = Some(Length::Percent(100.));
        style.min_width = Some(Length::Px(0.));
        style.max_width = Some(Limit::Percent(100.));
        style.padding = Some(Edges::all(if plain { 0. } else { 4. }));
        if !plain {
            if !frame_enabled.get() {
                style.background = Some(theme.disabled);
                style.color = Some(theme.disabled_text);
            }
            if metadata.with(|v| v.as_ref().is_some_and(|s| s.invalid)) {
                style.border_color = Some(theme.danger);
            } else if focused.get() {
                style.border_color = Some(theme.focus_ring);
            }
        }
        crate::appearance::merge(style, own.get())
    });
    let own = common.style.clone();
    let theme = common.theme.clone();
    let input = scope.bond(move || {
        let source = own.get();
        let mut style = if plain {
            Style::new()
        } else {
            theme.control(size)
        };
        style.width = Some(if actions.wrapped {
            Length::Percent(100.)
        } else {
            Length::Px(72.)
        });
        style.min_width = Some(Length::Px(0.));
        style.min_height = Some(Length::Px(
            (theme.tokens.control(size).min_height - 10.).max(0.),
        ));
        style.grow = (!actions.wrapped).then_some(1.);
        style.shrink = Some(1.);
        style.padding = Some(Edges {
            left: if actions.leading { 38. } else { 0. },
            right: match actions.trailing {
                0 => 0.,
                1 => 38.,
                _ => 64.,
            },
            top: 0.,
            bottom: 0.,
        });
        style.border_width = Some(Edges::all(0.));
        style.background = Some(Color(0, 0, 0, 0));
        style.radius = Some(0.);
        if !plain && !enabled.get() {
            style.color = Some(theme.disabled_text);
        }
        style.color = source.color.or(style.color);
        style.font_family = source.font_family.or(style.font_family);
        style.font_size = source.font_size.or(style.font_size);
        style.font_weight = source.font_weight.or(style.font_weight);
        style.line_height = source.line_height.or(style.line_height);
        style.letter_spacing = source.letter_spacing.or(style.letter_spacing);
        style
    });
    EditorFrame {
        classes: format!(
            "{} {}{}",
            common.theme.class(),
            common.size.class(),
            if common.unstyled {
                " sapid-unstyled"
            } else {
                ""
            }
        ),
        style: style.into(),
        focused,
        input: input.into(),
    }
}
impl EditorFrame {
    pub fn mount(
        &self,
        scope: &glory::Scope,
        id: glory::render::SemanticId,
        class: &str,
    ) -> glory::ui::Result<glory::Scope> {
        use glory::ui::Widget;
        let reference = scope.cage(None);
        glory::ui::view()
            .class(format!("{class} {}", self.classes))
            .style(self.style.clone())
            .semantics(Some(Box::new(glory::render::Semantics {
                id: Some(id),
                ..Default::default()
            })))
            .node_ref(reference)
            .show_in(scope)?;
        scope.child_in(reference.get().ok_or(glory::ui::Error::Gone)?)
    }
}
