# Overlay documentation pages

The 11 shared overlay pages now use separate, complete Rust primary examples
and 43 independently compiled detail modules in
`examples/web/src/showcase_overlay_docs`. Existing centered/full-screen Modal
section IDs remain stable. Each section's source contains its helpers and is
compiled with its actual preview; the table of contents matches these sections.

## Modal, Drawer and Popover

Modal edits a real profile and saves the accepted value. Drawer edits a retained
workspace draft. Popover applies actual notification preferences. All start
closed. Shared layers own focus containment where modal, Escape, outside routing,
collision handling and return focus. They do not install component browser
listeners. Controlled `open` is read-only and callbacks propose changes; examples
show refusal without replacing the panel or editor.

Default ordinary layer headers put the heading and a 24 px icon-only close
control in one flex row. The 16 px vector has a localized reactive `close_label`.
Long headings shrink/wrap beside the close control. AlertDialog retains its
visible named Cancel action and safe initial focus. Custom `close_content`
remains supported. Panel body and hint text inherit theme font metrics.
Default managed focus chooses the first enabled control; use `initial_focus`
with a semantic ID when a particular control should receive it.

Explicit Style width controls panel dimensions, while `size` controls theme
metrics. Full-screen Modal retains the same host behavior. Local scrolling-body
examples keep the heading/close outside a fixed-height overflow box. Nested
Modal examples demonstrate the innermost focus and Escape scope. Drawer exposes
four physical edges; native edge-layer/sheet gestures remain explicitly
unsupported and are not inferred from ordinary Dialog/Popover acceptance.

Popover demonstrates requested sides/alignment, gap, collision-aware arrows,
Auto/Px/Target width, outside-close opt-out, nested layers and retained notes.
A requested side can flip at a viewport boundary. Target width follows its live
trigger border box. Long content uses a bounded local scroll box.

## Nonmodal Dialog

The Dialog page uses `floating_dialog`, preserving background focus and normal
Tab. It defaults closed, with outside dismissal off. New
`position(ViewportPlacement)` supplies finite viewport x/y, padding and constraint
policy through the shared host. Position is bounded to +/-1,000,000 CSS px and
padding to 0..=4096. The example does not claim movable/resizable window services.
Retained/recreated content examples make their lifecycle difference observable.

## Local shades and loading

Overlay is decorative and pointer-transparent. Its examples show actual
underlying content, alpha composition and operable shaded controls. A fixed
viewport example is explicit application composition and starts closed; it does
not imply a modal focus scope or scrolling lock. Backdrop blur and CSS filters
are outside the finite typed API.

LoadingOverlay coordinates busy, content inert and mask visibility from the same
source. Its main example wraps one genuine settings editor, with the completion
switch outside the inert region. It preserves the editor/value and does not
cancel tasks or steal focus on completion. Content or explicit dimensions set
height; the mask contributes none. `loader_type`, `loader_size`, `loader_color`
and `animated` configure the retained default loader independently of shade
color/opacity. Three finite kinds share the host's reduced-motion policy.

## Tooltip and HoverCard

Tooltip supplies a passive description, preserving the actual target's name.
Long text, explicit line breaks, finite placement/arrows, input adapters and
disabled targets have separate examples. Important touch guidance remains
visible or reachable through an enabled help action; there is no implicit
long-press behavior. A disabled target cannot be the only route to an explanation.

HoverCard is an interactive, named nonmodal Dialog. It preserves focus when
opening and coalesces movement between target and card controls. Its browser
examples explicitly supply cancellable close waits to let the pointer cross the
anchor gap. Delay examples include their complete timer helper: native completion
there is immediate, and a native application supplies its own local clock Future.
No timer provider is installed by the widget. A separate toggle offers a touch
entry. Controlled dismissal and retained note examples exercise source ownership.
Native HoverCard Dialog/Preserve has real layout/focus evidence; actual Tooltip
layers remain explicitly rejected. Neither proves OS reader speech or hover UI.

## ActionBar, FloatingIndicator and Tour

ActionBar starts from actual project selection. Archive and Clear update that
selection; unavailable actions remain disabled. The count uses the theme's text
metrics. The frame and horizontal action groups wrap without squeezing text
labels into vertical fragments. Directional roving navigation remains the same
Toolbar controller. A fixed example makes placement explicit and starts closed.

FloatingIndicator follows real mounted target boxes. Its primary selects workspace
sections; details cover resizing, keyed control reordering without replacing
buttons, row/column layout, color/motion and target removal/re-registration.
Position-only changes and RTL layout changes require explicit
`measurement_revision` or context refresh when resize does not detect them.

Tour points at genuine draft/action controls. Further examples cover accepted or
refused step/close proposals, missing-target recovery and reveal inside a local
scroll viewport. It retains one current target/query and the same guide panel;
no unmounted business row is queried. Native/CPU and browser protocol checks
remain distinct from actual OS input and screen-reader acceptance.
