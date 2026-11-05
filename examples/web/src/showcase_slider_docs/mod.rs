use super::showcase_examples::DetailSection;
mod bounds;
mod committed;
mod controlled;
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
            "slider-states",
            "Read-only, disabled and live keyboard steps",
            "Arrow keys use the current valid step, PageUp/PageDown use the page step, and Home/End reach bounds. Toggle five-unit versus one-unit steps on the same control. Read-only preserves focus and value; disabled prevents normal interaction. A separate visible percentage and bounded value_text explain units without covering the thumb in a narrow container. Invalid dynamic configurations retain the last accepted snapshot; pending external values are not interaction cursors."
        ),
        section!(
            controlled,
            "controlled",
            "Controlled value",
            "A Cage remains the accepted source; callbacks propose changes without creating a second mirrored value."
        ),
        section!(
            committed,
            "change-end",
            "Committed changes",
            "Use the commit callback for the final keyboard or drag value, independently from continuous updates."
        ),
        section!(
            bounds,
            "bounds-and-step",
            "Min, max, step and decimals",
            "Bounds and steps are finite values and are validated together before the live range changes."
        ),
        section!(
            marks,
            "marks",
            "Marks and restricted selection",
            "Up to 64 sorted marks may carry labels. Restricted sliders snap pointer input and keyboard commands to declared marks only."
        ),
    ]
}
