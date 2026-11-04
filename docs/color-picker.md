# ColorPicker and ColorInput

`color_picker(key)` edits one finite `Hsva` value. A layered continuous saturation /
brightness gradient plane supports immediate mouse and touch capture, including release
outside the plane. Optional Saturation and Brightness precision controls are genuine keyboard Sliders;
Hue and optional Opacity reuse the existing color sliders. The plane itself has
no invented two-dimensional accessibility role or extra Tab stop. Pressing it
preserves an adjacent editor's actual focus and selection. RTL mirrors its
saturation axis; brightness still increases upwards.

```rust
use sapid::{color_picker, color_input, Hsva};
color_picker("accent").default_value(Hsva {
    hue: 210., saturation: 0.6, value: 0.8, alpha: 0.5,
});
color_input("accent-hex").default_value("#6496c880");
```

Picker `.value(...)` is a read-only controlled source. `.on_value_change` and
`.on_value_commit` propose complete `Hsva` values; local defaults accept changes.
Refusal leaves the source and rendered channels intact. Same-tick accepted edits
read the live source, preserving changes to the other channels. Program writes
are silent; invalid/nonfinite values preserve the accepted tree and return an
error until a later valid value recovers. Coordinates retain f64 precision;
RGBA8 conversion quantizes only visual previews. Hue zero and 360 remain distinct
numeric endpoints, and achromatic values retain the supplied hue.

The standalone picker accepts up to 64 hexadecimal `.swatches(...)`, arranged
with `.swatches_per_row(1..=16)`. Each swatch is a genuine named action and
publishes one complete Hsva proposal followed by one commit. `.show_picker(false)`
creates a finite swatches-only picker and therefore requires at least one swatch;
`.full_width(true)` stretches its root to the available width. These modes are
shown in three dedicated showcase sections together with the optional keyboard
precision controls. `.saturation_label(...)`, `.brightness_label(...)`,
`.hue_label(...)` and `.alpha_label(...)` configure the accessible names on the
four real Slider controls without changing their visible geometry.
`.form_name(...)` adds one real hidden successful Input whose value is formatted
with `.form_format(...)`; the picker's original Hsva source is never replaced by
that quantized string representation.

The plane contains three typed layers: the current hue surface, a continuous
white-to-transparent saturation gradient and a continuous transparent-to-black
brightness gradient. Its side is finite in 128..=512 pixels, default 240. This
is an sRGB HSV surface; it does not claim a perceptual color space or gamut
conversion. One pending geometry query and one current
start/latest pointer batch belong to the live gesture. Cancellation, disabling
and Owner retirement revoke pending work. No visited-color cache is retained.
`unstyled()` preserves physical geometry, color content and keyboard alternatives.

`color_input(key)` has one String source and aligns its finite editor contract
with Mantine's hex, hexa, rgb, rgba, hsl and hsla formats. It preserves incomplete
and invalid drafts while editing and links a parse error to the real single-line
TextField. By default `.fix_on_blur(true)` restores the last valid value on commit;
it can be disabled explicitly. Empty text stays empty. `.on_validation` reports
Empty, Valid(RGBA8) or Invalid(ColorError), while
business changes remain proposals through `.on_value_change`. Field naming,
descriptions, required/readonly/disabled state and successful-control name belong
to the actual editor. A Field's own error takes precedence over the parse message.

The input has an enabled color preview by default. `.disallow_input(true)` makes
the editor readonly without disabling picker or swatch proposals. Up to 64
hexadecimal `.swatches(...)` are arranged with `.swatches_per_row(1..=16)`;
`.show_picker(false)` yields a swatches-only popup when swatches exist, and
`.close_on_color_swatch_click(true)` closes after selection. The focused showcase
uses the ordinary hex configuration and follows it with sixteen independently
addressable controlled, loading, format, draft, commit, free-input, swatch,
dropdown, sampler, section and state examples. `.loading(...)` places a real
finite Spinner on either `ColorInputSectionPosition`; custom `.left_section(...)`
and `.right_section(...)` content replaces the corresponding default adornment.
`.success(...)` applies the theme success border to the real editor and yields to
invalid/error styling.

