# RangeSlider

`range_slider(key)` contains exactly two independently named Slider controls on one typed track. Its value is `[f64; 2]`; local state defaults to `[25.0, 75.0]`, or applications supply `.value(...)` and accept proposals in `on_value_change` or `on_value_commit`. It never copies a business table or accepts an unbounded thumb iterator.

```rust
let prices = scope.cage([25.0, 75.0]);
range_slider("price")
    .labels(["Minimum price".into(), "Maximum price".into()])
    .value(prices)
    .min(0.0).max(100.0).step(0.5).minimum_gap(10.0)
    .on_value_change(move |next| prices.revise(|values| *values = next))
```

Both global bounds and values must be finite, the global interval must be strict, the values must be ordered and their distance must meet the nonnegative minimum gap. Bounds and positive steps use the single Slider's 10^12 component limit. Initial invalid configuration creates no nodes; later invalid sources preserve accepted UI state. Names are nonblank and limited to 512 UTF-8 bytes each; value text is nonblank and limited to 1024 bytes each. Raw reactive text is checked by borrowing before cloning. Validation and recovery are silent.

`.maximum_gap(...)` adds a finite upper distance bound. A controlled source that is already wider than the bound remains representable. Push moves the other thumb to restore the maximum span, Hold rejects a violating proposal, and Clamp allows inward movement while preventing further widening until the pair is valid. The maximum must be at least the minimum gap and no larger than the global interval. `.inverted(true)` reverses only the painted active/inactive track colors; it does not reverse values, keyboard direction or thumb identities.

`.domain([start, end])` separates the full visual/pointer track from the selectable `min`/`max` interval. The domain is a finite strict interval and must contain both selectable bounds. Values, marks and accessible thumb limits remain expressed in the selectable interval; only their track positions and pointer-coordinate conversion use the wider domain. Changing a live domain cancels an in-flight geometry request just like changing min, max or direction.

Value labels are finite presentation derived from each accepted thumb value. They appear while that thumb is dragged by default; `.value_labels_always_on(true)` keeps both visible and `.show_value_labels(false)` omits them. `.scale(factor)` applies a finite numeric display scale to the visual label and `aria-valuetext` while preserving raw values, geometry and callbacks. `.value_text([lower, upper])` overrides the generated strings with bounded formatted text so applications can expose units. The paint-only label nodes are inert and do not duplicate the slider's accessibility output. Sapid deliberately uses a finite factor instead of retaining an arbitrary formatter closure inside reactive paint.

RangeSlider shares `SliderMark` declarations with Slider. At most 64 marks must
be strictly increasing and inside the global range. In restricted mode both
accepted values must be marks; pointer and keyboard proposals choose only marks
that also satisfy the active collision policy and finite gap bounds. No thumb is
ever swapped. `SliderMark::hidden(true)`
suppresses both marker and label rendering without removing that value from
restricted pointer or keyboard snapping.

Lower/upper identities, DOM order and Tab order never change. The default `RangeCollision::Push` atomically moves the other thumb when the active thumb reaches the minimum or maximum gap. `.push_on_overlap(false)` selects `RangeCollision::Hold` and rejects that overlapping proposal without publishing a partial pair. Explicit `RangeCollision::Clamp` retains Sapid's stricter legacy policy where the active thumb stops at its current neighbour-derived bound. Restricted marks push only to another declared mark; if no pair can satisfy all finite constraints, the accepted pair is retained. No policy swaps thumb identities or introduces arbitrary-count state.

Each thumb exposes its actual policy-dependent accessible min/max, value, orientation, optional value text, enabled and readonly state. Clamp advertises the current neighbour-limited interval; Push and Hold advertise the endpoints that preserve room for the configured minimum gap. A fixed Clamp interval (`min == max`) retains real focus and becomes readonly; moving the other thumb can unlock it without replacing either node. The general `RangeValue` helper and the single Slider component retain their strict interval contract. Numeric rounding at a gap boundary moves inward by a representable value when needed, so decimal floating-point subtraction cannot publish a distance below the requested gap.

Arrow direction, RTL, vertical Up/Down, PageUp/PageDown and Home/End follow Slider. Two thumb events in one host tick accumulate an accepted typed pair, including commit-only controlled applications, instead of overwriting the other thumb's earlier change. A rejected request cannot later become an accepted cursor because an unrelated write contains the same value; Cage versions invalidate the confirmation on every later write.

Pointer gestures start on a thumb and query the shared track's actual Bounds. Both thumbs use the same global coordinate system, avoiding the distorted geometry that would result from stacking independent neighbour-limited single sliders. Captured release remains with the original physical thumb. One live gesture, one pending request and bounded start/latest points are retained. Resize, external/domain changes, invalid configuration, readonly/disable and Owner retirement cancel the request and reject late replies. Cancel does not commit. Track-click selection is not part of this API.

`.field(...)` carries label/description/error, required/readonly and theme/size to both actual Slider nodes. The Field label focuses the lower thumb; `.labels(...)` provides the complete accessible names of the individual controls. With a Field form name and no explicit form value, each thumb has that repeated successful-control snapshot name and its own numeric text. This remains explicit custom-control snapshot metadata; a div is not a native HTML successful input and the component never inserts hidden inputs or performs HTTP submission.

Evidence: 14 integration tests and 2 pure model tests pass, covering Push/Hold/Clamp, fixed identities and tab order, neighbour bounds and readonly recovery, controlled callback ordering and commit-only same-tick updates, invalid source recovery and bounded names/text, Field state and repeated snapshot names, decimal gap precision, Owner/request retirement, value labels and SSR. Two native tests pass across four themes with horizontal LTR/RTL and vertical tracks, actual Bounds and thumb-center geometry, captured semantic drag mapping, gap clamping, real Focus and stable engine identities.

Unstyled mode retains default dimensions for all five sizes and both axes, while preserving explicit `Auto` and custom reactive dimensions. Three range-family geometry regressions and a native stacked-layout regression cover the zero-height/overlap defect and dynamic restoration of absent dimensions. Native evidence is `D:/BuildCache/sapid/angle-geometry-native.log`.

The shared Slider contract has 5 passing tests, including readonly closed intervals and rejection of editable degenerate intervals. Native queued Mouse/Finger capture/cancellation and AccessKit range, numeric step, value text, required/readonly actions and presentational descendants have separate passing bridge tests. The HTML differential field oracle includes Slider snapshots. Native event/AX bridge testing does not claim physical touchscreen or screen-reader announcement acceptance.

Chromium 148.0.7778.96 passed **10/10** cases on the unified root **Web16** snapshot, with no console/page errors: four themes × CSR/hydration, exact per-thumb names and neighbour-limited ranges, singleton readonly/focus/unlock, controlled rejection and a same-frame queued two-thumb commit-only pair, captured mouse release/clamp, vertical/RTL keys, Field errors and repeated snapshot metadata, stable thumb/editor identity and caret. Extra cases cover dirty-editor hydration and Owner cleanup, and CDP touch start/move/end/cancel without page scrolling, editor-selection loss or cancel commits. The final green/dark hydration screenshot was visually inspected.

Reports: `D:/BuildCache/sapid/range-slider-browser-root16.json` and `.log`; screenshots `range-slider-{hue}-{scheme}-{csr|hydrate}.png`; Rust transcript `range-slider-rust.log`; native transcript `range-slider-native.log`. A private pre-fix browser run exposed HTML's aria-labelledby priority over each thumb's explicit name. The final implementation retains the Field label on the group and clears the conflicting per-thumb labelled_by, keeping both explicit thumb names consistent across HTML and native AX. That failed private run is not counted as passing evidence.
