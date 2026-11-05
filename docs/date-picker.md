# DatePicker / DatePickerInput

`DatePickerInput` uses inset calendar and optional clear icon buttons with full
accessible names. `.placeholder(Vary<String>)`, `.clearable(true)` and
`.clear_label(String)` configure the display and clear action. Clear proposes
the empty value for the current Single/Multiple/Range mode through the normal
callback, then returns focus to the same display input. Controlled refusal keeps
the accepted value; readonly and disabled suppress changes. Escape and modal
focus restoration continue through the shared Dialog controller.

The readonly display input retains Field labels, errors, node_ref and focus.
When `.form_name(...)` or Field supplies a successful-control name, a separate
hidden real input submits the accepted ISO value exactly once. Display formats
can use slashes, commas and an en dash without changing that ISO value. The
hidden proxy has no accessible label or independent focus stop. Form reset and
required validation remain application policies, demonstrated with a local form
and explicit reset callback. Percent width, margins and flex sizing apply once
to the full input/action frame.

Enable Sapid's optional `dates` feature. It adds no dependency. Dates are civil
Gregorian values; providers explicitly supply the reference date, optional today,
locale, week start and bounds. No browser clock, timezone, HTTP or serialization
framework is inferred.

```rust
use sapid::prelude::*;
let provider = DatesProvider::new(CivilDate::new(2024, 2, 29)?);
let selection = scope.cage(DateSelection::empty(DatePickerMode::Range));
date_picker("travel", provider)
    .mode(DatePickerMode::Range)
    .value(selection)
    .on_value_change(move |next| selection.revise(|old| *old = next));
```

`DatePickerMode::{Single, Multiple, Range}` is fixed per builder. The controlled
`DateSelection` must retain that mode. Defaults become local state only when no
controlled source exists. `.month` and `.on_month_change` separately control the
visible month. `.default_month`, `.direction`, `.disable_weekday`, `.exclude_date`
and `.exclude_dates` have the Calendar policies. `.allow_deselect(true)` enables
single-date clearing; multiple selection always toggles the activated day.

`.number_of_columns(2)` renders consecutive months that share one selection and
one controlled month anchor. `.columns_to_scroll(n)` controls how far the outer
previous/next month actions advance; it defaults to the column count. Only the
first column exposes previous controls and only the last exposes next controls,
and outside dates are hidden between columns so one civil date never appears as
two actionable Buttons. The finite contract admits 1–4 columns/scroll steps.
`DatePickerInput` forwards the same builders to its kept-mounted popup.
Both builders also forward `.with_week_numbers(true)` and
`.hide_weekdays(true)` to every visible month.

`.level`, `.default_level` and `.max_level` use
`DatePickerLevel::{Month, Year, Decade}`. The month header opens the real
MonthPicker grid, its year header opens the real YearPicker decade grid, and
choosing a year/month descends one level. `.on_level_change` follows the same
controlled proposal contract as month and value changes: refusing the external
level leaves the accepted grid visible. DatePickerInput forwards all four level
settings to its kept-mounted popup.

`SelectedDates` is a sorted unique set in a private fixed array with capacity 32.
Iterator admission consumes at most 33 items, including duplicate inputs.
`DateSelection::Multiple` stores this finite value rather than a `Vec`.
Activating a 33rd distinct date calls `.on_reject(DateSelectionError::TooManyDates)`
and preserves the accepted set without faulting the App. Invalid external mode
or bounds changes reject and keep the previous rendered selection until recovery.

Ranges have explicit `DateRangeSelection::{Empty, Start, Complete}` states.
The first activation starts a range; the second orders both endpoints; activating
a completed range starts another. A completed inclusive range uses constant-time
membership tests and stores two dates. Endpoints must be selectable. **Disabled
interior dates are allowed and remain selected but disabled**; no hidden full-span
scan is performed. Bounds validate both endpoints.

