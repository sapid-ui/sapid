# Layout boxes

`box_()`, `flex()`, `stack()`, `group()`, `center()`, `container()` and `space()`
compose semantic View boxes. `sapid::Group` is a flex row with wrapping and a
12px gap; `glory::ui::group()` remains boxless. No layout helper introduces an
implicit accessible role or keyboard behavior. A supplied name follows Glory's
named View group rule.

All builders accept `theme`, `size`, `unstyled`, reactive `class`/`style`, `name`,
`node_ref`, `fill`, `width`, `height` and `padding`. Flex/Stack/Group additionally
accept finite `gap`, `align`, `justify`, `wrap(FlexWrap::{NoWrap,Wrap})`; Flex
accepts `direction`. The gap and direction are mount configuration; reactive
Style can update any supplied declaration without replacing children. Unstyled
preserves structural layout while removing Container's cosmetic padding.

Center aligns children on both axes. `inline(true)` selects `Display::InlineFlex`;
`inline(false)` restores the default Flex display. Container uses a real centered inner box,
100% available width, a default 960px cap and 16px horizontal padding.
`max_width(Limit)` changes the cap; `fluid()` explicitly removes it. The cap
shrinks with the parent's available width. Space defaults to zero by zero and
accepts explicit px/percent dimensions. Percent height needs a definite containing
height according to the selected engine. Breakpoint providers and responsive
spacing token selection remain part of the P0 theme infrastructure.

Invalid mount dimensions/gaps/caps are rejected before creating nodes. Explicit
Style overrides replace only their supplied fields; clearing an override restores
the configured layout. No source list, window data or visited history is retained.

## Public layout examples

Center, Container, Flex, Group, Space and Stack now have dedicated primary
examples and separate compiled source sections in
`examples/web/src/showcase_layout_docs`. The controls change existing typed
styles without rebuilding children. They demonstrate axes/inline centering,
visible container caps and nested padding, flex direction/wrapping/spacing,
explicit child growth, long-text shrinking, both spacer axes and height-dependent
stack distribution.

For a no-wrap row, choose deliberately between shrinkable children with
`min_width(0)` and stable-width children with `shrink(0)` inside a local scrolling
box. A zero width basis with equal `grow` shares remaining space; Group does not
assign that policy to children automatically. Parent `gap` is usually clearer
than individual Space widgets for uniform spacing. Space's width has no visible
horizontal effect when it is a separate row of a Stack.

The responsive Flex example uses `responsive_root(640.)` and explicit
`ResponsiveValues<Style>`. Its source width is deterministic for SSR, followed
by actual queued container measurements. The same editor moves between row and
column layout. Layout props do not implicitly read a browser viewport or parse
JavaScript breakpoint objects.

`tests/layout.rs` covers boxed Group, defaults, independent style merging,
reactive identity/Owner cleanup, unstyled, invalid configuration and SSR escaping.
`examples/web/src/layout_form.rs` is shared by CSR/SSR/native verification;
`layout-acceptance.js` verifies real center/max-width/wrap/gaps/resize/identity,
four themes and teardown. Actual run results belong in the D-drive acceptance
reports. `examples/native/tests/layout.rs` checks real engine geometry in all four
themes. A present test file is not a claim that the host run has passed.

The current shared-page remediation passed 32 component tests, seven native
tests (including real responsive grid/simple-grid/flex reflow and stable editors),
strict Clippy and the public release gate. The ten layout pages passed 80 browser
cases across four themes and 390/1280px widths, with no page/console errors:
`examples/web/target/remediation/layout-remediation.json`, asset
`assets-175211fec743fb1c04f5`. The previous Center inline example was corrected
to use ordinary inline flow; a flex parent would blockify that child.
