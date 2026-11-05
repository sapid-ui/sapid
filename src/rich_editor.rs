//! Optional finite rich editing. The document is data, never HTML markup.
use crate::appearance::{Common, common_methods, merge, observe};
use crate::composite::{self, Callback};
use glory::render::{
    self, Action, Event, EventKind, NodeId, Query, QueryToken, QueryValue, RichCommand,
    RichDocument, RichFormat, RichMarks, RichSelection, RichSnapshot, Semantics,
};
use glory::style::{
    BorderStyle, Display, Edges, FlexDirection, FlexWrap, Length, Overflow, PointerEvents,
    Position, Style, WhiteSpace,
};
use glory::ui::{Result, Widget};
use glory::{Cage, Scope, Vary};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub const MAX_EDITOR_HISTORY: usize = 32;
pub const MAX_EDITOR_HISTORY_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_EDITOR_TOOLBAR_ACTIONS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorToolbarAction {
    Bold,
    Italic,
    Code,
    Undo,
    Redo,
    Paragraph,
    Heading(u8),
    Quote,
}
impl EditorToolbarAction {
    fn command(self) -> Option<RichCommand> {
        Some(match self {
            Self::Bold => RichCommand::Bold,
            Self::Italic => RichCommand::Italic,
            Self::Code => RichCommand::Code,
            Self::Undo => RichCommand::Undo,
            Self::Redo => RichCommand::Redo,
            _ => return None,
        })
    }
    fn block(self) -> Option<render::RichBlockKind> {
        Some(match self {
            Self::Paragraph => render::RichBlockKind::Paragraph,
            Self::Heading(level) => render::RichBlockKind::Heading(level),
            Self::Quote => render::RichBlockKind::Quote,
            _ => return None,
        })
    }
    fn name(self) -> String {
        match self {
            Self::Bold => "Bold".into(),
            Self::Italic => "Italic".into(),
            Self::Code => "Code".into(),
            Self::Undo => "Undo".into(),
            Self::Redo => "Redo".into(),
            Self::Paragraph => "Paragraph".into(),
            Self::Heading(2) => "Heading".into(),
            Self::Heading(level) => format!("Heading {level}"),
            Self::Quote => "Quote".into(),
        }
    }
    fn glyph(self) -> &'static str {
        match self {
            Self::Bold => "B",
            Self::Italic => "I",
            Self::Code => "</>",
            Self::Undo => "↶",
            Self::Redo => "↷",
            Self::Paragraph => "¶",
            Self::Heading(1) => "H1",
            Self::Heading(2) => "H2",
            Self::Heading(3) => "H3",
            Self::Heading(4) => "H4",
            Self::Heading(5) => "H5",
            Self::Heading(6) => "H6",
            Self::Heading(_) => "H?",
            Self::Quote => "❞",
        }
    }
    fn pressed(self, state: ToolbarState) -> Option<bool> {
        match self {
            Self::Bold => Some(state.bold),
            Self::Italic => Some(state.italic),
            Self::Code => Some(state.code),
            Self::Undo | Self::Redo => None,
            _ => Some(state.block == self.block()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ToolbarState {
    bold: bool,
    italic: bool,
    code: bool,
    block: Option<render::RichBlockKind>,
}
fn toolbar_state(document: &RichDocument, selection: RichSelection) -> ToolbarState {
    if document.validate_selection(selection).is_err() {
        return ToolbarState::default();
    }
    let (start, end) = selection.ordered();
    let first_kind = document.blocks[start.block].kind;
    let mut uniform = true;
    let (mut found, mut bold, mut italic, mut code) = (false, true, true, true);
    for (index, block) in document
        .blocks
        .iter()
        .enumerate()
        .take(end.block + 1)
        .skip(start.block)
    {
        uniform &= block.kind == first_kind;
        if selection.is_collapsed() {
            continue;
        }
        let first = if index == start.block {
            start.offset
        } else {
            0
        };
        let last = if index == end.block {
            end.offset
        } else {
            block.utf16_len()
        };
        let mut position = 0;
        for run in &block.runs {
            let length = run.text.encode_utf16().count();
            if position < last && position + length > first {
                found = true;
                bold &= run.marks.bold;
                italic &= run.marks.italic;
                code &= run.marks.code;
            }
            position += length;
        }
    }
    ToolbarState {
        bold: found && bold,
        italic: found && italic,
        code: found && code,
        block: uniform.then_some(first_kind),
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EditorHistory {
    pub undo: usize,
    pub redo: usize,
    pub charged_bytes: usize,
}
#[derive(Clone)]
struct Entry {
    document: Rc<RichDocument>,
    selection: RichSelection,
}
#[derive(Default)]
struct History {
    undo: VecDeque<Entry>,
    redo: VecDeque<Entry>,
    bytes: usize,
}
impl History {
    fn summary(&self) -> EditorHistory {
        EditorHistory {
            undo: self.undo.len(),
            redo: self.redo.len(),
            charged_bytes: self.bytes,
        }
    }
    fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
        self.bytes = 0;
    }
    fn charge(entry: &Entry) -> usize {
        entry.document.charged_bytes() + std::mem::size_of::<Entry>()
    }
    fn push(&mut self, entry: Entry, redo: bool) {
        self.bytes += Self::charge(&entry);
        if redo {
            self.redo.push_back(entry)
        } else {
            self.undo.push_back(entry)
        }
        while self.undo.len() + self.redo.len() > MAX_EDITOR_HISTORY
            || self.bytes > MAX_EDITOR_HISTORY_BYTES
        {
            let old = if !self.undo.is_empty() {
                self.undo.pop_front()
            } else {
                self.redo.pop_front()
            };
            let Some(old) = old else { break };
            self.bytes -= Self::charge(&old);
        }
    }
    fn accept(&mut self, previous: Entry, kind: ChangeKind) {
        match kind {
            ChangeKind::Edit => {
                for entry in self.redo.drain(..) {
                    self.bytes -= Self::charge(&entry);
                }
                self.push(previous, false);
            }
            ChangeKind::Undo => {
                if let Some(entry) = self.undo.pop_back() {
                    self.bytes -= Self::charge(&entry);
                }
                self.push(previous, true);
            }
            ChangeKind::Redo => {
                if let Some(entry) = self.redo.pop_back() {
                    self.bytes -= Self::charge(&entry);
                }
                self.push(previous, false);
            }
        }
    }
}
#[derive(Clone, Copy)]
enum ChangeKind {
    Edit,
    Undo,
    Redo,
}
struct State {
    scope: glory::ui::WeakScope,
    node: Cage<Option<NodeId>>,
    owner: glory::core::InstanceId,
    source: Option<Vary<Rc<RichDocument>>>,
    accepted: Cage<Rc<RichDocument>>,
    selection: Cage<RichSelection>,
    accepted_selection: Cage<RichSelection>,
    composing: Cage<bool>,
    history: Cage<EditorHistory>,
    entries: RefCell<History>,
    pending: RefCell<Option<QueryToken>>,
    changed: Callback<Rc<RichDocument>>,
}
impl State {
    fn current(&self) -> Result<(Scope, NodeId)> {
        let scope = self.scope.upgrade().ok_or(glory::ui::Error::Gone)?;
        let node = self.node.try_get()?.ok_or(glory::ui::Error::Gone)?;
        Ok((scope, node))
    }
    fn publish_history(&self) -> Result {
        let summary = self.entries.borrow().summary();
        self.history.try_revise(|value| *value = summary)?;
        Ok(())
    }
    fn cancel_query(&self) -> Result {
        let token = self.pending.borrow_mut().take();
        if let Some(token) = token
            && let Some(scope) = self.scope.upgrade()
        {
            scope.cancel_request(token)?;
        }
        Ok(())
    }
    fn replace_program(&self, document: Rc<RichDocument>) -> Result {
        document.validate()?;
        self.cancel_query()?;
        let (scope, node) = self.current()?;
        scope.set_rich_document(node, document.clone())?;
        self.accepted.try_revise(|value| *value = document)?;
        self.accepted_selection
            .try_revise(|value| *value = RichSelection::default())?;
        self.entries.borrow_mut().clear();
        self.publish_history()
    }
    fn propose(&self, next: Entry, kind: ChangeKind) -> Result<bool> {
        next.document.validate()?;
        next.document.validate_selection(next.selection)?;
        let (scope, node) = self.current()?;
        if !scope.rich_editor_editable(node)? || self.composing.try_get()? {
            return Ok(false);
        }
        let current = self.accepted.try_get()?;
        let previous = Entry {
            document: current.clone(),
            selection: self.accepted_selection.try_get()?,
        };
        // No graph/history/registry borrow spans an application callback.
        let changes_document = current != next.document;
        if self.source.is_some() && changes_document {
            self.changed.emit(self.owner, next.document.clone());
            if !self.owner.is_alive() {
                return Ok(false);
            }
        }
        let accepted = if let Some(source) = &self.source {
            source.try_get()?
        } else {
            next.document.clone()
        };
        accepted.validate()?;
        let accepted_proposal = accepted == next.document;
        // Refusal still allocates a new accepted revision. The host repairs its
        // physical draft even when the controlled source published no mutation.
        scope.set_rich_document(node, accepted.clone())?;
        if accepted_proposal {
            if current != accepted {
                self.entries.borrow_mut().accept(previous, kind);
            }
            self.accepted.try_revise(|value| *value = accepted)?;
            self.selection.try_revise(|value| *value = next.selection)?;
            self.accepted_selection
                .try_revise(|value| *value = next.selection)?;
            self.publish_history()?;
            if scope.capabilities()?.input {
                scope.action(Action::Focus(node))?;
                scope.set_rich_selection(node, next.selection)?;
            }
            if self.source.is_none() && changes_document {
                self.changed.emit(self.owner, next.document);
            }
        } else if accepted != current {
            self.accepted.try_revise(|value| *value = accepted)?;
            self.entries.borrow_mut().clear();
            self.publish_history()?;
        }
        Ok(accepted_proposal)
    }
    fn history_move(&self, redo: bool) -> Result<bool> {
        self.current()?;
        let next = {
            let entries = self.entries.borrow();
            if redo {
                entries.redo.back().cloned()
            } else {
                entries.undo.back().cloned()
            }
        };
        if let Some(next) = next {
            self.propose(
                next,
                if redo {
                    ChangeKind::Redo
                } else {
                    ChangeKind::Undo
                },
            )
        } else {
            Ok(false)
        }
    }
}
/// A weak, Owner-scoped controller. Formatting reads the actual host selection;
/// explicit `_at` methods accept a validated finite program range.
#[derive(Clone)]
pub struct EditorController(Rc<State>);
impl EditorController {
    pub fn document(&self) -> Vary<Rc<RichDocument>> {
        self.0.accepted.into()
    }
    pub fn selection(&self) -> Vary<RichSelection> {
        self.0.selection.into()
    }
    pub fn composing(&self) -> Vary<bool> {
        self.0.composing.into()
    }
    pub fn history(&self) -> Vary<EditorHistory> {
        self.0.history.into()
    }
    pub fn format_at(&self, selection: RichSelection, format: RichFormat) -> Result<bool> {
        let document = self.0.accepted.try_get()?;
        let document = Rc::new(document.format(selection, format)?);
        self.0.propose(
            Entry {
                document,
                selection,
            },
            ChangeKind::Edit,
        )
    }
    pub fn insert_at(
        &self,
        selection: RichSelection,
        text: &str,
        marks: RichMarks,
    ) -> Result<bool> {
        let current = self.0.accepted.try_get()?;
        let (document, selection) = current.replace(selection, text, marks)?;
        self.0.propose(
            Entry {
                document: Rc::new(document),
                selection,
            },
            ChangeKind::Edit,
        )
    }
    pub fn undo(&self) -> Result<bool> {
        self.0.history_move(false)
    }
    pub fn redo(&self) -> Result<bool> {
        self.0.history_move(true)
    }
    /// Silent program replacement; it clears history and cancels pending commands.
    /// In controlled mode, replace the source instead of bypassing it.
    pub fn reset(&self, document: Rc<RichDocument>) -> Result {
        if self.0.source.is_some() {
            return Err(render::Error::Unsupported(
                "controlled editor reset requires updating its source",
            )
            .into());
        }
        self.0.replace_program(document)
    }
    pub fn format(&self, format: RichFormat) -> Result<QueryToken> {
        format.validate()?;
        self.query(move |snapshot| {
            Ok((
                Rc::new(snapshot.document.format(snapshot.selection, format)?),
                snapshot.selection,
            ))
        })
    }
    pub fn insert(&self, text: impl Into<String>, marks: RichMarks) -> Result<QueryToken> {
        let text = text.into();
        if text.len() > render::MAX_RICH_TEXT_BYTES {
            return Err(render::Error::InvalidProps("rich insertion exceeds text budget").into());
        }
        marks.validate()?;
        self.query(move |snapshot| {
            let (document, selection) =
                snapshot
                    .document
                    .replace(snapshot.selection, &text, marks)?;
            Ok((Rc::new(document), selection))
        })
    }
    fn query(
        &self,
        operation: impl FnOnce(RichSnapshot) -> Result<(Rc<RichDocument>, RichSelection)> + 'static,
    ) -> Result<QueryToken> {
        self.0.cancel_query()?;
        let (scope, node) = self.0.current()?;
        if !scope.rich_editor_editable(node)? || self.0.composing.try_get()? {
            return Err(render::Error::Unsupported("editor is not editable").into());
        }
        let state = self.0.clone();
        let token = scope.request(Query::RichSelection(node), move |reply| {
            state.pending.borrow_mut().take();
            let result = (|| -> Result {
                let QueryValue::RichSelection(snapshot) = reply? else {
                    return Err(
                        render::Error::InvalidResponse("rich selection response kind").into(),
                    );
                };
                let (scope, current) = state.current()?;
                if current != node
                    || snapshot.composing
                    || scope.rich_editor_value(node)?.1 != snapshot.revision
                {
                    return Ok(());
                }
                let (document, selection) = operation(*snapshot)?;
                state.propose(
                    Entry {
                        document,
                        selection,
                    },
                    ChangeKind::Edit,
                )?;
                Ok(())
            })();
            if let Err(error) = result
                && let Some(scope) = state.scope.upgrade()
            {
                scope.report_error(error);
            }
        })?;
        *self.0.pending.borrow_mut() = Some(token);
        Ok(token)
    }
    fn toggle(&self, command: RichCommand) -> Result {
        match command {
            RichCommand::Undo => {
                self.undo()?;
            }
            RichCommand::Redo => {
                self.redo()?;
            }
            _ => {
                self.query(move |snapshot| {
                    let flags = toolbar_state(&snapshot.document, snapshot.selection);
                    let value = !match command {
                        RichCommand::Bold => flags.bold,
                        RichCommand::Italic => flags.italic,
                        _ => flags.code,
                    };
                    let format = match command {
                        RichCommand::Bold => RichFormat::Bold(value),
                        RichCommand::Italic => RichFormat::Italic(value),
                        _ => RichFormat::Code(value),
                    };
                    Ok((
                        Rc::new(snapshot.document.format(snapshot.selection, format)?),
                        snapshot.selection,
                    ))
                })?;
            }
        }
        Ok(())
    }
}
pub struct RichTextEditor {
    common: Common,
    source: Option<Vary<Rc<RichDocument>>>,
    initial: Rc<RichDocument>,
    readonly: Vary<bool>,
    toolbar: bool,
    toolbar_actions: Vec<EditorToolbarAction>,
    subtle: bool,
    placeholder: Vary<String>,
    controller_ref: Option<Cage<Option<EditorController>>>,
    changed: Option<Box<dyn FnMut(Rc<RichDocument>)>>,
}
pub fn rich_text_editor() -> RichTextEditor {
    RichTextEditor {
        common: Common {
            name: Some("Rich text editor".into()),
            focus: render::FocusMode::TabStop,
            ..Default::default()
        },
        source: None,
        initial: Rc::new(RichDocument::default()),
        readonly: false.into(),
        toolbar: true,
        toolbar_actions: vec![
            EditorToolbarAction::Bold,
            EditorToolbarAction::Italic,
            EditorToolbarAction::Code,
            EditorToolbarAction::Undo,
            EditorToolbarAction::Redo,
            EditorToolbarAction::Paragraph,
            EditorToolbarAction::Heading(2),
            EditorToolbarAction::Quote,
        ],
        subtle: false,
        placeholder: String::new().into(),
        controller_ref: None,
        changed: None,
    }
}
impl RichTextEditor {
    common_methods!();
    pub fn document(mut self, source: impl Into<Vary<Rc<RichDocument>>>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn default_document(mut self, value: Rc<RichDocument>) -> Self {
        self.initial = value;
        self
    }
    pub fn readonly(mut self, value: impl Into<Vary<bool>>) -> Self {
        self.readonly = value.into();
        self
    }
    pub fn toolbar(mut self, value: bool) -> Self {
        self.toolbar = value;
        self
    }
    /// A static bounded command declaration; heading levels are 1–6.
    pub fn toolbar_actions(
        mut self,
        actions: impl IntoIterator<Item = EditorToolbarAction>,
    ) -> Self {
        self.toolbar_actions = actions
            .into_iter()
            .take(MAX_EDITOR_TOOLBAR_ACTIONS + 1)
            .collect();
        self
    }
    pub fn subtle(mut self, value: bool) -> Self {
        self.subtle = value;
        self
    }
    pub fn placeholder(mut self, value: impl Into<Vary<String>>) -> Self {
        self.placeholder = value.into();
        self
    }
    pub fn controller_ref(mut self, value: Cage<Option<EditorController>>) -> Self {
        self.controller_ref = Some(value);
        self
    }
    pub fn on_document_change(mut self, callback: impl FnMut(Rc<RichDocument>) + 'static) -> Self {
        self.changed = Some(Box::new(callback));
        self
    }
}
impl Widget for RichTextEditor {
    fn build(&mut self, scope: &Scope) -> Result {
        if self.toolbar_actions.len() > MAX_EDITOR_TOOLBAR_ACTIONS
            || self.toolbar && self.toolbar_actions.is_empty()
            || self
                .toolbar_actions
                .iter()
                .enumerate()
                .any(|(index, action)| {
                    matches!(action,EditorToolbarAction::Heading(level) if !(1..=6).contains(level))
                        || self.toolbar_actions[..index].contains(action)
                })
        {
            return Err(render::Error::InvalidProps(
                "rich toolbar requires unique finite actions and heading levels 1–6",
            )
            .into());
        }
        let initial = if let Some(source) = &self.source {
            source.try_get()?
        } else {
            self.initial.clone()
        };
        initial.validate()?;
        let placeholder = composite::validated(scope, &self.placeholder, |value| {
            render::validate_input_placeholder(value, true).map_err(Into::into)
        })?;
        let reference = self.common.node_ref.unwrap_or_else(|| scope.cage(None));
        let state = Rc::new(State {
            scope: scope.downgrade(),
            node: reference,
            owner: scope.id(),
            source: self.source.clone(),
            accepted: scope.cage(initial.clone()),
            selection: scope.cage(RichSelection::default()),
            accepted_selection: scope.cage(RichSelection::default()),
            composing: scope.cage(false),
            history: scope.cage(EditorHistory::default()),
            entries: RefCell::new(History::default()),
            pending: RefCell::new(None),
            changed: Callback::new(self.changed.take()),
        });
        let controller = EditorController(state.clone());
        let output = self.controller_ref;
        let cleaned = state.clone();
        scope.on_cleanup(move || {
            cleaned.changed.clear();
            cleaned.pending.borrow_mut().take();
            cleaned.entries.borrow_mut().clear();
            if let Some(output) = output {
                let _ = output.try_revise(|value| *value = None);
            }
        })?;
        let parent = scope.cage(None);
        glory::ui::view()
            .class(if self.subtle {
                "sapid-rich-editor-shell sapid-rich-editor-shell-subtle"
            } else {
                "sapid-rich-editor-shell"
            })
            .style(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .gap(8.),
            )
            .node_ref(parent)
            .show_in(scope)?;
        let parent = parent.try_get()?.ok_or(glory::ui::Error::Gone)?;
        let content = scope.child_in(parent)?;
        if self.toolbar {
            let history = state.history;
            let composing = state.composing;
            let enabled = self.common.enabled.clone();
            let readonly = self.readonly.clone();
            let available =
                scope.bond(move || enabled.get() && !readonly.get() && !composing.get());
            let accepted = state.accepted;
            let selection = state.accepted_selection;
            let formatting = scope
                .bond(move || accepted.with(|document| toolbar_state(document, selection.get())));
            let mut toolbar_style = Style::new()
                .flex_wrap(FlexWrap::Wrap)
                .gap(2.)
                .min_width(Length::Px(0.));
            if !self.common.unstyled {
                toolbar_style = toolbar_style.padding(Edges::all(4.));
                if !self.subtle {
                    toolbar_style = toolbar_style
                        .border_style(BorderStyle::Solid)
                        .border_color(self.common.theme.border)
                        .border_width(Edges {
                            top: 0.,
                            bottom: 1.,
                            left: 0.,
                            right: 0.,
                        });
                }
            }
            let mut toolbar = crate::button_group()
                .toolbar(true)
                .joined(false)
                .class("sapid-rich-toolbar")
                .name("Rich text formatting")
                .theme(self.common.theme.clone())
                .size(self.common.size)
                .style(toolbar_style);
            if self.common.unstyled {
                toolbar = toolbar.unstyled();
            }
            for action in self.toolbar_actions.iter().copied() {
                let current = controller.clone();
                let weak = scope.downgrade();
                let active = scope.bond(move || {
                    available.get()
                        && match action {
                            EditorToolbarAction::Undo => history.get().undo > 0,
                            EditorToolbarAction::Redo => history.get().redo > 0,
                            _ => true,
                        }
                });
                let mut button = crate::button()
                    .label(action.glyph())
                    .name(action.name())
                    .theme(self.common.theme.clone())
                    .variant(crate::ButtonVariant::Subtle)
                    .compact(true)
                    .style(Style {
                        shrink: Some(0.),
                        font_weight: Some(if action == EditorToolbarAction::Bold {
                            700.
                        } else {
                            400.
                        }),
                        ..Style::new()
                            .min_width(Length::Px(28.))
                            .white_space(WhiteSpace::NoWrap)
                            .padding(Edges {
                                top: 2.,
                                bottom: 2.,
                                left: 6.,
                                right: 6.,
                            })
                    })
                    .enabled(active)
                    .on_press(move || {
                        let result = if let Some(command) = action.command() {
                            current.toggle(command)
                        } else {
                            current
                                .format(RichFormat::Block(
                                    action.block().expect("validated block action"),
                                ))
                                .map(|_| ())
                        };
                        if let Err(error) = result
                            && let Some(scope) = weak.upgrade()
                        {
                            scope.report_error(error);
                        }
                    });
                if action.pressed(ToolbarState::default()).is_some() {
                    button = button.pressed(scope.bond(move || {
                        action.pressed(formatting.get()).expect("formatting action")
                    }));
                }
                if self.common.unstyled {
                    button = button.unstyled();
                }
                toolbar = toolbar.button(button);
            }
            toolbar.show_in(&content)?;
        }
        let editor_parent = scope.cage(None);
        glory::ui::view()
            .class("sapid-rich-editor-content")
            .style(Style::new().position(Position::Relative))
            .node_ref(editor_parent)
            .show_in(&content)?;
        let editor_scope = scope.child_in(editor_parent.get().ok_or(glory::ui::Error::Gone)?)?;
        let accepted = state.accepted;
        let visible_placeholder = scope.bond(move || {
            !placeholder.get().is_empty()
                && accepted.with(|document| document.plain_text().is_empty())
        });
        let placeholder_text = placeholder;
        let placeholder_color = self.common.theme.muted;
        glory::ui::when(visible_placeholder, move || {
            glory::ui::text()
                .class("sapid-rich-editor-placeholder")
                .content(placeholder_text)
                .style(Style {
                    color: Some(placeholder_color),
                    ..Style::new()
                        .position(Position::Absolute)
                        .left(Length::Px(12.))
                        .top(Length::Px(10.))
                        .pointer_events(PointerEvents::None)
                })
        })
        .show_in(&editor_scope)?;
        let mut base = self.common.theme.control(self.common.size);
        base.height = Some(Length::Auto);
        let (classes, custom) = self.common.appearance(
            scope,
            if self.subtle {
                "sapid-rich-text-editor sapid-rich-text-editor-subtle"
            } else {
                "sapid-rich-text-editor"
            },
            base,
        );
        let geometry = Style::new()
            .width(Length::Percent(100.))
            .min_width(Length::Px(0.))
            .min_height(Length::Px(120.))
            .overflow_x(Overflow::Auto)
            .overflow_y(Overflow::Auto)
            .white_space(WhiteSpace::PreWrap);
        let style = scope.bond(move || merge(geometry.clone(), custom.get()));
        let readonly = self.readonly.clone();
        let semantics = scope.bond(move || {
            Some(Box::new(Semantics {
                readonly: readonly.get(),
                ..Default::default()
            }))
        });
        let mut editor = glory::ui::rich_editor()
            .document(initial)
            .class(classes)
            .style(style)
            .semantics(semantics)
            .enabled(self.common.enabled.clone())
            .focus(self.common.focus)
            .node_ref(reference);
        if let Some(name) = self.common.name.take() {
            editor = editor.name(name);
        }
        let mut generic = self
            .common
            .handlers
            .remove(&EventKind::RichEdit)
            .unwrap_or_default();
        let changed = state.clone();
        editor = editor.on(EventKind::RichEdit, move |event| {
            let result = (|| -> Result {
                if let Event::RichEdit(snapshot) = &event {
                    changed
                        .composing
                        .try_revise(|value| *value = snapshot.composing)?;
                    changed
                        .selection
                        .try_revise(|value| *value = snapshot.selection)?;
                    if !snapshot.composing {
                        changed.propose(
                            Entry {
                                document: snapshot.document.clone(),
                                selection: snapshot.selection,
                            },
                            ChangeKind::Edit,
                        )?;
                    }
                }
                Ok(())
            })();
            if let Err(error) = result {
                if let Some(scope) = changed.scope.upgrade() {
                    scope.report_error(error);
                }
                return;
            }
            observe(&mut generic, changed.owner, event);
        });
        let mut generic = self
            .common
            .handlers
            .remove(&EventKind::RichSelection)
            .unwrap_or_default();
        let selected = state.clone();
        editor = editor.on(EventKind::RichSelection, move |event| {
            if let Event::RichSelection(snapshot) = &event {
                let result = (|| -> Result {
                    selected
                        .selection
                        .try_revise(|value| *value = snapshot.selection)?;
                    if snapshot.document == selected.accepted.try_get()? && !snapshot.composing {
                        selected
                            .accepted_selection
                            .try_revise(|value| *value = snapshot.selection)?;
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    if let Some(scope) = selected.scope.upgrade() {
                        scope.report_error(error);
                    }
                    return;
                }
            }
            observe(&mut generic, selected.owner, event);
        });
        let mut generic = self
            .common
            .handlers
            .remove(&EventKind::Composition)
            .unwrap_or_default();
        let composing = state.clone();
        editor = editor.on(EventKind::Composition, move |event| {
            if let Event::Composition { active } = event
                && let Err(error) = composing.composing.try_revise(|value| *value = active)
            {
                if let Some(scope) = composing.scope.upgrade() {
                    scope.report_error(error);
                }
                return;
            }
            observe(&mut generic, composing.owner, event);
        });
        let mut generic = self
            .common
            .handlers
            .remove(&EventKind::RichCommand)
            .unwrap_or_default();
        let command = controller.clone();
        editor = editor.on(EventKind::RichCommand, move |event| {
            if let Event::RichCommand { command: kind, .. } = event
                && let Err(error) = command.toggle(kind)
            {
                if let Some(scope) = command.0.scope.upgrade() {
                    scope.report_error(error);
                }
                return;
            }
            observe(&mut generic, command.0.owner, event);
        });
        for (kind, mut generic) in std::mem::take(&mut self.common.handlers) {
            let owner = scope.id();
            editor = editor.on(kind, move |event| observe(&mut generic, owner, event));
        }
        editor.show_in(&editor_scope)?;
        if let Some(source) = self.source.clone() {
            let current = state.clone();
            scope.structural(move || {
                let result = (|| -> Result {
                    let next = source.try_get()?;
                    next.validate()?;
                    if next != glory::core::untracked(|| current.accepted.try_get())? {
                        current.replace_program(next)?;
                    }
                    Ok(())
                })();
                if let Err(error) = result
                    && let Some(scope) = current.scope.upgrade()
                {
                    scope.report_error(error);
                }
            });
        }
        if let Some(output) = output {
            output.try_revise(|value| *value = Some(controller))?;
        }
        scope.retain(editor_scope)?;
        scope.retain(content)?;
        scope.retain(state)?;
        Ok(())
    }
}

impl Default for RichTextEditor {
    fn default() -> Self {
        rich_text_editor()
    }
}
