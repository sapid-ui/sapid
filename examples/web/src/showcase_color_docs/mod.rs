use super::showcase_examples::DetailSection;
mod alpha_controlled;
mod alpha_states;
mod angle_controls;
mod angle_direction;
mod hue_controls;
mod hue_sizes;
mod picker_formats;
mod picker_palette;
mod picker_precision;
mod picker_swatches;
pub fn sections(id: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident,$id:literal,$title:literal,$note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    match id {
        "alpha-slider" => vec![
            section!(
                alpha_controlled,
                "alpha-controlled",
                "Controlled opacity and keyboard steps",
                "Alpha is a finite number from 0 to 1, independent of the supplied RGB color. The checker represents transparency beneath the color ramp, and the visible percentage explains the current thumb value. value is read-only; declining on_value_change retains the accepted value. This example uses 5% arrow increments and 20% PageUp/PageDown increments. Home/End reach the endpoints; on_value_commit is a separate completed-interaction proposal."
            ),
            section!(
                alpha_states,
                "alpha-states",
                "Surfaces and unavailable states",
                "The same RGB ramp is shown over light and dark surroundings. Its internal two-tone checker remains distinct from an opaque gray gradient. Read-only preserves the value and blocks changes while remaining focusable; disabled also removes normal interaction. The passive graphic is bounded and clipped by the track, not a grid of focusable color cells. Pointer and touch retain the shared slider capture and cancellation behavior."
            ),
        ],
        "hue-slider" => vec![
            section!(
                hue_controls,
                "hue-controls",
                "Degrees, controlled value and keyboard steps",
                "Hue ranges from 0 to 360 degrees. Both endpoints represent red; the numeric value distinguishes them. The primary example shows a degree readout and color preview so selection does not rely only on color. This controlled example uses 15-degree arrows and 60-degree page steps; declining changes retains its source. Home/End reach exact bounds. Saturation/value/alpha belong to a separate full color model."
            ),
            section!(
                hue_sizes,
                "hue-sizes",
                "Sizes and disabled state",
                "Size changes the control's hit target and thumb geometry while retaining the same hue domain. Keep a descriptive name and visible readout in application UI. Disabled controls do not accept pointer or keyboard proposals. direction(Rtl) mirrors a horizontal slider; it does not rename or recolor the hue domain. Explicit styles can set a responsive width without replacing the controller."
            ),
        ],
        "angle-slider" => vec![
            section!(
                angle_controls,
                "angle-controls",
                "Steps, controlled values and keyboard input",
                "The circular control represents finite 0..=360 degrees. Its primary example contains only the dial and readable angle. This example changes step between 15 and 1 degrees on the same control, with 45-degree page steps. Arrow commands follow the shared slider contract; Home/End retain distinct 0/360 values at their common top position. Controlled callbacks propose changes and can refuse them. Pointer geometry uses the actual circle's center, not a horizontal drag approximation."
            ),
            section!(
                angle_direction,
                "angle-direction",
                "Direction, read-only and disabled controls",
                "LTR/RTL changes the circular pointer/arrow mapping. Compare equal initial values in the two directions. Read-only blocks changes and preserves the current angle; disabled prevents normal focus and interaction. The finite API uses the shared themed dial and size/style configuration; it does not add an arbitrary per-degree marks or custom pointer renderer. OS input acceptance remains separate from browser and native protocol checks."
            ),
        ],
        "color-picker" => vec![
            section!(
                picker_swatches,
                "picker-swatches",
                "With swatches",
                "Preset buttons commit a complete color atomically below the continuous picker. The hue/alpha trails and two-dimensional plane keep the same HSVA model; transparent colors use a real checker beneath the alpha ramp. Swatches are finite validated colors with current pressed/disabled state and accessible names."
            ),
            section!(
                picker_palette,
                "picker-swatches-only",
                "Swatches only and full width",
                "show_picker(false) removes the continuous controls while the finite palette fills its bounded parent. Seven swatches share the available width. The wrapper is capped at 420px and can shrink on narrow screens; it does not force a 420px overflow region. Selection updates the same controlled HSVA source."
            ),
            section!(
                picker_precision,
                "picker-precision",
                "Keyboard precision and accessibility labels",
                "Optional saturation and brightness sliders complement the two-dimensional plane. The four scalar controls use application-specific names and expose numeric values/value text. Pointer plane input proposes both coordinates together; keyboard precision controls avoid relying on color or pointer positioning alone. Source values remain full-precision HSVA until presentation formatting."
            ),
            section!(
                picker_formats,
                "picker-form-value",
                "Formatted form value",
                "The same accepted color is shown as HEXA, RGBA and HSLA so its channels and alpha are readable. form_name publishes a real successful-control input for an enclosing form, with the configured RGBA format. Display formatting does not replace the HSVA model or reduce its internal precision. Use ColorInput when users need an editable formatted string."
            ),
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(id: &str) -> Vec<(&'static str, &'static str)> {
    match id {
        "alpha-slider" => vec![
            ("Controlled opacity and steps", "#alpha-controlled"),
            ("Surfaces and states", "#alpha-states"),
        ],
        "hue-slider" => vec![
            ("Degrees and keyboard", "#hue-controls"),
            ("Sizes and disabled state", "#hue-sizes"),
        ],
        "angle-slider" => vec![
            ("Steps and controlled values", "#angle-controls"),
            ("Direction and states", "#angle-direction"),
        ],
        "color-picker" => vec![
            ("With swatches", "#picker-swatches"),
            ("Swatches only", "#picker-swatches-only"),
            ("Keyboard precision", "#picker-precision"),
            ("Formatted form value", "#picker-form-value"),
        ],
        _ => Vec::new(),
    }
}