The inline component reuses Calendar's real 42 GridCells and real day Buttons.
Cell selection, physical focus and `Current::Date` remain independent. Multiple
and range modes set typed Grid multiselection, mapped to HTML
`aria-multiselectable` and AccessKit's multiselectable flag. It does not make a
Button into a selected option. Arrow, Home/End and Page/Shift-Page navigation keep
the Calendar protocol. Same-tick activations compose against one live proposal
cursor; no action history is retained. Typed callbacks run before generic observers.

## Input and popup

`date_picker_input(key, provider)` displays the selection in a real readonly text
Input with an independent, named Dialog trigger. It supports the
same three modes, Field relationships, required/invalid/disabled state,
controlled/local selection, controlled/local `.open`, `.on_open_change`, and
`.trigger_ref`, `.panel_ref`, `.calendar_ref` plus the ordinary Input `.node_ref`.
Keys are stable ASCII namespaces of at most 57 bytes; the derived Dialog key
must fit the common 64-byte namespace budget.

The readonly display remains selectable; a hidden input carries its ISO successful value. `.readonly(true)` also
locks calendar selection while keeping navigation available. This picker does
not parse arbitrary typed dates; editable date drafts belong to DateInput.
A `Field` supplies its accessible label/description/error and successful name;
`.form_name` explicitly overrides only the successful name. No ComboBox/Listbox
role is fabricated for the display.

`.display_format(DateInputFormat::{Iso, DayMonthYear, MonthDayYear})` selects a
finite locale-independent presentation. The formatter covers Single, sorted
Multiple and both partial and complete Range values; multiple values use `, `
and ranges use ` – ` as visible separators. It changes presentation only—the
controlled source remains a typed `DateSelection`. Arbitrary date-fns format
strings, ambient locale negotiation and unbounded formatter callbacks are not
accepted.

Successful values use an explicit locale-independent spelling:

| Mode | Empty | Value |
| --- | --- | --- |
| Single | empty string | `2024-02-29` |
| Multiple | empty string | sorted `2024-02-26,2024-02-29` |
| Range | empty string | partial `2024-02-26/`, complete `2024-02-26/2024-02-29` |

`DateSelection::format_iso` and `parse_iso` implement this bounded contract. A
partial range is distinguishable from an empty range; application validation can
require `is_complete()`. Submission and routing remain application decisions.

`DatePickerPreset` adds a bounded labeled complete `DateSelection`. The same
typed preset contract supports Single, Multiple and Range modes; labels and
values must be unique and no more than 32 presets are admitted. A preset uses
the ordinary controlled proposal path, respects readonly/disabled state and
current provider bounds, and never replaces the accepted source after refusal.
`DatePickerInput` closes only after its source accepts a complete preset.

The shared host Layer owns modal focus containment, Escape, outside dismissal
and return focus. The explicit initial target is the named close Button. The
panel shell and calendar are kept mounted so closing preserves month and child
identities. A successfully accepted complete single/range selection proposes
closing; multiple selection stays open. Selection refusal leaves the dialog open.
Open refusal is independent from selection refusal. Retiring the Owner removes
all callbacks, references and the physical Layer. The Native host supports this
default Dialog and is tested; no unsupported-layer fallback is substituted.

## Evidence

- Six pure selection-model tests: fixed iterator admission and sorted unique
  membership, capacity rejection, partial/reverse/restarted ranges, exact ISO and
  mode/bounds, plus finite single/multiple/partial/complete display formats.
  `D:/BuildCache/sapid/component-parity`.
- Seventeen component tests in `tests/date_pickers.rs`: same-tick multiple selection,
  typed-before-generic ordering, disabled interior policy, controlled refusal and
  invalid recovery, 33rd-date nonfault rejection, readonly/disabled/Owner cleanup,
  atomic Single/Multiple/Range presets, input close and invalid preset admission,
  real readonly Input/Dialog, stable shells, Field relationships, open refusal,
  admission budgets, explicit range/multiple Input open/text policies, shared
  consecutive columns, cross-column range selection, multi-column scrolling and
  controlled month refusal, week-number forwarding, hidden weekday rows, real
  month/year level traversal, controlled level refusal and max-level limits.
  Calendar's seven tests pass unchanged.
  `D:/BuildCache/sapid/date-picker-rust-final.log` (Calendar regression in
  `date-picker-rust.log`).
