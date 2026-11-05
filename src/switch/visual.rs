use super::SwitchLabelPosition;
use crate::{Size, Theme};
use glory::render::{
    GraphicPoint as Point, GraphicProps, GraphicStroke, PathCommand as Path, Semantics, StrokeCap,
};
use glory::style::{
    Align, BorderStyle, Display, Edges, FlexDirection, Length, PointerEvents, Position, Style,
};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};

pub(super) struct Visual {
    classes: Vary<String>,
    style: Vary<Style>,
    label: Vary<String>,
    checked: Vary<bool>,
    metadata: Vary<Option<Box<Semantics>>>,
    focused: Cage<bool>,
    theme: Theme,
    size: Size,
    unstyled: bool,
    pub(super) label_position: SwitchLabelPosition,
    pub(super) marks: bool,
}
impl Visual {
    pub(super) fn new(
        scope: &Scope,
        switch: &super::Switch,
        checked: Vary<bool>,
        enabled: Vary<bool>,
        metadata: Vary<Option<Box<Semantics>>>,
        focused: Cage<bool>,
    ) -> Self {
        let common = &switch.common;
        let label = switch.label.clone();
        let label_position = switch.label_position;
        let (classes, declared) =
            common.appearance(scope, "sapid-switch", common.theme.checkbox(common.size));
        let custom = common.style.clone();
        let disabled = common.theme.disabled_text;
        let unstyled = common.unstyled;
        let visible_label = label.clone();
        let style = scope.bond(move || {
            let own = custom.get();
            let mut style = declared.get();
            style.display.get_or_insert(Display::InlineFlex);
            style.position.get_or_insert(Position::Relative);
            style.flex_direction.get_or_insert(FlexDirection::Row);
            style.align.get_or_insert(Align::Center);
            style.gap.get_or_insert(9.);
            if label_position == SwitchLabelPosition::Before
                && !visible_label.with(String::is_empty)
            {
                style.justify.get_or_insert(glory::style::Justify::Between);
            }
            if !unstyled && !enabled.get() {
                if own.color.is_none() {
                    style.color = Some(disabled);
                }
                if own.opacity.is_none() {
                    style.opacity = Some(0.65);
                }
            }
            style
        });
        Self {
            classes,
            style: style.into(),
            label,
            checked,
            metadata,
            focused,
            theme: common.theme.clone(),
            size: common.size,
            unstyled,
            label_position,
            marks: switch.marks,
        }
    }
    pub(super) fn show(self, scope: &Scope, control: glory::ui::Check) -> Result {
        let (width, height) = match self.size {
            Size::ExtraSmall => (26., 14.),
            Size::Small => (30., 16.),
            Size::Medium => (34., 20.),
            Size::Large => (42., 24.),
            Size::ExtraLarge => (50., 28.),
        };
        let checked = self.checked.clone();
        let theme = self.theme.clone();
        let unstyled = self.unstyled;
        let mut track = glory::ui::view()
            .class("sapid-switch-track")
            .semantics(Some(Box::new(Semantics {
                inert: true,
                ..Default::default()
            })))
            .style(scope.bond(move || {
                let mut style = Style::new()
                    .position(Position::Relative)
                    .width(Length::Px(width))
                    .height(Length::Px(height))
                    .radius(height / 2.);
                style.shrink = Some(0.);
                style.background = Some(if checked.get() {
                    if unstyled {
                        theme.foreground
                    } else {
                        theme.accent
                    }
                } else if unstyled {
                    theme.surface
                } else {
                    theme.border
                });
                if unstyled {
                    style.border_width = Some(Edges::all(1.));
                    style.border_style = Some(BorderStyle::Solid);
                    style.border_color = Some(theme.foreground);
                }
                style
            }));
        let checked = self.checked.clone();
        let theme = self.theme.clone();
        track = track.fill(
            glory::ui::view()
                .class("sapid-switch-thumb")
                .style(scope.bond(move || {
                    let mut style = Style::new()
                        .position(Position::Absolute)
                        .left(Length::Px(if checked.get() {
                            width - height + 2.
                        } else {
                            2.
                        }))
                        .top(Length::Px(2.))
                        .width(Length::Px(height - 4.))
                        .height(Length::Px(height - 4.))
                        .radius((height - 4.) / 2.);
                    style.background = Some(if unstyled {
                        if checked.get() {
                            theme.surface
                        } else {
                            theme.foreground
                        }
                    } else {
                        theme.control_thumb
                    });
                    style
                })),
        );
        let states = self.metadata;
        let focused = self.focused;
        let ring = self.theme.focus_ring;
        let danger = self.theme.danger;
        track = track.fill(
            glory::ui::view()
                .class("sapid-switch-focus")
                .style(scope.bond(move || {
                    let invalid =
                        states.with(|states| states.as_ref().is_some_and(|state| state.invalid));
                    Style::new()
                        .position(Position::Absolute)
                        .left(Length::Px(-2.))
                        .top(Length::Px(-2.))
                        .width(Length::Px(width + 4.))
                        .height(Length::Px(height + 4.))
                        .radius((height + 4.) / 2.)
                        .border_width(Edges::all(if focused.get() || invalid { 2. } else { 0. }))
                        .border_style(BorderStyle::Solid)
                        .border_color(if invalid { danger } else { ring })
                        .pointer_events(PointerEvents::None)
                })),
        );
        if self.marks {
            let position = self.checked.clone();
            let color = self.checked.clone();
            let theme = self.theme.clone();
            track = track.fill(
                glory::ui::graphic()
                    .class("sapid-switch-mark")
                    .style(scope.bond(move || {
                        Style::new()
                            .position(Position::Absolute)
                            .left(Length::Px(if position.get() { 4. } else { width - 12. }))
                            .top(Length::Px((height - 8.) / 2.))
                            .width(Length::Px(8.))
                            .height(Length::Px(8.))
                            .pointer_events(PointerEvents::None)
                    }))
                    .drawing(scope.bond(move || {
                        mark(
                            color.get(),
                            if color.get() {
                                if unstyled {
                                    theme.surface
                                } else {
                                    theme.on_accent
                                }
                            } else {
                                theme.foreground
                            },
                        )
                    })),
            );
        }
        let text = self.label.clone();
        let label = glory::ui::view()
            .class("sapid-switch-label")
            .semantics(scope.bond(move || {
                Some(Box::new(Semantics {
                    inert: true,
                    hidden: text.with(String::is_empty),
                    ..Default::default()
                }))
            }))
            .style(Style::new().min_width(Length::Px(0.)))
            .fill(glory::ui::text().content(self.label));
        let mut root = glory::ui::view().class(self.classes).style(self.style);
        root = if self.label_position == SwitchLabelPosition::Before {
            root.fill((label, track))
        } else {
            root.fill((track, label))
        };
        root.fill(control).show_in(scope)
    }
}
fn mark(checked: bool, color: glory::style::Color) -> GraphicProps {
    GraphicProps {
        width: 8.,
        height: 8.,
        commands: if checked {
            vec![
                Path::Move(Point::new(1., 4.)),
                Path::Line(Point::new(3., 6.)),
                Path::Line(Point::new(7., 2.)),
            ]
        } else {
            vec![
                Path::Move(Point::new(1.5, 1.5)),
                Path::Line(Point::new(6.5, 6.5)),
                Path::Move(Point::new(6.5, 1.5)),
                Path::Line(Point::new(1.5, 6.5)),
            ]
        },
        stroke: Some(GraphicStroke {
            color,
            width: 1.5,
            cap: StrokeCap::Round,
        }),
        ..Default::default()
    }
}
