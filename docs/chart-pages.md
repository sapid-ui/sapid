# Chart pages and portable presentation

The 21 remaining chart pages have dedicated primary examples and 90 standalone,
compiled source sections in `examples/web/src/showcase_chart_docs`. Cartesian
pages separate data/gaps, stacking, annotations, brush/domain, controlled proposals
and recovery. Other pages separate their actual data schema, presentation,
controlled selection and empty/rejected updates. Each source panel includes its
real data and helpers.

The primary plot occupies the available width. Legends wrap as compact pressed
buttons with `aria-pressed`; named Previous, Next and Clear controls remain
keyboard operable. `navigation(false)` omits those controls and the inline reading
while retaining direct activation, callbacks and the default data table. The
compact Sparkline overview uses this with `legend(false)` and supplies all seven
exact weekly values beside the metric. Its data section retains full navigation.

## Data and exact readings

| Pages | Admitted data | Interpretation |
| --- | --- | --- |
| Area, Bar, Line, Composite | `ChartData` categories/series | USD thousands; None creates a gap, zero remains a value; Composite uses series-specific line/bar geometry. |
| Candlestick | `CandleDatum` | Open/close body, high/low wick, rising/falling colors; exact OHLC in table and selection. |
| Donut, Pie | nonnegative values | Visible parts determine the total; selection includes percentages; zero invents no slice. |
| Funnel | ordered nonnegative values | Input order defines stages; width compares counts with the largest visible stage. |
| Radar | comparable value axes | One polygon on a common zero-based domain; normalize unlike units in the application. |
| Scatter, Bubble | x/y/size points | Five ticks per axis, complete domains and portable grid; Bubble adds square-root area scaling. Colors follow observation order. |
| RadialBar | independent values | Shared explicit 0–100 maximum; tracks are independent, rather than shares of a sum. |
| Sparkline | ordered, optionally missing values | Straight segments split at gaps; complete readings remain available independently of the summary. |
| BarsList | ordered signed values | Signed zero baseline; application order defines ranking; exact readings retain record identities. |
| Bullet, Gauge | `TargetDatum` | Explicit range, actual value, target and ordered band endpoints. |
| Waffle | nonnegative values | Largest remainders allocate exactly the configured cell count; details retain original values. |
| Treemap, Sunburst | keyed hierarchy | Leaf weights determine area/angle; Next, First child and Parent node reach small current regions. |
| Sankey | nodes and directed links | Existing endpoints, acyclic nonnegative flows, exact relationships and weights. |
| Matrix | rows/columns/keyed cells | Five-stop signed scale; absent cells remain absent and zero stays zero. Date contributions belong to Heatmap. |

Tables are enabled by default and remain complete when a Cartesian brush narrows
the drawing. Applications hiding them must supply an equivalent representation.
Default exact readings use Click/Previous/Next. The existing standalone `Tooltip`
takes validated Cartesian `ChartPointDetails` and uses the shared anchored
overlay; default inline readings are not described as floating hover tooltips.

## Configured geometry

```rust
use sapid::charts::*;
let ring = PlotOptions {
    donut_inner_ratio: Some(0.45),
    angles: Some(PlotAngles { start: -180., sweep: 270. }),
    center_label: Some("Completion".into()),
    ..Default::default()
};
let grid = PlotOptions {
    waffle_grid: Some(WaffleGrid { rows: 4, columns: 5, gap: 3. }),
    ..Default::default()
};
let bubbles = PlotOptions {
    bubble_radius: Some(BubbleRadius { min: 3., max: 32. }),
    ..Default::default()
};
```

Angles are clockwise degrees: start −360..360, sweep greater than zero and at
most 360. They apply to Pie, Donut and RadialBar, including unfilled tracks.
Defaults remain −90/360. Donut inner radius accepts 0..0.95 (default 0.62).
Center text is ordinary application supplied text, at most 64 UTF-8 bytes without
NUL, CR/LF or Unicode line separators. It is clipped to the inner area and does
not silently recompute a total after legend changes.

Waffle accepts 1..20 rows and columns, at most 400 cells. Gap is finite 0..16
frame units and smaller than the computed cell. Defaults remain 10×10/gap 2.
Largest remainders use the actual total: a twenty-cell grid represents 5% per
cell. Geometry retains the shared 256-command batch limit. Bubble radii are
finite, 0 ≤ min ≤ max ≤ 64 with positive max; defaults remain 0..24. An explicit
nonzero minimum makes zero-size observations visible without changing their
exact values. Unsupported plot/configuration pairs and invalid values are
rejected before publishing a new scene.

## Controlled values and local errors

`selection` and `options` are read-only sources. Callbacks propose changes; the
application accepts or refuses them. Each controlled section demonstrates both
paths while retaining the plot and keyed controls.

`Chart` and `Plot` offer `on_error(FnMut(glory::ui::Error))` for rejected reactive
updates. The last admitted drawing, labels and table stay together; every
recovery section publishes real local error feedback. Without this observer,
the owning Glory scope still receives the error. Invalid initial declarations
fail construction even with an observer. Callbacks clear with the Owner; no
update history or visited-data registry is retained.

Current finite geometry uses straight Cartesian lines, vertical bars and a
shared value axis. Radar represents one polygon. Frame and palette are explicit;
application text/table composition supplies units and custom formatting. These
contracts do not advertise arbitrary Recharts callbacks, SVG injection or remote
hierarchy loading. HTML/SVG and native consume the same admitted scene.

## Verification and comparison

`tests/plot_presentation.rs` checks unchanged defaults, exact arc positions,
square-root radii, 20/400-cell conservation, unsupported/nonfinite configuration,
UTF-8 limits, retained admission and local/fallback rejection handling.
`tests/charts.rs` and `tests/plots.rs` retain keyed, controlled, table and cleanup
coverage. `examples/native/tests/chart_docs.rs` imports actual public sources and
checks four-theme narrow geometry, compact controls, identities, selection,
centered text, twenty real cells and recovery. Existing Cartesian/eighteen-plot
raster/hit/focus suites remain in use. Website tests mount and retire all 90
sections; browser acceptance is `scripts/showcase-remediation-charts.cjs`.
Release/browser results are recorded in the shared remediation ledger. Native
CPU/protocol and browser AX evidence do not establish OS input or reader speech.

Comparison references: [AreaChart](https://mantine.dev/charts/area-chart/),
[DonutChart](https://mantine.dev/charts/donut-chart/),
[PieChart](https://mantine.dev/charts/pie-chart/),
[RadialBarChart](https://mantine.dev/charts/radial-bar-chart/),
[WaffleChart](https://mantine.dev/charts/waffle-chart/),
[ScatterChart](https://mantine.dev/charts/scatter-chart/),
[BubbleChart](https://mantine.dev/charts/bubble-chart/).
