# TimePicker / TimeGrid

Enable the optional dependency-free `dates` feature. `time_picker(key)` composes
real NumberInput spinbuttons for hour/minute and optional seconds. It stores one
typed `Option<LocalTime>` with a separate finite segment draft. No date, midnight
or timezone is fabricated while a segment is missing.

```rust
use sapid::prelude::*;
let time = scope.cage(Some(LocalTime::new(9, 30, 0)?));
let grid = TimeGridSpec::new(
    LocalTime::new(9, 0, 0)?, LocalTime::new(17, 0, 0)?, 1800, 4,
)?;
let picker = time_picker("appointment")
    .name("Appointment time")
    .value(time)
    .clearable(true)
    .grid(grid)
    .on_value_change(move |next| time.revise(|old| *old = next));
# Ok::<(), sapid::DateError>(())
```

`TimeFormat` and `TimeBounds` have the same explicit precision/hour-cycle/locale
rules as TimeInput. Nanoseconds and seconds outside the selected precision are
rejected rather than truncated. Same-day bounds do not represent an overnight
interval. Twelve-hour mode uses an actual pressed `Afternoon (PM)` toggle
(`下午` in Chinese); false means AM. Midnight and noon both use hour 12, with
different period state. This toggle is not labelled as a radio button.

`TimeParts` holds optional hour/minute/second and the explicit period flag.
`TimePickerDraft` distinguishes Empty (all displayed segments empty), Incomplete,
Valid, Invalid, OutOfBounds, Unavailable and TooLong. Partial or invalid/outside
parts retain the last complete business time. Only a complete valid time or all
empty segments propose a new value. `on_draft_change` runs before the typed value
callback. The controller stores at most two ASCII digits per numeric segment,
checks proposals before copying, and uses NumberInput's bounded input protocol.
Rejected oversized segments restore the accepted time; no whole-day source scan
or input history is retained. Composition and caret remain owned by the actual
editors, with NumberInput's revision and terminal echo guards.

`time_picker(key).duration()` changes to a typed `DurationPicker` whose
`TimeDuration` can represent 0 through 999,999 hours plus minutes/seconds. It is
not stored in `LocalTime`, so a duration beyond one day cannot leak into calendar
or timezone APIs. `min_hours_digits(1..=6)` controls the initial placeholder,
committed zero padding and minimum editor width; the real hour editor expands as
more digits are entered. Duration mode is always 24-hour and deliberately has no
period or presets dropdown. It retains partial drafts, controlled refusal,
independent steps/refs/labels, clearable state and optional second precision.

`hour_step`, `minute_step`, `second_step` default to one; admission requires
1–24 for hour and 1–59 for minute/second. They lower to real SpinValue step
semantics. Page keys use ten steps. Tab moves among separate native editors;
left/right move within the native caret. There is no implicit focus auto-advance.
An explicit segment commit pads valid numeric text to two digits. NumberInput
clamps its own numeric hour/minute/second domain on explicit commit; the aggregate
TimeBounds or unavailable-time policy does not clamp the resulting wall clock.
An invalid/outside/unavailable aggregate commit emits `on_reject`.

Controlled `value` is read-only and accepted only by the callback. Refusal restores
all accepted parts after the host tick. Invalid external precision/bounds source
updates return a typed flush error and keep accepted UI, with silent valid recovery.
`clearable(true)` adds a bounded `Clear time` action while any displayed segment
has content, including an incomplete draft. It proposes `None` through that same
controlled callback, so refusal restores an accepted complete value and keeps the
action; an incomplete empty-source draft is cleared without fabricating a time.
`.clear_label(...)` changes its bounded accessible name. Readonly disables the
action; disabled state reaches it with all other controls. No hidden editor is
replaced when the action mounts or retires.
`readonly` preserves focus/caret but forbids spin/period/Grid proposals; `enabled`
uses actual disabled Fieldsets for the editors. All refs, effects, callbacks and
grid members retire with their Owner. Common `node_ref` refers to the named Group;
`hour_ref`, `minute_ref`, `second_ref` refer to actual editors.

