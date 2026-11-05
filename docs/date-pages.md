# Date and time page contracts

The nine remaining Dates pages have dedicated compiled primary examples and
58 complete-source sections with matching contents links. DatePicker retains its
seven existing picker/input examples and adds four focused sections. TimeInput's
five sections and TimePicker's fifteen sections are now independently compiled
files, rather than short fragments shown beside a different acceptance fixture.
The popup DatePickerInput, DateTimePicker, MonthPickerInput and YearPickerInput
pages keep their separately verified examples.

| Page | Public examples |
| --- | --- |
| DatePicker | Single/multiple/range, columns, grid levels, week rows, explicit locale/week start, bounds/exclusions, refusal/recovery and keyboard/disabled states. |
| InlineDateTimePicker | An always-visible review calendar and time editor; seconds/12-hour format, complete/time-only presets, complete-value bounds, controlled draft reset, ordered range, readonly and disabled. |
| DateInput | A real editable appointment field with an inset calendar action; three finite spellings, incomplete/invalid drafts, commit, custom parser, refusal/recovery, bounds and editor states. |
| MonthPicker | Actual civil months, multiple/range, bounded year navigation, unavailable months, locale/RTL and controlled readonly policy. |
| YearPicker | Actual civil years, multiple/range, bounded decade navigation, unavailable years, locale/RTL and controlled readonly policy. |
| TimeInput | One actual time editor, controlled local value, 12/24-hour cycle, seconds, placeholders, bounds, empty/readonly/disabled states. |
| TimePicker | Real hour/minute/second spinbuttons, typed controlled selection/clear, duration, minimum hour digits, whole-value paste, actual dropdown columns, step/bounds, grid, grouped presets, popup policy, labels/focus and finite positioning. |
| TimeValue | Plain formatted display, 12/24-hour cycle, seconds/locale, midnight versus empty, custom empty text and a retained reactive text. |
| Calendar | Application-owned date selection and passive custom day content, actual full-width cells/week numbers, locale/week start, static display, outside-date/weekday presentation and date constraints. |

## Editing and accepted values

DatesProvider supplies an explicit civil reference, optional today, finite locale,
week start and bounds. The examples use fixed references and install no system
clock, timezone, network or persistence service. Calendar is the lower-level day
grid. Custom day content replaces the visible number once while retaining the
full accessible date name and actual button. DatePicker owns the higher-level single/multiple/range selection policy.
MonthPicker and YearPicker use their own period domains without manufacturing a
representative day or timestamp.

DateInput keeps one editable Input. Its calendar action is a distinct named
button inside the same border, with a 16px vector glyph and a theme-sized hit box.
Editor padding reserves the action space. Choosing an accepted date updates the
same editor; refusing the proposal leaves the picker open. Field labels,
descriptions, error relations and editor refs still address the real Input.
`with_picker(false)` omits the action and its padding reservation.

The accepted value is `Option<CivilDate>`; a draft has separate finite status.
An incomplete or invalid draft remains editable and does not replace the accepted
date. Invalid/incomplete commit retains that draft for correction and reports
rejection. A valid commit confirms the value; empty input proposes None.
Controlled refusal restores the accepted presentation. A custom parser returns
the finite DateParse result and is still subject to independent bounds/budgets.
The tomorrow example uses a fixed application reference, not a host clock.

DateInputFormat selects ISO, day-first or month-first **editor spelling**.
DateInput's ordinary successful editor carries that spelling; applications can
serialize the accepted CivilDate explicitly as ISO. The primary appointment form
uses ISO. Readonly DatePickerInput has its separate ISO submission proxy; that
proxy contract must not be assumed for an editable, differently formatted editor.

## Time controls and responsive actions

TimeInput is one editor. TimePicker composes actual NumberInput spinbuttons and
keeps incomplete segments separate from a complete LocalTime. Duration is a
distinct type and supports hours beyond 24; it is not a wall-clock rollover.
Step, precision, bounds, disabled times and full-value paste are explicit finite
policies. TimeValue has no editor or selection controller. Minute precision
requires zero seconds; it does not silently discard second precision.

TimePicker and DurationPicker now use compact vector clear actions with their
full accessible names. TimePicker's chooser uses an arrow glyph with the existing
Choose time name, popup relation, controlled policy and anchor. Numeric fields and
their colon separators stay in one group. The separate action group can wrap
below them while preserving every editor, value, caret and popup controller.
An empty action group takes no layout space. The 300px native acceptance combines
12-hour time, seconds, clear and dropdown: it previously compressed the hour editor
to 26px. The repaired layout retains usable editor widths and moves the actions
as a group; enlarging the column brings them back beside the same editors.

The inline date-time picker keeps calendar and time controls on the page; the
popup DateTimePicker uses a compact trigger input. Clear and Done share a compact, wrapping footer; Clear is a secondary action.
The primary example reports the last actual Done confirmation. Both validate a
complete civil date-time. A day can be available while its proposed time is outside the bound;
the component reports rejection without inventing a clamped time. The controlled
inline source shows the explicit reset revision for unfinished drafts.

## Verification boundaries

All 58 public source sections mount and retire in all four built-in themes and
match their navigation. Native consumer tests import the actual public examples,
measure narrow full-width calendar cells and inline alignment, select actual
month/year domains, preserve the physical editor after calendar selection and
preserve a reactive display text. The combined time case also resizes, opens and
closes its real popup and clears without replacing editors. Existing native tests
continue to cover drafts, selection/caret, keyboard, hit tests, layers and CPU
pixels. Browser acceptance covers all four themes at desktop and phone widths.
These protocol/layout checks do not establish OS keyboard, system pickers,
screen-reader speech or device acceptance.

## Captured numeric input and whole-value paste

Ordinary segment edits and whole-value paste both update the existing numeric
controller. A successful full-value parser runs before numeric rejection; an
unmatched capture follows ordinary number editing. Numeric composition terminal
and echo events coalesce. Repeating an equal full-value paste reasserts the
accepted formatted editor text. Over-budget rejection carries no clipboard body
and does not call the parser. The uncontrolled inline example now exercises real
consecutive edits and Done confirmation, as well as the controlled examples.
