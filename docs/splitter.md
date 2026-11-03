# Splitter

`splitter(key)` owns two stable panes and one focusable separator. A horizontal
layout uses a vertical separator line; a vertical layout uses a horizontal line.
`.axis(Vary<Orientation>)` changes the live direction without rebuilding panes.
Changing direction retires the current drag query and rejects its late result;
an old-axis pointer command cannot commit a new-axis size.
The line carries finite `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, and
`aria-controls`. Plain `Separator` remains passive.

```rust
use glory::prelude::*;
use sapid::splitter;

let percentage = scope.cage(50.0);
splitter("editor")
    .value(percentage)
    .min(10.0)
    .max(90.0)
    .on_value_change(move |next| percentage.revise(|value| *value = next))
    .first(|| glory::ui::input().name("Document"))
    .second(|| glory::ui::text().content("Preview"));
```

Without `.value`, `.default_value(50.0)` supplies local state. Controlled sources
are read-only: change callbacks propose a percentage and the application chooses
whether to write it back. External changes are silent. `.on_value_commit` runs
after a keyboard adjustment or completed pointer drag. Typed callbacks precede
generic Range/Drag observers. Same-tick local key events accumulate; rejected
external source values leave callbacks using the accepted configuration.

Percentages must stay within `0..=100`, with finite `min < max` and an inclusive
current value. `.step` is positive and at most 100; `.thickness` is positive and
at most 128 pixels. Defaults are min10, max90, step1 and thickness6. Bounds are
reactive. Invalid configurations return errors before creating nodes; invalid
later updates preserve accepted descriptors. This component has two panes;
nested splitters compose additional regions.

The showcase includes fixed-width sidebar composition, four nested workspaces and
coordinated controlled redistribution. These are application composition patterns,
not claims of CSS-unit pane sizes, direct multi-pane input or automatic redistribution.
The existing orientation, collapse, line-size, grip and nested examples remain available.

The arrow keys perpendicular to the separator adjust by one step; Home/End select
the configured boundaries. Modifier and composition keys retain their host
meaning. Disabled handles use programmatic focus mode, leave the Tab sequence,
and reject physical focus or range actions. At an allowed 0/100 endpoint the
collapsed pane is hidden while its Owner and children remain mounted. Focus in a
newly hidden pane moves to an enabled handle. `.first_ref`, `.second_ref`, and
`.handle_ref` expose stable live nodes and clear on retirement.

The inner layout reserves the draggable area set by `.thickness(6.0)`, then
divides the remaining extent between panes. `.line_size(1.0)` sets the visible
line thickness and accepts zero to hide it. `.grip_size(16.0, 3.0)` sets the
centered grip's length and width. `.with_grip(false)` hides it while keeping
the separator draggable and keyboard accessible. Line and grip widths are
capped at the draggable area's width. Line size and grip visibility accept
reactive sources, and both decorations follow the layout axis. Outer padding and borders do not enter that
measurement. Panes have zero minimum flex extent and retain their child nodes,
editor values and engine identities while resizing. Application CSS can still
change a child's overflow policy; the pane wrapper clips overflowing content.

The browser and native shell capture the original handle on primary press and
keep it through release over a sibling. The browser reads the pane extent at
the press's committed barrier, so a move queued with the press can update the
panes in the same frame. Hosts with queued bounds replies retain the latest
point until that reply arrives. Moves and release then use the measured extent.
A press and release without movement along the resize axis emits no value
change or commit.
The browser and native shell coalesce consecutive moves before delivery. The
browser retains fractional CSS-pixel pointer coordinates. A pane update that
changes only an existing `flex-grow` writes that property directly in the
browser. The value still comes from the accepted state.
During captured mouse drags, browsers that expose `pointerrawupdate` can
deliver newer coordinates before the next animation frame. Glory samples
those events and can run a paced turn after the listener returns. A single
timer (with at least 4 ms after a completed turn) or the next animation
frame, whichever arrives first, delivers the latest queued move. A turn
that exceeds the 2.5 ms script budget retires the raw listener for the rest
of that drag, leaving more time for layout and paint. Release and
cancellation also retire the listener and its timer.
The divider stays at its size limit while the pointer is beyond the allowed
range. When the pointer returns, the divider follows with the original grab
offset. The browser uses its pointer-focus behavior, with a themed
outline for keyboard focus. Cancellation and Owner retirement remove the
request and its captures immediately; late results cannot publish. Each drag has a non-wrapping ticket,
with no visited-pointer or historical geometry cache.
HTML's typed splitter contract includes `touch-action:none`. Native Finger
pointer protocol simulation exercises the real engine input path; it does not
establish physical-device or screen-reader acceptance.

Validation currently includes nine Sapid component tests, including initial bounds
coalescing, same-tick updates and immediate Drop commit; two native component
tests covering four themes, both axes, measured layout conservation, controlled
requests, disabled behavior and editor identity; three shared render tests; two
shared native tests covering queued keyboard, mouse/Finger capture/cancel and
AccessKit numeric actions. The HTML full-shape versus field-delta oracle passes
with splitter range set/reset metadata. `examples/web/splitter-acceptance.js`
passed all ten checks on root build8 with Chromium 148.0.7778.96: four-theme
CSR/hydration, dirty editor identity/value/selection takeover, Owner retirement,
and actual CDP touch Start/Move/End/Cancel. Touch capture preserved page scroll
and the editor selection; cancellation produced no commit. Reports and screenshots
are in `D:/BuildCache/sapid/splitter-browser.json`, `splitter-browser.log`, and
`splitter-*.png`; native evidence is in `splitter-native.log`. The green/dark
hydrated screenshot was inspected for readable content and handle placement.

The showcase layout suite also checks line thickness, hidden-grip dragging and
axis changes in all four themes with CSR and hydration. It checks subpixel
movement, reversing at either limit and keyboard focus styling. A prior Chromium probe found the
first update at one pixel after the change, compared with six pixels before.
Five successive moves after initial measurement went from five handle bounds
reads to zero. The browser reads the splitter frame's bounds once per press.
The layout suite queues press and move together, verifies the first frame's
accepted value, and checks for one `flex-grow` write per pane while preserving
the other inline declarations.