Empty segments use real shared-host placeholders. The default is `--` for each
visible segment, matching the documented upstream default; `hour_placeholder`,
`minute_placeholder` and `second_placeholder` accept bounded reactive strings.
The same NumberInput node and native hint layout are retained as text appears or
is cleared. Invalid newline/control/over-budget updates roll back atomically.

`presets(Vec<LocalTime>)` and grouped `TimePickerPresetGroup` values open in a
real anchored Popover when a segment receives focus or the adjacent `Choose time`
trigger is activated. The trigger exposes the popup/controls/expanded relation;
Escape and outside dismissal use the shared layer protocol. `.open(...)`,
`.default_open(...)` and `.on_open_change(...)` support controlled or local open
state. Controlled refusal leaves the popup unchanged. Selecting a preset uses the
same typed value proposal as segment and grid input; `.close_on_preset_select(true)`
then proposes closing only after that action. The accepted value is represented by
pressed state, while out-of-bounds and explicitly disabled presets remain visible
but unavailable.

Preset admission is finite: at most 96 unique times in at most 16 nonempty groups;
group labels are bounded and reject controls/newlines. Values must match the active
precision. Duplicate values, empty collections, invalid labels, over-budget input,
and combining presets with the inline TimeGrid are rejected before node creation.
Minute precision uses three columns and second precision two, matching the denser
labels without retaining a visited-value registry.

`with_dropdown(true)` adds real hour/minute/optional-second and AM/PM control
columns to the same anchored popup. The independent segment steps determine the
finite choices. `reverse_time_controls_list(true)` reverses every column and
`max_dropdown_content_height(80..=1200)` bounds the scroll viewport. Presets and
controls may share one popup and one controlled open/value protocol.

Paste handling observes the bounded post-edit host event on every segment.
The default parser accepts complete ISO wall-clock text such as `17:33:43`;
`paste_split(...)` installs an application parser and `handle_paste(false)`
disables capture. At most 32 UTF-8 bytes enter this path, and accepted results use
the same bounds, disabled-time and controlled-value checks as every other input.

`hour_label`, `minute_label`, `second_label` and `period_label` provide independent
accessible names. `period_labels(am, pm)` controls the visible 12-hour marker
without changing the typed 24-hour value. `on_focus` and `on_blur` observe the
composite focus boundary through Glory's derived `FocusWithin` protocol. Popup
geometry uses a validated `AnchorPlacement` and optional bounded
`popup_width(80..=1200)`; invalid placement or width is rejected before nodes.

Field label/description names the containing Group, and its label targets the first
real hour Input. Required, readonly and invalid state reaches each editor; the
Field's own error relation stays on the associated first control. Explicit
`.form_name("appointment")` produces successful `appointment_hour`,
`appointment_minute`, and optional `appointment_second` numeric controls. The base
is limited to 240 bytes, before cloning, to leave room for the suffixes. There is
no hidden combined field: applications requiring canonical ISO transport should
serialize the typed `LocalTime` and validate the independent draft in the Form
submit callback. Ordinary Enter commits the segment before Form submission.

The optional grid uses `time_grid(key)` with the same TimeGridSpec. It is an actual
Grid/Row/GridCell/Button tree, and selected state belongs to the cell. Grid and
segments share the same accepted time. A valid time between grid slots remains
valid and selects no cell; the component does not snap it to the visual sampling.
The default spec is 00:00–23:30 at 30-minute steps in four columns. A spec has an
inclusive upper bound; if the step does not divide the interval, no extra final
endpoint is invented. It can contain real seconds when the format includes them.

