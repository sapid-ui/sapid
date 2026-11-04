use crate::appearance::{Common, configure};
use glory::render::{DocumentPart, NodeId, SemanticId, Semantics, TextKind};
use glory::style::{
    Align, BorderStyle, Display, Edges, FlexDirection, Justify, Length, Position, Style,
};
use glory::ui::{Children, IntoChildren, Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{collections::HashMap, rc::Rc};

pub const MAX_TIMELINE_ITEMS: usize = 256;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TimelineProgress {
    #[default]
    Pending,
    Active(String),
    Complete,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Pending,
    Active(usize),
    Complete,
}
impl Phase {
    fn state(self, index: usize) -> usize {
        match self {
            Self::Pending => 2,
            Self::Complete => 0,
            Self::Active(active) if index < active => 0,
            Self::Active(active) if index == active => 1,
            _ => 2,
        }
    }
}
macro_rules! appearance_methods {
    () => {
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
pub struct Timeline {
    common: Common,
    key: String,
    items: Vec<TimelineItem>,
    progress: Vary<TimelineProgress>,
    labels: [String; 3],
    level: u8,
    bullet_size: f64,
    line_width: f64,
}
pub fn timeline(key: impl Into<String>) -> Timeline {
    Timeline {
        common: Common::default(),
        key: key.into(),
        items: Vec::new(),
        progress: TimelineProgress::Pending.into(),
        labels: ["Completed".into(), "Current".into(), "Upcoming".into()],
        level: 3,
        bullet_size: 22.,
        line_width: 2.,
    }
}
impl Timeline {
    appearance_methods!();
    pub fn bullet_size(mut self, value: f64) -> Self {
        self.bullet_size = value;
        self
    }
    pub fn line_width(mut self, value: f64) -> Self {
        self.line_width = value;
        self
    }
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.common.theme = theme;
        self
    }
    pub fn size(mut self, size: crate::Size) -> Self {
        self.common.size = size;
        self
    }
    pub fn name(mut self, name: impl Into<Vary<String>>) -> Self {
        self.common.name = Some(name.into());
        self
    }
    pub fn item(mut self, item: TimelineItem) -> Self {
        crate::document::extend_bounded(&mut self.items, std::iter::once(item), MAX_TIMELINE_ITEMS);
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = TimelineItem>) -> Self {
        crate::document::extend_bounded(&mut self.items, items, MAX_TIMELINE_ITEMS);
        self
    }
    pub fn progress(mut self, progress: impl Into<Vary<TimelineProgress>>) -> Self {
        self.progress = progress.into();
        self
    }
    pub fn active(self, key: impl Into<String>) -> Self {
        self.progress(TimelineProgress::Active(key.into()))
    }
    pub fn heading_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
    pub fn state_labels(
        mut self,
        completed: impl Into<String>,
        current: impl Into<String>,
        upcoming: impl Into<String>,
    ) -> Self {
        self.labels = [completed.into(), current.into(), upcoming.into()];
        self
    }
}
fn select(
    progress: &TimelineProgress,
    keys: &HashMap<String, usize>,
) -> std::result::Result<Phase, glory::render::Error> {
    match progress {
        TimelineProgress::Pending => Ok(Phase::Pending),
        TimelineProgress::Complete => Ok(Phase::Complete),
        TimelineProgress::Active(key) => {
            keys.get(key)
                .copied()
                .map(Phase::Active)
                .ok_or(glory::render::Error::InvalidProps(
                    "timeline active key is not mounted",
                ))
        }
    }
}
impl Widget for Timeline {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.items.len() > MAX_TIMELINE_ITEMS
            || !(1..=6).contains(&self.level)
            || !self.bullet_size.is_finite()
            || !(8.0..=64.0).contains(&self.bullet_size)
            || !self.line_width.is_finite()
            || !(0.0..=8.0).contains(&self.line_width)
            || self.line_width > self.bullet_size
            || self
                .labels
                .iter()
                .any(|label| label.trim().is_empty() || label.len() > 256)
        {
            return Err(glory::render::Error::InvalidProps(
                "timeline requires <=256 items, heading 1..=6 and nonempty <=256-byte state labels",
            )
            .into());
        }
        let root_id = crate::composite::id("timeline", &self.key, "root", "root")?;
        let mut keys = HashMap::with_capacity(self.items.len());
        let mut ids = Vec::with_capacity(self.items.len());
        for (index, item) in self.items.iter().enumerate() {
            ids.push(crate::composite::id(
                "timeline", &self.key, "item", &item.key,
            )?);
            if keys.insert(item.key.clone(), index).is_some() {
                return Err(
                    glory::render::Error::InvalidProps("duplicate timeline item key").into(),
                );
            }
        }
        let initial = self
            .progress
            .try_with(|progress| select(progress, &keys))??;
        let phase = scope.cage(initial);
        if self.progress.is_reactive() {
            let source = self.progress.clone();
            let keys = Rc::new(keys);
            let weak = scope.downgrade();
            scope.effect(move || {
                let Some(scope) = weak.upgrade() else { return };
                let result = (|| -> Result {
                    let next = source.try_with(|progress| select(progress, &keys))??;
                    if phase.try_with_untracked(|previous| *previous != next)? {
                        phase.try_revise(|previous| *previous = next)?;
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    scope.report_error(error);
                }
            });
        }
        let count = self.items.len();
        let mut items = Vec::with_capacity(count);
        for (index, (mut item, id)) in self.items.drain(..).zip(ids).enumerate() {
            item.common.theme = item
                .theme_override
                .take()
                .unwrap_or_else(|| self.common.theme.clone());
            item.common.size = item.size_override.unwrap_or(self.common.size);
            item.common.unstyled |= self.common.unstyled;
            items.push(RenderedItem {
                item,
                id,
                index,
                phase,
                labels: self.labels.clone(),
                level: self.level,
                last: index + 1 == count,
                bullet_size: self.bullet_size,
                line_width: self.line_width,
            });
        }
        let mut base = Style::new();
        base.margin = Some(Edges::all(0.0));
        base.padding = Some(Edges::all(0.));
        base.color = Some(self.common.theme.foreground);
        configure!(
            self.common,
            glory::ui::view(),
            scope,
            "sapid-timeline",
            base
        )
        .document_part(DocumentPart::OrderedList)
        .semantics(Some(Box::new(Semantics {
            document_part: Some(DocumentPart::OrderedList),
            id: Some(root_id),
            ..Default::default()
        })))
        .fill(items)
        .show_in(scope)
    }
}
pub struct TimelineItem {
    common: Common,
    theme_override: Option<crate::Theme>,
    size_override: Option<crate::Size>,
    key: String,
    title: Vary<String>,
    description: Option<Vary<String>>,
    timestamp: Option<Vary<String>>,
    marker: Children,
    children: Children,
}
pub fn timeline_item(key: impl Into<String>) -> TimelineItem {
    TimelineItem {
        common: Common::default(),
        theme_override: None,
        size_override: None,
        key: key.into(),
        title: "".into(),
        description: None,
        timestamp: None,
        marker: ().into_children(),
        children: ().into_children(),
    }
}
impl TimelineItem {
    appearance_methods!();
    pub fn theme(mut self, theme: crate::Theme) -> Self {
        self.theme_override = Some(theme);
        self
    }
    pub fn size(mut self, size: crate::Size) -> Self {
        self.size_override = Some(size);
        self
    }
    pub fn title(mut self, title: impl Into<Vary<String>>) -> Self {
        self.title = title.into();
        self
    }
    pub fn description(mut self, description: impl Into<Vary<String>>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<Vary<String>>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn marker(mut self, marker: impl IntoChildren) -> Self {
        self.marker = marker.into_children();
        self
    }
    pub fn fill(mut self, children: impl IntoChildren) -> Self {
        self.children = children.into_children();
        self
    }
}
struct RenderedItem {
    item: TimelineItem,
    id: SemanticId,
    index: usize,
    phase: Cage<Phase>,
    labels: [String; 3],
    level: u8,
    last: bool,
    bullet_size: f64,
    line_width: f64,
}
impl Widget for RenderedItem {
    fn build(&mut self, scope: &Scope) -> Result {
        let phase = self.phase;
        let index = self.index;
        let extra = self.item.common.class.clone();
        self.item.common.class = scope
            .bond(move || {
                format!(
                    "{} sapid-timeline-{}",
                    extra.get(),
                    ["completed", "current", "upcoming"][phase.get().state(index)]
                )
            })
            .into();
        let phase = self.phase;
        let index = self.index;
        let labels = self.labels.clone();
        let status = scope.bond(move || labels[phase.get().state(index)].clone());
        let mut title = glory::ui::text()
            .class("sapid-timeline-title")
            .kind(TextKind::Heading(self.level))
            .content(self.item.title.clone());
        let mut state = glory::ui::text()
            .class("sapid-timeline-state")
            .content(status);
        if !self.item.common.unstyled {
            let mut style = self.item.common.theme.label(self.item.common.size);
            style.margin = Some(Edges::all(0.));
            style.font_weight = Some(600.);
            title = title.style(style);
            let phase = self.phase;
            let theme = self.item.common.theme.clone();
            let size = self.item.common.size;
            state = state.style(scope.bond(move || {
                let current = phase.get().state(index) == 1;
                let mut style = theme.description(size);
                style.color = Some(if current { theme.accent } else { theme.muted });
                style.font_weight = Some(if current { 600.0 } else { 400.0 });
                style
            }));
        }
        let mut body = glory::ui::view()
            .class("sapid-timeline-content")
            .fill((title, state));
        if let Some(timestamp) = self.item.timestamp.take() {
            let mut text = glory::ui::text()
                .class("sapid-timeline-timestamp")
                .content(timestamp);
            if !self.item.common.unstyled {
                let mut style = self.item.common.theme.description(self.item.common.size);
                style.margin = Some(Edges::all(0.));
                text = text.style(style);
            }
            body = body.fill(text);
        }
        if let Some(description) = self.item.description.take() {
            let mut text = glory::ui::text()
                .class("sapid-timeline-description")
                .kind(TextKind::Paragraph)
                .content(description);
            if !self.item.common.unstyled {
                text = text.style(self.item.common.theme.description(self.item.common.size));
            }
            body = body.fill(text);
        }
        body = body.fill(std::mem::replace(
            &mut self.item.children,
            ().into_children(),
        ));
        if !self.item.common.unstyled {
            body = body.style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(4.0),
            );
        }
        let mut base = Style::new();
        base.position = Some(Position::Relative);
        base.display = Some(Display::Flex);
        base.gap = Some(12.);
        base.margin = Some(Edges::all(0.));
        base.padding = Some(Edges {
            top: 0.,
            bottom: if self.last { 0. } else { 24. },
            left: 0.,
            right: 0.,
        });
        let mut marker = glory::ui::view()
            .class("sapid-timeline-marker")
            .fill(std::mem::replace(&mut self.item.marker, ().into_children()));
        let theme = self.item.common.theme.clone();
        let bullet = self.bullet_size;
        if !self.item.common.unstyled {
            marker = marker.style(scope.bond(move || {
                let state = phase.get().state(index);
                Style {
                    shrink: Some(0.),
                    align: Some(Align::Center),
                    justify: Some(Justify::Center),
                    background: Some(if state == 0 {
                        theme.accent
                    } else {
                        theme.surface
                    }),
                    color: Some(if state == 0 {
                        theme.on_accent
                    } else {
                        theme.foreground
                    }),
                    ..Style::new()
                        .position(Position::Relative)
                        .display(Display::InlineFlex)
                        .width(Length::Px(bullet))
                        .height(Length::Px(bullet))
                        .border_width(Edges::all(2.))
                        .border_style(BorderStyle::Solid)
                        .border_color(if state < 2 {
                            theme.accent
                        } else {
                            theme.border
                        })
                        .radius(bullet / 2.)
                }
            }));
        }
        let mut root = configure!(
            self.item.common,
            glory::ui::view(),
            scope,
            "sapid-timeline-item",
            base
        )
        .document_part(DocumentPart::ListItem)
        .semantics(Some(Box::new(Semantics {
            document_part: Some(DocumentPart::ListItem),
            id: Some(self.id.clone()),
            ..Default::default()
        })));
        root = root.fill((marker, body));
        if !self.last && !self.item.common.unstyled && self.line_width > 0. {
            let theme = self.item.common.theme.clone();
            let width = self.line_width;
            root = root.fill(
                glory::ui::view()
                    .class("sapid-timeline-line")
                    .style(scope.bond(move || {
                        Style {
                            background: Some(if phase.get().state(index) == 0 {
                                theme.accent
                            } else {
                                theme.border
                            }),
                            ..Style::new()
                                .position(Position::Absolute)
                                .left(Length::Px((bullet - width) / 2.))
                                .top(Length::Px(bullet))
                                .bottom(Length::Px(0.))
                                .width(Length::Px(width))
                        }
                    })),
            );
        }
        root.show_in(scope)
    }
}
