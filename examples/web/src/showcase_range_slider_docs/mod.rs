use super::showcase_examples::DetailSection;
mod advanced;
mod collisions;
mod controlled;
mod domain;
mod gap;
mod labels;
mod marks;
mod states;
pub fn sections(theme: sapid::Theme) -> Vec<DetailSection> {
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
    vec![
        section!(
            states,
            "range-states",
            "Live steps, read-only and disabled thumbs",
            "Each thumb keeps its own accessible name and identity. Arrow, Page and Home/End commands obey current steps and collision constraints, without swapping lower and upper controls. Toggle live configuration on this same accepted pair; a pending external pair is reconciled after the host packet. Read-only and disabled states block changes. An external readout is useful when thumb labels would compete for space."
        ),
        section!(
            controlled,
            "controlled",
            "Controlled range",
            "Both thumb values are proposed and accepted atomically as one pair."
        ),
        section!(
            gap,
            "range-gap",
            "Bounds, decimals and minimum gap",
            "The two values retain their identities and cannot cross the configured finite gap."
        ),
        section!(
            marks,
            "range-marks",
            "Marks and restricted range",
            "Both thumbs use the shared mark declaration and stay on allowed values while preserving the minimum gap."
        ),
        section!(
            advanced,
            "range-advanced",
            "Maximum range, inverted track and hidden marks",
            "maximumGap prevents widening beyond a finite span; inverted changes track coloring only, and hidden marks still participate in restricted snapping."
        ),
        section!(
            domain,
            "range-domain",
            "Independent visual domain",
            "domain controls the full visual track while min and max remain the actual selectable and accessible bounds."
        ),
        section!(
            labels,
            "range-labels",
            "Control labels and accessible value text",
            "Value labels appear during dragging by default, can remain visible, or can be disabled. The same bounded formatted text is exposed as aria-valuetext."
        ),
        section!(
            collisions,
            "range-collision",
            "Push, clamp or hold on overlap",
            "The default pushes the other thumb atomically when minimum or maximum range is reached. Clamp stops the moving thumb at its neighbour's gap boundary. Hold refuses an overlap proposal and keeps the entire accepted pair unchanged. Neither mode swaps thumb identities."
        ),
    ]
}