`.eye_dropper(ColorEyeDropper)` adds a compact named sampling action. The service
is supplied explicitly by the application and returns an asynchronous optional
color; Sapid never reaches for an ambient browser or operating-system API.
Successful samples follow the same accepted change and commit path as picker
interactions, cancellation is silent, and service failures are available through
`.on_eye_dropper_error(...)`. `.eye_dropper_label(...)` and
`.eye_dropper_icon(...)` keep the accessible name independent from visible icon
content. Owner retirement cancels the local task and ignores late completion.

Focusing the editor opens the shared Popover anchored to that same input; there
is no separate trigger button. The layer retains the established dismissal and
focus-restoration rules. Palette changes emit the selected `ColorFormat` and
intentionally quantize channels to RGBA8. An invalid draft leaves the
palette at its last valid color. Active composition or readonly/disabled state
blocks palette changes. Controlled `.open` uses `.on_open_change` proposals.

Native Apace supports the standalone picker, its real Slider alternatives and
ColorInput's default anchored Popup through the finite Dialog/Popover layer
facet. Four-theme native tests verify actual anchor geometry, palette changes,
CPU pixels, retained editor identity and closing. Explicit `.show_picker(false)`
continues to select the hex-editor-only mode and rejects simultaneous popup
configuration. The standalone ColorPicker form proxy is an ordinary finite Input
and therefore participates in host form snapshots; ColorInput submits the real
editor value through Field. An actual native screen sampler remains an explicit
application service and is not inferred from the component API.

Keys are stable ASCII, at most 48 bytes for Picker and 46 for Input, reserving
space for internal channel/popup namespaces. Five sizes affect the shared channel
controls; plane geometry is configured independently with `.plane_size(...)`.
Names and class/style overrides preserve the established component conventions.

Evidence: sixteen component regressions pass, including controlled refusal and
commit-only acceptance, exact cross-channel updates, invalid recovery, linked
errors, Field state, composition, standalone swatches-only selection/full-width
layout, formatted form proxy, explicit sampler service, complete popup child
retention, configurable channel labels and Owner cleanup. The focused showcase
browser suite adds ten checks covering all sixteen ColorInput sections, all four
ColorPicker sections, real palette actions, close-on-swatch, sampler results,
control state and the submitted form value. Native tests cover four-theme
actual plane pixels, two-axis native bounds/mapping, channel updates, stable
engine identities, the real hex editor and explicit top-layer refusal. Shared
DragCapture has two render regressions, an HTML full-shape differential oracle,
a native queued Mouse/Finger focus/capture/cancel test and a classic stream
Shell/Patch/End test. The fixed Web19 artifact passed eleven actual Chromium 148.0.7778.96 scenarios,
including four themes in CSR/hydration, two-axis mouse capture, controlled
refusal, readonly/disabled and composition policy, actual AX roles, dirty editor
adoption, CDP touch/cancellation and classic stream final metadata. The report is
`D:/BuildCache/sapid/color-picker-browser-web19.json`. Visual review then found
that a 128-pixel plane retained 240-pixel channel widths and that the indicator
needed centering. Both repairs passed strengthened actual native bounds checks and all eleven
browser scenarios against the fixed `command-next` WASM/SSR artifact; see
`D:/BuildCache/sapid/color-picker-geometry-native20.log` and
`D:/BuildCache/sapid/color-picker-browser-geometry20.json`. The initially omitted
DragCapture classic support page was generated with its Rust v5 producer before
the final complete replay; no assertion or production behavior was removed.
The final green/dark hydration screenshot was visually reviewed: 128-pixel
channels now fit their plane, and the indicator centers on the actual value.
Browser protocol and AX-tree inspection do not establish OS IME or screen-reader
announcements.

The native consumer migration passed all seven alerts/color tests in
`D:/BuildCache/sapid/native-new-layer-consumers-final.log`. It does not extend
native support to Menu/Tooltip/Listbox, Sheet or ContextPoint layer facets.