- Shared Grid target/rollback tests, all eighteen HTML tests including field-delta
  oracle and reset cases, and native AX multiselection on/off passed:
  `date-grid-rust.log`, `date-grid-html.log`, `date-grid-native-ax.log` in the same
  build root.
- Five actual native component tests in `examples/native/tests/date_picker.rs`
  passed. Four themes open the default physical Dialog, check top-layer entries,
  actual day bounds/hit/selected pixels, real focus, Grid navigation, accepted
  value/close/return focus, stable Input engine identity, reopen/Escape and Owner
  cleanup. The second checks range, disabled interior, readonly and disabled
  rejection; another checks an initially open Range Input, partial text,
  completed-range close and focus return. The multi-column case verifies two
  physical side-by-side grids and one range selection spanning both months; the
  preset case verifies its real action geometry and controlled close.
  `D:/BuildCache/sapid/date-picker-native-final.log`.
- Sapid lib/components, Web WASM and native focused strict Clippy passed:
  `date-input-clippy.log` (includes the final DatePicker tests), `date-picker-web-clippy.log`,
  `date-input-native-clippy.log` (includes final DatePicker native tests). SSR bin check passed.
- The original browser suite passed **12/12**, errors `[]`, with Chromium
  **148.0.7778.96** against fixed FORMAT-8 Web28 at
  `http://127.0.0.1:8794/web28`. Eight theme/CSR/hydration combinations cover
  native keyboard focus, Tab/Escape/return, readonly Field and successful FormData,
  Form Enter, controlled refusal, range/multiple selection, disabled/readonly,
  real browser Dialog/Grid/Button AX tree and stable input/draft caret. Four
  additional cases cover same-tick multi/range actions, 33rd-date nonfault
  rejection, controlled open/Owner cleanup and remount, dirty SSR takeover, and
  actual touch popup/range. Report:
  `D:/BuildCache/sapid/date-picker-browser-web28.json`; log:
  `D:/BuildCache/sapid/date-picker-browser-web28.log`.
  The initial script incorrectly tried clicking an inert trigger after a
  controlled-open Owner remounted; its failure is retained in
  `date-picker-browser-web28-first.log`. The final assertion instead requires
  that the fresh Owner honors the still-true open source, then closes by Escape.
  No production change or forced pointer action was needed. Screenshots are
  `date-picker-{hue}-{scheme}-{csr|hydrate}.png`; blue light CSR and green dark
  CSR were visually inspected.

The same original 12/12 suite also passed against FORMAT-9 Web29, errors `[]`,
with Chromium 148.0.7778.96: `date-picker-browser-web29.json` and
`date-picker-browser-web29.log` under the D build root.

The current public showcase adds a dedicated Value format section with real
Single, Multiple and Range DatePickerInputs. The unified browser suite verifies
their exact values together with presets, two-column range selection, levels,
week rows and DateTimePicker forwarding; errors are `[]` in
`D:/BuildCache/sapid/date-picker-presets-browser.json`. The release showcase is
the fingerprinted `assets-5fadcd185e60de03d78a` artifact.

Native component tests drive typed host commands and query physical layout,
focus, hit testing and raster output. They do not establish OS keyboard/IME or
screen-reader announcement acceptance. IANA/DST and absolute instant conversion
remain separate from civil date selection.

## Complete public page examples

The remaining date/time pages now have dedicated primary examples, independently
compiled full-source sections and matching contents links. See [date page
contracts](date-pages.md) for selection domains, editor/draft behavior, inline versus
popup use, responsive actions and current browser/native acceptance boundaries.
