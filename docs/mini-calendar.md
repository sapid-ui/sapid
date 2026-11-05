# MiniCalendar

Enable `dates` and use `mini_calendar(key, provider)` for a compact strip of
consecutive civil dates. `Calendar` remains the separate monthly grid.

```rust
let start = CivilDate::parse_iso("2024-02-27")?;
let value = scope.cage(Some(start));
let date = scope.cage(start);
let strip = mini_calendar("appointment", DatesProvider::new(start))
    .number_of_days(6)
    .value(value)
    .date(date)
    .on_value_change(move |next| value.revise(|old| *old = next))
    .on_date_change(move |next| date.revise(|old| *old = next));
```

`value` selects a date; `date` controls the start of the visible range. Both
sources are read only. Their callbacks propose changes, so an application can
refuse selection or navigation independently. Without a source, `default_value`
and `default_date` initialize local state. The default start comes from the
explicit provider reference date, never the host clock. Range changes do not
select a date; `allow_deselect(true)` permits clearing the selected date.

The default count is seven, with a finite 1–31 day limit. Previous/Next move by
the count. At years 1 and 9999, the window shifts inward to retain all slots.
Large counts scroll inside the widget instead of widening the page. Visible
day buttons retain their identities across range and locale changes.

`DatesProvider::bounds` limits selection. `exclude_date` accepts at most 366
declarations; `disable_weekday` excludes an entire weekday. Keyboard movement
skips unavailable dates with a bounded 366-day search. `readonly` allows range
navigation but prevents selection; `enabled(false)` prevents both. A selected
source outside bounds is rejected before initial nodes are published; invalid
later updates retain the last valid snapshot and report an error.

The provider supplies English/Chinese full accessible labels and optional today
metadata. `day_style(|date| Style)` supplies explicit per-day style declarations.
`theme`, `size`, `class`, `style` and `unstyled` use the shared appearance contract;
unstyled keeps structural row layout. Custom colors should account for selected
and unavailable states. There is no arbitrary DOM-props escape hatch.

The strip exposes one Grid row with selected GridCells and real day buttons.
One available day is a Tab stop. Arrows move without selecting; Home/End reach
available window edges, PageUp/PageDown move by a window, and Enter/Space select.
RTL reverses physical date order. Host focus is applied after the new range is
accepted and visible; a refused range never focuses an absent date. Queued
commands read the latest source and preserve a bounded current cursor.

Public documentation includes independent range, controlled/refusal,
localization/bounds/customization and keyboard/state examples. Source panels
include those exact compiled modules. Browser/native protocol and geometry
checks do not establish OS IME or screen-reader acceptance.
