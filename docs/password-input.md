# PasswordInput

`password_input()` is one native input and an inset visibility icon button.
The eye uses a passive vector drawing; Show password / Hide password remain
the button's accessible names. `visibility_labels(show, hide)` localizes both.
Use Field for the input's label, description, required/error state and form name.

`value` and `visible` accept independent read-only Cage/Bond/Vary sources.
`on_value_change` and `on_visibility_change` propose changes; the caller decides
whether to accept them. `default_value` and `default_visible` configure local
state. Revealing changes the purpose of the same editor, retaining its value,
edit revision, selection and active composition. It does not copy the password
into a second text control. `node_ref` refers to that actual editor.

`left_section` preserves the default eye. `right_section` replaces it, so custom
visibility controls own their callback and state handling. Reserve space with
`left_section_width` / `right_section_width`; dimensions are applied to the frame
once, including percentage widths. A right-side loading indicator hides the eye
while it occupies that slot. Left-side loading leaves the eye available. Loading
is presentation and does not perform validation or submission.

Read-only prevents text editing and permits visibility changes. Disabled blocks
both editing and the eye. Both the field and local enabled state participate in
availability. All five sizes scale the inset drawing and hit target; class/style
customizations compose with the theme. `unstyled` opts out of themed paint and
control dimensions, while retaining the real editor and section structure.

Browser and Apace use the same purpose/controlled-input protocol. Masked native
inputs retain the protected role and omit their accessibility value. Revealing
intentionally changes the control to ordinary text. Native engine/AX evidence is
separate from OS IME, selection gesture and reader acceptance.
