# Checkbox, Radio, Chip, Switch and Fieldset

These controls keep native check/radio editors and their real label activation.
Use `Checkbox` for one boolean, `RadioGroup` for one key, and `ChipGroup` when the
same choice should have a compact pill presentation. Use `Button` for an action.

`checked`, `value`, `values` and `selected` are read-only controlled sources.
Callbacks propose replacements; declining them restores the accepted native
state. Local defaults apply only without a source. Checkbox, MixedCheckbox,
Switch and Chip also recheck current enabled/Field readonly state during a
host packet, before callbacks or generic observers. Program writes remain
application-owned. Owners retire handlers and input references with the widget.

`CheckboxGroup` uses explicit stable item keys, up to 1024 mounted members.
Its select-all state is derived from enabled members: none, all, or mixed.
Disabled members retain their prior selection and are excluded from select-all
changes. `MixedCheckbox` accepts `CheckState::Mixed`; activation proposes a binary
checked state. Web and native Apace keep a distinct indeterminate property and
paint the dash. Ordinary checkbox Space and label activation use that same input.

`Radio` belongs to a `RadioGroup`, with a separate key for every member.
Orientation controls the layout and directional navigation. Navigation skips
unavailable members; normal Tab uses the current group's roving stop. Repeated
activation of the selected radio keeps it selected. An empty controlled key
clears an optional selection. Required and form-name configuration belong to
the group; accessible names remain separate from successful-control names.

`ChipGroup::Single` uses the scalar radio contract, while `Multiple` uses the
vector checkbox contract. Multiple activation removes an already selected key,
including its final value. Single selection is cleared through an explicit
application action. Read-only preserves focus and selection; disabled prevents
normal activation. Long pill labels wrap within the parent width.

`Field` supplies linked labels, descriptions, required state and error messages.
A horizontal Field places a linked label before its checkbox; the ordinary
checkbox label follows its input. Styling and ordering must preserve the actual
control relation. A validation message does not itself change selection.

`Fieldset` owns a real legend and descendant disabled boundary. Disabling an
outer fieldset also blocks nested controls; an enabled descendant cannot opt
itself back in. Keep re-enable actions outside the boundary. Disabling preserves
the same editor, draft and selection. Explicit styles change padding, gap and
borders; a borderless fieldset still retains its group/legend semantics.

The public pages contain four dedicated primary examples and ten independently
compiled source sections. Sources live in
`examples/web/src/showcase_choice_docs`; the browser and native acceptance
fixtures exercise those same examples. Native protocol, identity and layout
checks do not establish OS input or reader announcements.

`Switch` retains one real native switch input beneath its passive track, thumb,
caption and optional marks. Its `node_ref` refers to that input. The
`sapid-switch` root receives explicit class/style overrides once; the internal
input has class `sapid-switch-control`. Five track sizes range from 26×14 to
50×28 pixels. Checkedness moves the thumb inside the same track; focus and Field
invalid state add an outline without changing its geometry. Theme control-thumb
contrast applies in both light and dark modes.

`label_position(SwitchLabelPosition::Before)` places the caption before the
track; `After` is the default. Caption and track clicks activate the same input,
and Space uses its native checked command. `with_marks(true)` adds a passive
check/cross inside the track. A linked Field supplies the accessible name when
present; otherwise the switch caption supplies it. Decorative captions are
inert and never add duplicate controls. Empty reactive captions release their
layout space without replacing the input. Four independently compiled sections
demonstrate sizes, label/validation relationships, controlled refusal and a
disabled Fieldset containing independent boolean switches. These APIs do not
provide a second string-valued SwitchGroup or arbitrary interactive track children.

`SegmentedControl::full_width` accepts a local boolean or reactive source.
Horizontal full-width segments share the available row equally; the default
uses content widths. Vertical full-width segments fill the same parent width.
Changing the mode preserves the actual radio editors, their selection and focus.
Explicit segment styles retain precedence. The independent width/orientation/
long-label/controlled examples live beside the other choice documentation.
`Rating` has separate half/clear, display/read-only/size and static custom-symbol
examples; see `docs/rating.md` for the finite drawing and precision boundaries.
