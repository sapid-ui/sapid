# Heatmap

Enable `charts` and import `sapid::charts::*`. `heatmap(key)` displays dated
contributions in week columns and weekday rows. It is independent of MatrixChart
and does not require the `dates` widgets feature. The shared civil model is pure
Rust; no host clock, parser, timezone service or JavaScript chart library is installed.

```rust
let start = CivilDate::parse_iso("2024-01-01")?;
let end = CivilDate::parse_iso("2024-12-31")?;
let range = HeatmapRange::new(start, end)?;
let data = HeatmapData::new([(start, 2.), (start.checked_add_days(1)?, 0.)])?;
let widget = heatmap("contributions")
    .range(range)
    .data(data)
    .with_month_labels(true)
    .with_weekday_labels(true)
    .with_legend(true);
```

An explicit `HeatmapRange` is required and admits 1–366 inclusive dates. It
handles leap years and the civil endpoints without inventing padded dates.
`HeatmapData` accepts at most 366 unique dated records, each finite within ±1e12.
`new` bounds iterator consumption; duplicate declarations are rejected. Use
`set(date, Some(value))` to update a record and `set(date, None)` to remove it.
Missing data and an explicit zero have distinct values, labels and colors.
Records outside the displayed range remain data but do not affect automatic scaling.

`data`, `range`, `locale` and `domain` accept Cage/Bond/Vary. Sources are read only;
external updates are silent. Date-keyed cells retain their Owners and physical
buttons when data changes or date ranges overlap. Removed cells retire their
references, tooltip state and callbacks. No visited-date history or business-data
scan runs on scrolling or pointer events. `on_day_click` receives `HeatmapCell`
with the current date and optional value; disabled or retired cells cannot invoke it.

Automatic domains use visible minimum/maximum values. `domain(Some(...))` fixes
the scale; `None` restores inference. Values clamp at domain edges. An equal-endpoint
domain uses the first color. Configure one to eight colors, `empty_color`, cell
size 8–32px, gap 0–8px, and radius up to half the cell size. `cell_style` merges
explicit per-date declarations. Widget namespaces are at most 48 ASCII bytes.

`week_start` defaults to Sunday. `with_weekday_labels`, `with_month_labels` and
`with_legend` are independent. `weekday_labels` accepts Sunday–Saturday;
`month_labels` accepts January–December, each entry bounded to 32 UTF-8 bytes.
English and Chinese defaults are supplied by `locale`. Month captions identify
the first represented month of each week column; a changed month appears at
the next column boundary. The numeric legend reflects the active domain.

`label_format` controls both accessible names and optional tooltip text. Each
label is 1–256 bytes; the complete displayed text stays within 64 KiB. Invalid
initial declarations publish no nodes, and rejected later labels retain the last
valid presentation. `with_tooltip(true)` uses the existing shared tooltip protocol.
It supports browser hover/focus and remains explicitly unsupported on Apace;
native dated cells, layout, keyboard, names and CPU painting work without it.

The chart has a semantic seven-row Grid and one date Tab stop. Left/Right move
one week; Up/Down stay inside the current week column. Home/End reach the same
weekday's first/last available column. PageUp/PageDown move four weeks and
Shift+PageUp/PageDown reach the range edges through the shared grid protocol. Enter/Space
activate the current date. Focus and reveal run after current bindings; queued
navigation and activation share one bounded cursor until the host packet ends.
Large ranges scroll locally instead of widening the document.

The public page has independent examples for sparse/zero values, controlled
domains and labels, appearance/localization, and reactive ranges/availability.
Source panels include the exact compiled modules. Native CPU/protocol evidence
does not establish operating-system input or screen-reader acceptance.
