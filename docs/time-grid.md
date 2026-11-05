# TimeGrid

`time_grid(key)` displays real grid rows/cells and native buttons. A
`TimeGridSpec::new(start, end, step_seconds, columns)` admits 1–288 slots,
positive steps up to one day and 1–12 columns before mounting any slot. End is
inclusive only when it lies on the arithmetic step. Values have no time zone or
date; overnight intervals must be split explicitly.

The root defaults to 100% width capped at its preferred width. Ordinary columns
share the available width. Labels keep a 52px minimum column for 24-hour values
and 72px for 12-hour values; dense configurations scroll horizontally inside the
grid. Explicit dimensions remain customizable. Grid rows and columns retain
logical metadata; RTL reverses physical column placement without reversing the
value model. No Resize listener or per-frame layout state is required.

`value(Cage/Bond/Vary<Option<LocalTime>>)` is controlled; `on_value_change`
proposes a value without mutating that source. `default_value` enables local
state. `TimeBounds` sets inclusive same-day limits, and `disable_time` /
`disable_times` supplies up to 128 explicit unavailable slots. A selected
off-step value has no selected cell; nearest allowed focus is separate.

The single current Tab stop supports arrows, Home/End and page navigation,
skipping unavailable slots. Readonly retains navigation and prevents changes;
disabled prevents both. Pending commands use one bounded current cursor, not a
visited-slot history. Accepted updates retain every button identity and Owner;
invalid updates keep the last accepted state. Retirement clears refs and pending
focus intent. Keyboard focus requests explicitly reveal the target with the
shared ScrollIntoView action; dense columns remain reachable without manually
dragging a scrollbar. Typed changes precede generic observers.

The public page has a clean appointment preview and independent compiled
examples for intervals/12-hour formatting/dense columns, bounds/controlled
refusal, and readonly/disabled/RTL states. Source panels contain the same Rust
modules compiled by the preview. Component behavior is covered by
`tests/time_grids.rs`; native geometry/navigation is covered by
`examples/native/tests/time_picker.rs`. Browser checks use
`scripts/showcase-remediation-geometry.cjs`.