Spec construction computes the slot count in O(1) before allocating anything and
rejects more than **288 slots**, step 0/>86400, fractional nanoseconds, reversed
same-day endpoints or column counts outside 1–12. At most 128 disabled times are
admitted, with a 129th iterator sentinel; a million-item lazy iterator is not fully
consumed. Disabled exact times deny user segment proposals and Grid endpoints;
programmatic values are still authoritative. There is no disabled-time predicate
scanning an unbounded source. The mounted cell/ref list is bounded by the current
spec and freed on Owner retirement.

Grid arrows rove actual focus and skip disabled slots. Home/End stay in a row;
Page keys move four rows; Shift+Page jumps to the first/last allowed slot. RTL
reverses horizontal commands and physical placement. Focus and selection remain
independent, and readonly retains roving focus. A single current cursor composes
several queued keys followed by activation in the same tick. No query, task,
timer, routing or transport is installed.

Validation completed before browser acceptance:

- `time_grid_model::tests`: two arithmetic admission/seconds/endpoint tests.
- `time_parts::tests`: two tests, including every second of a day in both cycles.
- Six TimeGrid, fifteen TimePicker and three DurationPicker component tests pass:
  `D:/BuildCache/sapid/time-picker-rust-final.log`; pure model logs are
  `time-grid-model-rust.log` and `time-parts-rust.log`.
- Four actual native tests pass (`time-picker-native.log`): four-theme physical
  spin editor geometry/focus, real Spin commands and shared Grid selection,
  real hit testing, selected accent pixels, clear-action geometry/value removal,
  native placeholder layout after clearing and stable editor identity; separate
  Grid disabled-skip navigation, readonly focus/selection and Owner cleanup;
  anchored presets geometry/hit testing; duration dynamic width beyond 24 hours.
- Sapid lib/component strict, WASM strict, native focused strict and SSR check
  pass in `time-picker-{clippy,web-clippy,native-clippy,ssr-check}.log`.

The joint TimePicker/TimeGrid browser suite passed **12/12**, errors `[]`, on
Chromium **148.0.7778.96** against fixed FORMAT-10
`http://127.0.0.1:8794/web33`. Production was built from the immutable source copy
`D:/BuildCache/sapid/web33-source`; later DateTime work did not enter that artifact.
Eight theme/CSR/hydration cases cover physical spinbutton/Field label/Tab, partial
and empty values, Enter padding, Spin keyboard, Form barrier and separate successful
names, controlled refusal, Grid/disabled/readonly/RTL, twelve-hour/Chinese period,
browser AX, stable caret and unstyled geometry. Four additional cases cover
same-tick three-segment edits and bounded rejection, same-tick Grid arrows plus
activation, composition/Owner, dirty SSR takeover and real touch slot/segment editing.

The initial test incorrectly treated Tab as a Commit and failed the padding
assertion. The shared host contract explicitly emits Commit on Enter, not focus
loss (`docs/number-input.md`). The corrected test retains the strong `03` padding
assertion after Enter and separately checks native Tab focus order. No production
change or forced action was needed. The failure is preserved in
`D:/BuildCache/sapid/time-browser-web33-first-tab-contract.log`; the final JSON
records this protocol correction:
`D:/BuildCache/sapid/time-picker-browser-web33.json`. The raw runner log is
`time-browser-web33.log` (also includes the independent TimeInput12 regression,
which passed errors `[]`). Screenshots are
`time-picker-{hue}-{scheme}-{csr|hydrate}.png`; blue light CSR and green dark
hydration were visually inspected.

Native typed commands/layout/pixel evidence does not establish OS IME or
screen-reader announcements.

The showcase now has fifteen dedicated TimePicker sections: usage/Field metadata,
controlled clearable value, seconds/12-hour format/per-segment placeholders,
bounds and independent segment steps, shared TimeGrid/disabled slots, and
readonly/disabled states, flat presets, grouped presets, controlled popup state,
custom labels/focus boundaries, typed popup placement/width, typed duration mode,
minimum-hours digits, bounded complete-time paste/custom parsing, and the full
hour/minute/second/period controls dropdown with reversed lists and bounded scroll.
