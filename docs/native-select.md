# NativeSelect

`native_select(key)` uses the host's native selection control. `select_option`
provides a stable key, display label and optional successful form value;
`select_group` provides an optgroup and its disabled boundary. Options are bounded
static declarations. Key identity is distinct from the form value, so two options
may have equal successful values while remaining different choices.

Single mode uses `Option<String>` with `value`/`on_value_change`; multiple mode
uses `Vec<String>` with `values`/`on_values_change`, `multiple(true)` and optional
`rows`. Mixed cardinality configuration is rejected. Program updates are silent;
callbacks propose selected keys, and a refusal repairs the same native selection.
Disabled options/groups cannot be chosen through user commands. Disabling a Field
or control in the current host packet blocks later selection callbacks.

Field supplies labels, description, required, error and successful-control
relationships to the select. A placeholder is an unselected prompt, not a label.
`readonly(true)` is explicitly rejected: native select has no readonly contract.
Use disabled when selection must be unavailable, with its corresponding focus
behavior. Five theme sizes affect the actual select; platform popup appearance,
keyboard conventions and multiple-selection gestures remain host-owned.

The public page separates native groups/disabled options, single/multiple modes,
controlled/required/disabled states and sizes into four complete compiled source
examples. Custom popup children, matching and filtering belong to Select or
Combobox. Native protocol/layout and browser automation are separate evidence
from actual OS popup input and assistive-technology announcements.
