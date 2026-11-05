# Schedule pages and current-window examples

The thirteen public pages have dedicated primary examples and 62 independently
compiled source sections in `examples/web/src/showcase_schedule_docs`. Their source
panels contain the actual data, callbacks and helpers. Primary resource views show
bookings; editing has a separate section with real acceptance and rejection.

| Page | Presentation and explicit input |
| --- | --- |
| Schedule | Six selectable views, with matching view/data updates in one application batch. |
| DayView / WeekView | Exact civil time window, overlap lanes, independent all-day region, local two-axis scrolling and keyboard slot navigation. |
| MonthView | Date grid, bounded event previews and actual disclosure. |
| YearView | Twelve month summary cells with finite event disclosure and month-range selection. |
| MobileMonthView | Date counts above a full agenda for the inspected day; activation works without a physical-focus event. |
| AgendaView | Current events grouped by date, exact timed/all-day readings and a real empty state. |
| ResourcesDayView / ResourcesWeekView / ResourcesMonthView | Independent date/resource windows, complete resource/date headers, row/column layouts and local scrolling. |
| ResourcesSchedule | Three real resource views with paired application-owned data/view acceptance. |
| Recurring events | Actual current-window expansion, intervals, weekday rules, termination, exceptions, missing days and one-off replacement. |
| Events data | Complete typed declarations connected to actual views, half-open intervals, explicit zones and validation failures. |

## Keys, headers and current views

Unified schedules now pass their caller key into their current child instead of
using a fixed child key shared by instances. Multiple instances can share a view and
update independently. This also preserves the complete 48-byte Schedule and
40-byte ResourceSchedule key budgets. Both modes validate nonempty ASCII
identifier keys and geometry before mounting the shell. Explicit duplicate keys
remain invalid.

View buttons wrap at narrow widths and publish their accepted pressed state.
Window navigation uses compact visible arrows or date/room labels and retains
complete accessible action names. View switches accept matching data and view
in one batch; refusing a proposal leaves the existing presentation in place.
The stable outer shell and unrelated instances retain their identities. Resource
updates within the same view/date/resource window also retain the child view and
current edit target; replacing that window or view constructs a new bounded child.
Agenda consumes grid observers without registering them on its ordinary Group.

## Window and selection ownership

`ScheduleData` owns one authoritative window and explicit zone. Previous/Next
request exact replacement data; resource paging additionally requests a finite
resource slice. The examples synchronously accept those requests. Applications
own asynchronous loading, cancellation and response sequencing. No application
event history, HTTP transport, clock or route is discovered by the component.

`selection` is read only. Its callback proposes an Event or Slot; the application
chooses whether to accept it. The controlled sections demonstrate refusal,
acceptance and readonly behavior. Date navigation remains available in readonly
views. MobileMonth inspection follows keyboard focus or date activation; accepted
selection remains separate. Activation updates the logical date cursor without
inventing a physical-focus change, including when a controlled value is refused.

MobileMonth always shows the inspected day's complete current agenda. The
Month/Year preview limit controls their disclosure rows. Calendar grids have
intrinsic bounded geometry; Agenda/timeline/resource views expose explicit
viewport heights. The timeline scrolling example supplies 10:00 explicitly and
uses the shared `Action::ScrollTo` on its current viewport reference. It installs
no wall clock. Grid keyboard commands also reveal the current cell locally.

Headers participate in local scrolling. Frozen/sticky headers are not included
in this cross-host contract; native typed Sticky remains explicitly unsupported.
Narrow dense timelines/resources use their own viewport instead of overflowing
the document. Locale controls finite date headings; application event and room
titles remain application supplied. RTL is explicit grid navigation/layout.

## Resource editing and conflict policy

The editing sections enable `on_edit`, open a booking's Edit action, and exercise
the actual move/resize commands and drag handles. A proposal identifies the
current event/candidate, original and proposed half-open intervals, and original
and proposed resources. The application validates it against current declarations
and permissions before publishing a replacement model.

The examples accept data and selected interval together in one batch. Their
feedback displays the saved title and complete endpoints. An acceptance switch
can refuse a valid proposal; a separate policy can reject overlaps involving the
edited event in the same room. Existing unrelated overlaps do not cause every
edit to fail. All-day notes retain their declared edit permissions. Recurring
edits require an explicit application policy rather than silently rewriting a
series.

## Recurrence and event declarations

The recurrence primary expands seven daily candidates beginning November 2, 2026.
Suppressing November 5 produces six current occurrences; restoring it produces
seven. Another example uses a two-day interval through an inclusive UNTIL date
and a Monday/Wednesday/Friday rule with COUNT. Missing January-31/leap-day cases
render separate Skip/Clamp expansions in bounded windows. The replacement example
suppresses one original date and adds a distinct afternoon event.

This is Sapid's finite typed Daily/Weekly/Monthly/Yearly model. Its public examples
do not advertise an arbitrary RFC 5545 string parser. Count is evaluated before
exception removal, so exceptions do not extend the rule. Expansion does not walk
an unbounded history from an ancient anchor.

The event-data page demonstrates adjacent intervals, an all-day exclusive end,
and Civil, FixedOffset and Application context identities. Context resolution
runs outside reactive borrows. An application key names its supplied resolver;
it installs no timezone database. Validation buttons expose real invalid-key,
reversed-interval and unknown-resource failures before replacing the valid agenda.

Budgets remain: at most 512 declarations, 2048 current occurrences, 128 exceptions
per event, a 366-day window, 64 declared resources and 1–8 visible resources.
Timeline windows admit at most 96 slots. Admission rejects overflow rather than
silently truncating. See [schedule-model.md](schedule-model.md) for exact rules.

## Verification and comparison

Root regressions cover multiple maximum-length namespaces, accepted/refused view
state, current-data replacement and MobileMonth activation without physical
focus. Existing timeline/calendar/resource protocol suites remain in use.
`examples/native/tests/schedule_docs.rs` imports actual primary and editing
sources. Website tests mount and retire all 62 complete source sections.
Browser coverage is maintained in `scripts/showcase-remediation-schedule.cjs`;
release/browser evidence is recorded in the shared remediation ledger after it
passes. Protocol, emulator-style browser input and native CPU evidence do not
establish OS touch, input-device or reader speech acceptance.

Comparison references: [Schedule](https://mantine.dev/schedule/schedule/),
[MobileMonthView](https://mantine.dev/schedule/mobile-month-view/),
[ResourcesSchedule](https://mantine.dev/schedule/resources-schedule/),
[Recurring events](https://mantine.dev/schedule/recurring-events/) and
[Events data](https://mantine.dev/schedule/events-data/).
