# Data display examples and contracts

The thirteen shared pages have independent public primary examples and 42
standalone source sections in `examples/web/src/showcase_data_display_docs`.
Examples demonstrate product content, controlled proposals and actual actions.
Source panels contain the complete compiled example, including local media
resolution. The two fictional showcase photos and their exact generation prompts
are recorded in `generated-showcase-media.json`.

## People, status and surfaces

Avatar displays an optional real photo, explicit initials or a passive vector
fallback. `fallback_icon(Icon)` selects the drawing; a later `fallback(text)`
selects text again. The named person wrapper remains independent of image load
state. Current request completion hides or restores the fallback, and accepted
source changes preserve the same wrapper, image and fallback identities. Groups
use explicit keys, bounded overlap and a named `+N` overflow marker; they do not
create a people menu. See `avatar-images.md`.

Badge supports Soft, Solid, Outline and Dot, five compact theme sizes, optional
color and passive left/right sections. A `BadgeSection` is an Icon or a nonempty
single-line string of at most 32 UTF-8 bytes. Central content is reactive. Plain
badges remain semantic Text, including inside finite intrinsic button content.
Badges with sections or Dot use a boxed passive layout and belong beside a real
action, rather than inside a raw button. Badge never becomes interactive just
because it has a color or icon. Long labels clip to the chosen maximum width;
complete content remains in the semantic tree. See `display-widgets.md`.

Card exposes header, body and footer slots plus a semantic title/description.
Border and shadow are independent choices; radius, padding and size are ordinary
Style fields. The photo example uses a real local landscape, a bounded cover
crop, wrapping product details and a Save action with observable local feedback.
There is no undocumented full-bleed section shorthand or whole-card activation.

ColorSwatch paints actual alpha over a checkerboard. Passive samples have text
names; interactive samples use the real button's pressed/disabled protocol.
`pressed` is read-only and changes are application proposals. Diameter is selected
with finite `side(8..=128)` and `round` controls the outline. The public primary
selects a named color and reports its RGBA value. Copying requires an explicit
clipboard service and is not silently installed by this component.

## Description data and images

DataList uses real DescriptionList/Term/Definition semantics. Its boxless item
controllers keep terms and definitions as direct physical children. The default
Horizontal orientation uses `label_width(120)` plus a flexible value column;
Vertical stacks terms and values. `gap` defaults to 12. Bounds are 1..=512 for
label width and 0..=128 for gap, in finite pixels. Essential grid/stack layout,
zero definition margins and minimum widths survive `unstyled`; theme decoration
does not. Long prose wraps in the available value column. Applications choose an
explicit local overflow policy for unbreakable identifiers. Definitions accept
real widgets, including separate actions. See `document-widgets.md`.

Image uses the real semantic Image, alternative text and host-owned requests.
The public examples compare Cover/Contain, natural proportions, explicit square
crops, radius, informative/decorative content and one bounded fallback retry.
`fallback_src` provides another image resource; it is not a replacement arbitrary
widget. A source change retains the image node and current revision barriers.
Photo failure tests use an invalid inline PNG payload, avoiding network services
or expected HTTP errors in the public console. See `avatar-images.md`.

## Anchors, hints and formatted values

Indicator offers all nine physical anchors, a finite 4..=128 square marker,
optional short text and optional color. `offset(-128..=128)` uses signed pixels:
positive values inset selected edges, negative values move outward, centered
axes remain centered. Labels do not resize the marker. Its pointer-transparent
presentation preserves the independent action underneath, including overlap.
`visible(false)` hides only the marker. See `indicators-icons.md`.
The wrapper follows ordinary parent layout rules. The content-sized examples
use `stack().align(Align::Start)` so column stretching does not place an edge
marker against unused frame width instead of the actual button.

Kbd is a passive semantic keycap with explicit Windows/Mac/Linux label policy.
Combinations use separate keycaps and text separators. Theme sizes and colors
apply normally; the component installs no global shortcut or keyboard listener.
The public input example remains an ordinary editor. See `document-widgets.md`.

NumberFormatter keeps a numeric read-only source and one text node. Explicit
EnUs/DeDe/ZhCn locale policies, grouping, fraction limits, percent and USD/EUR/CNY
currency formats are finite deterministic Rust formatting, not arbitrary Intl
coverage. `prefix` and `suffix` accept reactive literal strings up to 256 UTF-8
bytes each. Empty strings are valid; NUL and line separators are rejected. Affixes
surround the complete formatted amount, including its sign/currency policy;
they never mutate the number. Invalid updates retain the last accepted text and
valid updates recover without remounting. See `number-formatter.md`.

OverflowList measures the current bounded registrations and moves the same
widgets into the existing More popup. Examples cover container resizing, a
controlled visible prefix, refused/accepted popup proposals and retained editing
through moves. Explicit item widths are finite pixels; they make flexible
editors independent of the measured available width. `initial_visible` controls
the deterministic first SSR shape; it does not disable later measurements.
Hidden content changes can explicitly revise `measurement_revision`. Native
popup, focus and measurements retain the existing host-specific support bounds;
compiling the Rust source is not an OS keyboard or reader acceptance claim.
The summary trigger stays on one line and is measured at its natural width,
rather than feeding a squeezed flex width back into packing. Examples use a
concise localized More label. A controlled `visible_count` deliberately overrides
automatic packing; callers choose a prefix that fits their container.

RollingNumber shares the finite formatter and exposes actual increase/decrease,
carry, sign and digit-count changes. The semantic string is always the complete
current value. Decorative outgoing digits are not separate readable values.
`animated(false)` or duration zero settles immediately; browser reduced motion
is honored, and pause affects presentation without changing the source. The
host owns presentation timing, with no per-frame application Cage or digit
history. Prefix/suffix are NumberFormatter features, not RollingNumber aliases.
Digit line boxes, translation distance and separators follow reactive explicit
font size or line height together. Large type is not cropped to a fixed 24px row;
changing typography retains the current reel nodes.

Spoiler measures and clips real plain text at an explicit `max_height`. Short
content omits the disclosure action. Controlled expansion may be refused; text
changes remeasure the same viewport. Labels are explicit localized strings.
Spoiler currently settles immediately without a transition, including reduced
motion; it does not advertise an animation API or arbitrary hidden controls.
See `spoiler.md`.

ThemeIcon is passive finite vector presentation. Container sizes are 24, 30, 36,
42 and 50 pixels; `icon_size(1..=128)` independently controls the drawing and may
be reactive. Invalid sizes preserve the last accepted glyph and can recover.
Soft/Solid/Outline use theme surfaces. `color` controls the glyph stroke;
container background/radius are explicit Style fields. For a keyboard/pointer
action use ActionIcon with a real accessible name. See `indicators-icons.md`.

## Focused verification

`tests/data_display_docs.rs`, `tests/avatar.rs` and the existing component suites
cover finite budgets, rejected proposals, reactive recovery, plain badge button
compatibility, real description-list shape and identity/Owner retirement.
`examples/native/tests/data_display_docs.rs` verifies actual engine description
layout, wrapping, indicator hit testing, vector geometry and raster pixels in
four themes. These checks do not establish OS pointer/keyboard input, IME or
spoken screen-reader acceptance. Release and browser evidence is recorded per
page in `shared-page-remediation.json` after the actual public run.
