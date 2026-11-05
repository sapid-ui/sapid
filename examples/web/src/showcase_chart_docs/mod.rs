use super::showcase_examples::DetailSection;
mod area_chart_controlled;
mod area_chart_recovery;
mod area_chart_references;
mod area_chart_series;
mod area_chart_stacked;
mod area_chart_window;
mod bar_chart_controlled;
mod bar_chart_recovery;
mod bar_chart_references;
mod bar_chart_series;
mod bar_chart_stacked;
mod bar_chart_window;
mod bars_list_controlled;
mod bars_list_data;
mod bars_list_presentation;
mod bars_list_recovery;
mod bubble_chart_controlled;
mod bubble_chart_data;
mod bubble_chart_presentation;
mod bubble_chart_recovery;
mod bullet_chart_controlled;
mod bullet_chart_data;
mod bullet_chart_presentation;
mod bullet_chart_recovery;
mod candlestick_chart_controlled;
mod candlestick_chart_data;
mod candlestick_chart_presentation;
mod candlestick_chart_recovery;
mod composite_chart_controlled;
mod composite_chart_recovery;
mod composite_chart_references;
mod composite_chart_series;
mod composite_chart_window;
mod donut_chart_controlled;
mod donut_chart_data;
mod donut_chart_presentation;
mod donut_chart_recovery;
mod funnel_chart_controlled;
mod funnel_chart_data;
mod funnel_chart_presentation;
mod funnel_chart_recovery;
mod gauge_chart_controlled;
mod gauge_chart_data;
mod gauge_chart_presentation;
mod gauge_chart_recovery;
mod line_chart_controlled;
mod line_chart_recovery;
mod line_chart_references;
mod line_chart_series;
mod line_chart_window;
mod matrix_chart_controlled;
mod matrix_chart_data;
mod matrix_chart_presentation;
mod matrix_chart_recovery;
mod pie_chart_controlled;
mod pie_chart_data;
mod pie_chart_presentation;
mod pie_chart_recovery;
mod radar_chart_controlled;
mod radar_chart_data;
mod radar_chart_presentation;
mod radar_chart_recovery;
mod radial_bar_chart_controlled;
mod radial_bar_chart_data;
mod radial_bar_chart_presentation;
mod radial_bar_chart_recovery;
mod sankey_chart_controlled;
mod sankey_chart_data;
mod sankey_chart_presentation;
mod sankey_chart_recovery;
mod scatter_chart_controlled;
mod scatter_chart_data;
mod scatter_chart_presentation;
mod scatter_chart_recovery;
mod sparkline_controlled;
mod sparkline_data;
mod sparkline_presentation;
mod sparkline_recovery;
mod sunburst_chart_controlled;
mod sunburst_chart_data;
mod sunburst_chart_presentation;
mod sunburst_chart_recovery;
mod treemap_controlled;
mod treemap_data;
mod treemap_presentation;
mod treemap_recovery;
mod waffle_chart_controlled;
mod waffle_chart_data;
mod waffle_chart_presentation;
mod waffle_chart_recovery;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "area-chart" => vec![
            DetailSection {
                id: "chart-area-chart-series",
                title: "Series, units and missing observations",
                note: "Values are thousands of USD. None creates a real gap; zero stays a visible exact observation. Composite uses a line for revenue and bars for costs.",
                example: Box::new(area_chart_series::demo(theme.clone())),
                source: include_str!("area_chart_series.rs"),
            },
            DetailSection {
                id: "chart-area-chart-stacked",
                title: "Stacked contributions",
                note: "Positive contributions accumulate. Bars also support independent negative stacks; signed stacked areas are rejected rather than silently reshaped.",
                example: Box::new(area_chart_stacked::demo(theme.clone())),
                source: include_str!("area_chart_stacked.rs"),
            },
            DetailSection {
                id: "chart-area-chart-references",
                title: "Target line, campaign area and peak",
                note: "Annotations share the same plot geometry and clip to the inclusive visible window.",
                example: Box::new(area_chart_references::demo(theme.clone())),
                source: include_str!("area_chart_references.rs"),
            },
            DetailSection {
                id: "chart-area-chart-window",
                title: "Brush and explicit value scale",
                note: "Toggle March through May; the data table retains all six months. The 0–18 domain clips geometry and reports clipped values without rewriting business data.",
                example: Box::new(area_chart_window::demo(theme.clone())),
                source: include_str!("area_chart_window.rs"),
            },
            DetailSection {
                id: "chart-area-chart-controlled",
                title: "Controlled legend and point selection",
                note: "Selection and hidden-series changes are proposals. Enable acceptance, select with Next, then toggle a legend item; the current point and legend identities are retained.",
                example: Box::new(area_chart_controlled::demo(theme.clone())),
                source: include_str!("area_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-area-chart-recovery",
                title: "Empty data and rejected updates",
                note: "An empty chart is admitted. Nonfinite input is refused atomically and retains the last admitted drawing and table; Restore republishes valid data.",
                example: Box::new(area_chart_recovery::demo(theme.clone())),
                source: include_str!("area_chart_recovery.rs"),
            },
        ],
        "bar-chart" => vec![
            DetailSection {
                id: "chart-bar-chart-series",
                title: "Series, units and missing observations",
                note: "Values are thousands of USD. None creates a real gap; zero stays a visible exact observation. Composite uses a line for revenue and bars for costs.",
                example: Box::new(bar_chart_series::demo(theme.clone())),
                source: include_str!("bar_chart_series.rs"),
            },
            DetailSection {
                id: "chart-bar-chart-stacked",
                title: "Stacked contributions",
                note: "Positive contributions accumulate. Bars also support independent negative stacks; signed stacked areas are rejected rather than silently reshaped.",
                example: Box::new(bar_chart_stacked::demo(theme.clone())),
                source: include_str!("bar_chart_stacked.rs"),
            },
            DetailSection {
                id: "chart-bar-chart-references",
                title: "Target line, campaign area and peak",
                note: "Annotations share the same plot geometry and clip to the inclusive visible window.",
                example: Box::new(bar_chart_references::demo(theme.clone())),
                source: include_str!("bar_chart_references.rs"),
            },
            DetailSection {
                id: "chart-bar-chart-window",
                title: "Brush and explicit value scale",
                note: "Toggle March through May; the data table retains all six months. The 0–18 domain clips geometry and reports clipped values without rewriting business data.",
                example: Box::new(bar_chart_window::demo(theme.clone())),
                source: include_str!("bar_chart_window.rs"),
            },
            DetailSection {
                id: "chart-bar-chart-controlled",
                title: "Controlled legend and point selection",
                note: "Selection and hidden-series changes are proposals. Enable acceptance, select with Next, then toggle a legend item; the current point and legend identities are retained.",
                example: Box::new(bar_chart_controlled::demo(theme.clone())),
                source: include_str!("bar_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-bar-chart-recovery",
                title: "Empty data and rejected updates",
                note: "An empty chart is admitted. Nonfinite input is refused atomically and retains the last admitted drawing and table; Restore republishes valid data.",
                example: Box::new(bar_chart_recovery::demo(theme.clone())),
                source: include_str!("bar_chart_recovery.rs"),
            },
        ],
        "line-chart" => vec![
            DetailSection {
                id: "chart-line-chart-series",
                title: "Series, units and missing observations",
                note: "Values are thousands of USD. None creates a real gap; zero stays a visible exact observation. Composite uses a line for revenue and bars for costs.",
                example: Box::new(line_chart_series::demo(theme.clone())),
                source: include_str!("line_chart_series.rs"),
            },
            DetailSection {
                id: "chart-line-chart-references",
                title: "Target line, campaign area and peak",
                note: "Annotations share the same plot geometry and clip to the inclusive visible window.",
                example: Box::new(line_chart_references::demo(theme.clone())),
                source: include_str!("line_chart_references.rs"),
            },
            DetailSection {
                id: "chart-line-chart-window",
                title: "Brush and explicit value scale",
                note: "Toggle March through May; the data table retains all six months. The 0–18 domain clips geometry and reports clipped values without rewriting business data.",
                example: Box::new(line_chart_window::demo(theme.clone())),
                source: include_str!("line_chart_window.rs"),
            },
            DetailSection {
                id: "chart-line-chart-controlled",
                title: "Controlled legend and point selection",
                note: "Selection and hidden-series changes are proposals. Enable acceptance, select with Next, then toggle a legend item; the current point and legend identities are retained.",
                example: Box::new(line_chart_controlled::demo(theme.clone())),
                source: include_str!("line_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-line-chart-recovery",
                title: "Empty data and rejected updates",
                note: "An empty chart is admitted. Nonfinite input is refused atomically and retains the last admitted drawing and table; Restore republishes valid data.",
                example: Box::new(line_chart_recovery::demo(theme.clone())),
                source: include_str!("line_chart_recovery.rs"),
            },
        ],
        "composite-chart" => vec![
            DetailSection {
                id: "chart-composite-chart-series",
                title: "Series, units and missing observations",
                note: "Values are thousands of USD. None creates a real gap; zero stays a visible exact observation. Composite uses a line for revenue and bars for costs.",
                example: Box::new(composite_chart_series::demo(theme.clone())),
                source: include_str!("composite_chart_series.rs"),
            },
            DetailSection {
                id: "chart-composite-chart-references",
                title: "Target line, campaign area and peak",
                note: "Annotations share the same plot geometry and clip to the inclusive visible window.",
                example: Box::new(composite_chart_references::demo(theme.clone())),
                source: include_str!("composite_chart_references.rs"),
            },
            DetailSection {
                id: "chart-composite-chart-window",
                title: "Brush and explicit value scale",
                note: "Toggle March through May; the data table retains all six months. The 0–18 domain clips geometry and reports clipped values without rewriting business data.",
                example: Box::new(composite_chart_window::demo(theme.clone())),
                source: include_str!("composite_chart_window.rs"),
            },
            DetailSection {
                id: "chart-composite-chart-controlled",
                title: "Controlled legend and point selection",
                note: "Selection and hidden-series changes are proposals. Enable acceptance, select with Next, then toggle a legend item; the current point and legend identities are retained.",
                example: Box::new(composite_chart_controlled::demo(theme.clone())),
                source: include_str!("composite_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-composite-chart-recovery",
                title: "Empty data and rejected updates",
                note: "An empty chart is admitted. Nonfinite input is refused atomically and retains the last admitted drawing and table; Restore republishes valid data.",
                example: Box::new(composite_chart_recovery::demo(theme.clone())),
                source: include_str!("composite_chart_recovery.rs"),
            },
        ],
        "candlestick-chart" => vec![
            DetailSection {
                id: "chart-candlestick-chart-data",
                title: "OHLC per trading day",
                note: "Open and close are body endpoints; high and low are the wick extrema. Rising and falling candles use distinct palette entries. The table and point details expose all four values.",
                example: Box::new(candlestick_chart_data::demo(theme.clone())),
                source: include_str!("candlestick_chart_data.rs"),
            },
            DetailSection {
                id: "chart-candlestick-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(candlestick_chart_presentation::demo(theme.clone())),
                source: include_str!("candlestick_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-candlestick-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(candlestick_chart_controlled::demo(theme.clone())),
                source: include_str!("candlestick_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-candlestick-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(candlestick_chart_recovery::demo(theme.clone())),
                source: include_str!("candlestick_chart_recovery.rs"),
            },
        ],
        "donut-chart" => vec![
            DetailSection {
                id: "chart-donut-chart-data",
                title: "Shares, totals and the inner ring",
                note: "Values are nonnegative parts of one total. Point details include exact values and percentages. The default inner radius is 62% of the outer radius; configure thickness, angles and center text separately below.",
                example: Box::new(donut_chart_data::demo(theme.clone())),
                source: include_str!("donut_chart_data.rs"),
            },
            DetailSection {
                id: "chart-donut-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example changes the inner radius to 45%, uses a 270-degree arc from -180 degrees, and places bounded ordinary text in the center. Center text is application supplied; it does not silently recompute totals.",
                example: Box::new(donut_chart_presentation::demo(theme.clone())),
                source: include_str!("donut_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-donut-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(donut_chart_controlled::demo(theme.clone())),
                source: include_str!("donut_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-donut-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(donut_chart_recovery::demo(theme.clone())),
                source: include_str!("donut_chart_recovery.rs"),
            },
        ],
        "pie-chart" => vec![
            DetailSection {
                id: "chart-pie-chart-data",
                title: "Shares, angles and percentages",
                note: "Arc sweeps are proportional to the current visible total. Hidden slices change the denominator; zero and missing values do not invent a slice. Exact percentages remain available through selection.",
                example: Box::new(pie_chart_data::demo(theme.clone())),
                source: include_str!("pie_chart_data.rs"),
            },
            DetailSection {
                id: "chart-pie-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example starts at zero degrees and uses a 270-degree sweep. Exact shares still use the visible total. Angles are clockwise and never change business values.",
                example: Box::new(pie_chart_presentation::demo(theme.clone())),
                source: include_str!("pie_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-pie-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(pie_chart_controlled::demo(theme.clone())),
                source: include_str!("pie_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-pie-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(pie_chart_recovery::demo(theme.clone())),
                source: include_str!("pie_chart_recovery.rs"),
            },
        ],
        "funnel-chart" => vec![
            DetailSection {
                id: "chart-funnel-chart-data",
                title: "Stage order and conversion counts",
                note: "Input order defines stages. Width represents the count relative to the largest visible stage; colors, readable labels and exact counts share the same record keys.",
                example: Box::new(funnel_chart_data::demo(theme.clone())),
                source: include_str!("funnel_chart_data.rs"),
            },
            DetailSection {
                id: "chart-funnel-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(funnel_chart_presentation::demo(theme.clone())),
                source: include_str!("funnel_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-funnel-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(funnel_chart_controlled::demo(theme.clone())),
                source: include_str!("funnel_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-funnel-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(funnel_chart_recovery::demo(theme.clone())),
                source: include_str!("funnel_chart_recovery.rs"),
            },
        ],
        "radar-chart" => vec![
            DetailSection {
                id: "chart-radar-chart-data",
                title: "Comparable axes and a common domain",
                note: "Each record is one dimension, not a separate polygon series. Compare scores on the same 0–100 scale; unlike units should be normalized by the application before plotting.",
                example: Box::new(radar_chart_data::demo(theme.clone())),
                source: include_str!("radar_chart_data.rs"),
            },
            DetailSection {
                id: "chart-radar-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(radar_chart_presentation::demo(theme.clone())),
                source: include_str!("radar_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-radar-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(radar_chart_controlled::demo(theme.clone())),
                source: include_str!("radar_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-radar-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(radar_chart_recovery::demo(theme.clone())),
                source: include_str!("radar_chart_recovery.rs"),
            },
        ],
        "scatter-chart" => vec![
            DetailSection {
                id: "chart-scatter-chart-data",
                title: "Coordinates, series and units",
                note: "The horizontal coordinate is x and the vertical coordinate is y; palette entries color observations in input order. Points expose both exact coordinates. Scatter uses a fixed visual point radius.",
                example: Box::new(scatter_chart_data::demo(theme.clone())),
                source: include_str!("scatter_chart_data.rs"),
            },
            DetailSection {
                id: "chart-scatter-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(scatter_chart_presentation::demo(theme.clone())),
                source: include_str!("scatter_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-scatter-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(scatter_chart_controlled::demo(theme.clone())),
                source: include_str!("scatter_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-scatter-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(scatter_chart_recovery::demo(theme.clone())),
                source: include_str!("scatter_chart_recovery.rs"),
            },
        ],
        "bubble-chart" => vec![
            DetailSection {
                id: "chart-bubble-chart-data",
                title: "Three independent dimensions",
                note: "x/y determine position; nonnegative size determines bubble area through a square-root radius scale. Size is independent of the y domain. Observation colors and exact size remain available in the data representation.",
                example: Box::new(bubble_chart_data::demo(theme.clone())),
                source: include_str!("bubble_chart_data.rs"),
            },
            DetailSection {
                id: "chart-bubble-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example limits radii to 3–32 frame units. Square-root size scaling remains separate from x/y coordinates and the exact size values.",
                example: Box::new(bubble_chart_presentation::demo(theme.clone())),
                source: include_str!("bubble_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-bubble-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(bubble_chart_controlled::demo(theme.clone())),
                source: include_str!("bubble_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-bubble-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(bubble_chart_recovery::demo(theme.clone())),
                source: include_str!("bubble_chart_recovery.rs"),
            },
        ],
        "radial-bar-chart" => vec![
            DetailSection {
                id: "chart-radial-bar-chart-data",
                title: "Independent tracks and a fixed maximum",
                note: "Each track shows an independent value, not a share of a sum. Use the explicit 0–100 domain for comparable tracks. The default starts at twelve o’clock and sweeps clockwise; the presentation section configures a half-circle.",
                example: Box::new(radial_bar_chart_data::demo(theme.clone())),
                source: include_str!("radial_bar_chart_data.rs"),
            },
            DetailSection {
                id: "chart-radial-bar-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example shows independent 0–100 tracks over a 180-degree sweep beginning at -180 degrees. Unfilled tracks use the same configured arc.",
                example: Box::new(radial_bar_chart_presentation::demo(theme.clone())),
                source: include_str!("radial_bar_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-radial-bar-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(radial_bar_chart_controlled::demo(theme.clone())),
                source: include_str!("radial_bar_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-radial-bar-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(radial_bar_chart_recovery::demo(theme.clone())),
                source: include_str!("radial_bar_chart_recovery.rs"),
            },
        ],
        "sparkline" => vec![
            DetailSection {
                id: "chart-sparkline-data",
                title: "Compact trend and complete readings",
                note: "A sparkline belongs beside a metric. Hide its legend and navigation for the compact overview; this separate example retains the exact values, gaps and keyboard data navigation.",
                example: Box::new(sparkline_data::demo(theme.clone())),
                source: include_str!("sparkline_data.rs"),
            },
            DetailSection {
                id: "chart-sparkline-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(sparkline_presentation::demo(theme.clone())),
                source: include_str!("sparkline_presentation.rs"),
            },
            DetailSection {
                id: "chart-sparkline-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(sparkline_controlled::demo(theme.clone())),
                source: include_str!("sparkline_controlled.rs"),
            },
            DetailSection {
                id: "chart-sparkline-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(sparkline_recovery::demo(theme.clone())),
                source: include_str!("sparkline_recovery.rs"),
            },
        ],
        "bars-list" => vec![
            DetailSection {
                id: "chart-bars-list-data",
                title: "Ranked labels and signed values",
                note: "The application controls input order and units. Positive and negative values share a zero baseline; exact signed values remain in the table. Reorder keys to change rank without treating labels as identities.",
                example: Box::new(bars_list_data::demo(theme.clone())),
                source: include_str!("bars_list_data.rs"),
            },
            DetailSection {
                id: "chart-bars-list-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(bars_list_presentation::demo(theme.clone())),
                source: include_str!("bars_list_presentation.rs"),
            },
            DetailSection {
                id: "chart-bars-list-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(bars_list_controlled::demo(theme.clone())),
                source: include_str!("bars_list_controlled.rs"),
            },
            DetailSection {
                id: "chart-bars-list-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(bars_list_recovery::demo(theme.clone())),
                source: include_str!("bars_list_recovery.rs"),
            },
        ],
        "bullet-chart" => vec![
            DetailSection {
                id: "chart-bullet-chart-data",
                title: "Actual, target and qualitative bands",
                note: "The bar is actual usage 68, the target marker is 80, and the 40/70/100 band endpoints describe capacity. Values are compared with the explicit minimum and maximum; bands are not additional observations.",
                example: Box::new(bullet_chart_data::demo(theme.clone())),
                source: include_str!("bullet_chart_data.rs"),
            },
            DetailSection {
                id: "chart-bullet-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(bullet_chart_presentation::demo(theme.clone())),
                source: include_str!("bullet_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-bullet-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(bullet_chart_controlled::demo(theme.clone())),
                source: include_str!("bullet_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-bullet-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(bullet_chart_recovery::demo(theme.clone())),
                source: include_str!("bullet_chart_recovery.rs"),
            },
        ],
        "gauge-chart" => vec![
            DetailSection {
                id: "chart-gauge-chart-data",
                title: "Range, target and thresholds",
                note: "The semicircle maps 0–100 capacity. The current value is 68, target 80, and band endpoints are 40, 70 and 100. Exact values and thresholds remain accessible without relying on color.",
                example: Box::new(gauge_chart_data::demo(theme.clone())),
                source: include_str!("gauge_chart_data.rs"),
            },
            DetailSection {
                id: "chart-gauge-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(gauge_chart_presentation::demo(theme.clone())),
                source: include_str!("gauge_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-gauge-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(gauge_chart_controlled::demo(theme.clone())),
                source: include_str!("gauge_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-gauge-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(gauge_chart_recovery::demo(theme.clone())),
                source: include_str!("gauge_chart_recovery.rs"),
            },
        ],
        "waffle-chart" => vec![
            DetailSection {
                id: "chart-waffle-chart-data",
                title: "One hundred cells and deterministic rounding",
                note: "The 10×10 grid allocates exactly 100 cells by largest remainders in stable input order. One cell represents 1% of the visible total, not necessarily one raw observation. Selection reports original values and allocated cells.",
                example: Box::new(waffle_chart_data::demo(theme.clone())),
                source: include_str!("waffle_chart_data.rs"),
            },
            DetailSection {
                id: "chart-waffle-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses four rows, five columns and a three-unit gap: twenty total cells. Largest remainders still allocate every cell deterministically, and point details report the actual grid total.",
                example: Box::new(waffle_chart_presentation::demo(theme.clone())),
                source: include_str!("waffle_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-waffle-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(waffle_chart_controlled::demo(theme.clone())),
                source: include_str!("waffle_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-waffle-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(waffle_chart_recovery::demo(theme.clone())),
                source: include_str!("waffle_chart_recovery.rs"),
            },
        ],
        "treemap" => vec![
            DetailSection {
                id: "chart-treemap-data",
                title: "Hierarchy, weights and small regions",
                note: "Parent keys define a finite tree and leaf weights determine area. Internal weights are derived. Next, First child and Parent node retain access to small regions; selection changes the reading without loading a new subtree.",
                example: Box::new(treemap_data::demo(theme.clone())),
                source: include_str!("treemap_data.rs"),
            },
            DetailSection {
                id: "chart-treemap-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(treemap_presentation::demo(theme.clone())),
                source: include_str!("treemap_presentation.rs"),
            },
            DetailSection {
                id: "chart-treemap-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(treemap_controlled::demo(theme.clone())),
                source: include_str!("treemap_controlled.rs"),
            },
            DetailSection {
                id: "chart-treemap-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(treemap_recovery::demo(theme.clone())),
                source: include_str!("treemap_recovery.rs"),
            },
        ],
        "sunburst-chart" => vec![
            DetailSection {
                id: "chart-sunburst-chart-data",
                title: "Hierarchy levels and relative angles",
                note: "Rings represent levels and leaf weights determine sector angles. Parent node and First child navigate the current hierarchy. This is selection within the admitted tree; remote or arbitrary-depth drill-down is application owned.",
                example: Box::new(sunburst_chart_data::demo(theme.clone())),
                source: include_str!("sunburst_chart_data.rs"),
            },
            DetailSection {
                id: "chart-sunburst-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(sunburst_chart_presentation::demo(theme.clone())),
                source: include_str!("sunburst_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-sunburst-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(sunburst_chart_controlled::demo(theme.clone())),
                source: include_str!("sunburst_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-sunburst-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(sunburst_chart_recovery::demo(theme.clone())),
                source: include_str!("sunburst_chart_recovery.rs"),
            },
        ],
        "sankey-chart" => vec![
            DetailSection {
                id: "chart-sankey-chart-data",
                title: "Sources, destinations and flow weights",
                note: "Each link names existing source/target keys and a nonnegative flow. Link width encodes weight. Cycles and inconsistent references are rejected. Next reads exact link endpoints and values; the intrinsic table preserves node/link relationships.",
                example: Box::new(sankey_chart_data::demo(theme.clone())),
                source: include_str!("sankey_chart_data.rs"),
            },
            DetailSection {
                id: "chart-sankey-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(sankey_chart_presentation::demo(theme.clone())),
                source: include_str!("sankey_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-sankey-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(sankey_chart_controlled::demo(theme.clone())),
                source: include_str!("sankey_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-sankey-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(sankey_chart_recovery::demo(theme.clone())),
                source: include_str!("sankey_chart_recovery.rs"),
            },
        ],
        "matrix-chart" => vec![
            DetailSection {
                id: "chart-matrix-chart-data",
                title: "Row/column identity, signed scale and missing cells",
                note: "Rows and columns are independent keyed categories. The five-stop scale maps signed values; an absent cell remains absent while zero is a real value. This general matrix is separate from the date-based Heatmap.",
                example: Box::new(matrix_chart_data::demo(theme.clone())),
                source: include_str!("matrix_chart_data.rs"),
            },
            DetailSection {
                id: "chart-matrix-chart-presentation",
                title: "Palette, dimensions and exact readings",
                note: "This example uses the alternative categorical palette and a 480×300 coordinate frame. The frame scales to the actual container. Labels and values remain ordinary text outside vector drawing.",
                example: Box::new(matrix_chart_presentation::demo(theme.clone())),
                source: include_str!("matrix_chart_presentation.rs"),
            },
            DetailSection {
                id: "chart-matrix-chart-controlled",
                title: "Controlled point proposals",
                note: "Next and direct point activation propose a stable record key. The application can refuse a proposal; enable acceptance to publish the selection. A retained current snapshot owns drawing, labels and data together.",
                example: Box::new(matrix_chart_controlled::demo(theme.clone())),
                source: include_str!("matrix_chart_controlled.rs"),
            },
            DetailSection {
                id: "chart-matrix-chart-recovery",
                title: "Empty data, validation and recovery",
                note: "Empty data uses the matching schema. Invalid schema or nonfinite values are rejected before publication and retain the preceding chart. Restore admits the original data again without replacing the widget.",
                example: Box::new(matrix_chart_recovery::demo(theme.clone())),
                source: include_str!("matrix_chart_recovery.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn contents(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "area-chart" => vec![
            (
                "Series, units and missing observations",
                "#chart-area-chart-series",
            ),
            ("Stacked contributions", "#chart-area-chart-stacked"),
            (
                "Target line, campaign area and peak",
                "#chart-area-chart-references",
            ),
            ("Brush and explicit value scale", "#chart-area-chart-window"),
            (
                "Controlled legend and point selection",
                "#chart-area-chart-controlled",
            ),
            (
                "Empty data and rejected updates",
                "#chart-area-chart-recovery",
            ),
        ],
        "bar-chart" => vec![
            (
                "Series, units and missing observations",
                "#chart-bar-chart-series",
            ),
            ("Stacked contributions", "#chart-bar-chart-stacked"),
            (
                "Target line, campaign area and peak",
                "#chart-bar-chart-references",
            ),
            ("Brush and explicit value scale", "#chart-bar-chart-window"),
            (
                "Controlled legend and point selection",
                "#chart-bar-chart-controlled",
            ),
            (
                "Empty data and rejected updates",
                "#chart-bar-chart-recovery",
            ),
        ],
        "line-chart" => vec![
            (
                "Series, units and missing observations",
                "#chart-line-chart-series",
            ),
            (
                "Target line, campaign area and peak",
                "#chart-line-chart-references",
            ),
            ("Brush and explicit value scale", "#chart-line-chart-window"),
            (
                "Controlled legend and point selection",
                "#chart-line-chart-controlled",
            ),
            (
                "Empty data and rejected updates",
                "#chart-line-chart-recovery",
            ),
        ],
        "composite-chart" => vec![
            (
                "Series, units and missing observations",
                "#chart-composite-chart-series",
            ),
            (
                "Target line, campaign area and peak",
                "#chart-composite-chart-references",
            ),
            (
                "Brush and explicit value scale",
                "#chart-composite-chart-window",
            ),
            (
                "Controlled legend and point selection",
                "#chart-composite-chart-controlled",
            ),
            (
                "Empty data and rejected updates",
                "#chart-composite-chart-recovery",
            ),
        ],
        "candlestick-chart" => vec![
            ("OHLC per trading day", "#chart-candlestick-chart-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-candlestick-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-candlestick-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-candlestick-chart-recovery",
            ),
        ],
        "donut-chart" => vec![
            (
                "Shares, totals and the inner ring",
                "#chart-donut-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-donut-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-donut-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-donut-chart-recovery",
            ),
        ],
        "pie-chart" => vec![
            ("Shares, angles and percentages", "#chart-pie-chart-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-pie-chart-presentation",
            ),
            ("Controlled point proposals", "#chart-pie-chart-controlled"),
            (
                "Empty data, validation and recovery",
                "#chart-pie-chart-recovery",
            ),
        ],
        "funnel-chart" => vec![
            (
                "Stage order and conversion counts",
                "#chart-funnel-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-funnel-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-funnel-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-funnel-chart-recovery",
            ),
        ],
        "radar-chart" => vec![
            (
                "Comparable axes and a common domain",
                "#chart-radar-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-radar-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-radar-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-radar-chart-recovery",
            ),
        ],
        "scatter-chart" => vec![
            ("Coordinates, series and units", "#chart-scatter-chart-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-scatter-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-scatter-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-scatter-chart-recovery",
            ),
        ],
        "bubble-chart" => vec![
            ("Three independent dimensions", "#chart-bubble-chart-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-bubble-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-bubble-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-bubble-chart-recovery",
            ),
        ],
        "radial-bar-chart" => vec![
            (
                "Independent tracks and a fixed maximum",
                "#chart-radial-bar-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-radial-bar-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-radial-bar-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-radial-bar-chart-recovery",
            ),
        ],
        "sparkline" => vec![
            (
                "Compact trend and complete readings",
                "#chart-sparkline-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-sparkline-presentation",
            ),
            ("Controlled point proposals", "#chart-sparkline-controlled"),
            (
                "Empty data, validation and recovery",
                "#chart-sparkline-recovery",
            ),
        ],
        "bars-list" => vec![
            ("Ranked labels and signed values", "#chart-bars-list-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-bars-list-presentation",
            ),
            ("Controlled point proposals", "#chart-bars-list-controlled"),
            (
                "Empty data, validation and recovery",
                "#chart-bars-list-recovery",
            ),
        ],
        "bullet-chart" => vec![
            (
                "Actual, target and qualitative bands",
                "#chart-bullet-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-bullet-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-bullet-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-bullet-chart-recovery",
            ),
        ],
        "gauge-chart" => vec![
            ("Range, target and thresholds", "#chart-gauge-chart-data"),
            (
                "Palette, dimensions and exact readings",
                "#chart-gauge-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-gauge-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-gauge-chart-recovery",
            ),
        ],
        "waffle-chart" => vec![
            (
                "One hundred cells and deterministic rounding",
                "#chart-waffle-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-waffle-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-waffle-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-waffle-chart-recovery",
            ),
        ],
        "treemap" => vec![
            (
                "Hierarchy, weights and small regions",
                "#chart-treemap-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-treemap-presentation",
            ),
            ("Controlled point proposals", "#chart-treemap-controlled"),
            (
                "Empty data, validation and recovery",
                "#chart-treemap-recovery",
            ),
        ],
        "sunburst-chart" => vec![
            (
                "Hierarchy levels and relative angles",
                "#chart-sunburst-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-sunburst-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-sunburst-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-sunburst-chart-recovery",
            ),
        ],
        "sankey-chart" => vec![
            (
                "Sources, destinations and flow weights",
                "#chart-sankey-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-sankey-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-sankey-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-sankey-chart-recovery",
            ),
        ],
        "matrix-chart" => vec![
            (
                "Row/column identity, signed scale and missing cells",
                "#chart-matrix-chart-data",
            ),
            (
                "Palette, dimensions and exact readings",
                "#chart-matrix-chart-presentation",
            ),
            (
                "Controlled point proposals",
                "#chart-matrix-chart-controlled",
            ),
            (
                "Empty data, validation and recovery",
                "#chart-matrix-chart-recovery",
            ),
        ],
        _ => Vec::new(),
    }
}
