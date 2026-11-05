# Input, TextInput and Textarea

The public Input page documents Sapid's shared input shell. `field(key)` supplies
the label, description, required marker and error relationships;
`text_field()` builds the actual editor. The TextInput page uses that same
`TextField` builder. There is no second `input()` or `text_input()` facade alias.
`textarea()` uses the same value, Field, revision and composition contracts with
a multiline editor.

Use a Field label or an explicit `name` to name an editor. A placeholder is a
hint, and never supplies the accessible name. Labels activate the real editor;
description and error relationships also refer to that editor. Field ordering
is a complete permutation of Label, Description, Control and Error. Required,
readonly, invalid and success are independent declarations. An error takes
precedence over a success border without changing the accepted value.

`value` is a read-only source. `on_value_change` proposes text; declining the
proposal restores the accepted value at the host packet boundary. `default_value`
is local state only without a controlled source. `on_commit` receives final text
without manufacturing an additional change callback. Enter commits a single-line
editor; a multiline Enter remains a newline. Composition stays on the real editor
and ordinary text callbacks retain its composing/revision contract.

`left_section` and `right_section` place child widgets in the input shell.
Section widths must be finite and between 20 and 200 pixels. The editor reserves
their padding, so text does not run under the action. Use a named compact
ActionIcon for an action such as clearing, and a passive Text/Graphic for a
decoration. An application clear action proposes an empty value through the same
application state; it does not reach into DOM state. Loading can occupy either
section and remains separate from readonly and disabled policy. Explicit Style
declarations provide filled, borderless or other visual variants without
introducing different editor behavior.

Textarea `rows` sets a fixed height from the effective line height, padding and
border. `min_rows` and `max_rows` impose ordered positive limits. `autosize(true)`
uses typed `FieldSizing::Content`, including soft wrapping and later width/font
changes on HTML hosts that implement it. Native Apace currently rejects that
declaration explicitly; use fixed rows there. Browser resizing is evidence for
that browser's intrinsic sizing, and does not establish native autosizing. Fixed
rows keep the same editor when its value changes; long content scrolls inside it.

The dedicated primary examples and independent source sections are compiled in
`examples/web/src/dedicated_demos` and `showcase_text_docs`. Source panels include
those complete files, including helpers. The root input/Field regression suite,
actual browser editing and native fixed-row/availability checks cover distinct
parts of this contract.
